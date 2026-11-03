// Un closure que guarda estado entre llamadas: devuelve un FnMut.
fn nuevo_contador() -> impl FnMut() -> u32 {
    let mut cuenta = 0;
    move || {
        cuenta += 1;
        cuenta
    }
}

fn main() {
    let mut a = nuevo_contador();
    let mut b = nuevo_contador();
    println!("a: {} {} {}", a(), a(), a());
    println!("b: {}", b());
}
