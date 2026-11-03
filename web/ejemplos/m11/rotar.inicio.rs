// Escribe `rotar(v: &[i32], n: usize) -> Vec<i32>` que pase los n primeros
// elementos al final, usando un VecDeque (pop_front + push_back).

use std::collections::VecDeque;

fn rotar(v: &[i32], n: usize) -> Vec<i32> {
    todo!()
}

fn main() {
    println!("{:?}", rotar(&[1, 2, 3, 4, 5], 2));
    println!("{:?}", rotar(&[1, 2, 3], 0));
}
