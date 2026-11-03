fn main() {
    let notas = vec![7, 9, 5];

    // 1. Lo que escribes:
    for n in &notas {
        print!("{n} ");
    }
    println!();

    // 2. Lo que el compilador hace (aproximadamente):
    let mut it = IntoIterator::into_iter(&notas);
    while let Some(n) = it.next() {
        print!("{n} ");
    }
    println!();
}
