fn main() {
    for n in [3, 6, 10] {
        let t = match n {
            x if x % 2 == 0 => "par",
            x if x > 5 => "grande",
            _ => "otro",
        };
        print!("{t} ");
    }
    println!();
}
