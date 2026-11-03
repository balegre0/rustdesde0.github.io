fn main() {
    let mut n = 0;
    loop {
        n += 1;
        // 1. `continue` salta al principio de la siguiente vuelta
        if n % 3 == 0 {
            continue;
        }
        // 2. `break` sin valor sale del bucle (el `loop` vale `()`)
        if n > 10 {
            break;
        }
        print!("{n} ");
    }
    println!();
}
