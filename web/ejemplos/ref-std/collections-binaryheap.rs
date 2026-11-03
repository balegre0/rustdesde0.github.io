use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let mut monton = BinaryHeap::new();
    for n in [5, 1, 8, 3, 9, 2] {
        monton.push(n);
    }
    // peek mira el mayor; pop lo saca
    println!("{:?}", monton.peek());
    let mut orden = Vec::new();
    while let Some(n) = monton.pop() {
        orden.push(n);
    }
    println!("{orden:?}");
    // con Reverse, sale primero el menor
    let mut tareas = BinaryHeap::new();
    tareas.push(Reverse((2, "barrer")));
    tareas.push(Reverse((1, "comer")));
    tareas.push(Reverse((3, "dormir")));
    while let Some(Reverse((prioridad, t))) = tareas.pop() {
        println!("{prioridad}: {t}");
    }
}
