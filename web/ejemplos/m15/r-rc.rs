use std::rc::Rc;

fn main() {
    let foto = Rc::new(String::from("playa.jpg"));
    // Tres álbumes comparten la misma foto, sin copiarla.
    let albumes: Vec<Rc<String>> = (0..3).map(|_| Rc::clone(&foto)).collect();
    println!("álbumes = {}", albumes.len());
    println!("dueños = {}", Rc::strong_count(&foto));
}
