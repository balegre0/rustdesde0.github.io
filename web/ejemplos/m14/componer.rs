fn componer(f: impl Fn(i32) -> i32, g: impl Fn(i32) -> i32) -> impl Fn(i32) -> i32 {
    move |x| g(f(x))
}

fn main() {
    let mas_uno_y_doble = componer(|x| x + 1, |x| x * 2);
    println!("{}", mas_uno_y_doble(3));
    println!("{}", mas_uno_y_doble(10));
    let cuadrado_menos_uno = componer(|x| x * x, |x| x - 1);
    println!("{}", cuadrado_menos_uno(5));
}
