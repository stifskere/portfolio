use chrono::Utc;


fn main() {
    println!("cargo:rustc-env=BUILD_TIME={}", Utc::now().format("%Y-%M-%d %H:%m:%S"));
}
