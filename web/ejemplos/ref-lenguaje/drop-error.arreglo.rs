struct R;
impl Drop for R {
    fn drop(&mut self) { println!("drop"); }
}

fn main() {
    let r = R;
    // `drop(r)` (std::mem::drop) mueve el valor y lo destruye una sola vez.
    drop(r);
    println!("después");
}
