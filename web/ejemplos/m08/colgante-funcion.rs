fn mayor(v: &Vec<i32>) -> &i32 {
    let mut m = &v[0];
    for x in v {
        if x > m {
            m = x;
        }
    }
    m
}

fn main() {
    let numeros = vec![4, 9, 2];
    let m = mayor(&numeros);
    println!("el mayor es {m}");
}
