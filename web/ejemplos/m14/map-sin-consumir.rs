#![deny(unused_must_use)]

fn main() {
    let v = vec![1, 2, 3];
    v.iter().map(|x| println!("{x}"));
}
