fn multiplos(k: u32, cuantos: u32) -> impl Iterator<Item = u32> {
    (1..=cuantos).map(move |i| i * k)
}

fn main() {
    let v: Vec<u32> = multiplos(3, 5).collect();
    println!("{v:?}");
    println!("{}", multiplos(10, 4).sum::<u32>());
}
