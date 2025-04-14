use std::{fs, io::Write, path::PathBuf};

pub fn copy_dir_all(src: PathBuf, des: PathBuf) {
    fs::create_dir_all(&des).unwrap();

    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let entry_type = entry.file_type().unwrap();

        if entry_type.is_dir() {
            copy_dir_all(entry.path(), des.join(entry.file_name()));
        } else {
            fs::copy(entry.path(), des.join(entry.file_name())).unwrap();
        }
    }
}

pub fn is_status_code(name: &str) -> bool {
    name.chars().all(|c| c.is_numeric())
}

pub fn get_user_input(buf: &mut String, greeting: &str) {
    print!("{greeting}");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(buf).unwrap();
    *buf = buf.trim().to_string();
}
