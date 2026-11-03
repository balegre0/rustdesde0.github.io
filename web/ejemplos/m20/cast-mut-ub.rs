// El «truco» de convertir &T en *mut T para escribir es UB.
fn poner_a_cero(r: &i32) {
    let p = r as *const i32 as *mut i32;
    unsafe { *p = 0 };
}

fn main() {
    let x = 5;
    poner_a_cero(&x);
    println!("{x}");
}
