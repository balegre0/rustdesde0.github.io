struct Ruidoso(&'static str);

impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("  drop: {}", self.0);
    }
}

fn main() {
    let r = Ruidoso("primero");
    println!("r = {}", r.0);
    // 1. Sombrear crea una variable NUEVA con el mismo nombre...
    let r = Ruidoso("segundo");
    println!("r = {}", r.0);
    // 2. ...pero la vieja sigue siendo dueña de su valor hasta el final del ámbito
    println!("fin de main");
}
