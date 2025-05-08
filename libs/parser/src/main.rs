use std::{
    env::args,
    fs::{read_to_string, write},
};

use debug::print_as_tree;
use load_parser::{Language, get_tree_sitter_language};

use print::print;
use tree_sitter::Parser;

mod load_parser;

mod debug;
mod print;
mod render;
mod transform;
mod format_node;

// use crate::print::print;

fn main() {
    println!("Hello, world!");

    let mut parser = build_parser(&Language::Java);

    for arg in args().skip(1) {
        // TODO handle directories / paths not ending with java
        if arg.ends_with(".java") {
            handle(&mut parser, arg);
        }
    }
}

fn build_parser(language: &Language) -> Parser {
    let language = get_tree_sitter_language(language);

    let mut parser = Parser::new();

    parser
        .set_language(&language)
        .expect("Error loading Rust grammar");
    parser
}

fn handle(parser: &mut Parser, path: String) {
    let source_code = read_to_string(&path).unwrap();

    let tree = parser.parse(&source_code, None).unwrap();

    // TODO take as debug arg
    print_as_tree(&tree.root_node(), 0);

    // is this an issue for unicode characters outside ascii?
    let formatted = print(source_code.as_bytes(), &tree.root_node());

    write(&path, formatted).expect("Unable to write to file");
}

#[cfg(test)]
mod tests {

    use super::*;

    use test_each_file::test_each_file;

    test_each_file! { in "./data" => test }

    fn test(content: &str) {
        let mut parser = build_parser(&Language::Java);

        let tree = parser.parse(&content, None).unwrap();

        print_as_tree(&tree.root_node(), 0);

        // is this an issue for unicode characters outside ascii?
        let formatted = print(content.as_bytes(), &tree.root_node());

        // println!("{}", formatted);

        assert_eq!(content, formatted);
    }
}
