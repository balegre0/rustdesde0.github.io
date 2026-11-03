fn main() {
    let mut v = vec![String::from("a")];
    let primero = &v[0];
    v.clear();
    println!("{primero}");
}
