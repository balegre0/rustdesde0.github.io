type Metros = f64;
type Segundos = f64;

fn velocidad(d: Metros, t: Segundos) -> f64 { d / t }

fn main() {
    // Un alias no crea un tipo nuevo: `Metros` es exactamente `f64`,
    // y `10` es un literal entero, no un f64.
    let d: Metros = 10;
    let t: Segundos = 2.0;
    println!("{}", velocidad(d, t));
}
