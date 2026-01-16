/* tslint:disable */
/* eslint-disable */

export function setup_textdisplay(): void;

export function setup_textinput(): void;

export function start(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly start: () => void;
    readonly setup_textdisplay: () => void;
    readonly setup_textinput: () => void;
    readonly wasm_bindgen_5246dbd48bac81a0___closure__destroy___dyn_core_4a0f1e8823c288d9___ops__function__FnMut__js_sys_c1271839d7171681___Array____Output_______: (a: number, b: number) => void;
    readonly wasm_bindgen_5246dbd48bac81a0___closure__destroy___dyn_core_4a0f1e8823c288d9___ops__function__FnMut__wasm_bindgen_5246dbd48bac81a0___JsValue____Output_______: (a: number, b: number) => void;
    readonly wasm_bindgen_5246dbd48bac81a0___closure__destroy___dyn_core_4a0f1e8823c288d9___ops__function__FnMut__web_sys_f4c1a62b567513ed___features__gen_HtmlElement__HtmlElement____Output_______: (a: number, b: number) => void;
    readonly wasm_bindgen_5246dbd48bac81a0___convert__closures_____invoke___web_sys_f4c1a62b567513ed___features__gen_HtmlElement__HtmlElement__alloc_62d3df3bb6c27ab___string__String__core_4a0f1e8823c288d9___option__Option_alloc_62d3df3bb6c27ab___string__String___core_4a0f1e8823c288d9___option__Option_alloc_62d3df3bb6c27ab___string__String______: (a: number, b: number, c: any, d: number, e: number, f: number, g: number, h: number, i: number) => void;
    readonly wasm_bindgen_5246dbd48bac81a0___convert__closures_____invoke___js_sys_c1271839d7171681___Array_____: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_5246dbd48bac81a0___convert__closures_____invoke___core_4a0f1e8823c288d9___result__Result_____wasm_bindgen_5246dbd48bac81a0___JsValue__: (a: number, b: number) => [number, number];
    readonly wasm_bindgen_5246dbd48bac81a0___convert__closures_____invoke___wasm_bindgen_5246dbd48bac81a0___JsValue_____: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_5246dbd48bac81a0___convert__closures_____invoke___web_sys_f4c1a62b567513ed___features__gen_HtmlElement__HtmlElement_____: (a: number, b: number, c: any) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
