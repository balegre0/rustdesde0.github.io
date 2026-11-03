fn main() {
    let b: Box<[i32]> = vec![1, 2, 3].into_boxed_slice();
    // Edición 2024: `into_iter()` sobre un Box<[T]> da los valores T (no &T).
    let suma: i32 = b.into_iter().map(|x: i32| x * 10).sum();
    println!("{suma}");
}
