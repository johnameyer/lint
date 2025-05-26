use std::collections::HashSet;

use super::transform::FormatArguments;

pub(super) fn pre_visit(parent: &str, between: &mut FormatArguments, child: &str, previous: &str) {
    // if newline_before.contains(child_name) {
    //     stack.last_mut().unwrap().children.push(FormatNode::Newline);
    // }

    // TODO should we just have a list of syntax elements / nonleaf nodes
    if parent == "binary_expression" {
        if get().multi_operators.contains(child) {
            between.wrap = true;
            between.indent = true;
            between.child_wrap_prevents_wrap = true;
        }
    }

    if parent == "variable_declarator" {
        if previous == "=" {
            between.wrap = true;
            between.indent = true;
            between.child_wrap_prevents_wrap = true;
        }
    }

    if get().add_wrap_before.contains(child) && parent != "scoped_identifier" {
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

pub(super) fn post_visit(
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
pub(super) struct FormatConfig {
    block_elements: HashSet<&'static str>,
    unconditional_space: HashSet<&'static str>,
    spaced_nodes: HashSet<&'static str>,
    newline_after: HashSet<&'static str>,
    conditional_newline_after: HashSet<&'static str>,
    no_space_before: HashSet<&'static str>,
    wrap_list: HashSet<&'static str>,
    no_space_after: HashSet<&'static str>,
    add_wrap_before: HashSet<&'static str>,
    multi_operators: HashSet<&'static str>,
    pub stack_pushers: HashSet<&'static str>,
    pub stack_poppers: HashSet<&'static str>,
}

pub(super) fn get() -> FormatConfig {
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
            "formal_parameters",
        ]),

        // TODO generic method call
        no_space_after: HashSet::from([
            "(", "<", // unless as binary_operator
        ]),

        add_wrap_before: HashSet::from(["."]),

        stack_pushers: HashSet::from(["(", "{"]),

        stack_poppers: HashSet::from([")", "}"]),

        multi_operators: HashSet::from([
            "?",
            ":",
            "*",
            "/",
            "%",
            "+",
            "-",
            "<<",
            ">>",
            ">>>",
            "<",
            ">",
            "<=",
            ">=",
            "instanceof",
            "==",
            "!=",
            "&",
            "^",
            "|",
            "&&",
            "||",
            "=",
            "+=",
            "-=",
            "*=",
            "/=",
            "%=",
            "&=",
            "^=",
            "|=",
            "<<=",
            ">>=",
            ">>>=",
        ]),
    }
}
