use std::collections::HashSet;

use crate::format_node::{FormatNode, WrapArguments};

use parser::tree::Tree;

#[derive(Debug, Default)]
struct FormatArguments {
    space: bool,
    newline: bool,
    double_newline: bool, // ignores newline - TODO better way to model this?
    indent: bool,         // TODO do we need a separate one for wrap and indent?
    wrap: bool,
    prevent_wrap_cascade: bool,
    child_wrap_prevents_wrap: bool,
}

pub fn transform<'source>(node: &Tree<'source>) -> FormatNode {
    let parent_name = node.name();

    #[derive(Debug)]
    struct FormatContainer {
        children: Vec<FormatNode>,
        wrapping: Option<WrapArguments>,
    }

    // TODO consider writing using TreeCursor
    if node.children().len() > 0 {
        // TODO prevent double wrap with function parameters
        // TODO for WrapIfChild should we have a wrap boundary?
        let mut stack: Vec<FormatContainer> = vec![FormatContainer {
            children: Vec::new(),
            wrapping: None,
        }];

        let mut stack_pushers_depth: Vec<usize> = vec![];

        let mut between = FormatArguments::default();

        // TODO create a local pop function

        for index in 0..node.children().len() {
            let child = &node.children()[index];
            let child_name = child.name();

            // preprocess
            if let Some(previous) = index
                .checked_sub(1)
                .and_then(|prev_index| node.children().get(prev_index))
            {
                let previous_name = previous.name();

                pre_visit(parent_name, &mut between, child_name, previous_name);
            }

            if between.wrap {
                let previous = stack.pop().unwrap();
                if let Some(wrapping) = previous.wrapping {
                    stack.last_mut().unwrap().children.push(FormatNode::Wrap(
                        FormatNode::Group(previous.children).into(),
                        wrapping,
                    ));
                    // stack.last_mut().unwrap().children.push(FormatNode::Group(previous.children));
                } else {
                    stack.push(FormatContainer {
                        children: previous.children,
                        wrapping: previous.wrapping,
                    }); // TODO makes sense?
                }
                stack.push(FormatContainer {
                    children: Vec::new(),
                    wrapping: Some(WrapArguments {
                        child_wrap_prevents_wrap: between.child_wrap_prevents_wrap,
                        wrap_with_indent: between.indent,
                        or_space: between.space,
                    }),
                });

                // TODO condense nested groups?
            }

            // stack push
            if let Some(previous) = index
                .checked_sub(1)
                .and_then(|prev_index| node.children().get(prev_index))
            {
                if get().stack_pushers.contains(previous.name())
                    && !get().stack_poppers.contains(child_name)
                {
                    stack_pushers_depth.push(stack.len());

                    stack.push(FormatContainer {
                        children: Vec::new(),
                        wrapping: None,
                    });
                }
            }

            // process
            let processed = transform(&child);

            match between {
                FormatArguments { wrap: true, .. } => {}
                FormatArguments {
                    double_newline: true,
                    ..
                } => {
                    stack.last_mut().unwrap().children.push(FormatNode::Newline);
                    stack.last_mut().unwrap().children.push(FormatNode::Newline);
                }
                FormatArguments { indent: true, .. } => {
                    stack.last_mut().unwrap().children.push(FormatNode::Newline);
                }
                FormatArguments { newline: true, .. } => {
                    stack.last_mut().unwrap().children.push(FormatNode::Newline)
                }
                FormatArguments { space: true, .. } => {
                    stack.last_mut().unwrap().children.push(FormatNode::Space)
                }
                _ => {}
            }

            stack.last_mut().unwrap().children.push(match between {
                // TODO how do we capture the right nodes in this? Do we just keep capturing until we reach the next wrap node?
                // FormatArguments { wrap: true, ..} => FormatNode::Wrap(WrapArguments {
                //     wrap_with_indent: between.indent,
                //     or_space: between.space,
                // }),
                FormatArguments { wrap: true, .. } => processed,
                FormatArguments { indent: true, .. } => FormatNode::Indent(processed.into()),
                _ => processed,
            });

            between = FormatArguments::default();

            // stack pop
            if let Some(next) = node.children().get(index + 1) {
                if stack_pushers_depth.len() == 0 {
                    // Warn
                } else if get().stack_poppers.contains(next.name()) {
                    let expected_depth = stack_pushers_depth.pop().unwrap();

                    while stack.len() > expected_depth {
                        let last = stack.pop().unwrap();

                        stack.last_mut().unwrap().children.push(match last {
                            FormatContainer {
                                children,
                                wrapping: Some(wrapping),
                            } => FormatNode::Wrap(FormatNode::Group(children).into(), wrapping),
                            FormatContainer { children, .. } => FormatNode::Group(children),
                        });
                    }
                }
            }

            // postprocess
            if let Some(next) = node.children().get(index + 1) {
                let next_name = next.name();

                let has_multiple_newlines =
                    (next.range().start_point.row - child.range().end_point.row) > 1;

                post_visit(
                    parent_name,
                    &mut between,
                    child_name,
                    has_multiple_newlines,
                    next_name,
                );
            }
        }

        // TODO need to put the last item in a wrap as well?

        FormatNode::Group(
            stack
                .into_iter()
                .rev()
                .reduce(|child, mut parent| {
                    if let Some(wrap_arguments) = child.wrapping {
                        parent.children.push(FormatNode::Wrap(
                            FormatNode::Group(child.children).into(),
                            wrap_arguments,
                        ));
                    } else {
                        parent.children.push(FormatNode::Group(child.children));
                    }
                    return parent;
                })
                .unwrap()
                .children,
        )
    } else {
        let original_content = node.text();
        let content = if node.name() == "block_comment" {
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

        FormatNode::Content(content.into())
    }
}

fn pre_visit(parent: &str, between: &mut FormatArguments, child: &str, previous: &str) {
    // if newline_before.contains(child_name) {
    //     stack.last_mut().unwrap().children.push(FormatNode::Newline);
    // }

    if get().add_wrap_before.contains(child) {
        between.wrap = true;
        between.indent = true;
        between.child_wrap_prevents_wrap = true;
    }

    if get().wrap_list.contains(parent) {
        if previous == "(" || previous == "{" {
            between.wrap = true;
            between.indent = true; // TODO do we need to separate both?
        // between.child_wrap_prevents_wrap = false;
        } else if previous == "," {
            between.wrap = true;
            // between.indent = true; // TODO do we need to separate both?
            between.space = true;
        }

        if child == ")" || child == "}" {
            between.wrap = true;
        }
    }

    if parent == "program" || get().block_elements.contains(parent) {
        between.prevent_wrap_cascade = true;

        if child == "}" {
            if previous == "{" {
                between.space = true;
            } else {
                between.newline = true;
            }
        } else {
            between.newline = true;
            if parent != "program" {
                between.indent = true;
            }
        }
    } else if get().unconditional_space.contains(parent) {
        between.space = true;
    }
    if get().spaced_nodes.contains(parent) {
        if !get().no_space_after.contains(previous) && !get().no_space_before.contains(child) {
            between.space = true;
        }
    }
}

fn post_visit(
    parent: &str,
    between: &mut FormatArguments,
    child: &str,
    has_multiple_newlines: bool,
    next: &str,
) {
    if get().newline_after.contains(child) {
        between.double_newline = true;
    }

    if get().conditional_newline_after.contains(child) && next != "}" {
        between.double_newline = true;
    }

    if child == "import_declaration" && next != "import_declaration" {
        between.double_newline = true;
    }

    if parent.ends_with("_body") {
        if child == "method_declaration" || child == "constructor_declaration" {
            if next != "}" {
                between.double_newline = true;
            }
        }

        if child == "field_declaration" && next != "field_declaration" && next != "}" {
            between.double_newline = true;
        }
    }

    if get().block_elements.contains(parent) {
        if has_multiple_newlines {
            between.double_newline = true;
        }
    }
}

// TODO to be replaced by DSL
struct FormatConfig {
    block_elements: HashSet<&'static str>,
    unconditional_space: HashSet<&'static str>,
    spaced_nodes: HashSet<&'static str>,
    newline_after: HashSet<&'static str>,
    conditional_newline_after: HashSet<&'static str>,
    no_space_before: HashSet<&'static str>,
    wrap_list: HashSet<&'static str>,
    no_space_after: HashSet<&'static str>,
    add_wrap_before: HashSet<&'static str>,
    stack_pushers: HashSet<&'static str>,
    stack_poppers: HashSet<&'static str>,
}

fn get() -> FormatConfig {
    FormatConfig {
        block_elements: HashSet::from([
            "class_body",
            "enum_body",
            "interface_body",
            "block",
            "constructor_body",
        ]), // children are indented unless there is no non-bracket element

        unconditional_space: HashSet::from([
            "assignment_expression",
            "binary_expression",
            "ternary_expression",
            "instanceof_expression",
            "lambda_expression",
        ]),

        spaced_nodes: HashSet::from([
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
        ]), // items to add spaces between

        // let newline_before = HashSet::from([
        // ]);
        newline_after: HashSet::from([
            "if_statement",
            "try_statement",
            "for_statement",
            "do_statement",
            "while_statement",
            "package_declaration",
        ]),

        conditional_newline_after: HashSet::from([
            "class_declaration",
            "enum_declaration",
            "record_declaration",
        ]),

        // TODO implements
        no_space_before: HashSet::from([
            ">",
            ")",
            ".",
            ",",
            ";",
            "argument_list",
            "formal_parameters",
            "catch",
        ]),

        wrap_list: HashSet::from([
            "argument_list",
            "parenthesized_expression",
            "array_initializer",
        ]),

        // TODO generic method call
        no_space_after: HashSet::from([
            "(", "<", // unless as binary_operator
        ]),

        add_wrap_before: HashSet::from(["."]),

        stack_pushers: HashSet::from(["(", "{"]),

        stack_poppers: HashSet::from([")", "}"]),
    }
}
