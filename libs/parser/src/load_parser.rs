// Adapted from https://github.com/DataDog/datadog-static-analyzer/blob/0fa5eeed99dc91112b1cbb19209e0b36093ef388/crates/static-analysis-kernel/src/analysis/tree_sitter.rs#L22

#![allow(warnings)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Language {
    Csharp,
    Dockerfile,
    Elixir,
    Go,
    Java,
    JavaScript,
    Json,
    Kotlin,
    Python,
    Ruby,
    Rust,
    Swift,
    Terraform,
    TypeScript,
    Yaml,
    Starlark,
    Bash,
    PHP,
    Markdown,
    Apex,
    R,
    SQL,
}

pub fn get_tree_sitter_language(language: &Language) -> tree_sitter::Language {
    unsafe extern "C" {
        fn tree_sitter_c_sharp() -> tree_sitter::Language;
        fn tree_sitter_dockerfile() -> tree_sitter::Language;
        fn tree_sitter_elixir() -> tree_sitter::Language;
        fn tree_sitter_go() -> tree_sitter::Language;
        fn tree_sitter_java() -> tree_sitter::Language;
        fn tree_sitter_javascript() -> tree_sitter::Language;
        fn tree_sitter_json() -> tree_sitter::Language;
        fn tree_sitter_kotlin() -> tree_sitter::Language;
        fn tree_sitter_python() -> tree_sitter::Language;
        fn tree_sitter_ruby() -> tree_sitter::Language;
        fn tree_sitter_rust() -> tree_sitter::Language;
        fn tree_sitter_swift() -> tree_sitter::Language;
        fn tree_sitter_tsx() -> tree_sitter::Language;
        fn tree_sitter_hcl() -> tree_sitter::Language;
        fn tree_sitter_yaml() -> tree_sitter::Language;
        fn tree_sitter_starlark() -> tree_sitter::Language;
        fn tree_sitter_bash() -> tree_sitter::Language;
        fn tree_sitter_php() -> tree_sitter::Language;
        fn tree_sitter_markdown() -> tree_sitter::Language;
        fn tree_sitter_apex() -> tree_sitter::Language;
        fn tree_sitter_r() -> tree_sitter::Language;
        fn tree_sitter_sql() -> tree_sitter::Language;
    }

    match language {
        Language::Csharp => unsafe { tree_sitter_c_sharp() },
        Language::Dockerfile => unsafe { tree_sitter_dockerfile() },
        Language::Go => unsafe { tree_sitter_go() },
        Language::Elixir => unsafe { tree_sitter_elixir() },
        Language::Java => unsafe { tree_sitter_java() },
        Language::JavaScript => unsafe { tree_sitter_javascript() },
        Language::Kotlin => unsafe { tree_sitter_kotlin() },
        Language::Json => unsafe { tree_sitter_json() },
        Language::Python => unsafe { tree_sitter_python() },
        Language::Ruby => unsafe { tree_sitter_ruby() },
        Language::Rust => unsafe { tree_sitter_rust() },
        Language::Swift => unsafe { tree_sitter_swift() },
        Language::Terraform => unsafe { tree_sitter_hcl() },
        Language::TypeScript => unsafe { tree_sitter_tsx() },
        Language::Yaml => unsafe { tree_sitter_yaml() },
        Language::Starlark => unsafe { tree_sitter_starlark() },
        Language::Bash => unsafe { tree_sitter_bash() },
        Language::PHP => unsafe { tree_sitter_php() },
        Language::Markdown => unsafe { tree_sitter_markdown() },
        Language::Apex => unsafe { tree_sitter_apex() },
        Language::R => unsafe { tree_sitter_r() },
        Language::SQL => unsafe { tree_sitter_sql() },
    }
}
