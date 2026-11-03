fn main() {
    // 1. `loop` repite para siempre hasta un `break`.
    let mut intentos = 0;
    loop {
        intentos += 1;
        if intentos == 3 { break; }
    }
    println!("intentos: {intentos}");
    // 2. `loop` es una expresión: `break valor` le da su valor.
    let mut a = 0u64;
    let mut b = 1u64;
    let primer_fib_mayor_que_100 = loop {
        let c = a + b;
        a = b;
        b = c;
        if c > 100 { break c; }
    };
    println!("{primer_fib_mayor_que_100}");
}
