pub fn capitalise(s: &str) -> String {
    let mut first = s.chars().nth(0).unwrap().to_uppercase().to_string();
    first.push_str(&s[1..]);
    first
}

pub fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[22m", s)
}

pub fn italic(s: &str) -> String {
    format!("\x1b[3m{}\x1b[23m", s)
}

pub fn underline(s: &str) -> String {
    format!("\x1b[4m{}\x1b[24m", s)
}
