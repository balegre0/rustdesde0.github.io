use std::cell::RefCell;

// Este programa compila, pero entra en pánico con «RefCell already borrowed».
// Arréglalo para que imprima [10, 20, 30].
fn main() {
    let puntos = RefCell::new(vec![10, 20]);
    let lectura = puntos.borrow();
    let total: i32 = lectura.iter().sum();
    puntos.borrow_mut().push(total);
    println!("{:?}", puntos.borrow());
}
