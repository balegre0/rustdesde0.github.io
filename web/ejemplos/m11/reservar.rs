fn main() {
    let mut v: Vec<u64> = Vec::with_capacity(50);
    for i in 1..=50u64 {
        v.push(i * i);
    }
    println!("len {}, cap {}", v.len(), v.capacity());
    println!("último: {}", v[49]);
}
