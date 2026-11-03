fn agregar_y_ver(v: &mut Vec<i32>) -> &i32 {
    v.push(0);
    &v[0]
}

fn main() {
    let mut v = vec![7];
    let primero = agregar_y_ver(&mut v);
    let otra = &v;
    println!("{primero} {}", otra.len());
}
