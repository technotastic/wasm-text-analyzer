/* tslint:disable */
/* eslint-disable */
export function analyze_text(text: string): any;
export function start(): void;
export class TextAnalysis {
  free(): void;
  constructor(text: string);
  word_count(): number;
  character_count(): number;
  sentence_count(): number;
  paragraph_count(): number;
  calculate_readability(): number;
  get_top_keywords(count: number): any;
  get_sentiment(): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_textanalysis_free: (a: number, b: number) => void;
  readonly textanalysis_new: (a: number, b: number) => number;
  readonly textanalysis_word_count: (a: number) => number;
  readonly textanalysis_character_count: (a: number) => number;
  readonly textanalysis_sentence_count: (a: number) => number;
  readonly textanalysis_paragraph_count: (a: number) => number;
  readonly textanalysis_calculate_readability: (a: number) => number;
  readonly textanalysis_get_top_keywords: (a: number, b: number) => any;
  readonly textanalysis_get_sentiment: (a: number) => number;
  readonly analyze_text: (a: number, b: number) => any;
  readonly start: () => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
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
