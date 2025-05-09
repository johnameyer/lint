use std::collections::HashSet;

use tree_sitter::Node;

pub fn print2(source_code: &[u8], node: &Node) -> String {
    println!("{:?}", transform2(source_code, node));
    return prettyprint(&transform2(source_code, node), false) + "\n";
}

#[derive(Debug)]
enum FormatNode {
    Content(Box<str>),
    Group(Vec<FormatNode>),
    // Indent(Vec<FormatNode>),
    Indent(Box<FormatNode>),
    Space,
    // AllowNewline, // How would we determine in `prettyprint` that a newline exists?
    Wrap {
        wrap_with_indent: bool,
        or_space: bool,
    }, // Or should each be separate types
    // WrapIfParent
    Newline,
    Empty,
}

struct FormatContainer {
    children: Vec<FormatNode>,
}

struct FormatArguments {
    space: bool,
    newline: bool,
}

// stack

// parent: String
// next: String

// Node matchers emit to stack?
// pop off created groups at conclusion of element

fn transform2(source_code: &[u8], node: &Node) -> FormatNode {
    let block_elements = HashSet::from([
        "class_body",
        "enum_body",
        "interface_body",
        "block",
        "constructor_body",
    ]); // children are indented unless there is no non-bracket element

    let unconditional_space = HashSet::from([
        "assignment_expression",
        "binary_expression",
        "ternary_expression",
        "instanceof_expression",
        "lambda_expression"
    ]);

    let spaced_nodes = HashSet::from([
        "class_declaration",
        "enum_declaration",
        "record_declaration",
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
        "package_declaration",
        "import_declaration",
        "interface_declaration",
        "modifiers",
        "throws",
        "if_statement",
        "try_statement",
        "for_statement",
        "do_statement",
        "while_statement",
        "finally_clause",
        "catch_formal_parameter",
        "resource",
        "resource_specification",
        "element_value_pair",
        "try_with_resources_statement",
        "type_parameters",
        "type_arguments",
        // "argument_list", // specifically handled with wrap
        "inferred_parameters",
        "formal_parameters",
        "annotation_argument_list",
        "element_value_array_initializer",
        "catch_clause",
        "enhanced_for_statement",
        // TODO superclasses
        "super_interfaces", // TODO test
        "implements",       // TODO test
    ]); // items to add spaces between

    // let newline_before = HashSet::from([
    // ]);

    let newline_after = HashSet::from([
        "if_statement",
        "try_statement",
        "for_statement",
        "do_statement",
        "while_statement",
        "package_declaration",
    ]);

    let conditional_newline_after = HashSet::from([
        "class_declaration",
        "enum_declaration",
        "record_declaration",
    ]);

    // TODO implements

    let no_space_before = HashSet::from([
        ">",
        ")",
        ".",
        ",",
        ";",
        "argument_list",
        "formal_parameters",
        "catch",
    ]);

    // TODO generic method call

    let no_space_after = HashSet::from([
        "(", "<", // unless as binary_operator
    ]);

    let add_wrap_before = HashSet::from(["."]);

    let stack_pushers = HashSet::from(["(", "{", "["]);

    let stack_poppers = HashSet::from([")", "}", "]"]);

    let node_type = node.grammar_name(); // TODO parent
    // TODO determine whether to use _type or _name

    // TODO consider writing using TreeCursor
    if node.child_count() > 0 {
        // TODO prevent double wrap with function parameters
        // TODO for WrapIfChild should we have a wrap boundary?
        let mut stack: Vec<FormatContainer> = vec![FormatContainer {
            children: Vec::new(),
        }];

        for child in (0..node.child_count()).map(|i| node.child(i).unwrap()) {
            let child_name = child.grammar_name();

            let mut indent = false;

            // TODO how do we allow existing newlines 

            // preprocess
            if let Some(previous) = child.prev_sibling() {
                let previous_name = previous.grammar_name();

                // if newline_before.contains(child_name) {
                //     stack.last_mut().unwrap().children.push(FormatNode::Newline);
                // }

                if add_wrap_before.contains(child_name) {
                    stack.last_mut().unwrap().children.push(FormatNode::Wrap {
                        wrap_with_indent: true,
                        or_space: false,
                    });
                }

                if node_type == "argument_list" {
                    if previous_name == "(" {
                        stack.last_mut().unwrap().children.push(FormatNode::Wrap {
                            wrap_with_indent: true,
                            or_space: false,
                        });
                    } else if previous_name == "," {
                        stack.last_mut().unwrap().children.push(FormatNode::Wrap {
                            wrap_with_indent: true,
                            or_space: true,
                        });
                    }

                    if child_name == ")" {
                        stack.last_mut().unwrap().children.push(FormatNode::Wrap {
                            wrap_with_indent: false,
                            or_space: false,
                        });
                    }
                }

                // TODO wrap if child wrapped

                if node_type == "program" || block_elements.contains(node_type) {
                    if child_name == "}" {
                        if previous_name == "{" {
                            stack.last_mut().unwrap().children.push(FormatNode::Space);
                        } else {
                            stack.last_mut().unwrap().children.push(FormatNode::Newline);
                        }
                    } else {
                        stack.last_mut().unwrap().children.push(FormatNode::Newline);
                        if node_type != "program" {
                            indent = true;
                        }
                    }
                } else if unconditional_space.contains(node_type) {
                    stack.last_mut().unwrap().children.push(FormatNode::Space);
                }
                if spaced_nodes.contains(node_type) {
                    if !no_space_after.contains(previous_name)
                        && !no_space_before.contains(child_name)
                    {
                        stack.last_mut().unwrap().children.push(FormatNode::Space);
                    }
                }
            }

            // stack push
            if stack_pushers.contains(child_name) {
                stack.push(FormatContainer {
                    children: Vec::new(),
                });
            }

            // process
            let processed = transform2(source_code, &child);

            stack.last_mut().unwrap().children.push(if indent {
                FormatNode::Indent(processed.into())
            } else {
                processed
            });

            // stack pop
            if stack_poppers.contains(child_name) {
                let last = stack.pop().unwrap();

                stack
                    .last_mut()
                    .unwrap()
                    .children
                    .push(FormatNode::Group(last.children));
            }

            // postprocess
            if let Some(next) = child.next_sibling() {
                if newline_after.contains(child_name) {
                    stack.last_mut().unwrap().children.push(FormatNode::Newline);
                }

                if conditional_newline_after.contains(child_name) && next.grammar_name() != "}" {
                    stack.last_mut().unwrap().children.push(FormatNode::Newline);
                }

                if child_name == "import_declaration" && next.grammar_name() != "import_declaration"
                {
                    stack.last_mut().unwrap().children.push(FormatNode::Newline);
                }

                if node_type.ends_with("_body") {
                    if child_name == "method_declaration" || child_name == "constructor_declaration"
                    {
                        if next.grammar_name() != "}" {
                            // TODO some sort of merging of newlines except where double is intentional?
                            stack.last_mut().unwrap().children.push(FormatNode::Newline);
                        }
                    }

                    if child_name == "field_declaration"
                        && next.grammar_name() != "field_declaration"
                        && next.grammar_name() != "}"
                    {
                        stack.last_mut().unwrap().children.push(FormatNode::Newline);
                    }
                }
            }
        }

        // TODO add newline collapsing

        return FormatNode::Group(
            stack
                .into_iter()
                .rev()
                .reduce(|child, mut parent| {
                    parent.children.push(FormatNode::Group(child.children));
                    return parent;
                })
                .unwrap()
                .children,
        );
    } else {
        if let Ok(original_content) = node.utf8_text(source_code) {
            let content = if node.grammar_name() == "block_comment" {
                original_content
                    .lines()
                    .map(|line| line.trim_start())
                    .enumerate()
                    .map(|(index, line)| {
                        if index == 0 {
                            line.to_owned()
                        } else {
                            " ".to_owned() + line
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("\n")
            } else {
                original_content.into()
            };

            return FormatNode::Content(content.into());
        }
    }

    return FormatNode::Empty;
}

const INDENT: &str = "    ";

fn prettyprint(formatted: &FormatNode, parent_wrap: bool) -> String {
    // println!("{:?}", formatted);

    let transformed = print(formatted, false, parent_wrap);

    if transformed.lines().any(|line| line.len() > 100) {
        let new = print(formatted, true, parent_wrap);

        if transformed.lines().any(|line| line.len() > 100) {
            // panic!("Unexpectedly long line! {:?} {:?}", formatted, transformed);
        }

        new
    } else {
        transformed
    }
}

fn print(formatted: &FormatNode, wrap: bool, parent_wrap: bool) -> String {
    match formatted {
        FormatNode::Content(content) => content.to_string(),
        FormatNode::Group(elements) => elements
            .iter()
            .map(|element| prettyprint(element, wrap))
            .collect(),
        // FormatNode::Indent(elements) => {
        //     if elements.len() > 3 {
        //         let transformed = elements
        //             .iter()
        //             .map(|element| prettyprint(element, wrap))
        //             .collect::<String>();

        //         let as_lines = transformed.lines().collect::<Vec<&str>>();

        //         let len = as_lines.len();

        //         as_lines
        //             .into_iter()
        //             .enumerate()
        //             .map(|(i, line)| {
        //                 if i == 0 || i == len - 1 {
        //                     line.to_string()
        //                 } else {
        //                     (INDENT.to_owned() + line).to_owned()
        //                 }
        //             })
        //             .collect::<Vec<String>>()
        //             .join("\n")
        //             .to_owned()
        //     } else {
        //         "{ }".to_owned()
        //     }
        // }
        FormatNode::Wrap {
            wrap_with_indent,
            or_space,
        } => {
            if parent_wrap {
                "\n".to_owned() + if *wrap_with_indent { INDENT } else { "" }
            } else {
                if *or_space {
                    " ".to_owned()
                } else {
                    "".to_owned()
                }
            }
        }
        // TODO when we indent, we need to indent all of the next element
        FormatNode::Indent(element) => prettyprint(element, wrap)
            .lines()
            .map(|line| (INDENT.to_owned()) + line)
            .collect::<Vec<String>>()
            .join("\n"),
        FormatNode::Newline => "\n".to_owned(),
        FormatNode::Space => " ".to_owned(),
        _ => "".to_string(),
    }
}
