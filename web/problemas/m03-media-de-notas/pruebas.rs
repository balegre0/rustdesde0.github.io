
fn main() {
    let casos: [([u8; 5], f64); 6] = [
        ([10, 10, 10, 10, 10], 10.0),
        ([1, 2, 3, 4, 5], 3.0),
        ([0, 0, 0, 0, 1], 0.2),
        ([255, 255, 255, 255, 255], 255.0),
        ([0, 0, 0, 0, 0], 0.0),
        ([200, 100, 50, 25, 0], 75.0),
    ];
    let mut fallos = 0;
    for (notas, esperado) in casos {
        let obtenido = media(notas);
        if (obtenido - esperado).abs() < 1e-9 {
            println!("OK: media({:?}) = {obtenido}", notas);
        } else {
            println!("FALLO: media({:?}): esperaba {esperado}, obtuve {obtenido}", notas);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
