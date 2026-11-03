mod a {
    pub fn publica() -> &'static str { "a::publica" }
    fn privada() -> &'static str { "a::privada" }

    pub mod b {
        pub(crate) fn para_el_crate() -> &'static str { "b::para_el_crate" }
        pub(super) fn para_a() -> &'static str { "b::para_a" }
        pub fn usar_privada_del_padre() -> &'static str {
            // Un hijo ve lo privado de sus antepasados.
            super::privada()
        }
    }

    pub fn llamar_b() -> &'static str { b::para_a() }
}

trait Saludo { fn hola() -> &'static str; }
trait Otro { fn hola() -> &'static str; }
struct S;
impl Saludo for S { fn hola() -> &'static str { "Saludo::hola" } }
impl Otro for S { fn hola() -> &'static str { "Otro::hola" } }

fn main() {
    println!("{}", a::publica());
    println!("{}", crate::a::b::para_el_crate());
    println!("{}", a::llamar_b());
    println!("{}", a::b::usar_privada_del_padre());
    // Ruta calificada para desambiguar.
    println!("{} {}", <S as Saludo>::hola(), <S as Otro>::hola());
    // Turbofish: los argumentos genéricos en una expresión van tras `::`.
    let n = "42".parse::<u8>().unwrap();
    let v = Vec::<char>::with_capacity(2);
    println!("{n} {}", v.capacity() >= 2);
    // Ruta absoluta a un crate externo con `::`.
    println!("{}", ::std::cmp::max(3, 7));
}
