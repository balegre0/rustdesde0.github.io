struct Siempre;

impl Iterator for Siempre {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        Some(1)
    }
}

fn main() {
    println!("{:?}", Siempre.next());
}
