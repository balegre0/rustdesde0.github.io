fn main() {
    // 1. push_str y push: añadir a un String existente (necesita mut).
    let mut s = String::from("Hola");
    s.push_str(", mundo");
    s.push('!');
    println!("{s}");

    // 2. format!: la forma más clara de componer. No mueve nada.
    let nombre = "Zoe";
    let edad = 7;
    let ficha = format!("{nombre} tiene {edad} años");
    println!("{ficha}");

    // 3. El operador + : mueve el String de la IZQUIERDA y pide &str a la derecha.
    let saludo = String::from("Buenos");
    let completo = saludo + " días";
    println!("{completo}");

    // 4. join: unir una lista con un separador. concat: sin separador.
    let palabras = ["rojo", "verde", "azul"];
    println!("{}", palabras.join(", "));
    println!("{}", palabras.concat());

    // 5. repeat y replace.
    println!("{}", "ja".repeat(3));
    println!("{}", "café con leche".replace("leche", "hielo"));

    // 6. Números ↔ texto: to_string y parse.
    let n = 42.to_string();
    let m: i32 = "-17".parse().unwrap();
    println!("{n} {m}");

    // 7. Bytes → texto: from_utf8 comprueba que sea UTF-8 válido.
    let bytes = vec![0x61, 0xC3, 0xB1, 0x6F]; // a, ñ, o
    println!("{:?}", String::from_utf8(bytes));
    let rotos = vec![0x61, 0xC3]; // una ñ a medias
    println!("{:?}", String::from_utf8(rotos.clone()).is_err());
    println!("{}", String::from_utf8_lossy(&rotos));

    // 8. Construir en un bucle: reservar de antemano evita mudanzas.
    let mut lista = String::with_capacity(32);
    for i in 1..=5 {
        if !lista.is_empty() {
            lista.push('-');
        }
        lista.push_str(&i.to_string());
    }
    println!("{lista} (len {}, cap {})", lista.len(), lista.capacity());
}
