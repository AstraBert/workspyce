use std::path::Path;
use std::fs;
use std::process::{Command};

pub fn release(token: &str) {
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
        }
        Ok(false) => {
            println!("No `.workspyce/release.txt` file found, nothing to release...");
            return;
        }
        Err(e) => println!("{}", e),
    }
}