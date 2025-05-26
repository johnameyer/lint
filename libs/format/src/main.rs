use std::{
    env::args,
    ffi::OsStr,
    fs::{read_to_string, write},
    path::{Path, absolute},
};

#[allow(unused_imports)]
use debug::print_as_tree;

use editorconfig::EditorConfigResolver;
use parser::{language::Language, parser::Parser};
use print::print;
use render::PrettyPrintParameters;
use walkdir::WalkDir;

mod debug;
mod editorconfig;
mod format_node;
mod print;
mod render;
mod transform;

fn main() {
    println!("Hello, world!");

    let mut parser = Parser::of(Language::Java);

    let mut editor_config_resolver: EditorConfigResolver = EditorConfigResolver::new();

    for arg in args().skip(1) {
        let path = absolute(Path::new(&arg)).unwrap();

        for entry_option in WalkDir::new(path).sort_by_file_name() {
            let entry: walkdir::DirEntry = entry_option.unwrap();
            println!(
                "Processing {:?} {}",
                entry.path().extension(),
                entry.path().display()
            );

            if entry.file_type().is_file() {
                if entry.path().extension().and_then(OsStr::to_str) == Some("java") {
                    handle(&mut parser, &mut editor_config_resolver, entry.path());
                }
                // TODO non-java files
            }
        }
    }
}

fn handle(parser: &mut Parser, editor_config_resolver: &mut EditorConfigResolver, path: &Path) {
    let editorconfig = editor_config_resolver.resolve(path);

    println!("Resolved to {:?}", editorconfig);

    let source_code = read_to_string(&path).unwrap();

    let tree = parser.parse(&source_code).unwrap();

    // TODO take as debug arg
    // print_as_tree(&tree, 0);

    // is this an issue for unicode characters outside ascii?
    let formatted = print(&tree, &PrettyPrintParameters {
        indent_size: editorconfig.indent_size.unwrap_or(4),
        max_line_length: 100,
    });

    write(&path, formatted).expect("Unable to write to file");
}

#[cfg(test)]
mod tests {

    use super::*;

    use test_each_file::test_each_file;

    test_each_file! { in "./data" => compare_parsed_to_original }

    fn compare_parsed_to_original(content: &str) {
        let mut parser = Parser::of(Language::Java);

        let tree = parser.parse(&content).unwrap();

        print_as_tree(&tree, 0);

        // is this an issue for unicode characters outside ascii?
        let formatted = print(&tree, &PrettyPrintParameters { indent_size: 4, max_line_length: 100 });

        // println!("{}", formatted);

        assert_eq!(content, formatted);
    }
}
