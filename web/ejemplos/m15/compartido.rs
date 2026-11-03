use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // 1. Una lista de la compra con dos dueños que pueden escribir en ella.
    let lista = Rc::new(RefCell::new(Vec::new()));
    let ana = Rc::clone(&lista);
    let luis = Rc::clone(&lista);
    // 2. Cada uno añade cosas: borrow_mut() comprueba en ejecución.
    ana.borrow_mut().push("pan");
    luis.borrow_mut().push("leche");
    ana.borrow_mut().push("café");
    println!("lista = {:?}", lista.borrow());
    println!("dueños = {}", Rc::strong_count(&lista));
}
