fn main() {
    let v = [2, 4, 5, 6, 8];
    let a: Vec<_> = v.iter().take_while(|&&x| x % 2 == 0).collect();
    let b: Vec<_> = v.iter().filter(|&&x| x % 2 == 0).collect();
    println!("{a:?} {b:?}");
}
