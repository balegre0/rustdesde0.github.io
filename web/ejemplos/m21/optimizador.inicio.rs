// Reto: escribe la pasada `simplificar` con dos reglas algebraicas:
//   e * 1  =>  e        e + 0  =>  e
// (y deja todo lo demás como está). Simplifica primero los hijos.
#[derive(Debug)]
enum Ast { Num(i64), Var, Bin(char, Box<Ast>, Box<Ast>) }

fn simplificar(a: Ast) -> Ast {
    todo!("1. si es Bin, simplifica los dos hijos; 2. aplica las reglas; 3. si no, devuelve igual")
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
