fn main() {
    let x = 5;
    let p = &x as *const i32;
    // Desreferenciar un puntero crudo es una de las operaciones que exigen `unsafe`.
    println!("{}", *p);
}
