use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Expr, Lit, Meta, MetaNameValue};

#[proc_macro_derive(WebComponent, attributes(web_component))]
pub fn derive_web_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let struct_name = &input.ident;
    let struct_name_str = struct_name.to_string();
    
    let mut html_tag_name: Option<String> = None;
    let mut observed_attributes: Option<Vec<String>> = None;
    
    for attr in &input.attrs {
        if attr.path().is_ident("web_component") {
            if let Meta::List(list) = &attr.meta {
                let tokens = list.tokens.clone();
                let nested = syn::parse::Parser::parse_str(
                    syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
                    &tokens.to_string()
                ).expect("Failed to parse web_component attributes");
                
                for meta in nested {
                    if let Meta::NameValue(MetaNameValue { path, value, .. }) = meta {
                        if path.is_ident("name") {
                            if let Expr::Lit(expr) = value {
                                if let Lit::Str(lit_str) = expr.lit {
                                    html_tag_name = Some(lit_str.value());
                                }
                            }
                        } else if path.is_ident("observed_attributes") {
                            if let Expr::Array(array) = value {
                                observed_attributes = Some(
                                    array.elems.iter().filter_map(|elem| {
                                        if let Expr::Lit(expr) = elem {
                                            if let Lit::Str(lit_str) = &expr.lit {
                                                Some(lit_str.value())
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }).collect()
                                );
                            }
                        } else {
                            panic!("Unknown attribute '{}' in web_component", path.get_ident().unwrap());
                        }
                    }
                }
            }
        }
    }
    
    let html_tag_name = html_tag_name.expect("#[web_component(name = \"...\")] is required");
    let observed_attrs = observed_attributes.unwrap_or_default();
    
    let registry_name = Ident::new(
        &format!("{}_INSTANCES", struct_name_str.to_uppercase()),
        Span::call_site()
    );
    let setup_fn_name = Ident::new(
        &format!("setup_{}", struct_name_str.to_lowercase()),
        Span::call_site()
    );
    let js_class_name = format!("Rust{}Component", struct_name_str);
    
    let observed_attrs_js = if observed_attrs.is_empty() {
        String::from("[]")
    } else {
        format!("[{}]", observed_attrs.iter().map(|attr| format!("'{}'", attr)).collect::<Vec<_>>().join(", "))
    };
    
    let class_definition = format!(
        r#"
        class {js_class} extends HTMLElement {{
            constructor() {{
                super();
                window.__wasm_attach_{js_class}(this);
            }}

            connectedCallback() {{
                window.__wasm_connected_{js_class}(this);
            }}

            disconnectedCallback() {{
                window.__wasm_disconnected_{js_class}(this);
            }}

            adoptedCallback() {{
                window.__wasm_adopted_{js_class}(this);
            }}

            attributeChangedCallback(name, oldValue, newValue) {{
                window.__wasm_attribute_changed_{js_class}(this, name, oldValue, newValue);
            }}

            static get observedAttributes() {{
                return {observed_attrs};
            }}
        }}

        customElements.define('{tag_name}', {js_class});
        "#,
        js_class = js_class_name,
        observed_attrs = observed_attrs_js,
        tag_name = html_tag_name
    );
    
    let expanded = quote! {
        thread_local! {
            static #registry_name: std::cell::RefCell<std::collections::HashMap<u32, #struct_name>> 
                = std::cell::RefCell::new(std::collections::HashMap::new());
        }

        impl #struct_name {
            pub fn setup() {
                #setup_fn_name();
            }

            pub fn with_instance<F, R>(id: u32, f: F) -> Option<R>
            where
                F: FnOnce(&mut #struct_name) -> R,
            {
                #registry_name.with(|registry| {
                    registry.borrow_mut().get_mut(&id).map(f)
                })
            }

            pub fn with_element<F, R>(element: &web_sys::HtmlElement, f: F) -> Option<R>
            where
                F: FnOnce(&mut #struct_name) -> R,
            {
                let id = element
                    .get_attribute("data-wasm-id")
                    .and_then(|s| s.parse().ok())?;
                Self::with_instance(id, f)
            }

            fn __register(id: u32, instance: #struct_name) {
                #registry_name.with(|registry| {
                    registry.borrow_mut().insert(id, instance);
                });
            }

            fn __unregister(id: u32) {
                #registry_name.with(|registry| {
                    registry.borrow_mut().remove(&id);
                });
            }
        }

        #[wasm_bindgen::prelude::wasm_bindgen]
        pub fn #setup_fn_name() {
            use wasm_bindgen::JsCast;
            use wasm_bindgen::prelude::*;
            
            let window = web_sys::window().expect("no window exists");
            
            // Attach callback (called from constructor)
            let attach_callback = wasm_bindgen::closure::Closure::wrap(Box::new(|element: web_sys::HtmlElement| {
                let id = rust_web_component::next_id();
                element.set_attribute("data-wasm-id", &id.to_string())
                    .expect("Failed to set wasm-id attribute");
                
                let mut component = #struct_name::new();
                <#struct_name as rust_web_component::WebComponent>::attach(&mut component, &element);
                #struct_name::__register(id, component);
            }) as Box<dyn FnMut(web_sys::HtmlElement)>);
            
            // Connected callback
            let connected_callback = wasm_bindgen::closure::Closure::wrap(Box::new(|element: web_sys::HtmlElement| {
                #struct_name::with_element(&element, |comp| {
                    <#struct_name as rust_web_component::WebComponent>::connected(comp);
                });
            }) as Box<dyn FnMut(web_sys::HtmlElement)>);
            
            // Disconnected callback
            let disconnected_callback = wasm_bindgen::closure::Closure::wrap(Box::new(|element: web_sys::HtmlElement| {
                if let Some(id) = element.get_attribute("data-wasm-id").and_then(|s| s.parse::<u32>().ok()) {
                    #struct_name::with_instance(id, |comp| {
                        <#struct_name as rust_web_component::WebComponent>::disconnected(comp);
                    });
                    #struct_name::__unregister(id);
                }
            }) as Box<dyn FnMut(web_sys::HtmlElement)>);
            
            // Adopted callback
            let adopted_callback = wasm_bindgen::closure::Closure::wrap(Box::new(|element: web_sys::HtmlElement| {
                #struct_name::with_element(&element, |comp| {
                    <#struct_name as rust_web_component::WebComponent>::adopted(comp);
                });
            }) as Box<dyn FnMut(web_sys::HtmlElement)>);
            
            // Attribute changed callback
            let attribute_changed_callback = wasm_bindgen::closure::Closure::wrap(Box::new(|element: web_sys::HtmlElement, name: String, old: Option<String>, new: Option<String>| {
                #struct_name::with_element(&element, |comp| {
                    <#struct_name as rust_web_component::WebComponent>::attribute_changed(
                        comp,
                        &name,
                        old.as_deref(),
                        new.as_deref()
                    );
                });
            }) as Box<dyn FnMut(web_sys::HtmlElement, String, Option<String>, Option<String>)>);
            
            // Register callbacks on window
            let js_class_name = #js_class_name;
            
            js_sys::Reflect::set(
                &window,
                &JsValue::from_str(&format!("__wasm_attach_{}", js_class_name)),
                attach_callback.as_ref().unchecked_ref()
            ).unwrap();
            
            js_sys::Reflect::set(
                &window,
                &JsValue::from_str(&format!("__wasm_connected_{}", js_class_name)),
                connected_callback.as_ref().unchecked_ref()
            ).unwrap();
            
            js_sys::Reflect::set(
                &window,
                &JsValue::from_str(&format!("__wasm_disconnected_{}", js_class_name)),
                disconnected_callback.as_ref().unchecked_ref()
            ).unwrap();
            
            js_sys::Reflect::set(
                &window,
                &JsValue::from_str(&format!("__wasm_adopted_{}", js_class_name)),
                adopted_callback.as_ref().unchecked_ref()
            ).unwrap();
            
            js_sys::Reflect::set(
                &window,
                &JsValue::from_str(&format!("__wasm_attribute_changed_{}", js_class_name)),
                attribute_changed_callback.as_ref().unchecked_ref()
            ).unwrap();
            
            attach_callback.forget();
            connected_callback.forget();
            disconnected_callback.forget();
            adopted_callback.forget();
            attribute_changed_callback.forget();
            
            // Define the custom element
            rust_web_component::eval_js(#class_definition);
        }
    };
    
    TokenStream::from(expanded)
}