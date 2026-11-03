fn alargar<'a>(x: &'a String) -> &'a String {
    x
}

fn main() {
    let r;
    {
        let s = String::from("hola");
        r = alargar(&s);
    }
    println!("{r}");
}
