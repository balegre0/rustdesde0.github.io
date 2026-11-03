struct Metros(f64);
struct Pies(f64);

fn avanzar(distancia: Metros) -> f64 {
    distancia.0
}

fn pies_a_metros(p: Pies) -> Metros {
    Metros(p.0 * 0.3048)
}

fn main() {
    let altura = Pies(30.0);
    println!("{}", avanzar(pies_a_metros(altura)));
}
