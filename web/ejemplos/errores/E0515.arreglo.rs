fn crear() -> String {
    let s = String::from("temporal");
    s // devolvemos la propiedad, no una referencia
}

fn main() {
    println!("{}", crear());
}
