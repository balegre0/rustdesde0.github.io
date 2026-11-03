fn main() {
    // 1. crear
    let mut v: Vec<i32> = Vec::new();
    let w = vec![10, 20, 30];
    // 2. añadir y quitar
    v.push(1);
    v.push(2);
    v.push(3);
    let ultimo = v.pop();
    v.insert(0, 0);
    let quitado = v.remove(1);
    println!("{v:?} {ultimo:?} {quitado}");
    // 3. leer: indexar (puede entrar en pánico) o get (devuelve Option)
    println!("{} {:?} {:?}", w[1], w.get(1), w.get(9));
    // 4. recorrer y modificar
    for x in v.iter_mut() {
        *x *= 10;
    }
    println!("{v:?}");
    // 5. otras operaciones útiles
    let mut n = vec![5, 3, 8, 3, 1];
    n.sort();
    n.dedup();
    println!("{n:?} {} {}", n.len(), n.contains(&8));
    n.retain(|x| *x > 2);
    n.extend([9, 7]);
    n.truncate(3);
    println!("{n:?}");
}
