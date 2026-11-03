fn main() {
    let c = 'ñ';
    // Imprime el carácter, su número Unicode en hexadecimal,
    // si es alfabético (c.is_alphabetic()) y su mayúscula
    // (c.to_ascii_uppercase() NO sirve con la ñ: usa c.to_uppercase()).
    // Salida exacta:
    // ñ U+F1 true Ñ
    println!("{c}");
}
