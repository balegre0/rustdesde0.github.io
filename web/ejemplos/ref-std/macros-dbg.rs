fn factorial(n: u32) -> u32 {
    if dbg!(n <= 1) {
        dbg!(1)
    } else {
        dbg!(n * factorial(n - 1))
    }
}

fn main() {
    let x = dbg!(factorial(3)); // dbg! devuelve el valor
    let v = vec![1, 2];
    dbg!(&v); // con & no se mueve
    println!("{x} {v:?}");
}
