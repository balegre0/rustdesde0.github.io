fn main() {
    let mut v = vec![1, 2, 3];
    let hay_dos = v.contains(&2); // primero se lee...
    if hay_dos {
        v.push(20); // ...y luego se modifica
    }
    println!("{v:?}");
}
