#[derive(Debug, Clone, PartialEq)]
enum Pedido {
    Pendiente,
    Pagado { importe: u32 },
    Enviado { importe: u32, seguimiento: String },
    Cancelado,
}

impl Pedido {
    fn pagar(self, importe: u32) -> Result<Pedido, Pedido> {
        match self {
            Pedido::Pendiente => Ok(Pedido::Pagado { importe }),
            otro => Err(otro),
        }
    }

    fn enviar(self, seguimiento: &str) -> Result<Pedido, Pedido> {
        match self {
            Pedido::Pagado { importe } => Ok(Pedido::Enviado { importe, seguimiento: seguimiento.to_string() }),
            otro => Err(otro),
        }
    }

    fn cancelar(self) -> Result<Pedido, Pedido> {
        match self {
            Pedido::Pendiente => Ok(Pedido::Cancelado),
            otro => Err(otro),
        }
    }

    fn describir(&self) -> String {
        match self {
            Pedido::Pendiente => String::from("pendiente"),
            Pedido::Pagado { importe } => format!("pagado ({importe} €)"),
            Pedido::Enviado { seguimiento, .. } => format!("enviado ({seguimiento})"),
            Pedido::Cancelado => String::from("cancelado"),
        }
    }
}
