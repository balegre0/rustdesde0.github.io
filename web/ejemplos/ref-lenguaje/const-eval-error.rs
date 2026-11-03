const fn dividir(a: u32, b: u32) -> u32 {
    a / b
}

// El error ocurre al compilar: la división entre cero se evalúa en el compilador.
const X: u32 = dividir(10, 0);

fn main() {
    println!("{X}");
}
