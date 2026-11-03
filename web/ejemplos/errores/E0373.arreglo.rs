fn contador() -> impl Fn() -> u32 {
    let base = 10;
    move || base + 1 // la closure se lleva su propia copia de `base`
}

fn main() {
    let f = contador();
    println!("{}", f());
}
