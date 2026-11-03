// Un crate que promete no contener unsafe.
#![forbid(unsafe_code)]

fn main() {
    let x = 5;
    let p = &raw const x;
    let y = unsafe { *p };
    println!("{y}");
}
