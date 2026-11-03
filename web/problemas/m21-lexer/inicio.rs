#[derive(Debug, PartialEq)]
pub enum Token {
    Num(u64),
    Ident(String),
    Clave(String),
    Simbolo(char),
}

pub fn tokenizar(texto: &str) -> Result<Vec<Token>, String> {
    todo!("recorre texto.char_indices() y construye los tokens")
}
