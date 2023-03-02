use std::hash::Hash;
use std::process::Command;
use clap::Parser;
use chrono;

#[derive(Parser)]
struct Cli {
    pattern: String,
}

fn main() {
    // let args = Cli::parse();

    // Get the latest two commits hash
    let output = Command::new("git").arg("log").arg("-2").arg(r#"--pretty=format:"%h""#).output().unwrap();
    if !output.status.success() {
        panic!("git log failed");
    }
    let hash = String::from_utf8(output.stdout).unwrap();
    let mut lines = hash.lines();
    let (from, to) = (lines.next().unwrap(), lines.next().unwrap());
    println!("{from} {to}");

    // Get the diff between the two commits and save it to a text file
    let file_name= format!("diff-{}.txt", chrono::Local::now().timestamp());
    let output = Command::new("git")
        .args(& ["diff", "1419b34", "bb1792e", "", &file_name])
        .output().unwrap();

    if !output.status.success() {
        panic!("git diff failed: {}", String::from_utf8(output.stderr).unwrap());
    }
}

// fatal: ambiguous argument ''1419b34'': unknown revision or path not in the working tree.