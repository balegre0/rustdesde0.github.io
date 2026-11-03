trait Clonable {
    fn duplicar(&self) -> Box<dyn Clonable>;
    fn mostrar(&self) -> String;
}

impl Clonable for i32 {
    fn duplicar(&self) -> Box<dyn Clonable> {
        Box::new(*self)
    }
    fn mostrar(&self) -> String {
        self.to_string()
    }
}

fn main() {
    let cosas: Vec<Box<dyn Clonable>> = vec![Box::new(1)];
    let copia = cosas[0].duplicar();
    println!("{} {}", cosas.len(), copia.mostrar());
}
