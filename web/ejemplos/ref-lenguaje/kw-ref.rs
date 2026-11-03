fn main() {
    let par = (String::from("clave"), 7);
    // 1. `ref` enlaza por referencia en lugar de mover.
    let (ref k, n) = par;
    println!("{k} {n}");
    println!("{} sigue entero", par.0);
    // 2. `ref mut` enlaza un préstamo exclusivo.
    let mut o = Some(String::from("hola"));
    if let Some(ref mut s) = o {
        s.push_str(" mundo");
    }
    println!("{o:?}");
    // 3. Lo habitual hoy: hacer `match` sobre una referencia
    //    y dejar que las variables sean referencias solas.
    match &o {
        Some(s) => println!("longitud {}", s.len()),
        None => {}
    }
}
