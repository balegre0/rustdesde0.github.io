fn main() {
    let x = 5;
    let r = &x;
    // `*r` sigue la flecha: es el valor al que apunta r.
    let y = *r + 1;
    println!("x = {x}, *r = {}, y = {y}", *r);

    // Con métodos, Rust sigue la flecha él solo (auto-deref).
    let s = String::from("ñandú");
    let rs = &s;
    println!("{} {} {}", rs.len(), (*rs).len(), s.len());

    // Comparar referencias compara los VALORES, no las direcciones.
    let a = 7;
    let b = 7;
    println!("{}", &a == &b);
}
