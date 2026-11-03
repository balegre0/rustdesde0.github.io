fn multiplos(n: u32, hasta: u32) -> impl Iterator<Item = u32> {
    (1..=hasta).filter(|x| x % n == 0)
}

fn main() {
    println!("{:?}", multiplos(3, 20).collect::<Vec<_>>());
}
