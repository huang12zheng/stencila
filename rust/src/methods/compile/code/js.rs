use super::{apply_tags, remove_quotes, Capture, Compiler};
use crate::{
    graphs::{resources, Relation, Resource},
    utils::path::merge,
};
use once_cell::sync::Lazy;
use std::path::Path;

/// Query patterns for JavaScript (and reused for TypeScript)
pub const PATTERNS: &str = r#"
(program (comment) @comment)

(import_statement
    source: (string) @module
)
(call_expression
    function: [(import)(identifier)] @function (#match? @function "^import|require$")
    arguments: (arguments . (string) @module)
)

(call_expression
    function: [
        (
            (identifier) @function (#match? @function "^readFile")
        )
        (
            member_expression
                object: (_)
                property: (property_identifier)  @function (#match? @function "^readFile")
        )
    ]
    arguments: (arguments . (string) @path)
)

(call_expression
    function: [
        (
            (identifier) @function (#match? @function "^writeFile")
        )
        (
            member_expression
                object: (_)
                property: (property_identifier)  @function (#match? @function "^writeFile")
        )
    ]
    arguments: (arguments . (string) @path)
)

(program
    [
        (expression_statement
            (assignment_expression
                left: (identifier) @name
                right: (_) @value
            )
        )
        (variable_declaration
            (variable_declarator
                name: (identifier) @name
                .
                value: (_) @value
            )
        )
        (lexical_declaration
            (variable_declarator
                name: (identifier) @name
                .
                value: (_) @value
            )
        )
        (export_statement
            declaration: (lexical_declaration
                (variable_declarator
                    name: (identifier) @name
                    .
                    value: (_) @value
                )
            )
        )
    ]
)
(program
    [
        (function_declaration
            name: (identifier) @name
        )
        (export_statement
            declaration: (function_declaration
                name: (identifier) @name
            )
        )
    ]
)

((identifier) @identifer)
"#;

// Handle a pattern match
pub(crate) fn handle_patterns(
    path: &Path,
    code: &[u8],
    pattern: &usize,
    captures: &[Capture],
) -> Option<(Relation, Resource)> {
    match pattern {
        1 | 2 => {
            // Imports a module using `import` or `require`
            let capture = match pattern {
                1 => &captures[0],
                2 => &captures[1],
                _ => unreachable!(),
            };
            let range = capture.range;
            let module = remove_quotes(&capture.text.clone());
            let object = if module.starts_with("./") {
                resources::file(&merge(path, &[&module, ".js"].concat()))
            } else {
                resources::module("javascript", &module)
            };
            Some((Relation::Use(range), object))
        }
        3 => {
            // Reads a file
            Some((
                Relation::Read(captures[1].range),
                resources::file(&merge(path, remove_quotes(&captures[1].text))),
            ))
        }
        4 => {
            // Writes a file
            Some((
                Relation::Write(captures[1].range),
                resources::file(&merge(path, remove_quotes(&captures[1].text))),
            ))
        }
        5 | 6 => {
            // Assigns a symbol at the top level of the module
            let range = captures[0].range;
            let name = captures[0].text.clone();
            let kind = match pattern {
                5 => match captures[1].node.kind() {
                    "true" | "false" => "Boolean",
                    "number" => "Number",
                    "string" => "String",
                    "array" => "Array",
                    "object" => "Object",
                    "arrow_function" => "Function",
                    _ => "",
                },
                6 => "Function",
                _ => unreachable!(),
            };
            Some((
                Relation::Assign(range),
                resources::symbol(path, &name, kind),
            ))
        }
        7 => {
            // Uses an identifier assigned elsewhere
            let node = captures[0].node;
            let range = captures[0].range;
            let symbol = captures[0].text.clone();

            let mut parent = node.parent();
            while let Some(parent_node) = parent {
                match parent_node.kind() {
                    // Skip identifiers in import statements
                    // Could just skip children of `import_statement`, but specifying others in tree
                    // results in an earlier return while walking up tree.
                    "import_statement" | "import_clause" | "named_imports" | "import_specifier" => {
                        return None
                    }
                    // Skip identifiers that are the `name` of a declaration
                    "variable_declarator" => {
                        if Some(node) == parent_node.child_by_field_name("name") {
                            return None;
                        }
                    }
                    // Skip identifiers that are the `left` of an assignment
                    "assignment_expression" => {
                        if Some(node) == parent_node.child_by_field_name("left") {
                            return None;
                        }
                    }
                    // Skip any identifier used in a function
                    "function_declaration" | "arrow_function" | "formal_parameters" => {
                        return None;
                    }
                    // Skip identifiers that are the `left` of a for in loop, or that refer to it
                    // within the loop
                    "for_in_statement" => {
                        if let Some(left) = parent_node.child_by_field_name("left") {
                            if left == node || left.utf8_text(code).unwrap() == symbol {
                                return None;
                            }
                        }
                    }
                    // Skip identifiers that are the `name` of the variable in a for loop, or
                    // that refer to it within the loop
                    "for_statement" => {
                        if let Some(name) = parent_node
                            .child_by_field_name("initializer")
                            .and_then(|node| node.named_child(0)) // variable_declarator
                            .and_then(|node| node.child_by_field_name("name"))
                        {
                            if name == node || name.utf8_text(code).unwrap() == symbol {
                                return None;
                            }
                        }
                    }
                    _ => {}
                }
                parent = parent_node.parent();
            }

            Some((Relation::Use(range), resources::symbol(path, &symbol, "")))
        }
        _ => None,
    }
}

/// Compiler for JavaScript
static COMPILER: Lazy<Compiler> =
    Lazy::new(|| Compiler::new(tree_sitter_javascript::language(), PATTERNS));

/// Compile some JavaScript code
pub fn compile(path: &Path, code: &str) -> Vec<(Relation, Resource)> {
    let code = code.as_bytes();
    let tree = COMPILER.parse(code);
    let matches = COMPILER.query(code, &tree);

    let relations = matches
        .iter()
        .filter_map(|(pattern, capture)| handle_patterns(path, code, pattern, capture))
        .collect();

    apply_tags(path, "javascript", matches, 0, relations)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tests::snapshot_content;
    use insta::assert_json_snapshot;
    use std::path::PathBuf;

    #[test]
    fn js_fragments() {
        snapshot_content("fragments/js/*.js", |path, code| {
            assert_json_snapshot!(compile(&PathBuf::from(path), code));
        });
    }
}