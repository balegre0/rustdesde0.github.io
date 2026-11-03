struct Lector<'a> {
    texto: &'a str,
}

impl<'a> Lector<'a> {
    fn inicio(&self, otro: &str) -> &str {
        let _ = otro;
        &self.texto[..3]
    }
}

fn main() {
    let t = String::from("abcdef");
    let l = Lector { texto: &t };
    println!("{}", l.inicio("zzz"));
}
