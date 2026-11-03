// Parser de descenso recursivo con precedencias (estilo Pratt), como el de rustc_parse.
#[derive(Debug)]
enum Expr {
    Num(i64),
    Bin(char, Box<Expr>, Box<Expr>),
}

struct Parser { tokens: Vec<char>, pos: usize }

impl Parser {
    fn nuevo(texto: &str) -> Parser {
        Parser { tokens: texto.chars().filter(|c| !c.is_whitespace()).collect(), pos: 0 }
    }
    fn mirar(&self) -> Option<char> { self.tokens.get(self.pos).copied() }

    // 1. Cuánto «se pegan» los operadores: * se pega más fuerte que +
    fn precedencia(op: char) -> Option<u8> {
        match op { '+' | '-' => Some(1), '*' | '/' => Some(2), _ => None }
    }

    // 2. Un átomo: un dígito o una expresión entre paréntesis
    fn atomo(&mut self) -> Expr {
        let c = self.mirar().expect("se esperaba algo");
        self.pos += 1;
        if c == '(' {
            let e = self.expr(0);
            self.pos += 1; // la ')'
            e
        } else {
            Expr::Num(c.to_digit(10).expect("se esperaba un dígito") as i64)
        }
    }

    // 3. El corazón: mientras el siguiente operador pegue más fuerte que `minimo`, seguimos
    fn expr(&mut self, minimo: u8) -> Expr {
        let mut izq = self.atomo();
        while let Some(op) = self.mirar() {
            let Some(p) = Parser::precedencia(op) else { break };
            if p <= minimo { break; }
            self.pos += 1;
            let der = self.expr(p);
            izq = Expr::Bin(op, Box::new(izq), Box::new(der));
        }
        izq
    }
}

fn evaluar(e: &Expr) -> i64 {
    match e {
        Expr::Num(n) => *n,
        Expr::Bin(op, a, b) => {
            let (a, b) = (evaluar(a), evaluar(b));
            match op { '+' => a + b, '-' => a - b, '*' => a * b, _ => a / b }
        }
    }
}

fn main() {
    for texto in ["1 + 2 * 3", "(1 + 2) * 3", "8 - 3 - 2"] {
        let arbol = Parser::nuevo(texto).expr(0);
        println!("{texto:<12} => {arbol:?} = {}", evaluar(&arbol));
    }
}
