use std::env::args;
use std::process::{Command, Stdio};
use std::process::exit;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        eprintln!("Too few arguments.\nUsage: {} <SSH User@Host; e.g. matt@192.168.0.2>", args[0]);
        exit(1);
    }

    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        println!("Usage: {} <SSH User@Host; e.g. matt@192.168.0.2>", args[0]);
        return;
    }

    let ssh_location = &args[1];

    let powershell_command = format!(
        "Get-Content \"$env:USERPROFILE\\.ssh\\id*.pub\" | ssh {} \"mkdir -p ~/.ssh && cat >> ~/.ssh/authorized_keys\"",
        ssh_location
    );

    let status = Command::new("powershell")
        .args(&["-Command", &powershell_command])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to run PowerShell command");

    if !status.success() {
        eprintln!("Command failed with status: {}", status);
        exit(status.code().unwrap_or(1));
    } else {
        println!("✅ Public key copied successfully to {}", ssh_location);
    }
}
