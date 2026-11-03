struct Bloques {
    ocupados: u8,
}

impl Bloques {
    fn new() -> Bloques {
        Bloques { ocupados: 0 }
    }

    fn reservar(&mut self) -> Option<usize> {
        let i = (!self.ocupados).trailing_zeros();
        if i >= 8 {
            return None;
        }
        self.ocupados |= 1 << i;
        Some(i as usize)
    }

    fn liberar(&mut self, i: usize) -> Result<(), &'static str> {
        if i >= 8 {
            return Err("fuera de rango");
        }
        if self.ocupados & (1 << i) == 0 {
            return Err("doble liberación");
        }
        self.ocupados &= !(1 << i);
        Ok(())
    }

    fn libres(&self) -> u32 {
        self.ocupados.count_zeros()
    }
}
