enum Tipo {
    Unidad,
    Bool,
    U8,
    Vacio,
    Tupla(Vec<Tipo>),
    Enum(Vec<Tipo>),
    Opcion(Box<Tipo>),
    Array(Box<Tipo>, u32),
}

fn cardinalidad(t: &Tipo) -> u128 {
    todo!()
}
