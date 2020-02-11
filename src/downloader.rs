use std::io;
use std::fs::File;

fn download() {
    let mut resp = reqwest::get("https://sh.rustup.rs").expect("request failed");
    let mut out = File::create("rustup-init.sh").expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
}
