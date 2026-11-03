const fn dia_valido(d: u8) -> u8 {
    if d == 0 || d > 31 {
        panic!("día fuera de rango");
    }
    d
}

const DIA: u8 = dia_valido(40);

fn main() {
    println!("{DIA}");
}
