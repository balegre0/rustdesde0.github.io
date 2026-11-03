// Quien pide FnOnce acepta cualquier closure (todos se pueden llamar al menos una vez).
fn ejecutar_una<F: FnOnce() -> i32>(f: F) -> i32 {
    f()
}

fn main() {
    let x = 10;
    let solo_lee = || x * 2;          // Fn
    let mut n = 0;
    let modifica = || { n += 1; n };  // FnMut
    let s = String::from("abc");
    let consume = move || { let t = s; t.len() as i32 }; // FnOnce

    println!("{}", ejecutar_una(solo_lee));
    println!("{}", ejecutar_una(modifica));
    println!("{}", ejecutar_una(consume));
}
