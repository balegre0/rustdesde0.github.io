struct Cursor<'a> {
    texto: &'a str,
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn siguiente(&mut self) -> Option<&'a str> {
        let resto = &self.texto[self.pos..];
        let fin = resto.find(',').unwrap_or(resto.len());
        if resto.is_empty() {
            return None;
        }
        self.pos += fin + if fin < resto.len() { 1 } else { 0 };
        Some(&resto[..fin])
    }
}

fn main() {
    let datos = String::from("a,bb,ccc");
    let mut c = Cursor { texto: &datos, pos: 0 };
    let x = c.siguiente();
    let y = c.siguiente();
    println!("{:?} {:?} {:?} {:?}", x, y, c.siguiente(), c.siguiente());
}
