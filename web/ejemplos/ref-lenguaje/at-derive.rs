use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Version {
    mayor: u32,
    menor: u32,
}

fn main() {
    let a = Version { mayor: 1, menor: 4 };
    let b = Version { mayor: 1, menor: 10 };
    let c = a; // Copy
    println!("{a:?} {:?}", Version::default()); // Debug, Default
    println!("{} {}", a == c, a < b); // PartialEq, PartialOrd (campo a campo, en orden)
    let mut set = HashSet::new(); // Hash + Eq
    set.insert(a);
    set.insert(c);
    println!("{} distinta(s); máximo {:?}", set.len(), a.max(b)); // Ord
}
