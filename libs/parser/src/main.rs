use std::{
    collections::HashSet,
    env::args,
    fs::{read_to_string, write},
};

use load_parser::{Language, get_tree_sitter_language};

use tree_sitter::{Node, Parser};

mod load_parser;

fn main() {
    println!("Hello, world!");

    let language = get_tree_sitter_language(&Language::Java);

    let mut parser = Parser::new();

    parser
        .set_language(&language)
        .expect("Error loading Rust grammar");

    for arg in args().skip(1) {
        // TODO handle directories / paths not ending with java
        if arg.ends_with(".java") {
            handle(&mut parser, arg);
        }
    }
}

fn handle(parser: &mut Parser, path: String) {
    let source_code = read_to_string(&path).unwrap();

    let tree = parser.parse(&source_code, None).unwrap();

    print_as_tree(&tree.root_node(), 0);

    // is this an issue for unicode characters outside ascii?
    let formatted = print(source_code.as_bytes(), &tree.root_node(), 0);

    write(&path, formatted).expect("Unable to write to file");
}

fn print_as_tree(node: &Node, indent_level: usize) {
    println!("{}{}", "  ".repeat(indent_level), node.grammar_name());

    let count = node.child_count();
    for i in 0..count {
        print_as_tree(&node.child(i).unwrap(), indent_level + 1);
    }
}

fn print(source_code: &[u8], node: &Node, indent_level: usize) -> String {
    let indent_elements = HashSet::from([
        "class_body",
        "enum_body",
        "interface_body",
        "block",
        "constructor_body",
    ]); // children are indented
    let fully_spaced_nodes = HashSet::from([
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
        "package_declaration",
        "enum_declaration",
        "import_declaration",
        "interface_declaration",
        "modifiers",
        "assignment_expression",
        "throws",
        "if_statement",
        "try_statement",
    ]); // items to add spaces between
    let internally_spaced_nodes =
        HashSet::from(["type_arguments", "argument_list", "formal_parameters"]);
    let newline_elements = HashSet::from(["program"]); // indent_element but does not indent
    let syntax_elements = HashSet::from([",", ";", "argument_list", "formal_parameters"]); // TODO some better class?

    let extra_newline = HashSet::from([
        "program",
        "package_declaration",
        "constructor_declaration",
        "method_declaration",
    ]);

    // catch_clause
    // try catch finally
    // add extra newline between package declaration, static imports, imports
    // add extra newline after function declarations, after field group
    // add newline after marker_annotation but no space before next modifier - only for classes / functions but not arguments / variables

    // retain existing newlines
    // split long content

    let count = node.child_count().into();

    let mut output = String::new();

    if count == 0 {
        output += node.utf8_text(source_code).unwrap();
    }

    let node_type = node.grammar_name();

    for i in 0..count {
        let child = node.child(i).unwrap();
        // TODO convert to match
        if indent_elements.contains(node_type) {
            let new_indent = indent_level + (i < count - 1) as usize;
            if i > 0 && !syntax_elements.contains(child.grammar_name()) {
                output += "\n";
                output += &"    ".repeat(new_indent);
            }
            output += &print(source_code, &child, new_indent);
        } else if newline_elements.contains(node_type) {
            if i > 0 {
                output += "\n";
            }
            output += &print(source_code, &child, indent_level);
        } else if fully_spaced_nodes.contains(node_type) {
            if i > 0 && !syntax_elements.contains(child.grammar_name()) {
                output += " ";
            }
            output += &print(source_code, &child, indent_level);
        } else if internally_spaced_nodes.contains(node_type) {
            if i > 1 && i < count - 1 && !syntax_elements.contains(child.grammar_name()) {
                output += " ";
            }
            output += &print(source_code, &child, indent_level);
        } else {
            output += &print(source_code, &child, indent_level);
        }
    }

    if extra_newline.contains(node_type) {
        // but only if there is a following element...
        output += "\n";
    }

    return output;
}
