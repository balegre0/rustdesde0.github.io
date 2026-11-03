// 1. Recursión: contar es 1 + contar el resto
macro_rules! contar {
    () => { 0usize };
    ($cabeza:tt $($resto:tt)*) => { 1usize + contar!($($resto)*) };
}

fn main() {
    println!("{}", contar!());
    println!("{}", contar!(a));
    println!("{}", contar!(a b c d));
    // 2. Al ser una expresión constante, sirve para el tamaño de un array
    const N: usize = contar!(x y z);
    let arr = [0u8; N];
    println!("arr tiene {} elementos", arr.len());
}
