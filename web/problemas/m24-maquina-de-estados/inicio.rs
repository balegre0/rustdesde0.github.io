#[derive(Debug, Clone, PartialEq)]
enum Pedido {
    Pendiente,
    Pagado { importe: u32 },
    Enviado { importe: u32, seguimiento: String },
    Cancelado,
}

impl Pedido {
    fn pagar(self, importe: u32) -> Result<Pedido, Pedido> {
        todo!()
    }

    fn enviar(self, seguimiento: &str) -> Result<Pedido, Pedido> {
        todo!()
    }

    fn cancelar(self) -> Result<Pedido, Pedido> {
        todo!()
    }

    fn describir(&self) -> String {
        todo!()
    }
}
