fn main() {
    let mut s = String::from("hola");

    let a = &s; // préstamo compartido
    let b = &s; // otro préstamo compartido: sin problema
    println!("{a} {b}"); // último uso de a y b

    let c = &mut s; // préstamo exclusivo: a y b ya no se usan
    c.push('!');
    println!("{c}"); // último uso de c

    println!("{s}"); // la dueña recupera todos sus permisos
}
