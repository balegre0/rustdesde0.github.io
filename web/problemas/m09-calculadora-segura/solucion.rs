enum Op {
    Sumar,
    Restar,
    Multiplicar,
    Dividir,
}

fn aplicar(op: Op, a: i32, b: i32) -> Option<i32> {
    match op {
        Op::Sumar => a.checked_add(b),
        Op::Restar => a.checked_sub(b),
        Op::Multiplicar => a.checked_mul(b),
        Op::Dividir => a.checked_div(b),
    }
}
