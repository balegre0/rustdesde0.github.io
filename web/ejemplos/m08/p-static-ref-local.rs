fn guardar<T: 'static + std::fmt::Debug>(t: T) {
    println!("{t:?}");
}

fn main() {
    let s = String::from("hola");
    guardar(&s);
}
