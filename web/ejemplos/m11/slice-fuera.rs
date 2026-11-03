fn main() {
    let v = vec![1, 2, 3, 4];
    let desde = 2;
    let hasta = desde + 5;
    println!("{:?}", &v[desde..hasta]);
}
