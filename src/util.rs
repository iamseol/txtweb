use std::io::Write;

pub fn get_user_input(buf: &mut String, greeting: &str) {
    print!("{greeting}");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(buf).unwrap();
    *buf = buf.trim().to_string();
}
