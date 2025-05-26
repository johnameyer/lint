use std::{
    env::args, ffi::OsStr, fs::{read_to_string, write}, path::Path
};

use debug::print_as_tree;

use parser::{language::Language, parser::Parser};
use print::print;
use walkdir::WalkDir;

mod debug;
mod format_node;
mod print;
mod render;
mod transform;

fn main() {
    println!("Hello, world!");

    let mut parser = Parser::of(Language::Java);

    for arg in args().skip(1) {
        let path = Path::new(&arg);

        for entry_option in WalkDir::new(path) {
            let entry: walkdir::DirEntry = entry_option.unwrap();
            if entry.file_type().is_file() {
                if entry.path().extension().and_then(OsStr::to_str) == Some("java") {
                    handle(&mut parser, entry.path());
                }
                // TODO non-java files
            }
        }
    }
}

fn handle(parser: &mut Parser, path: &Path) {
    let source_code = read_to_string(&path).unwrap();

    let tree = parser.parse(&source_code).unwrap();

    // TODO take as debug arg
    print_as_tree(&tree, 0);

    // is this an issue for unicode characters outside ascii?
    let formatted = print(&tree);

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
        let formatted = print(&tree);

        // println!("{}", formatted);

        assert_eq!(content, formatted);
    }
}
