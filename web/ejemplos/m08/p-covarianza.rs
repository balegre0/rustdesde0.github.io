fn elige<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a < b { a } else { b }
}

fn main() {
    let literal: &'static str = "zeta";
    let local = String::from("alfa");
    println!("{}", elige(literal, &local));
}
