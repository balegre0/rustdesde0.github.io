fn main() {
    let a = 1;
    let b = 2;
    let c = 3;
    // Las comparaciones no se encadenan.
    if a < b < c {
        println!("ordenados");
    }
}
