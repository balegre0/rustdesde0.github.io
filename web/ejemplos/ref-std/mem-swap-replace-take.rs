use std::mem;

#[derive(Debug, Default)]
struct Jugador {
    nombre: String,
    puntos: u32,
}

fn main() {
    // swap: intercambia dos valores a través de &mut
    let mut a = String::from("izquierda");
    let mut b = String::from("derecha");
    mem::swap(&mut a, &mut b);
    println!("{a} {b}");
    // replace: pone un valor nuevo y devuelve el viejo
    let mut j = Jugador { nombre: String::from("Ana"), puntos: 10 };
    let viejo = mem::replace(&mut j.nombre, String::from("Eva"));
    println!("{viejo} -> {j:?}");
    // take: se lleva el valor y deja el valor por defecto
    let nombre = mem::take(&mut j.nombre);
    println!("{nombre:?} -> {j:?}");
    let todo = mem::take(&mut j);
    println!("{todo:?} {j:?} ({} puntos)", todo.puntos);
}
