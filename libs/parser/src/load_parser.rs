#![allow(warnings)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Language {
    Java,
}

pub fn get_tree_sitter_language(language: &Language) -> tree_sitter::Language {
    unsafe extern "C" {
        fn tree_sitter_java() -> tree_sitter::Language;
    }

    unsafe {
        match language {
            Language::Java => tree_sitter_java(),
        }
    }
}
