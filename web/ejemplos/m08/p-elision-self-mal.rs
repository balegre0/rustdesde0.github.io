struct Lector<'a> {
    texto: &'a str,
}

impl<'a> Lector<'a> {
    fn otro(&self, otro: &str) -> &str {
        let _ = self.texto;
        otro
    }
}

fn main() {
    let l = Lector { texto: "abc" };
    println!("{}", l.otro("zzz"));
}
