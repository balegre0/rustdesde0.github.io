use std::cell::RefCell;

fn main() {
    let notas = RefCell::new(vec![7, 9]);
    // 1. El préstamo de lectura vive solo dentro de este bloque.
    {
        let lectura = notas.borrow();
        println!("primera nota: {}", lectura[0]);
    } // aquí se devuelve
    // 2. Ahora sí podemos escribir.
    notas.borrow_mut().push(10);
    println!("notas: {:?}", notas.borrow());
    // 3. try_borrow_mut no entra en pánico: devuelve un Result.
    let l = notas.borrow();
    match notas.try_borrow_mut() {
        Ok(_) => println!("escritura concedida"),
        Err(e) => println!("escritura denegada: {e}"),
    }
    drop(l);
    println!("¿y ahora? {}", notas.try_borrow_mut().is_ok());
}
