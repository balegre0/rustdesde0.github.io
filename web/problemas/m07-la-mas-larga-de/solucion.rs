fn mas_larga_de(v: &[String]) -> Option<&str> {
    let mut mejor: Option<&str> = None;
    for s in v {
        match mejor {
            Some(m) if m.chars().count() >= s.chars().count() => {}
            _ => mejor = Some(s.as_str()),
        }
    }
    mejor
}
