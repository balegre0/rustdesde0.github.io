trait Hablar {
    fn hola(&self) -> String {
        String::from("hola")
    }
}

struct A;
struct B;

impl Hablar for A {}
impl Hablar for B {
    fn hola(&self) -> String {
        String::from("buenas")
    }
}

fn dos_veces(h: &dyn Hablar) -> String {
    format!("{0} {0}", h.hola())
}

fn main() {
    let v: Vec<Box<dyn Hablar>> = vec![Box::new(B), Box::new(A)];
    for h in &v {
        println!("{}", dos_veces(h.as_ref()));
    }
}
