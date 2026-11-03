// Leer a través de un puntero a memoria que ya no existe (colgante) es UB.
const MALO: u32 = {
    let p: *const u32 = {
        let local = 5u32;
        &raw const local
    }; // `local` ya no existe
    unsafe { *p }
};

fn main() {
    println!("{MALO}");
}
