
fn main() {
    let mut fallos = 0;
    let mut caso = |nombre: &str, ok: bool| {
        if ok { println!("OK: {nombre}"); } else { println!("FALLO: {nombre}"); fallos += 1; }
    };
    let c = MiCell::new(5);
    caso("get inicial", c.get() == 5);
    let a = &c;
    let b = &c;
    a.set(8);
    caso("set por un préstamo, get por otro", b.get() == 8);
    caso("reemplazar devuelve el viejo", b.reemplazar(13) == 8);
    caso("valor tras reemplazar", a.get() == 13);
    let p = MiCell::new((1, 'x'));
    p.set((2, 'y'));
    caso("con tuplas", p.get() == (2, 'y'));
    if fallos == 0 { println!("TODOS LOS CASOS SUPERADOS"); }
}
