fn sumador(n: i32) -> impl Fn(i32) -> i32 {
    |x| x + n
}

fn main() {
    let f = sumador(5);
    println!("{}", f(1));
}
