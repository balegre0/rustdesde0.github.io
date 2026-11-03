union Bits {
    entero: u32,
    real: f32,
}

fn main() {
    let b = Bits { entero: 7 };
    // Leer un campo de una union exige `unsafe`.
    println!("{}", b.real);
}
