use crate::{language::Language, tree::Tree};

pub struct Parser {
    parser: tree_sitter::Parser,
}

impl Parser {
    pub fn of(lang: Language) -> Parser {
        let language = get_tree_sitter_language(&lang);

        let mut parser = tree_sitter::Parser::new();

        parser
            .set_language(&language)
            .expect("Error loading Rust grammar");

        Parser { parser }
    }

    pub fn parse<'source>(&mut self, source_code: &'source str) -> Option<Tree<'source>> {
        self.parser
            .parse(&source_code, None)
            .map(|tree| convert_to_tree(tree.root_node(), source_code))
    }
}

// TODO should the method be moved onto Tree?
fn convert_to_tree<'source>(node: tree_sitter::Node, source_code: &'source str) -> Tree<'source> {
    Tree {
        name: node.grammar_name().to_string(),
        children: (0..node.child_count())
            .map(|i| convert_to_tree(node.child(i).unwrap(), source_code))
            .collect(),
        range: node.range(),
        source: source_code,
    }
}

fn get_tree_sitter_language(language: &Language) -> tree_sitter::Language {
    unsafe extern "C" {
        fn tree_sitter_java() -> tree_sitter::Language;
    }

    unsafe {
        match language {
            Language::Java => tree_sitter_java(),
        }
    }
}
