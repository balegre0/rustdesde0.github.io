fn maximo(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn maximo3(a: i32, b: i32, c: i32) -> i32 {
    maximo(maximo(a, b), c)
}

fn main() {
    println!("{}", maximo(3, 9));
    println!("{}", maximo(-2, -7));
    println!("{}", maximo3(4, 11, 6));
}
