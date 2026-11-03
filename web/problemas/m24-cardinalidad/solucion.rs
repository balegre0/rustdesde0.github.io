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
    match t {
        Tipo::Unidad => 1,
        Tipo::Bool => 2,
        Tipo::U8 => 256,
        Tipo::Vacio => 0,
        Tipo::Tupla(ts) => ts.iter().map(cardinalidad).product(),
        Tipo::Enum(ts) => ts.iter().map(cardinalidad).sum(),
        Tipo::Opcion(t) => cardinalidad(t) + 1,
        Tipo::Array(t, n) => cardinalidad(t).pow(*n),
    }
}
