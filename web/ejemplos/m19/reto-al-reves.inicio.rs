// al_reves![1, 2, 3] debe dar el array [3, 2, 1].
// Pista: una regla interna con dos listas: (@ [pendientes] [ya invertidos]).
macro_rules! al_reves {
    ($($x:expr),*) => { [$($x),*] };
}

fn main() {
    println!("{:?}", al_reves![1, 2, 3]);
    println!("{:?}", al_reves!['a', 'b']);
}
