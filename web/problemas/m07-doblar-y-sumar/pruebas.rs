
fn main() {
    let mut fallos = 0;
    let mut comprobar = |nombre: &str, ok: bool, detalle: String| {
        if ok {
            println!("OK: {nombre}");
        } else {
            println!("FALLO: {nombre}: {detalle}");
            fallos += 1;
        }
    };
    let mut v = vec![1, 2, 3];
    doblar(&mut v);
    comprobar("doblar(vec [1, 2, 3])", v == [2, 4, 6], format!("obtuve {v:?}"));
    let s = suma(&v);
    comprobar("suma([2, 4, 6])", s == 12, format!("obtuve {s}"));
    let mut a = [-5];
    doblar(&mut a);
    comprobar("doblar(array [-5])", a == [-10], format!("obtuve {a:?}"));
    let s = suma(&a);
    comprobar("suma([-10])", s == -10, format!("obtuve {s}"));
    let mut w = vec![1, 1, 1, 1];
    doblar(&mut w[1..3]);
    comprobar("doblar solo un trozo", w == [1, 2, 2, 1], format!("obtuve {w:?}"));
    let s = suma(&[]);
    comprobar("suma([])", s == 0, format!("obtuve {s}"));
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
