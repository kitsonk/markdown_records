use comrak::nodes::AstNode;
use comrak::nodes::NodeValue;
use comrak::parse_document;
use comrak::Anchorizer;
use comrak::Arena;
use comrak::ComrakOptions;
use serde::Serialize;
use std::cell::RefCell;
use std::cell::RefMut;
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase", tag = "kind", content = "codeInfo")]
enum MarkdownRecordKind {
  Heading,
  Paragraph,
  Code(String),
}

impl Default for MarkdownRecordKind {
  fn default() -> Self {
    Self::Paragraph
  }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct MarkdownRecord {
  #[serde(flatten)]
  kind: MarkdownRecordKind,
  position: u32,
  hierarchy: Vec<String>,
  anchor: Option<String>,
  content: String,
}

#[derive(Debug, Default)]
struct State {
  anchorizer: Anchorizer,
  anchor: Option<String>,
  depth: u32,
  position: u32,
  stack: Vec<String>,
}

fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &mut F)
where
  F: FnMut(&'a AstNode<'a>),
{
  f(node);
  for c in node.children() {
    iter_nodes(c, f);
  }
}

fn parse_paragraph<'a>(
  state: RefMut<State>,
  mut records: RefMut<Vec<MarkdownRecord>>,
  node: &'a AstNode<'a>,
) {
  let mut content = Vec::<String>::new();
  iter_nodes(node, &mut |child| match &child.data.borrow().value {
    NodeValue::Text(text) => {
      content.push(String::from_utf8(text.clone()).unwrap());
    }
    NodeValue::Code(code) => {
      content.push(String::from_utf8(code.literal.clone()).unwrap());
    }
    _ => (),
  });
  let content = content.join(" ");
  records.push(MarkdownRecord {
    kind: MarkdownRecordKind::Paragraph,
    position: state.position,
    hierarchy: state.stack.clone(),
    anchor: state.anchor.clone(),
    content,
  })
}

fn parse_header<'a>(
  mut state: RefMut<State>,
  mut records: RefMut<Vec<MarkdownRecord>>,
  node: &'a AstNode<'a>,
) {
  let mut content = Vec::<String>::new();
  for child in node.children() {
    match &child.data.borrow().value {
      NodeValue::Text(text) => {
        content.push(String::from_utf8(text.clone()).unwrap());
      }
      NodeValue::Code(code) => {
        content.push(String::from_utf8(code.literal.clone()).unwrap());
      }
      _ => (),
    }
  }
  let content = content.join(" ");
  let hierarchy = state.stack.clone();
  let anchor = Some(state.anchorizer.anchorize(content.clone()));
  state.anchor = anchor.clone();
  state.stack.push(content.clone());
  records.push(MarkdownRecord {
    content,
    anchor,
    hierarchy,
    position: state.position,
    kind: MarkdownRecordKind::Heading,
  });
}

fn parse(markdown: &str) -> Vec<MarkdownRecord> {
  let arena = Arena::new();
  let options = ComrakOptions {
    ..Default::default()
  };
  let root = parse_document(&arena, markdown, &options);
  let records = RefCell::new(Vec::new());
  let state = RefCell::new(State::default());
  for child in root.children() {
    match &child.data.borrow().value {
      NodeValue::Heading(heading) => {
        let mut state = state.borrow_mut();
        while state.stack.len() as u32 >= heading.level {
          state.stack.pop();
        }
        state.depth = heading.level;
        state.position += 1;
        parse_header(state, records.borrow_mut(), child)
      }
      NodeValue::Paragraph => {
        let mut state = state.borrow_mut();
        state.position += 1;
        parse_paragraph(state, records.borrow_mut(), child);
      }
      NodeValue::CodeBlock(code_block) => {
        let info = String::from_utf8(code_block.info.clone()).unwrap();
        let content = String::from_utf8(code_block.literal.clone()).unwrap();
        let mut state = state.borrow_mut();
        state.position += 1;
        records.borrow_mut().push(MarkdownRecord {
          kind: MarkdownRecordKind::Code(info),
          content,
          position: state.position,
          hierarchy: state.stack.clone(),
          anchor: state.anchor.clone(),
        })
      }
      _ => (),
    }
  }
  records.into_inner()
}

