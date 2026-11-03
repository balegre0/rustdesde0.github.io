fn main() {
    let mut lista = vec![|x: i32| x + 1];
    lista.push(|x: i32| x * 2);
    println!("{}", lista.len());
}
