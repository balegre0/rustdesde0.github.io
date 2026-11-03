struct Contador {
    valor: u32,
}

fn main() {
    let c = Contador { valor: 0 };
    c.valor = c.valor + 1;
    println!("{}", c.valor);
}
