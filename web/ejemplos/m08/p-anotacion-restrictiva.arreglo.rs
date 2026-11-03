fn primera<'a>(a: &'a str, b: &str) -> &'a str {
    let _ = b;
    a
}

fn main() {
    let a = String::from("hola");
    let r;
    {
        let b = String::from("adiós");
        r = primera(&a, &b);
    }
    println!("{r}");
}
