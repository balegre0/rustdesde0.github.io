#[derive(Clone, Copy)]
struct Recurso(u32);

impl Drop for Recurso {
    fn drop(&mut self) {
        println!("libero el recurso {}", self.0);
    }
}

fn main() {
    let r = Recurso(1);
    let _r2 = r;
}
