// 1. Puedes marcar tus propias funciones (y tipos) con #[must_use].
#[must_use = "comprueba si la reserva se pudo hacer"]
fn reservar(plazas_libres: u32) -> bool {
    plazas_libres > 0
}

// 2. Un tipo marcado: cualquier valor suyo ignorado da aviso.
#[must_use]
struct Ticket(u32);

fn sacar_ticket() -> Ticket {
    Ticket(17)
}

fn main() {
    reservar(0);
    sacar_ticket();
    let t = sacar_ticket();
    println!("ticket {}", t.0);
}
