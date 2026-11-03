use std::rc::Rc;

fn main() {
    let a = Rc::new(5);
    let w = Rc::downgrade(&a);
    let w2 = w.clone();
    println!("fuertes = {}, débiles = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    if let Some(r) = w.upgrade() {
        println!("dentro: fuertes = {}, valor = {r}", Rc::strong_count(&a));
    }
    println!("fuera: fuertes = {}", Rc::strong_count(&a));
    drop(w2);
    println!("débiles = {}", Rc::weak_count(&a));
}
