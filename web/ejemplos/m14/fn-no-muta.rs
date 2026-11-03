fn llama<F: Fn()>(f: F) {
    f();
}

fn main() {
    let mut pasos = 0;
    llama(|| pasos += 1);
    println!("{pasos}");
}
