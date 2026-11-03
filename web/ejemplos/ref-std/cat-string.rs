fn main() {
    let linea = "  nombre = Íñigo Pérez ; edad=41  ";

    // Partir en dos por un separador.
    let (clave, resto) = linea.trim().split_once('=').unwrap_or(("", ""));
    println!("split_once: {:?} | {:?}", clave.trim(), resto);

    // Quitar prefijos y sufijos solo si están.
    println!("strip_prefix: {:?}  {:?}", "v1.99".strip_prefix('v'), "1.99".strip_prefix('v'));

    // splitn, rsplit_once, split_terminator.
    println!("splitn(2): {:?}", "a:b:c".splitn(2, ':').collect::<Vec<_>>());
    println!("rsplit_once: {:?}", "archivo.tar.gz".rsplit_once('.'));
    println!("split_terminator: {:?}", "a;b;".split_terminator(';').collect::<Vec<_>>());

    // Buscar y contar.
    println!("find: {:?}  rfind: {:?}  matches: {}", linea.find('ñ'), linea.rfind('e'), linea.matches('e').count());
    println!("replacen: {}", "a-b-c".replacen('-', "+", 1));
    println!("eq_ignore_ascii_case: {}", "RUST".eq_ignore_ascii_case("rust"));

    // Modificar un String.
    let mut s = String::from("hola mundo");
    s.insert_str(0, "¡");
    s.push('!');
    s.replace_range(2..6, "adiós"); // la «¡» ocupa los bytes 0 y 1: «hola» va del 2 al 6
    println!("insert_str/replace_range: {s}");
    s.retain(|c| c != 'o');
    println!("retain: {s}");
    let resto = s.split_off(7);
    println!("split_off: {s:?} + {resto:?}");
    let borrado: String = s.drain(..3).collect();
    println!("drain: quitó {borrado:?}, queda {s:?}");

    // Mayúsculas con Unicode, repetir, bytes.
    println!("to_uppercase: {}  repeat: {}", "ñandú".to_uppercase(), "ja".repeat(3));
    println!("is_char_boundary(1) en «ñu»: {}  get(0..1): {:?}", "ñu".is_char_boundary(1), "ñu".get(0..1));
    println!("from_utf8_lossy: {}", String::from_utf8_lossy(&[72, 111, 0xFF, 108, 97]));
}
