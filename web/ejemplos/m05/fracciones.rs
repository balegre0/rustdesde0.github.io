// Máximo común divisor (Euclides), recursivo
fn mcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { mcd(b, a % b) }
}

// Devuelve DOS valores a la vez en una tupla: (numerador, denominador)
fn simplificar(num: u64, den: u64) -> (u64, u64) {
    let m = mcd(num, den);
    (num / m, den / m)
}

// a/b + c/d = (a·d + c·b) / (b·d), y después se simplifica
fn sumar(a: u64, b: u64, c: u64, d: u64) -> (u64, u64) {
    simplificar(a * d + c * b, b * d)
}

fn mostrar(a: u64, b: u64, c: u64, d: u64) {
    let (n, m) = sumar(a, b, c, d);
    println!("{a}/{b} + {c}/{d} = {n}/{m}");
}

fn main() {
    mostrar(1, 2, 1, 3);
    mostrar(3, 4, 1, 4);
    mostrar(5, 6, 7, 10);
}
