unsafe extern "C" {
    // Sin `safe`, cada llamada es una operación unsafe
    fn abs(x: i32) -> i32;
}

fn main() {
    println!("{}", abs(-3));
}
