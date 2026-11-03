#[allow(dead_code)]
fn sin_usar() {
    println!("Nadie me llama");
}

fn main() {
    // 1. Un nombre que empieza por _ le dice a Rust: «sé que no lo uso».
    let _total = 42;
    println!("Sin avisos");
}
