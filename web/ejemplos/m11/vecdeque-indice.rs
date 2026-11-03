use std::collections::VecDeque;

fn main() {
    let cola: VecDeque<i32> = VecDeque::from([1, 2, 3]);
    let trozo: &[i32] = &cola[..];
    println!("{:?}", trozo);
}
