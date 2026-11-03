#[derive(Clone, Copy)]
struct Ficha(u8);

impl Drop for Ficha {
    fn drop(&mut self) {}
}

fn main() {
    let f = Ficha(1);
    let _g = f;
}
