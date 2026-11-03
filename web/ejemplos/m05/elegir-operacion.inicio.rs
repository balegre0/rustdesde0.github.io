fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

fn restar(a: i32, b: i32) -> i32 {
    a - b
}

fn multiplicar(a: i32, b: i32) -> i32 {
    a * b
}

// Devuelve la FUNCIÓN que corresponde al símbolo: '+' sumar, '-' restar, cualquier otro multiplicar.
fn elegir(simbolo: char) -> fn(i32, i32) -> i32 {
    let _ = simbolo;
    todo!()
}

fn main() {
    for s in ['+', '-', '*'] {
        let op = elegir(s);
        println!("12 {s} 4 = {}", op(12, 4));
    }
}
