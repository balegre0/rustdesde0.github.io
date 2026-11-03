fn contador() -> impl Fn() -> u32 {
    let base = 10;
    || base + 1
}

fn main() {
    let f = contador();
    println!("{}", f());
}
