use std::cmp::{self, Ordering, Reverse};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    mayor: u32,
    menor: u32,
}

#[derive(Debug)]
struct Alumno {
    nombre: &'static str,
    nota: u32,
}

fn main() {
    // 1. Ordering: el resultado de comparar
    println!("{:?} {:?} {:?}", 1.cmp(&2), 2.cmp(&2), 3.cmp(&2));
    // 2. derive compara campo a campo, en orden
    let a = Version { mayor: 1, menor: 9 };
    let b = Version { mayor: 2, menor: 0 };
    println!("{} {:?}", a < b, a.cmp(&b));
    println!("{:?}", cmp::max(a, b));
    // 3. ordenar con una clave o un comparador propio
    let mut clase = vec![
        Alumno { nombre: "Luis", nota: 7 },
        Alumno { nombre: "Ana", nota: 9 },
        Alumno { nombre: "Eva", nota: 7 },
    ];
    clase.sort_by(|x, y| y.nota.cmp(&x.nota).then(x.nombre.cmp(y.nombre)));
    println!("{:?}", clase.iter().map(|a| a.nombre).collect::<Vec<_>>());
    let mut n = vec![3, 1, 2];
    n.sort_by_key(|&x| Reverse(x));
    println!("{n:?}");
    // 4. f64 solo es PartialOrd: NaN no se puede ordenar
    println!("{:?} {:?}", 1.0f64.partial_cmp(&f64::NAN), 1.0f64.total_cmp(&2.0));
    println!("{} {}", cmp::min(4, 9), 15.clamp(0, 10));
    println!("{}", Ordering::Less.reverse() == Ordering::Greater);
    println!("{:?}", clase[0]);
}
