struct Metros(f64);
struct Segundos(f64);
struct Vacio;

fn main() {
    let d = Metros(100.0);
    let t = Segundos(9.58);
    let Metros(m) = d; // desmontar con un patrón
    println!("{:.2} m/s", m / t.0);
    println!("{} bytes", std::mem::size_of::<Vacio>());
}
