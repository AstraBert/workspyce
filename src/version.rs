use std::path::Path;
use std::{fs, path::PathBuf};
use regex::Regex;

fn list_all_changelogs() -> Vec<PathBuf> {
    let mut changelogs: Vec<PathBuf> = vec![];
    match fs::read_dir(".workspyce/") {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let pt = entry.path();
                        let ext = pt.extension().expect("should be able to get an extension for the file");
                        if ext == "md" {
                            changelogs.push(pt);
                        }
                    },
                    Err(e) => eprintln!("Error while reading the `.workspyce` directory: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error while reading the `.workspyce` directory:: {}", e),
    }
    changelogs
}

fn bump_all_versions(changelog_paths: Vec<PathBuf>) {
    let mut to_release: Vec<String> = vec![];
    let re_package = Regex::new(r#"(?m)^package:\s*(.*)$"#).expect("Regex should compile");
    let re_pyproject = Regex::new(r#"(?m)^pyproject:\s*(.*)$"#).expect("Regex should compile");
    let re_vbump = Regex::new(r#"(?m)^release:\s*(.*)$"#).expect("Regex should compile");
    let re_version = Regex::new(r#"(?m)^version\s*=\s*"(\d+\.\d+\.\d+)"$"#).expect("Regex should compile correctly");
    for path in &changelog_paths {
        let content = fs::read_to_string(path).expect("should be able to read file, but failed");
        let package = re_package.captures(&content).expect("Package should be part of the changelog file");
        let pyproject = re_pyproject.captures(&content).expect("Pyproject path should be part of the changelog file");
        let vbumb = re_vbump.captures(&content).expect("Version bump information should be part of the chang elog file.");
        let package_name = package[1].to_string();
        let pyproject_path = pyproject[1].to_string().replace("\"", "");
        let package_path = pyproject_path.replace("/pyproject.toml", "");
        to_release.push(package_path);
        let vbump_release = vbumb[1].to_string();
        let changelog_messages: Vec<&str> = content.split("---").collect();
        let changelog_message = changelog_messages[2];
        let pyp_content = fs::read_to_string(Path::new(&pyproject_path)).expect("Should be able to read the path to pyproject");
        let version = re_version.captures(&pyp_content).expect("pyproject.toml file should contain a version field");
        let semver = version[1].to_string();
        let ver_components: Vec<&str> = semver.split(".").collect();
        // assert that the project follow semantic versioning, throw an error if not
        assert_eq!(ver_components.len(), 3);
        let new_semver: String = if vbump_release.trim() == "patch" {
            let num: i32 = ver_components[2].parse().expect("Version components should be numbers");
            format!("{}.{}.{}", ver_components[0], ver_components[1], num + 1)
        } else if vbump_release.trim() == "minor" {
            let num: i32 = ver_components[1].parse().expect("Version components should be numbers");
            format!("{}.{}.0", ver_components[0], num + 1)
        } else {
            let num: i32 = ver_components[0].parse().expect("Version components should be numbers");
            format!("{}.0.0", num + 1)
        };
        let new_content = pyp_content.replace(&semver, &new_semver);
        fs::write(Path::new(&pyproject_path), new_content).expect("no error while writing file");
        match fs::exists(Path::new("CHANGELOG.md")) {
            Ok(true) => {
                let changelog_content = fs::read_to_string(Path::new("CHANGELOG.md")).expect("Since CHANGELOG.md exists in .workspyce/, we should be able to read it");
                let new_content = format!("## {}\n\n{}\n\n", package_name, changelog_message);
                let full_content = new_content + &changelog_content;
                fs::write(Path::new("CHANGELOG.md"), full_content).expect("Should be able to write file");
            }
            Ok(false) => {
                let full_content = format!("## {} {}\n\n{}\n\n", package_name, new_semver,  changelog_message);
                fs::write(Path::new("CHANGELOG.md"), full_content).expect("Should be able to write file");
            }
            Err(e) => println!("{}", e),
        }
        fs::remove_file(path).expect("Should be able to delete file");
    }
    let to_release_str = to_release.join("\n");
    fs::write(".workspyce/release.txt", to_release_str).expect("Should be able to write release.txt");
}

pub fn version() {
    let paths = list_all_changelogs();
    bump_all_versions(paths);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_list_all_changelog() {
        let paths = list_all_changelogs();
        assert_eq!(paths[0], Path::new(".workspyce/test-file.md"))
    }
}