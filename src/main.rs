use std::process::Command;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
}

fn main() {
    // let args = Cli::parse();

    let output = Command::new("git").arg("log").arg("-2").arg(r#"-pretty=format:"%h""#).output().unwrap();
    if !output.status.success() {
        println!("err");
        // error_chain::bail!("Command executed with failing error code");
    }
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines = stdout.lines();
    for line in lines {
        println!("{}", line);
    }
}