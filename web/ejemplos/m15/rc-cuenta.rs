use std::rc::Rc;

fn main() {
    // 1. Un texto en el heap con un contador de dueños.
    let a = Rc::new(String::from("compartido"));
    println!("tras crear a: {}", Rc::strong_count(&a));
    // 2. Rc::clone no copia el texto: suma 1 al contador.
    let b = Rc::clone(&a);
    println!("tras clonar b: {}", Rc::strong_count(&a));
    {
        let c = Rc::clone(&a);
        println!("dentro del bloque: {} (c dice «{c}»)", Rc::strong_count(&a));
    }
    // 3. Al morir c, el contador baja.
    println!("tras el bloque: {}", Rc::strong_count(&a));
    // 4. a y b apuntan a la misma dirección.
    println!("¿mismo texto? {}", Rc::ptr_eq(&a, &b));
}
