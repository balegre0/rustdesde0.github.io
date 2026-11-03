fn contador() -> impl FnMut() -> u32 {
    let mut n = 0;
    move || { n += 1; n }
}

fn main() {
    let mut c = contador();
    c();
    println!("{}", c());
}
