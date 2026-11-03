fn crear() -> &String {
    let s = String::from("hola");
    &s
}

fn main() {
    println!("{}", crear());
}
