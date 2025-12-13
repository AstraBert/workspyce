use std::fs;
use regex::Regex;
use std::path::Path;
use std::process::{Command, Stdio};
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
    Err("Could not find a parent path that contains a pyproject.toml".to_string())
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
        if ! Path::new(".workspyce").exists() && let Err(why) = fs::create_dir(".workspyce") { 
            println!("Error while creating `.workspyce` directory: {:?}", why.kind()) 
        }
        let fl_name = format!(".workspyce/{}.md", generate_name().replace(" ", "-"));
        let contents = format!("---\npackage: {}\npyproject: {:?}\nrelease: {}\n---\n{}", project_name, pyproject, vbump.trim().to_lowercase(), changelog);
        if let Err(why) = fs::write(&fl_name, contents) { println!("Error while writing version changelog to `{}` directory: {:?}", &fl_name, why.kind()) }
        println!("Great! All changes have been saved :)")
    } else {
        println!("Gotcha! The change will be ignored :)")
    }
}

pub fn check(path: &str) {
    let members = find_workspace_members(Path::new(path));
    let files = git_status_files();
    let mut processed: Vec<String> = vec![];
    for file in &files {
        if !file.is_empty()
            && is_workspace_member(&members, file) {
                println!("Detected changes in {}, which is part of the workspace", file);
                match find_pyproject(Path::new(file)) {
                    Ok(result) => ask_and_save_version_bump(&result.join("pyproject.toml"), &mut processed),
                    Err(e) => println!("{}", e),
                }
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_workspace_members() {
        assert_eq!(find_workspace_members(Path::new("testfiles/proj/pyproject.toml"))[0], "testfiles/.*");
    }

    #[test]
    fn test_find_pyproject() {
        match find_pyproject(Path::new("testfiles/proj/childpath/anotherfile.txt")) {
            Ok(path) => {
                assert_eq!(path, Path::new("testfiles/proj/"))
            }
            Err(e) => {
                eprintln!("Error during test when no error should have happened: {}", e);
                assert!(false)
            }
        }
        match find_pyproject(Path::new("testfiles/notapyproject.toml")) {
            Ok(_path) => {
                eprint!("No error was thrown where it should have");
                assert!(false)
            }
            Err(e) => {
                assert_eq!(e, "Could not find a parent path that contains a pyproject.toml")
            }
        }
    }

    #[test]
    fn test_find_project_name() {
        assert_eq!(find_project_name(Path::new("testfiles/proj/pyproject.toml")), "toml-workspace");
    }    

    #[test]
    fn test_is_workspace_member() {
        let members: Vec<String> = vec!["packages/.*".to_string()];
        assert!(is_workspace_member(&members, "packages/coding_agent/hello.py"));
        assert!(!is_workspace_member(&members, "package/coding_agent/hello.py"))
    }

    #[test]
    // test that it does not throw an error
    fn test_git_status_files() {
        let _lines = git_status_files();
        assert!(true)
    }
}