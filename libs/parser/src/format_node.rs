
#[derive(Debug)]
pub enum FormatNode {
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

pub struct FormatContainer {
    pub children: Vec<FormatNode>,
}
