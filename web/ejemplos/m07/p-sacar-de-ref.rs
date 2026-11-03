fn primero(v: &Vec<String>) -> String {
    v[0]
}

fn main() {
    let v = vec![String::from("a")];
    println!("{}", primero(&v));
}
