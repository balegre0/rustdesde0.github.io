// #[cold] marca el camino raro: el optimizador lo aparta del código caliente.
#[cold]
#[inline(never)]
fn error_raro(n: i32) -> ! {
    panic!("valor imposible: {n}");
}

fn dividir(a: i32, b: i32) -> i32 {
    if b == 0 {
        error_raro(a); // camino frío
    }
    a / b // camino caliente
}

fn main() {
    let mut total = 0;
    for b in 1..=4 {
        total += dividir(100, b);
    }
    println!("{total}");
}
