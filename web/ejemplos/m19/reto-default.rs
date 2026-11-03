#[derive(Debug)]
struct Config {
    hilos: u32,
    verboso: bool,
    nombre: String,
}

impl Default for Config {
    fn default() -> Self {
        Config { hilos: Default::default(), verboso: Default::default(), nombre: Default::default() }
    }
}

impl PartialEq for Config {
    fn eq(&self, otro: &Self) -> bool {
        self.hilos == otro.hilos && self.verboso == otro.verboso && self.nombre == otro.nombre
    }
}

fn main() {
    let c = Config { hilos: 0, verboso: false, nombre: String::new() };
    println!("{:?}", c);
    println!("{:?}", Config::default());
    println!("{}", c == Config::default());
}
