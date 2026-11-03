
fn main() {
    let mut fallos = 0;
    let mut caso = |nombre: &str, ok: bool| {
        if ok { println!("OK: {nombre}"); } else { println!("FALLO: {nombre}"); fallos += 1; }
    };

    let mut a = [1, 2, 3, 4, 5];
    invertir(&mut a);
    caso("impar", a == [5, 4, 3, 2, 1]);

    let mut b = [1, 2, 3, 4];
    invertir(&mut b);
    caso("par", b == [4, 3, 2, 1]);

    let mut c: [u8; 0] = [];
    invertir(&mut c);
    caso("vacío", c.is_empty());

    let mut d = [9];
    invertir(&mut d);
    caso("uno", d == [9]);

    let mut e = vec![String::from("uno"), String::from("dos"), String::from("tres")];
    invertir(&mut e);
    caso("String", e == ["tres", "dos", "uno"]);

    if fallos == 0 { println!("TODOS LOS CASOS SUPERADOS"); }
}
