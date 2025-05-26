use std::panic::catch_unwind;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

const PARSER_CLONE_ROOT: &str = ".vendor";

struct TreeSitterParser<'a> {
    name: &'a str,
    repository: &'a str,
    commit: &'a str,
}

const PROJECTS: &[TreeSitterParser] = &[TreeSitterParser {
    name: "tree-sitter-java",
    repository: "https://github.com/tree-sitter/tree-sitter-java.git",
    commit: "a7db5227ec40fcfe94489559d8c9bc7c8181e25a",
}];

fn main() {
    let root_cwd = Path::new(env!("CARGO_MANIFEST_DIR"));
    env::set_current_dir(&root_cwd).unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();

    for project in PROJECTS {
        let project_path: PathBuf = [
            &out_dir,
            PARSER_CLONE_ROOT,
            &format!("{}@{}", &project.name, &project.commit),
        ]
        .iter()
        .collect();

        if !project_path.exists() {
            fs::create_dir_all(&project_path).expect("Failed to create project directory");

            env::set_current_dir(&project_path).unwrap();

            if process(project).is_none() {
                fs::remove_dir(&project_path).unwrap();
                panic!("Failed to build project {}", project.name)
            }

            env::set_current_dir(&root_cwd).unwrap();
        }
    }
}

fn process(project: &TreeSitterParser) -> Option<()> {
    clone(project)?;
    catch_unwind(|| compile(project)).ok()?;
    Some(())
}

fn clone(project: &TreeSitterParser) -> Option<()> {
    run("git", &["init", "-q"])?;
    run("git", &["remote", "add", "origin", project.repository])?;
    run(
        "git",
        &["fetch", "-q", "--depth", "1", "origin", project.commit],
    )?;
    run("git", &["checkout", "-q", "FETCH_HEAD"])?;
    fs::remove_dir_all(".git").ok()?;
    Some(())
}

fn compile(parser: &TreeSitterParser) {
    let dir = &PathBuf::from("src");
    cc::Build::new()
        .include(dir)
        .files(vec![dir.join("parser.c")])
        .cpp(false)
        .warnings(false)
        .compile(parser.name);
}

fn run(name: &str, args: &[&str]) -> Option<()> {
    let mut command = Command::new(name);
    command.args(args);
    println!("Running {command:?}");
    command
        .status()
        .inspect_err(|_| println!("Failed to execute {command:?}"))
        .ok()
        .filter(|cmd| cmd.success())
        .and(Some(()))
}
