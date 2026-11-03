// `unsafe extern`: al escribir estas firmas prometes que son correctas.
unsafe extern "C" {
    fn abs(x: i32) -> i32;
}

fn main() {
    // SAFETY: abs acepta cualquier i32 salvo i32::MIN, y -3 no lo es.
    println!("{}", unsafe { abs(-3) });
}
