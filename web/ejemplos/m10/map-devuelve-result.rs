fn main() {
    let r: Result<i32, String> = Ok(4);
    let doble_raiz: Result<i32, String> = r.map(|n| {
        if n < 0 { Err(String::from("negativo")) } else { Ok(n.isqrt() * 2) }
    });
    println!("{:?}", doble_raiz);
}
