#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u8)]
enum Color { Rojo = 0, Verde = 1 }

// 7 no es el discriminante de ninguna variante.
const MALO: Color = unsafe { std::mem::transmute::<u8, Color>(7) };

fn main() {
    println!("{MALO:?}");
}
