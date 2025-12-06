use std::fs;
use regex::Regex;
use std::path::Path;
use std::process::{Command, Stdio};
use clap::Parser;
use std::io;
use random_name::generate_name;

fn find_workspace_members(file_path: &Path) -> Vec<String> {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let re = Regex::new(r"\[tool.uv.workspace\]\s+members\s*=\s*\[(.*[^\]])\]").unwrap();
    let workspace_members = re.captures(&contents).expect("No match found for members in tool.uv.workspace, are you sure you are in a uv workspace?");
    let workspace_str = workspace_members[1].to_string();
    workspace_str
        .split(",")
        .map(|s| s.trim().replace("\"", "").replace("*", ".*").to_string())  
        .collect()
}

fn git_status_files() -> Vec<String> {
    let cmd = Command::new("git").arg("status").arg("--short").stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("Error executing `git status --short`, check for git installation");
    let output = cmd.wait_with_output().expect("Error executing `git status --short`, check for git installation and make sure that the current directory is a git repository");
    let output_str = String::from_utf8(output.stdout).expect("Could not convert standard output content to string");
    let re = Regex::new(r"[(M|T|A|D|R|C|U|\?|!)]{1,2}\s").unwrap();
    let lines: Vec<String> = output_str.split("\n").map(|s| re.replace(s, "").trim().to_string()).collect();
    lines
}

fn is_workspace_member(members: &Vec<String>, file: &str) -> bool {
    for member in members {
        let re = Regex::new(member).unwrap();
        if re.is_match(file) {
            return true
        }
    }
    false
}

fn find_pyproject(file: &Path) -> Result<&Path, String> {
    for ancestor in file.ancestors() {
        if ancestor.join("pyproject.toml").exists() {
            return Ok(ancestor)
        }
    }
    return Err("Could not find a parent path that contains a pyproject.toml".to_string())
}

fn find_project_name(pyproject: &Path) -> String {
    let re = Regex::new(r#"(?m)^\s*name\s*=\s*"(.+)""#).unwrap();
    let contents = fs::read_to_string(pyproject)
        .expect("Should have been able to read the file");
    let package_name = re.captures(&contents).expect("No package name found, ensure that you follow the `[package]\nname = .*` convention.");
    package_name[1].to_string()
}

fn ask_and_save_version_bump(pyproject: &Path, processed: &mut Vec<String>) {
    let project_name = find_project_name(pyproject);
    if processed.contains(&project_name) {
        return;
    } else {
        let project_name_to_push = find_project_name(pyproject);
        processed.push(project_name_to_push);
    }
    println!("Since modifications have been detected within package `{}`, what version bump would you like to apply? [major/minor/patch/ignore]", project_name);
    let mut vbump = String::new();

    io::stdin()
        .read_line(&mut vbump)
        .expect("Failed to read your input from stdin, please retry");

    if vbump.trim().to_lowercase() == "major" || vbump.trim().to_lowercase() == "minor" || vbump.trim().to_lowercase() == "patch" {
        println!("Please provide a brief description of what changed for this {} release", vbump.trim().to_lowercase());
        let mut changelog = String::new();
        io::stdin()
        .read_line(&mut changelog)
        .expect("Failed to read your input from stdin, please retry");
        if ! Path::new(".workspyce").exists() {
            match fs::create_dir(".workspyce") {
                Err(why) => println!("Error while creating `.workspyce` directory: {:?}", why.kind()),
                Ok(_) => {},
            }
        }
        let fl_name = format!(".workspyce/{}.md", generate_name().replace(" ", "-"));
        let contents = format!("---\npackage: {}\nrelease: {}\n---\n{}", project_name, vbump.trim().to_lowercase(), changelog);
        match fs::write(&fl_name, contents) {
            Err(why) => println!("Error while writing version changelog to `{}` directory: {:?}", &fl_name, why.kind()),
            Ok(_) => {},
        }
        println!("Great! All changes have been saved :)")
    } else {
        println!("Gotcha! The change will be ignored :)")
    }
}

#[derive(Parser)]
struct CliArgs {
    /// The path to the pyproject file containing uv workspace details
    #[arg(short = 'p', long ="pyproject")]
    path: String,
}

fn main() {
    let args = CliArgs::parse();
    let members = find_workspace_members(&Path::new(&args.path));
    let files = git_status_files();
    let mut processed: Vec<String> = vec![];
    for file in &files {
        if file.len() != 0 {
            if is_workspace_member(&members, file) {
                println!("Detected changes in {}, which is part of the workspace", file);
                match find_pyproject(Path::new(file)) {
                    Ok(result) => ask_and_save_version_bump(&result.join("pyproject.toml"), &mut processed),
                    Err(e) => println!("{}", e),
                }
            }
        }
    }
}
