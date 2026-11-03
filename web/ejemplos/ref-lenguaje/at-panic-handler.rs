// Biblioteca no_std: sin sistema operativo hay que decir qué hacer en un pánico.
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn al_entrar_en_panico(_info: &PanicInfo) -> ! {
    loop {} // en un microcontrolador: parar, reiniciar o encender un LED
}

pub fn dividir(a: u32, b: u32) -> u32 {
    a / b
}
