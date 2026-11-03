fn main() {
    let mut s = String::from("hola");
    let a = &mut s;
    let b = &mut s;
    a.push('!');
    b.push('?');
}
