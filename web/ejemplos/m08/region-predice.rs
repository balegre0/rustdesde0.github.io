fn main() {
    let a = String::from("abc");
    let r;
    {
        let b = String::from("xy");
        r = &a;
        println!("dentro: {r} {b}");
    }
    println!("fuera: {r}");
}
