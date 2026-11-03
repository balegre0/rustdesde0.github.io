fn main() {
    let x = 7;
    let p = &raw const x;
    let y = *p; // desreferenciar sin `unsafe`
    println!("{y}");
}
