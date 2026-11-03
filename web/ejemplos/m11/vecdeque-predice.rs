use std::collections::VecDeque;

fn main() {
    let mut cola = VecDeque::new();
    cola.push_back("Ana");
    cola.push_back("Luis");
    cola.push_front("Zoe");
    let atendido = cola.pop_front();
    println!("{:?} {:?}", atendido, cola);
}
