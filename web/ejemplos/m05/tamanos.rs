fn main() {
    let a: u8 = 1;
    let b: i32 = -5;
    let c: f64 = 3.5;
    let d: bool = true;
    let e: char = 'ñ';
    let f: [u8; 100] = [0; 100];
    let g: (u8, u32) = (1, 2);
    let h: usize = 9;
    // `size_of_val(&x)` dice cuántos bytes ocupa `x`
    println!("u8       {:>3} bytes", size_of_val(&a));
    println!("i32      {:>3} bytes", size_of_val(&b));
    println!("f64      {:>3} bytes", size_of_val(&c));
    println!("bool     {:>3} bytes", size_of_val(&d));
    println!("char     {:>3} bytes", size_of_val(&e));
    println!("[u8;100] {:>3} bytes", size_of_val(&f));
    println!("(u8,u32) {:>3} bytes", size_of_val(&g));
    println!("usize    {:>3} bytes", size_of_val(&h));
}
