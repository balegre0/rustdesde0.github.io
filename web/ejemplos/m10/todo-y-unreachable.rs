fn nivel(puntos: u32) -> &'static str {
    match puntos {
        0..=99 => "novato",
        100..=999 => "experto",
        _ => todo!("aún no he decidido qué pasa con 1000 o más"),
    }
}

fn main() {
    println!("{}", nivel(50));
    println!("{}", nivel(500));
    println!("{}", nivel(5000));
}
