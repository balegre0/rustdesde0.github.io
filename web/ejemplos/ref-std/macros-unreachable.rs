fn signo(n: i32) -> &'static str {
    match n.signum() {
        -1 => "negativo",
        0 => "cero",
        1 => "positivo",
        _ => unreachable!("signum solo devuelve -1, 0 o 1, no {}", n.signum()),
    }
}

fn main() {
    for n in [-5, 0, 8] {
        println!("{n}: {}", signo(n));
    }
    let x: u8 = 3;
    let mitad = match x % 2 {
        0 => "par",
        1 => "impar",
        _ => unreachable!(),
    };
    println!("{mitad}");
}
