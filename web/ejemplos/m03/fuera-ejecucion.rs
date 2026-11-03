fn main() {
    let notas = [7, 9, 5];
    // El índice se calcula mientras el programa corre
    let mut i = 0;
    for _ in 0..4 {
        println!("nota {i}: {}", notas[i]);
        i += 1;
    }
}
