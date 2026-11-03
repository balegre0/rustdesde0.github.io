fn main() {
    // 1. Sentencias: hacen algo, no producen valor útil
    let a = 3; // sentencia `let`
    let b = 4;

    // 2. Expresiones: producen un valor (y se pueden anidar)
    let hipotenusa = ((a * a + b * b) as f64).sqrt();
    println!("hipotenusa = {hipotenusa}");

    // 3. Un bloque que termina en sentencia no produce valor: vale `()`
    let unidad = {
        let _suma = a + b;
    };
    println!("el bloque vale {unidad:?}");

    // 4. Una asignación también es una expresión... que vale `()`
    let mut c = 0;
    println!("c antes = {c}");
    let resultado = c = 10;
    println!("c = {c}, la asignación vale {resultado:?}");
}
