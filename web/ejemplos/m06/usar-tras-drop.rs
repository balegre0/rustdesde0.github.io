fn main() {
    let s = String::from("secreto");
    drop(s);
    println!("{s}");
}
