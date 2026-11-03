use std::cell::RefCell;

fn main() {
    let c = RefCell::new(String::from("hola"));
    let a = c.borrow();
    let b = c.borrow();
    println!("{a} {b}");
    println!("¿escritura posible? {}", c.try_borrow_mut().is_ok());
}
