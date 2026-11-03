fn main() {
    let mut ok = true;
    let mut caso = |nombre: &str, bien: bool| {
        if bien { println!("OK: {}", nombre); } else { println!("FALLO: {}", nombre); ok = false; }
    };
    let m: ::std::collections::HashMap<&str, i32> = mapa! { "uno" => 1, "dos" => 2, "tres" => 3 };
    caso("tres parejas", m.len() == 3 && m.get("dos") == Some(&2));
    let n: ::std::collections::HashMap<char, &str> = mapa! { 'ñ' => "eñe", };
    caso("coma final", n.len() == 1 && n.get(&'ñ') == Some(&"eñe"));
    let vacio: ::std::collections::HashMap<i32, i32> = mapa! {};
    caso("mapa vacío", vacio.is_empty());
    let r: ::std::collections::HashMap<i32, &str> = mapa! { 1 => "a", 1 => "b" };
    caso("clave repetida: gana la última", r.len() == 1 && r.get(&1) == Some(&"b"));
    if ok { println!("TODOS LOS CASOS SUPERADOS"); }
}
