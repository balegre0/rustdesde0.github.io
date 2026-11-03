// En la edición 2024 un bloque extern debe marcarse `unsafe extern`:
// declarar una función de fuera es una promesa que el compilador no puede comprobar.
extern "C" {
    fn abs(x: i32) -> i32;
}

fn main() {
    println!("{}", unsafe { abs(-3) });
}
