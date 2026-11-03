#[derive(Debug, PartialEq)]
pub enum Token {
    Num(u64),
    Ident(String),
    Clave(String),
    Simbolo(char),
}

pub fn tokenizar(texto: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut it = texto.char_indices().peekable();
    while let Some((pos, c)) = it.next() {
        match c {
            ' ' | '\n' | '\t' => {}
            '0'..='9' => {
                let mut n = c.to_digit(10).unwrap() as u64;
                while let Some(&(_, d)) = it.peek() {
                    let Some(x) = d.to_digit(10) else { break };
                    n = n * 10 + x as u64;
                    it.next();
                }
                tokens.push(Token::Num(n));
            }
            c if c.is_alphabetic() || c == '_' => {
                let mut s = String::from(c);
                while let Some(&(_, d)) = it.peek() {
                    if d.is_alphanumeric() || d == '_' { s.push(d); it.next(); } else { break; }
                }
                if s == "let" || s == "fn" { tokens.push(Token::Clave(s)); } else { tokens.push(Token::Ident(s)); }
            }
            '+' | '-' | '*' | '/' | '=' | '(' | ')' | ';' => tokens.push(Token::Simbolo(c)),
            otro => return Err(format!("carácter inesperado '{otro}' en {pos}")),
        }
    }
    Ok(tokens)
}
