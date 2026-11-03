static mut CONTADOR: u32 = 0;

fn main() {
    unsafe {
        CONTADOR += 1;
        // En la edición 2024 está prohibido crear referencias a un `static mut`.
        let r = &CONTADOR;
        println!("{r}");
    }
}
