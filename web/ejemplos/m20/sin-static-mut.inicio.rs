// Este contador global usa `static mut` y unsafe. Cámbialo por un AtomicU32
// (std::sync::atomic) para que NO quede ningún unsafe en el programa.
static mut SIGUIENTE_ID: u32 = 100;

fn nuevo_id() -> u32 {
    unsafe {
        let id = SIGUIENTE_ID;
        SIGUIENTE_ID += 1;
        id
    }
}

fn main() {
    let a = nuevo_id();
    let b = nuevo_id();
    let c = nuevo_id();
    println!("{a} {b} {c}");
}
