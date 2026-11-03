fn primera_pareja(objetivo: u32) -> (u32, u32) {
    'buscar: {
        for a in 1..=9 {
            for b in a..=9 {
                if a * b == objetivo {
                    break 'buscar (a, b);
                }
            }
        }
        (0, 0)
    }
}
