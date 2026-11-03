fn precio(unidades: u32) -> u32 {
    unidades * 10
}

fn main() {
    let total = precio(2);
    println!("{total}");
}
