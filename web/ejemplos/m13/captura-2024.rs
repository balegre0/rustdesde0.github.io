// En la edición 2024, `impl Trait` en el retorno captura TODOS los
// parámetros genéricos en ámbito, también el tiempo de vida de `v`.
fn medidor(v: &Vec<i32>) -> impl Fn() -> usize {
    let n = v.len();
    move || n
}

fn main() {
    let mut v = vec![1, 2, 3];
    let f = medidor(&v);
    v.push(4);
    println!("{}", f());
}
