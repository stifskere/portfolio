use std::process::Command;

use chrono::Utc;

fn main() {
    println!("cargo::rerun-if-changed=../translations/*");

    // Set the build date as a compile time environment variable.
    println!("cargo:rustc-env=BUILD_TIME={}", Utc::now().format("%Y-%M-%d %H:%m:%S"));

    // Set the rustc version as a compile time environment variable.
    // if the variable is unknown, this prints <unknown>
    println!(
        "cargo:rustc-env=RUSTC_VERSION={}",
        Command::new("rustc")
            .arg("-V")
            .output()
            .ok()
            .and_then(|version| String::from_utf8(version.stdout).ok())
            .unwrap_or_else(|| "<unknown>".to_string())
    )
}
