fn agregar(v: &mut Vec<i32>) {
    v.push(v.len() as i32);
}

fn main() {
    let mut v = Vec::new();
    let r = &mut v;
    agregar(r);
    agregar(r);
    r.push(9);
    println!("{v:?}");
}
