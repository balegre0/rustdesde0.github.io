fn main() {
    // Regla de Collatz: si n es par, n/2; si es impar, 3n+1. ¿Cuántos pasos hasta llegar a 1?
    let mut n: u64 = 27;
    let mut pasos = 0;
    let mut maximo = n;
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        if n > maximo {
            maximo = n;
        }
        pasos += 1;
    }
    println!("27 llega a 1 en {pasos} pasos; el valor más alto fue {maximo}");
}
