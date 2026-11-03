fn mas_larga<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let a = String::from("sol");
    {
        let b = String::from("luna");
        let r = mas_larga(&a, &b);
        println!("{r}");
    }
}
