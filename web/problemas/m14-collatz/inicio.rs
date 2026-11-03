struct Collatz {
    siguiente: Option<u64>,
}

fn collatz(n: u64) -> Collatz {
    Collatz { siguiente: Some(n) }
}

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        todo!()
    }
}
