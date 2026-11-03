#[derive(Debug, Clone, Copy, PartialEq)]
enum Mes {
    Enero, Febrero, Marzo, Abril, Mayo, Junio,
    Julio, Agosto, Septiembre, Octubre, Noviembre, Diciembre,
}

#[derive(Debug, PartialEq)]
struct MesInvalido(u8);

impl TryFrom<u8> for Mes {
    type Error = MesInvalido;
    fn try_from(n: u8) -> Result<Mes, MesInvalido> {
        todo!()
    }
}
