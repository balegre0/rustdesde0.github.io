fn main() {
    let mut v = [5, 3, 9, 1, 7, 3];
    v.sort();
    println!("ordenado: {v:?}");
    println!("busco 7: {:?}, busco 4: {:?}", v.binary_search(&7), v.binary_search(&4));
    println!("trozos de 4: {:?}", v.chunks(4).collect::<Vec<_>>());
    println!("ventanas de 3: {:?}", v.windows(3).map(|w| w.iter().sum::<i32>()).collect::<Vec<_>>());
    let (izq, der) = v.split_at(2);
    println!("split_at(2): {izq:?} | {der:?}");
    println!("contiene 9: {}, empieza por [1, 3]: {}", v.contains(&9), v.starts_with(&[1, 3]));
    v.reverse();
    v.rotate_left(1);
    println!("invertido y rotado: {v:?}");
    let mut sin_rep = v.to_vec();
    sin_rep.sort_unstable();
    sin_rep.dedup();
    println!("sin repetidos: {sin_rep:?}");
    println!("primero {:?}, último {:?}, iter().max() {:?}", v.first(), v.last(), v.iter().max());
}
