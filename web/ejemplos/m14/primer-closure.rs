fn main() {
    // 1. Un closure sin nombre guardado en una variable.
    let doble = |x| x * 2;
    println!("doble(21) = {}", doble(21));

    // 2. Con tipos escritos y un bloque como cuerpo.
    let saluda = |nombre: &str| -> String {
        let s = format!("¡Hola, {nombre}!");
        s
    };
    println!("{}", saluda("Begoña"));

    // 3. Un closure puede usar variables de alrededor: las «captura».
    let iva = 21;
    let con_iva = |precio: u32| precio + precio * iva / 100;
    println!("100 con IVA = {}", con_iva(100));
}
