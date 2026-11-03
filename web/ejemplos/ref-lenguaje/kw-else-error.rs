fn main() {
    let o: Option<i32> = None;
    // El `else` de un `let ... else` tiene que salir (return, break, panic!...).
    let Some(x) = o else {
        println!("nada");
    };
    println!("{x}");
}
