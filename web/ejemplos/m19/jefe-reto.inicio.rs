// Escribe `menu!` para que genere:
//   - un enum `Plato` con las variantes dadas,
//   - un método `precio(&self) -> u32` con el precio de cada una,
//   - una función `carta() -> Vec<(&'static str, u32)>` con todos los platos.
macro_rules! menu {
    ($($plato:ident = $precio:literal),* $(,)?) => {
        #[derive(Debug, Clone, Copy)]
        enum Plato { Nada }
        impl Plato { fn precio(&self) -> u32 { 0 } }
        fn carta() -> Vec<(&'static str, u32)> { Vec::new() }
    };
}

menu! {
    Gazpacho = 6,
    Paella = 14,
    Flan = 4,
}

fn main() {
    for (nombre, precio) in carta() {
        println!("{:<10}{:>3} €", nombre, precio);
    }
    let total: u32 = carta().iter().map(|(_, p)| p).sum();
    println!("total: {} €", total);
}
