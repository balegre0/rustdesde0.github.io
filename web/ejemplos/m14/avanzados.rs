// 1. Devolver un iterador sin construir ningún Vec: `impl Iterator`.
fn pares_hasta(n: u32) -> impl Iterator<Item = u32> {
    (0..=n).filter(|x| x % 2 == 0)
}

fn main() {
    println!("pares: {:?}", pares_hasta(8).collect::<Vec<_>>());

    // 2. collect a Result: si TODOS son Ok, Ok(Vec); si alguno falla, el primer Err.
    let bien: Result<Vec<i32>, _> = ["1", "2", "3"].iter().map(|s| s.parse::<i32>()).collect();
    let mal: Result<Vec<i32>, _> = ["1", "dos", "3"].iter().map(|s| s.parse::<i32>()).collect();
    println!("{bien:?}");
    println!("{mal:?}");

    // 3. windows y chunks (en slices): ventanas solapadas o trozos.
    let temps = [14, 17, 16, 21];
    let subidas: Vec<i32> = temps.windows(2).map(|w| w[1] - w[0]).collect();
    println!("cambios: {subidas:?}");
    for trozo in [1, 2, 3, 4, 5].chunks(2) {
        print!("{trozo:?} ");
    }
    println!();

    // 4. peekable: mirar el siguiente sin consumirlo.
    let mut it = [1, 1, 2, 3, 3, 3].into_iter().peekable();
    while let Some(x) = it.next() {
        let mut veces = 1;
        while it.peek() == Some(&x) {
            it.next();
            veces += 1;
        }
        print!("{x}×{veces} ");
    }
    println!();
}
