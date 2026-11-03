fn main() {
    let mut v = vec![5, 0, 3, 0, 8, 1];

    // pop_if: saca el último solo si cumple algo.
    println!("pop_if: {:?} -> {v:?}", v.pop_if(|x| *x < 3));

    // extract_if: saca (y devuelve) los que cumplen, y deja el resto.
    let ceros: Vec<i32> = v.extract_if(.., |x| *x == 0).collect();
    println!("extract_if: sacó {ceros:?}, queda {v:?}");

    // retain_mut: decidir y modificar en la misma pasada.
    v.retain_mut(|x| { *x *= 10; *x < 60 });
    println!("retain_mut: {v:?}");

    // swap_remove: quitar en O(1) cambiando el orden.
    let mut w = vec!['a', 'b', 'c', 'd'];
    let quitado = w.swap_remove(0);
    println!("swap_remove: {quitado} -> {w:?}");

    // resize, resize_with, truncate, dedup_by_key.
    let mut r = vec![1, 2];
    r.resize(4, 0);
    r.resize_with(6, Default::default);
    println!("resize: {r:?}");
    r.truncate(3);
    let mut palabras = vec!["Sol", "sol", "Luna", "LUNA", "mar"];
    palabras.dedup_by_key(|p| p.to_lowercase());
    println!("truncate: {r:?}  dedup_by_key: {palabras:?}");

    // append, split_off, drain con rango, extend_from_slice.
    let mut a = vec![1, 2, 3];
    let mut b = vec![4, 5];
    a.append(&mut b);
    let cola = a.split_off(3);
    let medio: Vec<i32> = a.drain(1..).collect();
    a.extend_from_slice(&[9, 9]);
    println!("append/split_off/drain: a={a:?} b={b:?} cola={cola:?} medio={medio:?}");

    // Capacidad: reservar y devolver memoria.
    let mut c: Vec<u64> = Vec::with_capacity(100);
    c.push(1);
    println!("capacidad antes de shrink_to_fit: al menos 100 = {}", c.capacity() >= 100);
    c.shrink_to_fit();
    println!("después: {}", c.capacity());

    // De Vec a Box<[T]> (sin capacidad sobrante).
    let caja: Box<[i32]> = vec![1, 2, 3].into_boxed_slice();
    println!("into_boxed_slice: {caja:?}");
}
