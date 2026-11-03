fn main() {
    let mut vueltas = 0;
    for _ in 5..1 {
        vueltas += 1;
    }
    println!("vueltas = {vueltas}");
    for i in (1..5).rev() {
        print!("{i} ");
    }
    println!();
}
