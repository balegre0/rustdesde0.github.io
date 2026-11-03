fn suma_cifras(n: u64) -> u64 {
    let mut resto = n;
    let mut suma = 0;
    while resto > 0 {
        suma += resto % 10;
        resto /= 10;
    }
    suma
}
