fn main() {
    // 1. `mut` permite cambiar el valor más tarde
    let mut vidas = 3;
    println!("empiezas con {vidas} vidas");

    // 2. Pierdes una
    vidas = vidas - 1;
    println!("ahora tienes {vidas}");

    // 3. `+=` y `-=` son atajos de "x = x + ..."
    vidas += 5;
    println!("¡bonus! {vidas}");
    vidas -= 1;
    println!("final: {vidas}");
}
