#[derive(Debug, Clone, Copy)]
pub enum Instr {
    Empuja(i64),
    Suma,
    Resta,
    Mul,
    Div,
}

pub fn ejecutar(prog: &[Instr]) -> Result<i64, String> {
    todo!("usa un Vec<i64> como pila")
}
