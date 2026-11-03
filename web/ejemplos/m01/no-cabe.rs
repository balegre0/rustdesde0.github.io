// Adelanto: ¿qué pasa si sumas 1 al máximo? (el Mundo 3 lo trata a fondo)
fn main() {
    let max: u8 = 255;

    // 1. Pedir la suma «con comprobación»: None significa «no cabe»
    println!("255 + 1 comprobado:  {:?}", max.checked_add(1));

    // 2. Pedir la suma «dando la vuelta»: el bit que sobra se pierde
    println!("255 + 1 con vuelta:  {}", max.wrapping_add(1));
    println!("en bits: {:08b} + 1 = 1{:08b}", max, max.wrapping_add(1));
}
