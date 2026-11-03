struct Ticket {
    codigo: String,
}

impl Ticket {
    fn canjear(self) -> String {
        format!("canjeado {}", self.codigo)
    }
}

fn main() {
    let t = Ticket { codigo: String::from("A-7") };
    println!("{}", t.canjear());
    println!("{}", t.canjear());
}
