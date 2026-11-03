fn uno() -> i32 {
    println!("uno");
    1
}

fn dos() -> i32 {
    println!("dos");
    2
}

fn suma(a: i32, b: i32) -> i32 {
    println!("suma");
    a + b
}

fn main() {
    let r = suma(dos(), uno());
    println!("r = {r}");
}
