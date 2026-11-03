fn main() {
    let s = String::from("hola");
    let r = &s;
    println!("préstamo: {r}"); // último uso del préstamo
    let t = s; // ahora sí se puede mover
    println!("nueva dueña: {t}");
}
