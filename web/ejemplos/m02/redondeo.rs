fn main() {
    // 1. Con {:.0} no hay decimales. ¿Hacia dónde redondea?
    println!("{:.0}", 1.5);
    println!("{:.0}", 2.5);
    println!("{:.0}", 3.5);
    // 2. Con dos decimales.
    println!("{:.2}", 0.125);
}
