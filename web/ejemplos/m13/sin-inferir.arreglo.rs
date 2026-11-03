fn vacia<T>() -> Vec<T> {
    Vec::new()
}

fn main() {
    // Con turbofish: ::<T> dice qué tipo usar.
    let v = vacia::<String>();
    // O con una anotación en la variable.
    let w: Vec<i32> = vacia();
    println!("{} {}", v.len(), w.len());
}
