fn main() {
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    // consumidores: recorren el iterador y producen un resultado
    println!("{}", v.iter().sum::<i32>());
    println!("{}", v.iter().product::<i32>());
    println!("{}", v.iter().count());
    println!("{:?} {:?}", v.iter().min(), v.iter().max());
    println!("{:?}", v.iter().max_by_key(|x| (*x % 5, **x)));
    println!("{:?}", v.iter().find(|x| **x > 4));
    println!("{:?}", v.iter().position(|x| *x == 1));
    println!("{} {}", v.iter().any(|x| *x > 8), v.iter().all(|x| *x > 0));
    println!("{}", v.iter().fold(0, |acc, x| acc * 10 + x));
    println!("{:?}", v.iter().last());
    println!("{:?}", v.iter().nth(2));
    let (pares, impares): (Vec<i32>, Vec<i32>) = v.iter().partition(|x| *x % 2 == 0);
    println!("{pares:?} {impares:?}");
    let texto: String = vec!["a", "b", "c"].join("-");
    println!("{texto}");
    let r: Result<Vec<i32>, _> = ["1", "2", "x"].iter().map(|s| s.parse::<i32>()).collect();
    println!("{}", r.is_err());
    // perezoso: sin consumidor, map no hace nada
    let _sin_usar = v.iter().map(|x| { println!("nunca se imprime"); x });
    println!("fin");
}
