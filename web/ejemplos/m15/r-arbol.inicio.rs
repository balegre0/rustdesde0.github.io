// Un árbol binario: cada nodo tiene un valor y, quizá, un hijo a cada lado.
struct Arbol {
    valor: i32,
    izq: Option<Box<Arbol>>,
    der: Option<Box<Arbol>>,
}

fn hoja(valor: i32) -> Option<Box<Arbol>> {
    Some(Box::new(Arbol { valor, izq: None, der: None }))
}

// Suma todos los valores del árbol (pista: match con Some(n) y None).
fn suma(a: &Option<Box<Arbol>>) -> i32 {
    let _ = a;
    todo!()
}

fn main() {
    let arbol = Some(Box::new(Arbol { valor: 1, izq: hoja(2), der: hoja(3) }));
    println!("suma = {}", suma(&arbol));
}
