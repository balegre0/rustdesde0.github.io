fn main() {
    let v: Vec<i32> = [1, 2, 3]
        .iter()
        .filter(|x| { print!("f{x} "); **x != 2 })
        .map(|x| { print!("m{x} "); x * 10 })
        .collect();
    println!("\n{v:?}");
}
