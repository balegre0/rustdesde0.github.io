struct Id<T> {
    valor: u64,
}

fn main() {
    let tam = std::mem::size_of::<Id<String>>();
    println!("{tam}");
}
