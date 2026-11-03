use std::ptr;

// Simulamos los registros de un periférico con una zona de memoria.
// En un microcontrolador real serían direcciones fijas, como 0x5000_0504.
#[repr(C)]
struct Gpio {
    salida: u32,    // OUT: un bit por pin
    direccion: u32, // DIR: 1 = salida, 0 = entrada
}

fn pin_como_salida(gpio: *mut Gpio, pin: u32) {
    // SAFETY: gpio apunta a un Gpio válido durante toda la llamada.
    unsafe {
        let dir = ptr::addr_of_mut!((*gpio).direccion);
        // volatile: cada lectura y escritura ocurre DE VERDAD, en este orden.
        dir.write_volatile(dir.read_volatile() | (1 << pin));
    }
}

fn encender(gpio: *mut Gpio, pin: u32) {
    // SAFETY: igual que arriba.
    unsafe {
        let out = ptr::addr_of_mut!((*gpio).salida);
        out.write_volatile(out.read_volatile() | (1 << pin));
    }
}

fn main() {
    let mut periferico = Gpio { salida: 0, direccion: 0 };
    let gpio: *mut Gpio = &mut periferico;
    pin_como_salida(gpio, 13);
    encender(gpio, 13);
    println!("DIR = {:#034b}", periferico.direccion);
    println!("OUT = {:#034b}", periferico.salida);
    println!("¿LED del pin 13 encendido? {}", periferico.salida & (1 << 13) != 0);
}
