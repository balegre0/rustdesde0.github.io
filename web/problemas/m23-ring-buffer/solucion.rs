const CAP: usize = 4;

struct Anillo {
    datos: [u8; CAP],
    inicio: usize,
    n: usize,
}

impl Anillo {
    const fn new() -> Anillo {
        Anillo { datos: [0; CAP], inicio: 0, n: 0 }
    }

    fn meter(&mut self, b: u8) -> Result<(), u8> {
        if self.n == CAP {
            return Err(b);
        }
        self.datos[(self.inicio + self.n) % CAP] = b;
        self.n += 1;
        Ok(())
    }

    fn sacar(&mut self) -> Option<u8> {
        if self.n == 0 {
            return None;
        }
        let b = self.datos[self.inicio];
        self.inicio = (self.inicio + 1) % CAP;
        self.n -= 1;
        Some(b)
    }

    fn len(&self) -> usize {
        self.n
    }
}
