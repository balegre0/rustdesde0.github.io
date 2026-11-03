// 1. Las variables locales son higiénicas, pero los items (fn, struct...) no
macro_rules! crea_ayuda {
    () => {
        fn ayuda() -> &'static str { "soy un item creado por la macro" }
        struct Punto { x: i32 }
    };
}

crea_ayuda!();

fn main() {
    // 2. Podemos usar `ayuda` y `Punto` desde fuera
    println!("{}", ayuda());
    let p = Punto { x: 3 };
    println!("p.x = {}", p.x);
}
