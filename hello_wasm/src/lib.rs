#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello from inside WASM!");
}

