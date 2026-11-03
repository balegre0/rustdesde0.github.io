fn crear() -> String {
    // 1. La función crea un String y entrega la propiedad a quien la llama
    String::from("recién hecho")
}

fn medir(s: String) -> (String, usize) {
    // 2. Recibe la propiedad, mide y la devuelve junto con el resultado
    let largo = s.len();
    (s, largo)
}

fn main() {
    let a = crear();
    // 3. Hay que recoger lo que nos devuelven, o se perdería (y se liberaría)
    let (a, largo) = medir(a);
    println!("«{a}» mide {largo} bytes");
}
