fn main() {
    let tareas = vec![String::from("comprar"), String::from("estudiar")];
    for t in tareas {
        println!("hago {t}");
    }
    println!("quedan {} tareas", tareas.len());
}
