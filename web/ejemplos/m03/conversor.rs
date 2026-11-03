fn main() {
    let ciudad = String::from("Logroño");
    let celsius: [i32; 4] = [-5, 0, 21, 37];
    println!("{ciudad}: {} letras, {} bytes", ciudad.chars().count(), ciudad.len());

    for c in celsius {
        let f = c as f64 * 9.0 / 5.0 + 32.0;
        let k = c + 273;
        println!("{c:>3} °C = {f:>5.1} °F = {k} K");
    }

    let (minima, maxima) = (celsius[0], celsius[3]);
    let diferencia = maxima.checked_sub(minima);
    println!("diferencia: {diferencia:?}");
    let en_u8 = u8::try_from(maxima * 10);
    println!("máxima × 10 en un u8: {en_u8:?}");
}
