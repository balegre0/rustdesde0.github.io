fn main() {
    let v = vec![1, 5, 2, 7];
    // El patrón |&&x| quita las dos capas de referencia (x es i32, que es Copy).
    let grandes: Vec<&i32> = v.iter().filter(|&&x| x > 3).collect();
    println!("{grandes:?}");
}
