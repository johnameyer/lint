use tree_sitter::Node;

use crate::format_node::FormatNode;

pub fn print_as_tree(node: &Node, indent: usize) {
    println!("{}{}", " ".repeat(indent), node.grammar_name());

    if node.child_count() > 0 {
        for index in 0..node.child_count() {
            print_as_tree(&node.child(index).unwrap(), indent + 2);
        }
    }
}
