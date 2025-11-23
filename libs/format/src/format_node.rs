use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Hash)]
pub enum FormatNode {
    Content(Box<str>),
    Group(Vec<FormatNode>),
    // Indent(Vec<FormatNode>),
    Indent(Box<FormatNode>),
    Wrap(Box<FormatNode>, WrapArguments),
    // WrapGroup
    Space,
    // WrapBoundary(Box<FormatNode>), // do we need aside from indent?
    Newline,
    // Empty,
}

impl FormatNode {
    pub fn get_hash(self: &Self) -> u64 {
        let mut hasher = DefaultHasher::new();

        self.hash(&mut hasher);

        hasher.finish()
    }
}

#[derive(Debug, Hash)]
pub struct WrapArguments {
    // TODO consider inverting
    /// Should the content be wrapped if the child is wrapping?
    pub child_wrap_prevents_wrap: bool,
    /// If wrapping, should the contained content be indented?
    pub wrap_with_indent: bool,
    /// If not wrapping, should this be replaced with a space?
    pub or_space: bool,
    /// Should this element prevent wraps from applying to children elements?
    pub wrap_if_parent: bool,
}
