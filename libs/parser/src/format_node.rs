#[derive(Debug)]
pub enum FormatNode {
    Content(Box<str>),
    Group(Vec<FormatNode>),
    // Indent(Vec<FormatNode>),
    Indent(Box<FormatNode>),
    Wrap(Box<FormatNode>, WrapArguments),
    // WrapGroup
    Space,
    WrapBoundary(Box<FormatNode>), // do we need aside from indent?
    Newline,
    Empty,
}

#[derive(Debug)]
pub struct WrapArguments {
    pub child_wrap_prevents_wrap: bool,
    pub wrap_with_indent: bool,
    pub or_space: bool,
}
