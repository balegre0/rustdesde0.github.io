fn main() {
    // 1. Crear una variable y darle un valor
    let edad = 30;
    println!("edad = {edad}");

    // 2. Decir el tipo a mano (anotación de tipo)
    let altura: f64 = 1.75;
    println!("altura = {altura}");

    // 3. Una variable puede guardar el resultado de una expresión
    let el_doble = edad * 2;
    println!("el doble de {edad} es {el_doble}");

    // 4. Declarar ahora y dar el valor después (una sola vez)
    let saludo;
    saludo = "hola";
    println!("{saludo}");
}
