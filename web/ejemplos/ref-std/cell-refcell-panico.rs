use std::cell::RefCell;

fn main() {
    let dato = RefCell::new(String::from("hola"));
    let lector = dato.borrow();
    let mut escritor = dato.borrow_mut();
    escritor.push('!');
    println!("{lector}");
}
