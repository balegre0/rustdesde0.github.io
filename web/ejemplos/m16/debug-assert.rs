// debug_assert! solo se comprueba en el perfil debug; en release desaparece.
fn porcentaje(parte: u32, total: u32) -> u32 {
    debug_assert!(parte <= total, "parte ({parte}) mayor que total ({total})");
    parte * 100 / total
}

fn main() {
    println!("{}%", porcentaje(1, 4));
    println!("{}%", porcentaje(5, 4));
}
