use std::env::vars;

use dotenvy::dotenv;

fn main() {
    dotenv().ok();

    for (key, value) in vars() {
        let key = key.trim();
        let value = value.trim();
        println!("cargo:rustc-env={key}={value}");
    }
}
