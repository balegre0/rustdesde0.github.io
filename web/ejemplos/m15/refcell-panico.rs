use std::cell::RefCell;

fn main() {
    let notas = RefCell::new(vec![7, 9]);
    // 1. Un préstamo de lectura activo...
    let lectura = notas.borrow();
    println!("primera nota: {}", lectura[0]);
    // 2. ...y a la vez pedimos uno de escritura. Compila, pero en ejecución...
    notas.borrow_mut().push(10);
    println!("nunca llegamos aquí: {}", lectura.len());
}
