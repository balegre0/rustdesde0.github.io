use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("compartido"));
    println!("dueños: {}", Rc::strong_count(&a));
    let b = Rc::clone(&a); // no copia el String: suma 1 al contador
    {
        let c = Rc::clone(&a);
        println!("dueños: {} ({c})", Rc::strong_count(&a));
    } // c sale de ámbito: resta 1
    println!("dueños: {}", Rc::strong_count(&a));
    println!("{}", Rc::ptr_eq(&a, &b));
    drop(b);
    // con un único dueño, se puede recuperar el valor
    let s: String = Rc::try_unwrap(a).unwrap();
    println!("{s}");
}
