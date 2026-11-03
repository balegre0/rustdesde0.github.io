fn main() {
    let o: Option<i32> = Some(3);
    // Un `let` normal exige un patrón irrefutable: `Some(x)` puede fallar.
    let Some(x) = o;
    println!("{x}");
}
