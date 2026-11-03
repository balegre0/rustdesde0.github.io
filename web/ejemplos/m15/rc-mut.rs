use std::rc::Rc;

fn main() {
    let lista = Rc::new(vec![1, 2, 3]);
    let otra = Rc::clone(&lista);
    lista.push(4); // ¿añadir a una lista compartida?
    println!("{:?}", otra);
}
