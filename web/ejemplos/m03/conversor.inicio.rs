fn main() {
    let ciudad = String::from("Logroño");
    let celsius: [i32; 4] = [-5, 0, 21, 37];
    // 1. Imprime: Logroño: 7 letras, 8 bytes
    println!("{ciudad}");

    for c in celsius {
        // 2. Calcula f (°F, como f64) = c × 9 / 5 + 32  y  k (kelvin) = c + 273
        //    Imprime con anchos: {c:>3} y {f:>5.1}, por ejemplo:
        //     -5 °C =  23.0 °F = 268 K
        println!("{c}");
    }

    // 3. Saca la mínima y la máxima del array desestructurando una tupla,
    //    resta con checked_sub e imprime: diferencia: Some(42)
    // 4. Intenta meter maxima * 10 en un u8 con u8::try_from e imprime:
    //    máxima × 10 en un u8: Err(TryFromIntError(PosOverflow))
}
