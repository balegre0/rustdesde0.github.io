// Jefe del mundo 21: un compilador completo en miniatura.
// texto -> tokens -> árbol (AST) -> plegado de constantes -> código de máquina de pila -> ejecución
#[derive(Debug, Clone, Copy, PartialEq)]
enum Tok { Num(i64), Var, Op(char), Abre, Cierra }

#[derive(Debug)]
enum Ast { Num(i64), Var, Bin(char, Box<Ast>, Box<Ast>) }

#[derive(Debug)]
enum Instr { Empuja(i64), CargaX, Opera(char) }

// 1. Lexer
fn lexer(s: &str) -> Vec<Tok> {
    let mut out = Vec::new();
    let cs: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < cs.len() {
        let c = cs[i];
        if c.is_ascii_digit() {
            let mut n = 0;
            while i < cs.len() && cs[i].is_ascii_digit() { n = n * 10 + cs[i].to_digit(10).unwrap() as i64; i += 1; }
            out.push(Tok::Num(n));
            continue;
        }
        match c {
            'x' => out.push(Tok::Var),
            '+' | '-' | '*' => out.push(Tok::Op(c)),
            '(' => out.push(Tok::Abre),
            ')' => out.push(Tok::Cierra),
            _ => {}
        }
        i += 1;
    }
    out
}

// 2. Parser con precedencias
fn prec(op: char) -> u8 { if op == '*' { 2 } else { 1 } }
fn parse(t: &[Tok], pos: &mut usize, minimo: u8) -> Ast {
    let mut izq = match t[*pos] {
        Tok::Num(n) => { *pos += 1; Ast::Num(n) }
        Tok::Var => { *pos += 1; Ast::Var }
        Tok::Abre => { *pos += 1; let e = parse(t, pos, 0); *pos += 1; e }
        otro => panic!("token inesperado {otro:?}"),
    };
    while *pos < t.len() {
        let Tok::Op(op) = t[*pos] else { break };
        if prec(op) <= minimo { break; }
        *pos += 1;
        let der = parse(t, pos, prec(op));
        izq = Ast::Bin(op, Box::new(izq), Box::new(der));
    }
    izq
}

// 3. Una pasada de optimización: plegar constantes (como hace GVN/const-prop sobre MIR)
fn plegar(a: Ast) -> Ast {
    match a {
        Ast::Bin(op, x, y) => match (plegar(*x), plegar(*y)) {
            (Ast::Num(p), Ast::Num(q)) => Ast::Num(match op { '+' => p + q, '-' => p - q, _ => p * q }),
            (x, y) => Ast::Bin(op, Box::new(x), Box::new(y)),
        },
        otro => otro,
    }
}

// 4. Generación de código para una máquina de pila
fn codegen(a: &Ast, out: &mut Vec<Instr>) {
    match a {
        Ast::Num(n) => out.push(Instr::Empuja(*n)),
        Ast::Var => out.push(Instr::CargaX),
        Ast::Bin(op, x, y) => { codegen(x, out); codegen(y, out); out.push(Instr::Opera(*op)); }
    }
}

// 5. La «CPU»
fn ejecutar(prog: &[Instr], x: i64) -> i64 {
    let mut pila = Vec::new();
    for i in prog {
        match i {
            Instr::Empuja(n) => pila.push(*n),
            Instr::CargaX => pila.push(x),
            Instr::Opera(op) => {
                let b = pila.pop().unwrap();
                let a = pila.pop().unwrap();
                pila.push(match op { '+' => a + b, '-' => a - b, _ => a * b });
            }
        }
    }
    pila.pop().unwrap()
}

fn main() {
    let fuente = "x * (2 + 3) - 4 * 1";
    let toks = lexer(fuente);
    println!("tokens: {toks:?}");
    let ast = parse(&toks, &mut 0, 0);
    println!("AST:    {ast:?}");
    let ast = plegar(ast);
    println!("plegado: {ast:?}");
    let mut prog = Vec::new();
    codegen(&ast, &mut prog);
    println!("código: {prog:?}");
    for x in [0, 1, 10] {
        println!("x = {x:>2} -> {}", ejecutar(&prog, x));
    }
}
