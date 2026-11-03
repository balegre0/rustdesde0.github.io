// Aritmética de punteros que se sale de la asignación es UB.
const MALO: u8 = {
    let a = [1u8, 2, 3];
    let p = a.as_ptr();
    unsafe { *p.add(5) }
};

fn main() {
    println!("{MALO}");
}
