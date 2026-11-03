trait Clonable {
    fn duplicar(&self) -> Self;
}

impl Clonable for i32 {
    fn duplicar(&self) -> Self {
        *self
    }
}

fn main() {
    let cosas: Vec<Box<dyn Clonable>> = vec![Box::new(1)];
    println!("{}", cosas.len());
}
