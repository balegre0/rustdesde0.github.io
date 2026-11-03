macro_rules! saluda {
    () => { println!("¡Hola!") };
    ($nombre:expr) => { println!("¡Hola, {}!", $nombre) };
}

fn main() {
    saluda!("Ana", "Luis");
}
