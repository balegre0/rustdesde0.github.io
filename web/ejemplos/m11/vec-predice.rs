fn main() {
    let mut v = vec![10, 20, 30];
    v.push(40);
    let ultimo = v.pop();
    v.insert(0, 5);
    println!("{:?} {:?} {}", v, ultimo, v.len());
}
