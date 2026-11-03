trait Animal {
    fn nombre(&self) -> String;
    fn sonido(&self) -> &'static str;
}

struct Perro;
struct Gato {
    vidas: u8,
}

impl Animal for Perro {
    fn nombre(&self) -> String {
        String::from("Perro")
    }
    fn sonido(&self) -> &'static str {
        "guau"
    }
}

impl Animal for Gato {
    fn nombre(&self) -> String {
        format!("Gato ({} vidas)", self.vidas)
    }
    fn sonido(&self) -> &'static str {
        "miau"
    }
}

// 1. `&dyn Animal`: «una referencia a algo que es un Animal; ya veremos cuál».
fn presentar(a: &dyn Animal) {
    println!("{} dice {}", a.nombre(), a.sonido());
}

fn main() {
    // 2. Un Vec con tipos DISTINTOS dentro: cada uno en su Box.
    let zoo: Vec<Box<dyn Animal>> = vec![Box::new(Perro), Box::new(Gato { vidas: 7 }), Box::new(Perro)];

    // 3. El método correcto se elige al ejecutar, mirando la vtable de cada uno.
    for a in &zoo {
        presentar(a.as_ref());
    }

    // 4. Una referencia normal ocupa una palabra; una a `dyn` ocupa dos.
    println!("&Perro: {} bytes", size_of::<&Perro>());
    println!("&dyn Animal: {} bytes", size_of::<&dyn Animal>());
}
