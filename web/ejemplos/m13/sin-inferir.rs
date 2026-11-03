fn vacia<T>() -> Vec<T> {
    Vec::new()
}

fn main() {
    let v = vacia();
    println!("{}", v.len());
}
