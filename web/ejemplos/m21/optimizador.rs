// Reto: escribe la pasada `simplificar` con dos reglas algebraicas:
//   e * 1  =>  e        e + 0  =>  e
// (y deja todo lo demás como está). Simplifica primero los hijos.
#[derive(Debug)]
enum Ast { Num(i64), Var, Bin(char, Box<Ast>, Box<Ast>) }

fn simplificar(a: Ast) -> Ast {
    match a {
        Ast::Bin(op, x, y) => match (op, simplificar(*x), simplificar(*y)) {
            ('*', e, Ast::Num(1)) => e,
            ('+', e, Ast::Num(0)) => e,
            (op, x, y) => Ast::Bin(op, Box::new(x), Box::new(y)),
        },
        otro => otro,
    }
}

fn main() {
    use Ast::*;
    // (x * 1) + 0
    let e = Bin('+', Box::new(Bin('*', Box::new(Var), Box::new(Num(1)))), Box::new(Num(0)));
    println!("{:?}", simplificar(e));
    // (x + 0) * (x * 2)
    let e = Bin('*', Box::new(Bin('+', Box::new(Var), Box::new(Num(0)))), Box::new(Bin('*', Box::new(Var), Box::new(Num(2)))));
    println!("{:?}", simplificar(e));
}
