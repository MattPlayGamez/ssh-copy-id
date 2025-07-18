use std::env::args;
use std::process::{Command, Stdio};
use std::process::exit;

/// The main entry point of the application.
///
/// This function is called when the application is started. It checks the
/// command line arguments and prints a usage message if there are too few
/// arguments or if the user requested help. Otherwise, it copies the public
/// SSH key to the specified location.
///
/// The public key is read from the file `id_rsa.pub` in the user's
/// home directory. The private key is not used.
///
/// The public key is appended to the file `authorized_keys` in the user's
/// home directory on the remote machine. If the file does not exist, it is
/// created.
///
/// The user is asked to enter their password when connecting to the remote
/// machine. The password is not stored anywhere.
///
/// # Example
///
/// To copy the public key to the user `matt` on the host `192.168.0.2`, run
/// the following command:
///
///
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
