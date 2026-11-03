// Un `bool` solo puede valer 0 o 1. Crear otro valor es UB.
// En un contexto const, el intérprete del compilador lo detecta y se niega a compilar.
const MALO: bool = unsafe { std::mem::transmute::<u8, bool>(3) };

fn main() {
    println!("{MALO}");
}
