fn main() {
    let notas = vec![7, 4, 9, 5, 10, 3];

    let aprobados = notas.iter().filter(|&&n| n >= 5).count();
    let mejor = notas.iter().max();
    let media = notas.iter().sum::<i32>() as f64 / notas.len() as f64;
    let hay_diez = notas.iter().any(|&n| n == 10);

    println!("aprobados: {aprobados}");
    println!("mejor: {mejor:?}");
    println!("media: {media:.2}");
    println!("¿algún 10? {hay_diez}");
}
