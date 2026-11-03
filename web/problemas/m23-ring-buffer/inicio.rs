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
        todo!()
    }

    fn sacar(&mut self) -> Option<u8> {
        todo!()
    }

    fn len(&self) -> usize {
        self.n
    }
}
