// Comparar flotantes: mejor «¿están muy cerca?» que «¿son iguales?».
fn main() {
    let suma: f64 = 0.1 + 0.2;

    // 1. Lo que hay de verdad guardado
    println!("0.1 + 0.2 = {suma}");

    // 2. ❌ Igualdad exacta: falla por una diferencia diminuta
    println!("suma == 0.3: {}", suma == 0.3);

    // 3. ✅ Distancia menor que una tolerancia que eliges tú
    let tolerancia = 1e-9;
    println!("cerca de 0.3: {}", (suma - 0.3).abs() < tolerancia);
}
