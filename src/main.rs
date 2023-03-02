use std::process::Command;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
}

fn main() {
    // let args = Cli::parse();
    let output = Command::new("git").arg("log").arg("-2").arg(r#"--pretty=format:"%h""#).output().unwrap();
    if !output.status.success() {
        println!("err");
    }
    let hashVec = String::from_utf8(output.stdout).unwrap();
    let hashVec: Vec<&str> = hashVec.split_whitespace().collect();
}