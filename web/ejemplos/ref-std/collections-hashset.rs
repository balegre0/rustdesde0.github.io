use std::collections::HashSet;

fn main() {
    let mut vistos = HashSet::new();
    for n in [3, 1, 3, 2, 1] {
        // insert devuelve false si ya estaba
        let nuevo = vistos.insert(n);
        println!("{n}: nuevo = {nuevo}");
    }
    println!("{} {}", vistos.len(), vistos.contains(&2));
    let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = [2, 3, 4].into_iter().collect();
    let mut comun: Vec<_> = a.intersection(&b).collect();
    comun.sort();
    let mut union: Vec<_> = a.union(&b).collect();
    union.sort();
    let mut solo_a: Vec<_> = a.difference(&b).collect();
    solo_a.sort();
    println!("{comun:?} {union:?} {solo_a:?}");
}
