#![forbid(unsafe_code)]

#[allow(unsafe_code)] // forbid no se puede relajar más abajo
fn leer(p: *const i32) -> i32 {
    unsafe { *p }
}

fn main() {
    let x = 5;
    println!("{}", leer(&x));
}
