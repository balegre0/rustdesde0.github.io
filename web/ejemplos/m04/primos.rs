fn main() {
    'candidatos: for n in 2..=30 {
        let mut d = 2;
        while d * d <= n {
            if n % d == 0 {
                continue 'candidatos; // tiene un divisor: no es primo
            }
            d += 1;
        }
        print!("{n} ");
    }
    println!();
}
