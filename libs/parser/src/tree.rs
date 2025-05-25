// TODO should we have some sort of tree walking functionality like tree_sitter?
// TODO should we be assisting that by using prev_sibling / next_sibling?

pub struct Tree<'source> {
    pub(crate) name: String,
    pub(crate) children: Vec<Tree<'source>>,
    pub(crate) range: tree_sitter::Range, // TODO build own
    pub(crate) source: &'source str,
}

impl Tree<'_> {
    pub fn name(&self) -> &str {
        &self.name.as_str()
    }

    pub fn children(&self) -> &Vec<Tree> {
        &self.children
    }

    pub fn range(&self) -> &tree_sitter::Range {
        &self.range
    }

    pub fn text(&self) -> &str {
        &self.source[self.range.start_byte..self.range.end_byte]
    }
}
