fn main() {
    let mut v = Vec::new();
    v.push(1);
    Vec::push(&mut v, 2);
    let n = Vec::len(&v);
    println!("{n} {}", v.len());
}
