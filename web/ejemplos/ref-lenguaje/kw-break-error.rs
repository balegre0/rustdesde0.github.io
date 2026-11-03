fn main() {
    // Solo `loop` puede devolver un valor con `break`:
    // un `while` podría terminar sin pasar nunca por el `break`.
    let mut i = 0;
    let x = while i < 10 {
        i += 1;
        if i == 5 { break i; }
    };
    println!("{x:?}");
}
