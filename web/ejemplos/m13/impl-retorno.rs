// 1. Devolver `impl Trait`: «te doy algo que es un iterador de u32; no te digo cuál».
fn pares_hasta(n: u32) -> impl Iterator<Item = u32> {
    (0..=n).filter(|x| x % 2 == 0)
}

// 2. Devolver un closure: cada closure tiene un tipo sin nombre, así que
//    `impl Fn` es la forma de escribir su tipo.
fn sumador(k: i32) -> impl Fn(i32) -> i32 {
    move |x| x + k
}

// 3. En un argumento, `impl Trait` es un genérico sin nombre.
fn total(numeros: impl Iterator<Item = u32>) -> u32 {
    numeros.sum()
}

fn main() {
    let v: Vec<u32> = pares_hasta(10).collect();
    println!("{v:?}");

    let mas_tres = sumador(3);
    println!("{} {}", mas_tres(1), mas_tres(10));

    println!("{}", total(pares_hasta(10)));
}
