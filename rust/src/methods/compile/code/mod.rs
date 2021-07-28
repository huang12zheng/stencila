///! Functions etc for compiling programming languages in `CodeChunk` and `CodeExpression` nodes.
///!
///! Uses `tree-sitter` to parse source code into a abstract syntax tree which is then used to
///! derive properties of a `CodeAnalysis`.
use crate::graphs::{Resource, Triple};
use std::{collections::HashMap, sync::Mutex};
use tree_sitter::{Language, Parser, Query, QueryCursor};

#[cfg(feature = "compile-code-js")]
mod js;

#[cfg(feature = "compile-code-py")]
mod py;

#[cfg(feature = "compile-code-r")]
mod r;

/// Compile code in a particular language
pub fn compile(resource: &Resource, code: &str, language: &str) -> Vec<Triple> {
    let relations = match language {
        #[cfg(feature = "compile-code-js")]
        "js" | "javascript" => js::compile(code),
        #[cfg(feature = "compile-code-py")]
        "py" | "python" => py::compile(code),
        #[cfg(feature = "compile-code-r")]
        "r" => r::compile(code),
        _ => Vec::new(),
    };
    relations
        .into_iter()
        .map(|(relation, to)| (resource.clone(), relation, to))
        .collect()
}

/// A capture resulting from a `tree-sitter` query
pub(crate) struct Capture {
    #[allow(dead_code)]
    /// The index of the capture in the pattern
    index: u32,

    /// The name of the capture in the pattern
    name: String,

    /// The captured text
    text: String,
}

/// Convert a vector of `Capture`s into a
pub(crate) fn captures_as_args_map(captures: Vec<Capture>) -> HashMap<String, String> {
    let mut map = HashMap::new();

    let mut index = 0;
    let mut name = String::new();
    for capture in captures {
        match capture.name.as_str() {
            "arg" => {
                map.insert(index.to_string(), capture.text);
                index += 1;
            }
            "arg_name" => {
                name = capture.text;
            }
            "arg_value" => {
                map.insert(name.clone(), capture.text);
            }
            _ => {}
        }
    }

    map
}

/// Whether or not the text is quoted
pub(crate) fn is_quoted(text: &str) -> bool {
    (text.starts_with('"') && text.ends_with('"'))
        || (text.starts_with('\'') && text.ends_with('\''))
}

/// Remove single and double quotes from text
///
/// Useful for "unquoting" captured string literals.
pub(crate) fn remove_quotes(text: &str) -> String {
    text.replace(&['\"', '\''][..], "")
}

/// A compiler for a particular language
///
/// This simply encapsulates a `tree-sitter` usage pattern to
/// avoid repetitive boiler plate code in the language-specific sub-modules.
pub(crate) struct Compiler {
    /// The `tree-sitter` parser
    parser: Mutex<Parser>,

    /// The `tree-sitter` query
    query: Query,
}

impl Compiler {
    /// Create a new compiler for a language with a pre-defined query
    ///
    /// # Arguments
    ///
    /// - `language`: the `tree-sitter` language definition
    /// - `query`: the `tree-sitter` query definition
    fn new(language: Language, query: &str) -> Compiler {
        let mut parser = Parser::new();
        parser
            .set_language(language)
            .expect("Should be able to set language");
        let parser = Mutex::new(parser);

        let query = Query::new(language, query).expect("Query should compile");

        Compiler { parser, query }
    }

    /// Parse and query some code
    ///
    /// # Arguments
    ///
    /// - `code`: the code to parse
    ///
    /// # Returns
    ///
    /// A vector of `(pattern, captures)` enumerating the matches for
    /// patterns in the query.
    fn query(&self, code: &str) -> Vec<(usize, Vec<Capture>)> {
        let code = code.as_bytes();
        let tree = self
            .parser
            .lock()
            .expect("Unable to lock parser")
            .parse(code, None)
            .expect("Should be a tree result");
        let root = tree.root_node();

        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(&self.query, root, |node| {
            node.utf8_text(code).unwrap_or_default()
        });

        let capture_names = self.query.capture_names();
        matches
            .map(|query_match| {
                let pattern = query_match.pattern_index;
                let captures = query_match
                    .captures
                    .iter()
                    .map(|capture| Capture {
                        index: capture.index,
                        name: capture_names[capture.index as usize].clone(),
                        text: capture
                            .node
                            .utf8_text(code)
                            .expect("Should be able to get text")
                            .to_string(),
                    })
                    .collect();
                (pattern, captures)
            })
            .collect()
    }
}
