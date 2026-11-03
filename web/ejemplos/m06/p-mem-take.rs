fn main() {
    let mut v = vec![String::from("a"), String::from("b")];
    let sacado = std::mem::take(&mut v[0]);
    println!("{sacado} {:?}", v);
}
