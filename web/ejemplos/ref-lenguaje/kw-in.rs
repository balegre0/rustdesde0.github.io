mod a {
    pub mod b {
        // 2. `pub(in ruta)`: visible solo dentro de esa ruta.
        pub(in crate::a) fn secreto() -> u32 { 42 }
    }
    pub fn revelar() -> u32 { b::secreto() }
}

fn main() {
    // 1. `in` separa el patrón del iterador en un `for`.
    for letra in ['r', 'u', 's', 't'] {
        print!("{letra}");
    }
    println!();
    println!("{}", a::revelar());
}
