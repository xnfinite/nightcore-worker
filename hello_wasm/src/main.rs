use std::fs;
use std::io::{self, Read};

fn main() {
    println!("👋 Hello from inside WASM!");

    // Try to open the sandbox file
    match fs::File::open("/sandbox/msg.txt") {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Err(e) = file.read_to_string(&mut contents) {
                eprintln!("⚠️ Failed to read /sandbox/msg.txt: {e}");
                return;
            }
            println!("📄 Read from /sandbox/msg.txt:\n{}", contents);
        }
        Err(e) => {
            eprintln!("🚫 Could not open /sandbox/msg.txt (maybe no permission?): {e}");
        }
    }
}
