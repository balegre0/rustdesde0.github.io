fn main() {
    let mut v: Vec<u32> = Vec::new();
    print!("{} ", v.capacity());
    v.push(1);
    print!("{} ", v.capacity());
    for i in 0..4 {
        v.push(i);
    }
    println!("{} (len {})", v.capacity(), v.len());
}
