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
    comprobar("vacío -> None", NoVacio::<i32>::desde(vec![]).is_none());
    let notas = NoVacio::desde(vec![7.0, 9.5, 4.5, 9.0]).unwrap();
    comprobar("mínimo de las notas", *notas.minimo() == 4.5);
    comprobar("media de las notas", media(&notas) == 7.5);
    let uno = NoVacio::desde(vec![3.0]).unwrap();
    comprobar("media de un solo elemento", media(&uno) == 3.0);
    let palabras = NoVacio::desde(vec!["sol", "agua", "luna"]).unwrap();
    comprobar("mínimo de textos", *palabras.minimo() == "agua");
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
