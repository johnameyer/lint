use std::{cmp::max, collections::HashSet};

use tree_sitter::Node;

pub fn print2(source_code: &[u8], node: &Node) -> String {
    return prettyprint(&transform(source_code, node)) + "\n";
}

enum FormatNode {
    Content(Box<str>),
    Lines(Vec<FormatNode>),
    Block(Vec<FormatNode>),
    Group(Vec<FormatNode>),
    Space,
    AllowNewline, // How would we determine in `prettyprint` that a newline exists?
    Newline,
    Empty,
}

fn transform(source_code: &[u8], node: &Node) -> FormatNode {
    let node_type = node.grammar_name();

    let block_elements = HashSet::from([
        "class_body",
        "enum_body",
        "interface_body",
        "block",
        "constructor_body",
    ]); // children are indented unless there is no non-bracket element

    let spaced_nodes = HashSet::from([
        "class_declaration",
        "constructor_declaration",
        "static_initializer",
        "method_declaration",
        "formal_parameter",
        "return_statement",
        "throw_statement",
        "object_creation_expression",
        "field_declaration",
        "local_variable_declaration",
        "variable_declarator",
        "object_creation_expression",
        "binary_expression",
        "ternary_expression",
        "instanceof_expression",
        "package_declaration",
        "enum_declaration",
        "record_declaration",
        "import_declaration",
        "interface_declaration",
        "modifiers",
        "assignment_expression",
        "throws",
        "if_statement",
        "try_statement",
        "for_statement",
        "do_statement",
        "while_statement",
        "finally_clause",
        "catch_formal_parameter",
        "resource",
        "element_value_pair",
        "try_with_resources_statement",
    ]); // items to add spaces between

    let internally_spaced = HashSet::from([
        "type_parameters",
        "type_arguments",
        "argument_list",
        "formal_parameters",
        "annotation_argument_list",
        "element_value_array_initializer",
        "catch_clause",
        "enhanced_for_statement",
    ]);

    let syntax_elements = HashSet::from([
        // "(",
        ")",
        ".",
        ",",
        ";",
        "argument_list",
        "formal_parameters",
        "catch",
    ]);

    if node.child_count() > 0 {
        return if node_type == "program" {
            FormatNode::Lines(
                (0..node.child_count())
                    .map(|index| transform(source_code, &node.child(index).unwrap()))
                    .collect(),
            )
        } else if block_elements.contains(node_type) {
            FormatNode::Block(
                (0..node.child_count())
                    .map(|index| transform(source_code, &node.child(index).unwrap()))
                    .collect(),
            )
        } else if spaced_nodes.contains(node_type) {
            FormatNode::Group(
                (0..node.child_count())
                    .map(|index| node.child(index).unwrap())
                    .enumerate()
                    .flat_map(|(i, node)| {
                        if i == 0 || syntax_elements.contains(node.grammar_name()) {
                            vec![transform(source_code, &node)]
                        } else {
                            vec![FormatNode::Space, transform(source_code, &node)]
                        }
                    })
                    .collect(),
            )
        } else if internally_spaced.contains(node_type) {
            FormatNode::Group(
                (0..node.child_count())
                    .map(|index| node.child(index).unwrap())
                    .enumerate()
                    .flat_map(|(i, child)| {
                        if i < 2
                            || i == max(1, node.child_count()) - 1
                            || syntax_elements.contains(child.grammar_name())
                        {
                            vec![transform(source_code, &child)]
                        } else {
                            vec![FormatNode::Space, transform(source_code, &child)]
                        }
                    })
                    .collect(),
            )
        } else {
            // panic!("Hey!");
            FormatNode::Group(
                (0..node.child_count())
                    .map(|index| transform(source_code, &node.child(index).unwrap()))
                    .collect(),
            )
        };
    } else {
        if let Ok(content) = node.utf8_text(source_code) {
            return FormatNode::Content(content.into());
        }
    }

    return FormatNode::Empty;
}

const INDENT: &str = "    ";

fn prettyprint(formatted: &FormatNode) -> String {
    return match formatted {
        FormatNode::Content(content) => content.to_string(),
        FormatNode::Lines(elements) => elements
            .iter()
            .map(|element| prettyprint(element))
            .collect::<Vec<String>>()
            .join("\n"),
        FormatNode::Group(elements) => elements
            .iter()
            .map(|element| prettyprint(element))
            .collect(),
        FormatNode::Block(elements) => {
            if elements.len() > 2 {
                elements
                    .iter()
                    .map(|element| prettyprint(element))
                    .map(|str| {
                        if str == "{" || str == "}" {
                            str
                        } else {
                            str.lines()
                                .map(|line| INDENT.to_owned() + line)
                                .collect::<Vec<String>>()
                                .join("\n")
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("\n")
            } else {
                "{ }".to_owned()
            }
        }
        FormatNode::Space => " ".to_owned(),
        _ => "".to_string(),
    };
}
