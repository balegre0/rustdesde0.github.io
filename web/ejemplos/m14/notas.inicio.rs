fn main() {
    let notas = vec![7, 4, 9, 5, 10, 3];

    // Sustituye cada todo!() por una cadena de iterador (sin bucles):
    let aprobados: usize = todo!(); // cuántas notas son >= 5 (filter + count)
    let mejor: Option<&i32> = todo!(); // la nota más alta (max)
    let media: f64 = todo!(); // suma (sum::<i32>()) dividida entre notas.len(), como f64
    let hay_diez: bool = todo!(); // ¿hay algún 10? (any)

    println!("aprobados: {aprobados}");
    println!("mejor: {mejor:?}");
    println!("media: {media:.2}");
    println!("¿algún 10? {hay_diez}");
}
