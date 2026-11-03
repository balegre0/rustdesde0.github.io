struct Dado {
    caras: u32,
}

impl Dado {
    fn new(caras: u32) -> Self {
        Self { caras }
    }

    fn clasico() -> Self {
        Self::new(6)
    }

    fn describir(&self) -> String {
        format!("dado de {} caras", self.caras)
    }
}

fn main() {
    let d20 = Dado::new(20);
    let d6 = Dado::clasico();
    println!("{}", d20.describir());
    println!("{}", d6.describir());
}
