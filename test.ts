import { assertEquals } from "https://deno.land/std@0.152.0/testing/asserts.ts";

import { toRecords } from "./mod.ts";

Deno.test({
  name: "toSearchRecords",
  async fn() {
    const fixture = await Deno.readTextFile("./src/fixtures/example.md");
    const actual = await toRecords(fixture);
    assertEquals(actual, [
      {
        "kind": "heading",
        "position": 1,
        "hierarchy": [],
        "anchor": "an-example-markdown-file",
        "content": "An Example Markdown File",
      },
      {
        "kind": "paragraph",
        "position": 2,
        "hierarchy": [
          "An Example Markdown File",
        ],
        "anchor": "an-example-markdown-file",
        "content": "With some content here, including some  inline  code.",
      },
      {
        "kind": "heading",
        "position": 3,
        "hierarchy": [
          "An Example Markdown File",
        ],
        "anchor": "a-heading-at-level-2",
        "content": "A Heading at Level 2",
      },
      {
        "kind": "paragraph",
        "position": 4,
        "hierarchy": [
          "An Example Markdown File",
          "A Heading at Level 2",
        ],
        "anchor": "a-heading-at-level-2",
        "content":
          "Some more content that breaks across lines but is still a single paragraph. Some more content that breaks across lines but is still a single paragraph.",
      },
      {
        "kind": "code",
        "codeInfo": "ts",
        "position": 5,
        "hierarchy": [
          "An Example Markdown File",
          "A Heading at Level 2",
        ],
        "anchor": "a-heading-at-level-2",
        "content":
          '/**\n * With a code example inline\n */\n\nimport * as lib from "https://example.com/lib.ts";\n\nconsole.log(lib);\n',
      },
      {
        "kind": "paragraph",
        "position": 6,
        "hierarchy": [
          "An Example Markdown File",
          "A Heading at Level 2",
        ],
        "anchor": "a-heading-at-level-2",
        "content":
          "As well as another example here, which just uses indentation:",
      },
      {
        "kind": "code",
        "codeInfo": "",
        "position": 7,
        "hierarchy": [
          "An Example Markdown File",
          "A Heading at Level 2",
        ],
        "anchor": "a-heading-at-level-2",
        "content": 'console.log("hello world");\n',
      },
      {
        "kind": "heading",
        "position": 8,
        "hierarchy": [
          "An Example Markdown File",
        ],
        "anchor": "another-level-2-heading",
        "content": "Another Level 2 heading",
      },
      {
        "kind": "paragraph",
        "position": 9,
        "hierarchy": [
          "An Example Markdown File",
          "Another Level 2 heading",
        ],
        "anchor": "another-level-2-heading",
        "content": "With some content.",
      },
      {
        "kind": "heading",
        "position": 10,
        "hierarchy": [
          "An Example Markdown File",
          "Another Level 2 heading",
        ],
        "anchor": "a-level-3-heading-with-some--backticks",
        "content": "A Level 3 heading with some  backticks",
      },
      {
        "kind": "heading",
        "position": 11,
        "hierarchy": [
          "An Example Markdown File",
          "Another Level 2 heading",
          "A Level 3 heading with some  backticks",
        ],
        "anchor": "a-level-4-heading",
        "content": "A level 4 heading",
      },
      {
        "kind": "paragraph",
        "position": 12,
        "hierarchy": [
          "An Example Markdown File",
          "Another Level 2 heading",
          "A Level 3 heading with some  backticks",
          "A level 4 heading",
        ],
        "anchor": "a-level-4-heading",
        "content": "With some content.",
      },
      {
        "kind": "heading",
        "position": 13,
        "hierarchy": [
          "An Example Markdown File",
          "Another Level 2 heading",
        ],
        "anchor": "back-out-to-level-3",
        "content": "Back Out to Level 3",
      },
      {
        "kind": "paragraph",
        "position": 14,
        "hierarchy": [
          "An Example Markdown File",
          "Another Level 2 heading",
          "Back Out to Level 3",
        ],
        "anchor": "back-out-to-level-3",
        "content": "And even more content, and another:",
      },
      {
        "kind": "code",
        "codeInfo": "json",
        "position": 15,
        "hierarchy": [
          "An Example Markdown File",
          "Another Level 2 heading",
          "Back Out to Level 3",
        ],
        "anchor": "back-out-to-level-3",
        "content": '{\n  "code": "example"\n}\n',
      },
      {
        "kind": "heading",
        "position": 16,
        "hierarchy": [
          "An Example Markdown File",
        ],
        "anchor": "and-finally-level-2",
        "content": "And finally level 2",
      },
      {
        "kind": "paragraph",
        "position": 17,
        "hierarchy": [
          "An Example Markdown File",
          "And finally level 2",
        ],
        "anchor": "and-finally-level-2",
        "content": "Which is where we will end this.",
      },
    ]);
  },
});
