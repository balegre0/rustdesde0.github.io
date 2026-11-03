macro_rules! saluda {
    () => { println!("¡Hola!") };
    ($nombre:expr) => { println!("¡Hola, {}!", $nombre) };
    ($a:expr, $b:expr) => { println!("¡Hola, {} y {}!", $a, $b) };
}

fn main() {
    saluda!("Ana", "Luis");
}
