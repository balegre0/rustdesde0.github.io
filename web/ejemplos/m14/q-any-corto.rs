fn main() {
    let mut miradas = 0;
    let hay = [1, 9, 3, 9].iter().any(|&x| {
        miradas += 1;
        x == 9
    });
    println!("{hay} {miradas}");
}
