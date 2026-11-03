macro_rules! m {
    ($a:ident ! ! ) => { 2 };
    ($e:expr) => { 1 };
}
fn main() { let x = m!(a ! !); println!("{x}"); }
