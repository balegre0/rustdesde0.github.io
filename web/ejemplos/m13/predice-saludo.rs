trait Saludar {
    fn nombre(&self) -> String;

    // Método por defecto: usa `nombre`, que cada tipo pone a su manera.
    fn saludo(&self) -> String {
        format!("Hola, soy {}", self.nombre())
    }
}

struct Robot;
struct Gata;

impl Saludar for Robot {
    fn nombre(&self) -> String {
        String::from("R2")
    }
}

impl Saludar for Gata {
    fn nombre(&self) -> String {
        String::from("Misi")
    }
    // Gata reemplaza el método por defecto.
    fn saludo(&self) -> String {
        String::from("Miau")
    }
}

fn main() {
    println!("{}", Robot.saludo());
    println!("{}", Gata.saludo());
}
