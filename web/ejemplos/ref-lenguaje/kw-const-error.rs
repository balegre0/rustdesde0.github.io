fn leer_numero() -> u32 {
    7
}

// Un `const` se calcula al compilar: solo puede llamar a funciones `const fn`.
const N: u32 = leer_numero();

fn main() {
    println!("{N}");
}
