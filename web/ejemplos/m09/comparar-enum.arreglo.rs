#[derive(PartialEq)]
enum Modo {
    Claro,
    Oscuro,
}

fn main() {
    let m = Modo::Oscuro;
    if m == Modo::Oscuro {
        println!("modo oscuro");
    }
    // Alternativa sin derive: la macro matches! usa un patrón.
    if matches!(m, Modo::Oscuro) {
        println!("también con matches!");
    }
    let _otro = Modo::Claro;
}
