fn a_utf8(c: char) -> Vec<u8> {
    let v = c as u32;
    let cont = |desp: u32| (0x80 | ((v >> desp) & 0x3F)) as u8;
    if v < 0x80 {
        vec![v as u8]
    } else if v < 0x800 {
        vec![(0xC0 | (v >> 6)) as u8, cont(0)]
    } else if v < 0x10000 {
        vec![(0xE0 | (v >> 12)) as u8, cont(6), cont(0)]
    } else {
        vec![(0xF0 | (v >> 18)) as u8, cont(12), cont(6), cont(0)]
    }
}
