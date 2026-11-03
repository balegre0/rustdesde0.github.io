// `use<>` (vacío) dice: «el tipo devuelto no captura nada»,
// así que el préstamo de `v` termina al volver de la función.
fn medidor(v: &Vec<i32>) -> impl Fn() -> usize + use<> {
    let n = v.len();
    move || n
}

fn main() {
    let mut v = vec![1, 2, 3];
    let f = medidor(&v);
    v.push(4);
    println!("{} {}", f(), v.len());
}
