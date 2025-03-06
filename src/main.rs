use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use version_compare::Version;

fn main() -> io::Result<()> {
    let path = Path::new("versions.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }

    let mut versions: Vec<Version> = Vec::new();
    for line in &lines {
        match Version::from(line.as_str()) {
            Some(version) => versions.push(version),
            None => eprintln!("Invalid version format: {}", line),
        }
    }

    // this panics
    versions.sort_by(|a, b| a.compare(b).ord().unwrap());

    for version in versions {
        println!("{}", version);
    }

    Ok(())
}
