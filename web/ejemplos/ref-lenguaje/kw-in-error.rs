mod a {
    pub mod b {
        pub(in crate::a) fn secreto() -> u32 { 42 }
    }
}

fn main() {
    // `secreto` solo es visible dentro de `crate::a`.
    println!("{}", a::b::secreto());
}
