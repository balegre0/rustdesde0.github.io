fn mas_larga<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    println!("{}", mas_larga("uno", "tres"));
}
