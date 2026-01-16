/* tslint:disable */
/* eslint-disable */

export function setup_colorpicker(): void;

export function start(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly start: () => void;
    readonly setup_colorpicker: () => void;
    readonly wasm_bindgen_e91775a654bf622e___closure__destroy___dyn_core_4a0f1e8823c288d9___ops__function__FnMut__web_sys_7b01aa9435b43fbe___features__gen_HtmlElement__HtmlElement____Output_______: (a: number, b: number) => void;
    readonly wasm_bindgen_e91775a654bf622e___convert__closures_____invoke___web_sys_7b01aa9435b43fbe___features__gen_HtmlElement__HtmlElement__alloc_62d3df3bb6c27ab___string__String__core_4a0f1e8823c288d9___option__Option_alloc_62d3df3bb6c27ab___string__String___core_4a0f1e8823c288d9___option__Option_alloc_62d3df3bb6c27ab___string__String______: (a: number, b: number, c: any, d: number, e: number, f: number, g: number, h: number, i: number) => void;
    readonly wasm_bindgen_e91775a654bf622e___convert__closures_____invoke___web_sys_7b01aa9435b43fbe___features__gen_HtmlElement__HtmlElement_____: (a: number, b: number, c: any) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
