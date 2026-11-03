trait IteradorQuePresta {
    type Item<'a>;

    fn siguiente(&mut self) -> Option<Self::Item<'_>>;
}

fn main() {}
