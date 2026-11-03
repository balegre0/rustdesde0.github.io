fn main() {
    // 1. `while` comprueba la condición ANTES de cada vuelta
    let mut segundos = 3;
    while segundos > 0 {
        println!("{segundos}...");
        segundos -= 1;
    }
    println!("¡despegue!");

    // 2. Si la condición es falsa desde el principio, el cuerpo no se ejecuta nunca
    let mut vacio = 0;
    while vacio > 0 {
        println!("esto no se imprime");
        vacio -= 1;
    }
    println!("vacio sigue valiendo {vacio}");
}
