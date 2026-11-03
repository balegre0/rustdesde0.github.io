struct Ruidoso(&'static str);

impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("adiós, {}", self.0);
    }
}

#[allow(dead_code)] // los campos solo están para ver cuándo se destruyen
struct Pareja {
    primero: Ruidoso,
    segundo: Ruidoso,
}

impl Drop for Pareja {
    fn drop(&mut self) {
        println!("adiós, pareja");
    }
}

fn main() {
    let _p = Pareja { segundo: Ruidoso("segundo"), primero: Ruidoso("primero") };
    println!("fin de main");
}
