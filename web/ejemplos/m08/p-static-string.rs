fn guardar<T: 'static + std::fmt::Debug>(t: T) {
    println!("{t:?}");
}

fn main() {
    let s = String::from("vivo poco");
    guardar(s);
    guardar(42);
    guardar("literal");
}
