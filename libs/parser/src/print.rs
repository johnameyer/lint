use tree_sitter::Node;

use crate::format_node::FormatNode;
use crate::render::{prettyprint, WrapParameters};
use crate::transform::transform;

pub fn print(source_code: &[u8], node: &Node) -> String {
    print_as_tree(&transform(source_code, node), 0);
    return prettyprint(&transform(source_code, node), WrapParameters::default()).result + "\n";
}

pub fn print_as_tree(node: &FormatNode, indent: usize) {
    let name = match node {
        FormatNode::Content(content) => "Content",
        FormatNode::Group(_) => "Group",
        FormatNode::Indent(_) => "Indent",
        FormatNode::Wrap(_, _) => "Wrap",
        FormatNode::Space => "Space",
        FormatNode::WrapBoundary(_) => "WrapBoundary",
        FormatNode::Newline => "Newline",
        FormatNode::Empty => "Empty",
    };

    println!("{}{}", " ".repeat(indent), name);

    if let FormatNode::Wrap(_, wrap) = node {
        println!("{}{:?}", " ".repeat(indent + 4), wrap);
    }

    let print_children = |children: &Vec<FormatNode>| {
        for child in children {
            print_as_tree(&child, indent + 4);
        }
    };

    match node {
        FormatNode::Content(content) => println!("{}{}", " ".repeat(indent + 4), content),
        FormatNode::Group(format_nodes) => print_children(format_nodes),
        FormatNode::Indent(format_node) => print_as_tree(format_node, indent + 4),
        FormatNode::Wrap(format_node, _) => print_as_tree(format_node, indent + 4),
        FormatNode::Space => (),
        FormatNode::WrapBoundary(format_node) => print_as_tree(format_node, indent + 4),
        FormatNode::Newline => (),
        FormatNode::Empty => (),
    };
}

// stack

// parent: String
// next: String

// Node matchers emit to stack?
// pop off created groups at conclusion of element
