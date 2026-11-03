#[derive(Debug, Clone, Copy)]
pub enum Op { Local(usize), Const(i64) }

#[derive(Debug, Clone, Copy)]
pub enum Sentencia {
    Asigna(usize, Op),
    Suma(usize, Op, Op),
    Mul(usize, Op, Op),
}

#[derive(Debug, Clone)]
pub enum Terminador {
    Goto(usize),
    SwitchInt(usize, i64, usize, usize),
    Assert(usize, String),
    Return,
}

pub struct Bloque { pub sentencias: Vec<Sentencia>, pub terminador: Terminador }

pub fn ejecutar(bloques: &[Bloque], args: &[i64]) -> Result<i64, String> {
    todo!("8 locales a cero, copia los args a _1.., y recorre los bloques desde bb0")
}
