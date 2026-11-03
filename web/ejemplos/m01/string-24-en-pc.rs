// Comprobación en tiempo de compilación para un PC de 64 bits (x86-64):
// si String no midiera 24 bytes, este archivo no compilaría.
const _: () = assert!(std::mem::size_of::<String>() == 24);
const _: () = assert!(std::mem::size_of::<usize>() == 8);

fn main() {}
