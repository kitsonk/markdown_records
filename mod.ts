import { instantiate } from "./lib/markdown_records.generated.js";

let wasm: Awaited<ReturnType<typeof instantiate>> | undefined;

export interface MarkdownRecordBase {
  /** The kind of the markdown record record. */
  kind: string;
  /** The position within the document, with each record incrementing the
   * position. */
  position: number;
  /** The set of headings that this entry is a child of. */
  hierarchy: string[];
  /** The nearest heading anchor which includes this record. This can be used
   * to generate a link to the record. */
  anchor?: string;
  /** The string content of the record. */
  content: string;
}

export interface MarkdownHeadingContent extends MarkdownRecordBase {
  kind: "paragraph" | "heading";
}

export interface MarkdownCode extends MarkdownRecordBase {
  kind: "code";
  /** The information, if present that indicates how the code example should be
   * interpreted.  For example `"yaml"` or `"ts"`. */
  codeInfo: string;
}

/** A record produced from parsing markdown. */
export type MarkdownRecord = MarkdownCode | MarkdownHeadingContent;

/** Convert a markdown string into a set of records which can be used to feed
 * to a search engine. */
export async function toRecords(markdown: string): Promise<MarkdownRecord[]> {
  wasm = wasm ?? await instantiate();
  return wasm.toRecords(markdown);
}
