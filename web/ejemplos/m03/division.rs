fn main() {
    // 1. Entre enteros, la división descarta los decimales
    println!("7 / 2 = {}", 7 / 2);
    // 2. Redondea hacia el cero, también con negativos
    println!("-7 / 2 = {}", -7 / 2);
    // 3. El resto (%) lleva el signo del dividendo
    println!("7 % 3 = {}", 7 % 3);
    println!("-7 % 3 = {}", -7 % 3);
    // 4. Comprobación: (a / b) * b + (a % b) == a
    let a = -7;
    let b = 3;
    println!("{}", (a / b) * b + (a % b));
}
