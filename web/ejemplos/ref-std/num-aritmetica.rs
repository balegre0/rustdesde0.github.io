fn main() {
    let x: u8 = 250;
    // checked: Option, None si se sale del rango
    println!("{:?} {:?}", x.checked_add(5), x.checked_add(10));
    // wrapping: da la vuelta (módulo 256)
    println!("{}", x.wrapping_add(10));
    // saturating: se queda en el límite
    println!("{} {}", x.saturating_add(10), 3u8.saturating_sub(5));
    // overflowing: el resultado con vuelta y si hubo desbordamiento
    println!("{:?}", x.overflowing_add(10));
    // límites de cada tipo
    println!("{} {} {}", u8::MAX, i8::MIN, i32::MAX);
    println!("{:?} {:?}", 7i32.checked_div(0), (-128i8).checked_abs());
    println!("{} {}", (-7i32).rem_euclid(3), 2u32.pow(10));
    // el operador + normal: pánico en debug, vuelta en release
    let y = std::hint::black_box(x);
    println!("{}", y + 10);
}
