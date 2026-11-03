fn main() {
    // 1. Una tupla agrupa valores de tipos distintos
    let persona: (&str, u8, f64) = ("Lucía", 34, 1.68);
    // 2. Se accede por posición: .0, .1, .2
    println!("nombre: {}", persona.0);
    println!("edad: {}", persona.1);
    // 3. {:?} la muestra entera
    println!("{:?}", persona);
    // 4. Desestructurar: sacar las partes en variables
    let (nombre, edad, altura) = persona;
    println!("{nombre} tiene {edad} años y mide {altura}");
    // 5. La tupla vacía () se llama "unit"
    let nada = ();
    println!("unit: {:?}", nada);
}
