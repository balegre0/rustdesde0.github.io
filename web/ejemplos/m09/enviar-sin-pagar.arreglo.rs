struct PedidoPendiente { total: u32 }
struct PedidoPagado { total: u32 }

impl PedidoPendiente {
    fn pagar(self) -> PedidoPagado {
        PedidoPagado { total: self.total }
    }
}

impl PedidoPagado {
    fn enviar(&self) {
        println!("enviando pedido de {} €", self.total);
    }
}

fn main() {
    let p = PedidoPendiente { total: 30 };
    let pagado = p.pagar();
    pagado.enviar();
}
