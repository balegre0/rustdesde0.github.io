// 0xD800 es un «suplente» (surrogate): no es un valor escalar Unicode.
const MALO: char = unsafe { std::mem::transmute::<u32, char>(0xD800) };

fn main() {
    println!("{MALO}");
}
