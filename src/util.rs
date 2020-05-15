pub fn pos_to_location(src: &str, mut pos: usize) -> (usize, usize) {
    let mut line = 0;
    let mut column = 0;
    for c in src.chars() {
        if c == '\n' {
            line += 1;
            column = 0;
        } else {
            column += 1;
        }
        if pos == 0 {
            return (line, column);
        } else {
            pos -= 1;
        }
    }
    (line, column)
}

pub fn alert(msg: &str) -> String {
    format!("\u{001B}[31m{}\u{001B}[39m", msg)
}
