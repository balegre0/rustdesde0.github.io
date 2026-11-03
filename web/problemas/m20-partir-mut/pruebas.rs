
fn main() {
    let mut fallos = 0;
    let mut caso = |nombre: &str, ok: bool| {
        if ok { println!("OK: {nombre}"); } else { println!("FALLO: {nombre}"); fallos += 1; }
    };

    let mut v = [1, 2, 3, 4, 5];
    {
        let (a, b) = partir_mut(&mut v, 2);
        caso("trozos de [1,2,3,4,5] en 2", a == [1, 2] && b == [3, 4, 5]);
        a[0] = 10;
        b[2] = 50;
    }
    caso("escribir en los dos trozos", v == [10, 2, 3, 4, 50]);

    let mut w = [7];
    let (a, b) = partir_mut(&mut w, 0);
    caso("i = 0", a.is_empty() && b == [7]);

    let mut x = [1, 2];
    let (a, b) = partir_mut(&mut x, 2);
    caso("i = len", a == [1, 2] && b.is_empty());

    let mut vacio: [i32; 0] = [];
    let (a, b) = partir_mut(&mut vacio, 0);
    caso("slice vacío", a.is_empty() && b.is_empty());

    if fallos == 0 { println!("TODOS LOS CASOS SUPERADOS"); }
}
