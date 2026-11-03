// Edición 2024: los bloques extern deben marcarse `unsafe extern`.
extern "C" {
    fn abs(x: i32) -> i32;
}

fn main() {
    println!("{}", unsafe { abs(-3) });
}
