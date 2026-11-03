use std::cell::RefCell;

fn main() {
    let lista = RefCell::new(vec![1, 2]);
    // préstamo exclusivo comprobado al ejecutar
    lista.borrow_mut().push(3);
    {
        let a = lista.borrow();
        let b = lista.borrow(); // varios préstamos compartidos: bien
        println!("{:?} {}", *a, b.len());
        // mientras haya lectores, no se puede pedir borrow_mut
        println!("{}", lista.try_borrow_mut().is_err());
    }
    println!("{:?}", lista.borrow());
}
