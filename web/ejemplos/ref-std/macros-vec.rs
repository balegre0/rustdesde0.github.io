fn main() {
    let a = vec![1, 2, 3]; // con elementos
    let b = vec![0u8; 5]; // un valor repetido N veces
    let c: Vec<String> = vec![]; // vacío
    let d = vec![String::from("x"); 2]; // clona el valor para cada posición
    println!("{a:?} {b:?} {c:?} {d:?}");
    println!("{} {}", a.capacity() >= 3, b.len());
}
