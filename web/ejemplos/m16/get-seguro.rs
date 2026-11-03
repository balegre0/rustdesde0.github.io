// La misma lectura de antes, en Rust seguro: sin unsafe no hay UB posible.
fn main() {
    let v = vec![10, 20, 30];

    // 1. get devuelve Option: None si la posición no existe.
    println!("v.get(1) = {:?}", v.get(1));
    println!("v.get(3) = {:?}", v.get(3));

    // 2. Indexar con [] comprueba el límite y entra en pánico si te sales:
    //    un fallo ruidoso y seguro, nunca una lectura de memoria ajena.
    let i = v.len();
    println!("v[{i}] = ...");
    println!("{}", v[i]);
}
