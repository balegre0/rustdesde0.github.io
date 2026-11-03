// embedded-hal define traits como este. Un controlador (driver) escrito contra el trait
// funciona con cualquier microcontrolador que lo implemente.
trait PinSalida {
    fn alto(&mut self);
    fn bajo(&mut self);
}

// «Placa» simulada: guarda un registro de lo que pasa.
struct PinSimulado {
    nombre: &'static str,
    estado: bool,
    cambios: u32,
}

impl PinSalida for PinSimulado {
    fn alto(&mut self) {
        self.estado = true;
        self.cambios += 1;
    }
    fn bajo(&mut self) {
        self.estado = false;
        self.cambios += 1;
    }
}

// Un controlador genérico: no sabe nada de la placa.
struct Led<P: PinSalida> {
    pin: P,
}

impl<P: PinSalida> Led<P> {
    fn parpadear(&mut self, veces: u32) {
        for _ in 0..veces {
            self.pin.alto();
            self.pin.bajo();
        }
    }
}

fn main() {
    let mut led = Led { pin: PinSimulado { nombre: "P0.13", estado: false, cambios: 0 } };
    led.parpadear(3);
    let p = &led.pin;
    println!("{}: {} cambios, ahora {}", p.nombre, p.cambios, if p.estado { "alto" } else { "bajo" });
}
