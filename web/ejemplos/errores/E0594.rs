fn subir(n: &i32) {
    *n += 1;
}

fn main() {
    let x = 1;
    subir(&x);
    println!("{x}");
}
