fn main() {
    let mut lista = vec![1, 2, 3];
    let a = &mut lista;
    a.push(4); // el primer préstamo exclusivo termina aquí
    let b = &mut lista;
    b.push(5);
    println!("{lista:?}");
}
