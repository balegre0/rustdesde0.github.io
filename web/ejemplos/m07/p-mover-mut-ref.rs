fn main() {
    let mut v = vec![1];
    let r = &mut v;
    let r2 = r;
    r2.push(2);
    r.push(3);
}
