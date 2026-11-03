// Una referencia nunca puede ser nula.
const MALA: &u8 = unsafe { std::mem::transmute::<usize, &u8>(0) };

fn main() {
    println!("{MALA}");
}
