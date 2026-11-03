fn primero(v: &Vec<String>) -> &String {
    &v[0] // devolver un préstamo en vez de sacar el valor
}

fn main() {
    let v = vec![String::from("a")];
    println!("{}", primero(&v));
}
