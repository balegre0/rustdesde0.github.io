fn main() {
    let mut intento = 1;
    let resultado = loop {
        if intento * intento > 10 {
            break intento;
        }
        intento += 1;
    };
    println!("El primer número cuyo cuadrado pasa de 10 es {resultado}");
}
