fn contador() -> impl FnMut() -> u32 {
    let mut n = 0;
    // Sin `move`, la closure tomaría prestada `n`, que muere al salir de la función.
    || { n += 1; n }
}

fn main() {
    let mut c = contador();
    println!("{}", c());
}
