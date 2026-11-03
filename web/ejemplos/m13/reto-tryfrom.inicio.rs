#[derive(Debug)]
enum Dia {
    Lunes,
    Martes,
    Miercoles,
}

// Implementa TryFrom<u8> para Dia: 1 → Lunes, 2 → Martes, 3 → Miercoles;
// cualquier otro número da Err(n) (el propio número).

fn main() {
    println!("{:?}", [Dia::Lunes, Dia::Martes, Dia::Miercoles]);
    // Descomenta cuando lo tengas:
    // println!("{:?}", Dia::try_from(2));
    // println!("{:?}", Dia::try_from(9));
}
