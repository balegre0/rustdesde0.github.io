struct Lector<'a> {
    texto: &'a str,
}

impl<'a> Lector<'a> {
    fn otro<'b>(&self, otro: &'b str) -> &'b str {
        let _ = self.texto;
        otro
    }
}

fn main() {
    let l = Lector { texto: "abc" };
    println!("{}", l.otro("zzz"));
}
