struct Collatz {
    siguiente: Option<u64>,
}

fn collatz(n: u64) -> Collatz {
    Collatz { siguiente: Some(n) }
}

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let actual = self.siguiente?;
        self.siguiente = if actual == 1 {
            None
        } else if actual % 2 == 0 {
            Some(actual / 2)
        } else {
            Some(3 * actual + 1)
        };
        Some(actual)
    }
}
