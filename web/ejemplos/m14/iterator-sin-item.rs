struct Siempre;

impl Iterator for Siempre {
    fn next(&mut self) -> Option<u8> {
        Some(1)
    }
}

fn main() {
    println!("{:?}", Siempre.next());
}
