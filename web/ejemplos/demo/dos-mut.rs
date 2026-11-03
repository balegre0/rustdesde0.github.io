fn main() {
    let mut lista = vec![1, 2, 3];
    let a = &mut lista;
    let b = &mut lista;
    a.push(4);
    b.push(5);
    println!("{lista:?}");
}
