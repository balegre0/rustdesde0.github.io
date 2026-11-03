fn main() {
    let mut v = vec![String::from("a"), String::from("b"), String::from("c")];
    let x = v.remove(1);
    let y = v.pop();
    println!("{x} {:?} {:?}", y, v);
}
