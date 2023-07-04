pub fn format_semi_engineering(value: f32) -> String {
    static SUFFIXES: [char; 5] = [' ', 'k', 'M', 'B', 'T'];
    let suffix = (0..)
        .map(|x| 1000f32.powi(x) as f32)
        .zip(SUFFIXES)
        .filter(|x| value > x.0)
        .last()
        .unwrap();

    // explain with an example: value = 15942
    let normalized = value / suffix.0; // normalized = 15.942
    let exp = suffix.1; // exp = 'k'

    let mut ret = format!("{}", normalized as u16);
    if normalized < 10. {
        ret += &format!(".{}", ((normalized % 1.) * 10.) as u8);
    }

    ret.push(exp);
    ret += " views";

    ret
}
