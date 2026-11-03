macro_rules! menu {
    ($($plato:ident = $precio:literal),* $(,)?) => {
        #[derive(Debug, Clone, Copy)]
        #[allow(dead_code)]
        enum Plato { $($plato),* }
        impl Plato {
            #[allow(dead_code)]
            fn precio(&self) -> u32 {
                match self { $(Plato::$plato => $precio),* }
            }
        }
        fn carta() -> Vec<(&'static str, u32)> {
            vec![$((stringify!($plato), Plato::$plato.precio())),*]
        }
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
