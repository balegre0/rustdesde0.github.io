fn main() {
    let mut n = 0;
    let r = loop {
        n += 3;
        if n > 5 {
            break n;
        }
    };
    println!("{r}");
}
