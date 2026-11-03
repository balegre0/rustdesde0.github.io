// Edición 2024: crear referencias a un `static mut` está prohibido por defecto.
static mut CONTADOR: u32 = 0;

fn main() {
    unsafe { CONTADOR += 1 };
    let r: &u32 = unsafe { &CONTADOR };
    println!("{r}");
}
