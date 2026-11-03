use std::rc::Rc;

fn main() {
    let foto = Rc::new(String::from("playa.jpg"));
    // Crea un Vec con 3 Rc que apunten a la MISMA foto (usa Rc::clone).
    let albumes: Vec<Rc<String>> = todo!();
    println!("álbumes = {}", albumes.len());
    println!("dueños = {}", Rc::strong_count(&foto));
}
