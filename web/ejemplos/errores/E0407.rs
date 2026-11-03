trait Saludo {
    fn hola(&self);
}

struct Robot;

impl Saludo for Robot {
    fn hola(&self) {
        println!("hola");
    }
    fn adios(&self) {
        println!("adiós");
    }
}

fn main() {
    Robot.hola();
}
