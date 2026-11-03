use std::fmt;

// 1. Un newtype: un struct de un solo campo que envuelve un tipo ajeno.
struct Lista(Vec<String>);

// 2. Como Lista es nuestra, podemos implementarle un trait ajeno (Display).
impl fmt::Display for Lista {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// 3. Otro uso del newtype: tipos que no se pueden confundir entre sí.
struct Metros(f64);
struct Segundos(f64);

fn velocidad(d: Metros, t: Segundos) -> f64 {
    d.0 / t.0
}

fn main() {
    let l = Lista(vec![String::from("pan"), String::from("leche")]);
    println!("{l}");
    println!("{} m/s", velocidad(Metros(100.0), Segundos(9.58)));
    // velocidad(Segundos(9.58), Metros(100.0)) no compilaría: tipos cambiados.
}
