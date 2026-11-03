mod a {
    pub mod b {
        pub(super) fn solo_para_a() {}
    }
}

fn main() {
    // `pub(super)` en `a::b` significa «visible en `a`», no en la raíz.
    a::b::solo_para_a();
}
