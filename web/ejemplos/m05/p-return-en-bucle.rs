fn primer_multiplo(de: u32, desde: u32) -> u32 {
    for n in desde.. {
        if n % de == 0 {
            return n;
        }
    }
    0
}

fn main() {
    println!("{}", primer_multiplo(7, 50));
}
