fn main() {
    let s = String::from("hola");
    println!("{}", size_of::<String>() == 3 * size_of::<usize>());
    println!("{}", s.len());
}
