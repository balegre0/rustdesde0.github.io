#[derive(Debug, PartialEq)]
enum Triangulo {
    Equilatero,
    Isosceles,
    Escaleno,
    Invalido,
}

fn clasificar(a: u32, b: u32, c: u32) -> Triangulo {
    if a == 0 || b == 0 || c == 0 || a >= b + c || b >= a + c || c >= a + b {
        return Triangulo::Invalido;
    }
    match (a == b, b == c, a == c) {
        (true, true, _) => Triangulo::Equilatero,
        (false, false, false) => Triangulo::Escaleno,
        _ => Triangulo::Isosceles,
    }
}
