fn main() {
    let mut v = vec![1, 2, 3];
    let mut nuevos = Vec::new();
    for x in &v {
        if *x == 2 {
            nuevos.push(20); // se apunta aparte...
        }
    }
    v.extend(nuevos); // ...y se añade cuando el recorrido terminó
    println!("{v:?}");
}
