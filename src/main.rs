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

    // // Get the diff between the two commits and save it to a text file
    let file_name= format!("diff-{}.txt", chrono::Local::now().timestamp());
    let output = Command::new("git")
        .args(& ["diff", "--staged"])
        .output().unwrap();

    if !output.status.success() {
        panic!("git diff failed: {}", String::from_utf8(output.stderr).unwrap());
    }

    let diff = String::from_utf8(output.stdout).unwrap();
    std::fs::write(&file_name, diff).unwrap();
}