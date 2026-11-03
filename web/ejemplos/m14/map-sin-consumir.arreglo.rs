fn main() {
    let v = vec![1, 2, 3];
    // Para hacer algo con cada elemento, un `for` (o `for_each`), no `map`.
    for x in v.iter() {
        println!("{x}");
    }
}
