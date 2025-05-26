use parser::tree::Tree;

#[allow(dead_code)]
pub fn print_as_tree(node: &Tree, indent: usize) {
    println!("{}{}", " ".repeat(indent), node.name());

    for child in node.children() {
        print_as_tree(&child, indent + 2);
    }
}
