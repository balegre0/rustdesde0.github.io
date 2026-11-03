fn main() {
    // 1. Repetir mientras la condición sea verdadera.
    let mut n = 10;
    while n > 0 {
        print!("{n} ");
        n -= 3;
    }
    println!();
    // 2. `while let`: repetir mientras el patrón encaje.
    let mut pila = vec!['a', 'b', 'c'];
    while let Some(tope) = pila.pop() {
        print!("{tope} ");
    }
    println!();
    // 3. Cadena let en un while (edición 2024).
    let mut v = vec![1, 2, 30, 4];
    while let Some(&x) = v.last() && x < 10 {
        v.pop();
    }
    println!("{v:?}");
}
