trait Saludo {
    fn hola(&self);
}

struct Robot;

impl Saludo for Robot {
    fn hola(&self) {
        println!("hola");
    }
}

impl Robot {
    fn adios(&self) { // método propio del tipo, fuera del trait
        println!("adiós");
    }
}

fn main() {
    Robot.hola();
    Robot.adios();
}
