use std::sync::RwLock;

fn main() {
    let precios = RwLock::new(vec![10, 20, 30]);
    let suma: i32 = precios.read().unwrap().iter().sum();
    println!("suma = {suma}");
    for p in precios.write().unwrap().iter_mut() {
        *p *= 2;
    }
    println!("{:?}", precios.read().unwrap());
}
