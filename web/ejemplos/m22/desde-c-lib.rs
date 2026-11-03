use std::ffi::{c_char, CStr};
#[unsafe(no_mangle)]
pub extern "C" fn rs_contar_vocales(s: *const c_char) -> u32 {
    if s.is_null() { return 0; }
    let texto = unsafe { CStr::from_ptr(s) };
    texto.to_bytes().iter().filter(|b| b"aeiouAEIOU".contains(b)).count() as u32
}
#[unsafe(no_mangle)]
pub extern "C" fn rs_puede_entrar_en_panico(x: i32) -> i32 {
    let r = std::panic::catch_unwind(|| { if x < 0 { panic!("negativo"); } x * 2 });
    r.unwrap_or(-1)
}
