/* tslint:disable */
/* eslint-disable */
/**
* @returns {Map}
*/
export function init(): Map;
/**
*/
export class CanvasContext {
  free(): void;
/**
*/
  offset: Offset;
/**
*/
  scale: number;
}
/**
*/
export class Map {
  free(): void;
/**
* @param {number} move_x
* @param {number} move_y
*/
  move_view(move_x: number, move_y: number): void;
/**
* @param {number} scale
*/
  set_scale(scale: number): void;
/**
* @returns {number}
*/
  get_scale(): number;
/**
*/
  update_canvas(): void;
/**
*/
  view: CanvasContext;
}
/**
*/
export class Offset {
  free(): void;
/**
*/
  x: number;
/**
*/
  y: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_offset_free: (a: number) => void;
  readonly __wbg_get_offset_x: (a: number) => number;
  readonly __wbg_set_offset_x: (a: number, b: number) => void;
  readonly __wbg_get_offset_y: (a: number) => number;
  readonly __wbg_set_offset_y: (a: number, b: number) => void;
  readonly __wbg_canvascontext_free: (a: number) => void;
  readonly __wbg_get_canvascontext_offset: (a: number) => number;
  readonly __wbg_set_canvascontext_offset: (a: number, b: number) => void;
  readonly __wbg_get_canvascontext_scale: (a: number) => number;
  readonly __wbg_set_canvascontext_scale: (a: number, b: number) => void;
  readonly __wbg_map_free: (a: number) => void;
  readonly __wbg_get_map_view: (a: number) => number;
  readonly __wbg_set_map_view: (a: number, b: number) => void;
  readonly map_move_view: (a: number, b: number, c: number) => void;
  readonly map_set_scale: (a: number, b: number) => void;
  readonly map_get_scale: (a: number) => number;
  readonly map_update_canvas: (a: number) => void;
  readonly init: () => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
