// En const, reinterpretar un puntero como entero es UB: el entero llevaría «procedencia».
static X: u8 = 0;
const MALO: usize = unsafe { std::mem::transmute::<&u8, usize>(&X) };

fn main() {
    println!("{MALO}");
}
