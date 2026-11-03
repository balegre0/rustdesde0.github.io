fn dividir<'a>(s: &'a str, sep: char) -> (&'a str, &'a str) {
    match s.find(sep) {
        Some(i) => (&s[..i], &s[i + sep.len_utf8()..]),
        None => (s, ""),
    }
}
