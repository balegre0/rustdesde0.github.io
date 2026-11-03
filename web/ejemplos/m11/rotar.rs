use std::collections::VecDeque;

fn rotar(v: &[i32], n: usize) -> Vec<i32> {
    let mut d: VecDeque<i32> = v.iter().copied().collect();
    for _ in 0..n {
        if let Some(x) = d.pop_front() {
            d.push_back(x);
        }
    }
    d.into_iter().collect()
}

fn main() {
    println!("{:?}", rotar(&[1, 2, 3, 4, 5], 2));
    println!("{:?}", rotar(&[1, 2, 3], 0));
}
