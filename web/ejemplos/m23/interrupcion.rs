use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

// Datos compartidos entre el programa principal y una rutina de interrupción:
// atómicos, porque la interrupción puede llegar en mitad de cualquier instrucción.
static BOTON_PULSADO: AtomicBool = AtomicBool::new(false);
static PULSACIONES: AtomicU32 = AtomicU32::new(0);

// En un microcontrolador, el hardware llamaría a esta función al pulsar el botón.
fn interrupcion_boton() {
    BOTON_PULSADO.store(true, Ordering::Release);
    PULSACIONES.fetch_add(1, Ordering::Relaxed);
}

fn main() {
    let mut atendidas = 0;
    for tick in 0..10 {
        if tick % 3 == 0 {
            interrupcion_boton(); // simulamos que llega la interrupción
        }
        // Bucle principal: atiende el aviso y lo borra.
        if BOTON_PULSADO.swap(false, Ordering::Acquire) {
            atendidas += 1;
        }
    }
    println!("pulsaciones: {} · atendidas: {atendidas}", PULSACIONES.load(Ordering::Relaxed));
}
