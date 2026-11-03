fn porcentaje(parte: u32, total: u32) -> u32 {
    assert!(total != 0, "el total no puede ser 0");
    parte * 100 / total
}

fn main() {
    println!("{}", porcentaje(1, 4));
}
