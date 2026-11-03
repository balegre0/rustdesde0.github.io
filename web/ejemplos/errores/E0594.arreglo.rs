fn subir(n: &mut i32) {
    *n += 1;
}

fn main() {
    let mut x = 1;
    subir(&mut x);
    println!("{x}");
}
