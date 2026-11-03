struct Rectangulo {
    ancho: u32,
    alto: u32,
}

fn main() {
    let r = Rectangulo { ancho: 8, alto: 5 };
    println!("Área: {}", r.ancho * r.alto);
}
