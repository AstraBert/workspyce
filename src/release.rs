use std::path::Path;
use std::fs;
use std::process::{Command};

pub fn release(token: &str) -> Result<bool, String> {
    let packages: String;
    match fs::exists(Path::new(".workspyce/release.txt")) {
        Ok(true) => {
            packages = fs::read_to_string(Path::new(".workspyce/release.txt")).expect("Should be able to read release.txt, since it exists.");
            let to_release: Vec<&str> = packages.split("\n").collect();
            for p in to_release {
                if p == "" {
                    continue;
                }
                println!("Building {}", p);
                let cmd = Command::new("uv").arg("build").arg(p.trim()).spawn().expect("Error executing `uv build`, check for uv installation");
                let _output = cmd.wait_with_output().expect("Error executing `uv build`, check for uv installation and that the package being build is a python package.");
            }
            println!("Publishing all built packages...");
            let cmd = Command::new("uv").arg("publish").arg("--token").arg(token).spawn().expect("Error executing `uv publish`, check for uv installation and that `./dist` has been correctly created during build.");
            let _output = cmd.wait_with_output().expect("Error executing `uv publish`, check for uv installation and that `./dist` has been correctly created during build.");
            fs::remove_file(Path::new(".workspyce/release.txt")).expect("should be able to remove `.workspyce/release.txt`");
            return Ok(true);
        }
        Ok(false) => {
            return Err("No release.txt file, nothing to release".to_string());
        }
        Err(e) => {
            return Err(format!("{}", e));
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_release_failure() {
        let tok = "not a token";
        match release(tok) {
            Ok(_b) => {
                eprint!("No error occurred, but one expected");
                assert!(false)
            },
            Err(e ) => {
                assert_eq!(e, "No release.txt file, nothing to release")
            }
        }
    }
}