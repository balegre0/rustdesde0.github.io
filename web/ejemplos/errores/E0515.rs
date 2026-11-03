fn crear<'a>() -> &'a String {
    let s = String::from("temporal");
    &s
}

fn main() {
    println!("{}", crear());
}
