struct R;
impl Drop for R {
    fn drop(&mut self) { println!("drop"); }
}

fn main() {
    let r = R;
    // Llamar al método drop a mano está prohibido: se ejecutaría dos veces.
    r.drop();
}
