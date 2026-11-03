// Algoritmo de Euclides para el máximo común divisor:
//   mcd(a, 0) = a
//   mcd(a, b) = mcd(b, a % b)
// 1. Escribe `mcd` de forma recursiva.
// 2. Escribe `simplificar(num, den)` que imprima la fracción simplificada,
//    por ejemplo `12/18 = 2/3`, usando `mcd`.

fn main() {
    println!("mcd(48, 36) = {}", mcd(48, 36));
    println!("mcd(17, 5) = {}", mcd(17, 5));
    simplificar(12, 18);
    simplificar(100, 25);
}
