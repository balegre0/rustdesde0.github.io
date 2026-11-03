struct Metros(f64);
struct Pies(f64);

fn avanzar(distancia: Metros) -> f64 {
    distancia.0
}

fn main() {
    let altura = Pies(30.0);
    println!("{}", avanzar(altura));
}
