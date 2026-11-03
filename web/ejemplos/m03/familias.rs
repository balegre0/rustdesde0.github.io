fn main() {
    let x: u8 = 250;

    // 1. checked_: Some(resultado) o None si se sale del rango
    println!("checked    250 + 3  = {:?}", x.checked_add(3));
    println!("checked    250 + 10 = {:?}", x.checked_add(10));

    // 2. wrapping_: da la vuelta (módulo 256), siempre
    println!("wrapping   250 + 10 = {}", x.wrapping_add(10));

    // 3. saturating_: se queda en el límite
    println!("saturating 250 + 10 = {}", x.saturating_add(10));
    println!("saturating 3 - 10   = {}", 3u8.saturating_sub(10));

    // 4. overflowing_: (resultado con vuelta, ¿hubo desbordamiento?)
    println!("overflowing 250 + 10 = {:?}", x.overflowing_add(10));
    println!("overflowing 250 + 3  = {:?}", x.overflowing_add(3));
}
