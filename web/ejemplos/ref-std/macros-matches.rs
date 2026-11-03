#[derive(Debug)]
enum Token {
    Numero(i64),
    Suma,
    Resta,
    Espacio,
}

fn main() {
    let tokens = [Token::Numero(4), Token::Espacio, Token::Suma, Token::Numero(-2), Token::Resta];
    let operadores = tokens.iter().filter(|t| matches!(t, Token::Suma | Token::Resta)).count();
    let positivos = tokens.iter().filter(|t| matches!(t, Token::Numero(n) if *n > 0)).count();
    println!("{operadores} operadores, {positivos} positivos");
    println!("{}", matches!('k', 'a'..='z'));
    println!("{:?}", tokens.last());
}
