fn main() {
    // 1. Enteros: rango según bits y signo.
    println!("u8: {}..={}", u8::MIN, u8::MAX);
    println!("i8: {}..={}", i8::MIN, i8::MAX);
    println!("i128 máx: {}", i128::MAX);
    // 2. Flotantes IEEE 754.
    println!("{} {}", 0.1 + 0.2, f64::EPSILON);
    println!("{} {} {}", 1.0 / 0.0, -1.0 / 0.0, (0.0_f64 / 0.0).is_nan());
    // 3. char: un valor escalar Unicode, 4 bytes, no un byte.
    let c = 'ñ';
    println!("{} {} bytes, U+{:04X}, en UTF-8 ocupa {}", c, size_of::<char>(), c as u32, c.len_utf8());
    // 4. str: texto UTF-8; len() cuenta bytes, no caracteres.
    let s = "año";
    println!("{} bytes, {} caracteres", s.len(), s.chars().count());
    // 5. Tupla, array y slice.
    let t: (i32, f64, char) = (1, 2.5, 'x');
    let a: [u8; 4] = [1, 2, 3, 4];
    let sl: &[u8] = &a[1..3];
    println!("{} {} {} | {:?} | {:?}", t.0, t.1, t.2, a, sl);
    // 6. Unidad y tipos de tamaño cero.
    println!("() ocupa {} bytes; [u8; 0] ocupa {}", size_of::<()>(), size_of::<[u8; 0]>());
    // 7. Cada función tiene su propio tipo de tamaño cero; el puntero a función no.
    fn f() {}
    let item = f;
    let puntero: fn() = f;
    println!("item: {} bytes, puntero: {} bytes", size_of_val(&item), size_of_val(&puntero));
}
