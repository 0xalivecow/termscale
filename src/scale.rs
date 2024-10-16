use std::process::{Command, Output};

pub fn ts_status() -> String {
    let command_output = Command::new("tailscale")
        .arg("status")
        .arg("--json")
        .output()
        .expect("command failed");
    let string = String::from_utf8(command_output.stdout).unwrap();
    println!("{}", string);
    string
}
