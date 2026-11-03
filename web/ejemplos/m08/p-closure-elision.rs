fn main() {
    let identidad = |x: &i32| -> &i32 { x };
    let n = 3;
    println!("{}", identidad(&n));
}
