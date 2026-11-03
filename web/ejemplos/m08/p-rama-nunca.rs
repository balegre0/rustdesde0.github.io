fn main() {
    let a = String::from("a");
    let mut r = &a;
    {
        let b = String::from("b");
        if a.len() > 100 {
            r = &b;
        }
    }
    println!("{r}");
}
