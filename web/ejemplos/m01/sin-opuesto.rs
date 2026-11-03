// El rango de i8 no es simétrico: hay un negativo más que positivos.
fn main() {
    println!("i8: de {} a {}", i8::MIN, i8::MAX);

    // 1. El opuesto de 127 existe
    println!("opuesto de 127:  {:?}", 127i8.checked_neg());

    // 2. El opuesto de -128 sería 128, que no cabe en un i8
    println!("opuesto de -128: {:?}", i8::MIN.checked_neg());
}
