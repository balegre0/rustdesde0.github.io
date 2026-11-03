fn main() {
    let mut fallos = 0;
    let mut comprobar = |que: &str, ok: bool| {
        if ok {
            println!("OK: {que}");
        } else {
            println!("FALLO: {que}");
            fallos += 1;
        }
    };
    comprobar("enteros", maximo(&[3, 9, 2]) == Some(&9));
    comprobar("vacío", maximo::<i32>(&[]).is_none());
    comprobar("decimales", maximo(&[1.5, -2.0, 0.25]) == Some(&1.5));
    let palabras = vec![String::from("sol"), String::from("zorro"), String::from("luna")];
    comprobar("String (sin Copy)", maximo(&palabras).map(|s| s.as_str()) == Some("zorro"));
    let iguales = [(1, 'a'), (1, 'a')];
    comprobar("el primero de los iguales", maximo(&iguales).map(|p| std::ptr::eq(p, &iguales[0])) == Some(true));
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
