use std::env;
fn main() {
    let target_feature = env::var("CARGO_CFG_TARGET_FEATURE").unwrap();
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("/home/edvard/cargo-build-rs-rustflags/target_features.txt")
        .unwrap();

    println!("{}", target_feature);
    writeln!(file, "{}\n", target_feature).expect("Couldn't write to file");
}
