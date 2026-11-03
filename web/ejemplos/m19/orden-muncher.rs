macro_rules! pasos {
    ($($todo:tt)*) => { pasos!(@acc 0; $($todo)*) };
    (@acc $acc:expr;) => { $acc };
}
fn main() { let _ = pasos!(); }
