// Un lexer de juguete: convierte texto en tokens, como hace rustc_lexer.
#[derive(Debug, PartialEq)]
enum Token {
    Numero(u64),
    Mas,
    Por,
    ParenAbre,
    ParenCierra,
}

fn tokenizar(texto: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = texto.char_indices().peekable();
    while let Some((pos, c)) = chars.next() {
        match c {
            // 1. Los espacios separan tokens pero no son tokens
            ' ' => {}
            '+' => tokens.push(Token::Mas),
            '*' => tokens.push(Token::Por),
            '(' => tokens.push(Token::ParenAbre),
            ')' => tokens.push(Token::ParenCierra),
            // 2. Un número: seguimos leyendo mientras haya dígitos
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
            // 3. Cualquier otra cosa es un error con su posición (como los de rustc)
            otro => return Err(format!("carácter inesperado {otro:?} en la posición {pos}")),
        }
    }
    Ok(tokens)
}

fn main() {
    println!("{:?}", tokenizar("12 + 3 * (4+5)"));
    println!("{:?}", tokenizar("7 $ 2"));
}
