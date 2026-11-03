use std::collections::VecDeque;

fn main() {
    let mut cola = VecDeque::new();
    // una cola: entra por detrás, sale por delante
    cola.push_back("Ana");
    cola.push_back("Luis");
    cola.push_front("Eva"); // se cuela al principio
    println!("{cola:?}");
    println!("{:?}", cola.pop_front());
    println!("{:?} {:?}", cola.front(), cola.back());
    cola.push_back("Sol");
    println!("{:?} {}", cola.pop_back(), cola.len());
    let mut v = VecDeque::from([1, 2, 3, 4, 5]);
    v.rotate_left(2);
    println!("{v:?} {:?}", v[0]);
}