#[wasm_bindgen(js_name = "toRecords")]
pub fn to_records(markdown: String) -> Result<JsValue, js_sys::Error> {
  JsValue::from_serde(&parse(&markdown))
    .map_err(|err| js_sys::Error::new(&err.to_string()))
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_FIXTURE: &str = include_str!("./fixtures/example.md");

  #[test]
  fn test_to_search_records() {
    let actual = parse(EXAMPLE_FIXTURE);
    assert_eq!(
      serde_json::json!(actual),
      serde_json::json!([
        {
          "kind": "heading",
          "position": 1,
          "hierarchy": [],
          "anchor": "an-example-markdown-file",
          "content": "An Example Markdown File"
        },
        {
          "kind": "paragraph",
          "position": 2,
          "hierarchy": [
            "An Example Markdown File"
          ],
          "anchor": "an-example-markdown-file",
          "content": "With some content here, including some  inline  code."
        },
        {
          "kind": "heading",
          "position": 3,
          "hierarchy": [
            "An Example Markdown File"
          ],
          "anchor": "a-heading-at-level-2",
          "content": "A Heading at Level 2"
        },
        {
          "kind": "paragraph",
          "position": 4,
          "hierarchy": [
            "An Example Markdown File",
            "A Heading at Level 2"
          ],
          "anchor": "a-heading-at-level-2",
          "content": "Some more content that breaks across lines but is still a single paragraph. Some more content that breaks across lines but is still a single paragraph."
        },
        {
          "kind": "code",
          "codeInfo": "ts",
          "position": 5,
          "hierarchy": [
            "An Example Markdown File",
            "A Heading at Level 2"
          ],
          "anchor": "a-heading-at-level-2",
          "content": "/**\n * With a code example inline\n */\n\nimport * as lib from \"https://example.com/lib.ts\";\n\nconsole.log(lib);\n"
        },
        {
          "kind": "paragraph",
          "position": 6,
          "hierarchy": [
            "An Example Markdown File",
            "A Heading at Level 2"
          ],
          "anchor": "a-heading-at-level-2",
          "content": "As well as another example here, which just uses indentation:"
        },
        {
          "kind": "code",
          "codeInfo": "",
          "position": 7,
          "hierarchy": [
            "An Example Markdown File",
            "A Heading at Level 2"
          ],
          "anchor": "a-heading-at-level-2",
          "content": "console.log(\"hello world\");\n"
        },
        {
          "kind": "heading",
          "position": 8,
          "hierarchy": [
            "An Example Markdown File"
          ],
          "anchor": "another-level-2-heading",
          "content": "Another Level 2 heading"
        },
        {
          "kind": "paragraph",
          "position": 9,
          "hierarchy": [
            "An Example Markdown File",
            "Another Level 2 heading"
          ],
          "anchor": "another-level-2-heading",
          "content": "With some content."
        },
        {
          "kind": "heading",
          "position": 10,
          "hierarchy": [
            "An Example Markdown File",
            "Another Level 2 heading"
          ],
          "anchor": "a-level-3-heading-with-some--backticks",
          "content": "A Level 3 heading with some  backticks"
        },
        {
          "kind": "heading",
          "position": 11,
          "hierarchy": [
            "An Example Markdown File",
            "Another Level 2 heading",
            "A Level 3 heading with some  backticks"
          ],
          "anchor": "a-level-4-heading",
          "content": "A level 4 heading"
        },
        {
          "kind": "paragraph",
          "position": 12,
          "hierarchy": [
            "An Example Markdown File",
            "Another Level 2 heading",
            "A Level 3 heading with some  backticks",
            "A level 4 heading"
          ],
          "anchor": "a-level-4-heading",
          "content": "With some content."
        },
        {
          "kind": "heading",
          "position": 13,
          "hierarchy": [
            "An Example Markdown File",
            "Another Level 2 heading"
          ],
          "anchor": "back-out-to-level-3",
          "content": "Back Out to Level 3"
        },
        {
          "kind": "paragraph",
          "position": 14,
          "hierarchy": [
            "An Example Markdown File",
            "Another Level 2 heading",
            "Back Out to Level 3"
          ],
          "anchor": "back-out-to-level-3",
          "content": "And even more content, and another:"
        },
        {
          "kind": "code",
          "codeInfo": "json",
          "position": 15,
          "hierarchy": [
            "An Example Markdown File",
            "Another Level 2 heading",
            "Back Out to Level 3"
          ],
          "anchor": "back-out-to-level-3",
          "content": "{\n  \"code\": \"example\"\n}\n"
        },
        {
          "kind": "heading",
          "position": 16,
          "hierarchy": [
            "An Example Markdown File"
          ],
          "anchor": "and-finally-level-2",
          "content": "And finally level 2"
        },
        {
          "kind": "paragraph",
          "position": 17,
          "hierarchy": [
            "An Example Markdown File",
            "And finally level 2"
          ],
          "anchor": "and-finally-level-2",
          "content": "Which is where we will end this."
        }
      ])
    );
  }
}
