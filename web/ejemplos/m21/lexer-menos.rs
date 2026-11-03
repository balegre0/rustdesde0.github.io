// Reto: añade el token `Menos` para el carácter '-'.
#[derive(Debug, PartialEq)]
enum Token {
    Numero(u64),
    Mas,
    Por,
    // 1. Añade aquí la variante Menos
    Menos,
}

fn tokenizar(texto: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = texto.char_indices().peekable();
    while let Some((pos, c)) = chars.next() {
        match c {
            ' ' => {}
            '+' => tokens.push(Token::Mas),
            '*' => tokens.push(Token::Por),
            // 2. Reconoce aquí el '-'
            '-' => tokens.push(Token::Menos),
            '0'..='9' => {
                let mut n = c.to_digit(10).unwrap() as u64;
                while let Some(&(_, d)) = chars.peek() {
                    match d.to_digit(10) {
                        Some(x) => { n = n * 10 + x as u64; chars.next(); }
                        None => break,
                    }
                }
                tokens.push(Token::Numero(n));
            }
            otro => return Err(format!("carácter inesperado {otro:?} en la posición {pos}")),
        }
    }
    Ok(tokens)
}

fn main() {
    println!("{:?}", tokenizar("10 - 4 * 2"));
}
