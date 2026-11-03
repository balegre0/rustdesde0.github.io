fn mostrar<T: std::fmt::Debug>(t: T) {
    println!("{t:?}");
}

fn main() {
    let x = 5;
    let mut y = 6;
    mostrar(x);
    mostrar(&x);
    mostrar(&mut y);
}
