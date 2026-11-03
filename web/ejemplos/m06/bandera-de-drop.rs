struct Ruidoso(&'static str);

impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("  drop: {}", self.0);
    }
}

fn consumir(r: Ruidoso) {
    println!("  consumo {}", r.0);
} // si llegó aquí, se libera aquí

fn main() {
    for urgente in [true, false] {
        println!("urgente = {urgente}");
        let r = Ruidoso("nota");
        if urgente {
            consumir(r);
        }
        println!("  fin de la vuelta");
        // si `r` no se movió, se libera aquí; si se movió, NO se libera otra vez
    }
}
