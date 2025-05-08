use std::collections::HashSet;

use tree_sitter::Node;

use crate::transform::transform;
use crate::render::prettyprint;

pub fn print(source_code: &[u8], node: &Node) -> String {
    println!("{:?}", transform(source_code, node));
    return prettyprint(&transform(source_code, node), false) + "\n";
}

// stack

// parent: String
// next: String

// Node matchers emit to stack?
// pop off created groups at conclusion of element

