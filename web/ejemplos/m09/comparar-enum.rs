enum Modo {
    Claro,
    Oscuro,
}

fn main() {
    let m = Modo::Oscuro;
    if m == Modo::Oscuro {
        println!("modo oscuro");
    }
    let _otro = Modo::Claro;
}
