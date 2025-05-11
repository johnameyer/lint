use parser::tree::Tree;

use crate::format_node::FormatNode;
use crate::render::{WrapParameters, prettyprint};
use crate::transform::transform;

pub fn print(node: &Tree) -> String {
    print_as_tree(&transform(node), 0);
    return prettyprint(&transform(node), WrapParameters::default()).result + "\n";
}

pub fn print_as_tree(node: &FormatNode, indent: usize) {
    let name = match node {
        FormatNode::Content(_) => "Content",
        FormatNode::Group(_) => "Group",
        FormatNode::Indent(_) => "Indent",
        FormatNode::Wrap(_, _) => "Wrap",
        FormatNode::Space => "Space",
        FormatNode::Newline => "Newline",
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
        FormatNode::Newline => (),
    };
}
