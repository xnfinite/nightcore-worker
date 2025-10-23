#[no_mangle]
pub extern "C" fn _start() {
    let a = 7;
    let b = 5;
    let sum = a + b;
    println!("🧮 Math module says: {a} + {b} = {sum}");
}
