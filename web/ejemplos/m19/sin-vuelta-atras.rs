macro_rules! m {
    ($e:expr) => { 1 };
    ($a:ident ! ! ) => { 2 };
}
fn main() { let x = m!(a ! !); println!("{x}"); }
