fn main() {
    // 1. Un String con sitio para 4 bytes
    let mut s = String::from("hola");
    // 2. Otro String que se coloca justo detrás en el heap
    let vecino = String::from("vecino");
    println!("len {:>2}, cap {:>2}, bytes en {:p}  «{s}»", s.len(), s.capacity(), s.as_ptr());
    println!("el vecino está en {:p}", vecino.as_ptr());

    // 3. Al crecer, ya no cabe en su hueco: se reserva otro más grande y se copian los bytes
    for trozo in [", mundo", " ¡qué tal", " estás!"] {
        s.push_str(trozo);
        println!("len {:>2}, cap {:>2}, bytes en {:p}  «{s}»", s.len(), s.capacity(), s.as_ptr());
    }
}
