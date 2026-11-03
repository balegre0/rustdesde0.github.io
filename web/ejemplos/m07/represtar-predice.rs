fn sumar_uno(n: &mut i32) {
    *n += 1;
}

fn main() {
    let mut x = 0;
    let r = &mut x;
    sumar_uno(r);
    sumar_uno(r);
    sumar_uno(&mut *r);
    *r += 10;
    println!("{x}");
}
