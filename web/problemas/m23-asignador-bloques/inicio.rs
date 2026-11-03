struct Bloques {
    ocupados: u8,
}

impl Bloques {
    fn new() -> Bloques {
        Bloques { ocupados: 0 }
    }

    fn reservar(&mut self) -> Option<usize> {
        todo!()
    }

    fn liberar(&mut self, i: usize) -> Result<(), &'static str> {
        todo!()
    }

    fn libres(&self) -> u32 {
        todo!()
    }
}
