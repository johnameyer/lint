#[derive(Debug)]
pub enum FormatNode {
    Content(Box<str>),
    Group(Vec<FormatNode>),
    // Indent(Vec<FormatNode>),
    Indent(Box<FormatNode>),
    Wrap(WrapArguments),
    // WrapGroup
    Space,
    WrapBoundary(Box<FormatNode>), // do we need aside from indent?
    Newline,
    Empty,
}

#[derive(Debug)]
pub struct WrapArguments {
    pub wrap_with_indent: bool,
    pub or_space: bool,
}
