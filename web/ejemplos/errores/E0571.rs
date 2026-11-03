fn main() {
    let mut n = 0;
    let r = while n < 10 {
        n += 3;
        if n > 5 {
            break n;
        }
    };
    println!("{r:?}");
}
