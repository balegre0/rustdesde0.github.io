fn aplicar<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

fn repetir<F: FnMut()>(mut f: F, veces: u32) {
    for _ in 0..veces {
        f();
    }
}

fn una_vez<F: FnOnce() -> String>(f: F) -> String {
    f()
}

fn doble(x: i32) -> i32 {
    x * 2
}

fn main() {
    // Fn: solo lee lo que captura
    let base = 10;
    println!("{}", aplicar(|x| x + base, 5));
    println!("{}", aplicar(doble, 5)); // las funciones también implementan Fn
    // FnMut: modifica lo que captura
    let mut cuenta = 0;
    repetir(|| cuenta += 1, 3);
    println!("{cuenta}");
    // FnOnce: consume lo que captura
    let nombre = String::from("Ana");
    println!("{}", una_vez(move || nombre + " Gil"));
    // closures guardadas en variables y en una Box
    let operaciones: Vec<Box<dyn Fn(i32) -> i32>> = vec![Box::new(|x| x + 1), Box::new(|x| x * x)];
    println!("{:?}", operaciones.iter().map(|f| f(4)).collect::<Vec<_>>());
}
