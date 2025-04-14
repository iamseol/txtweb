use crate::translator::translate_file;
use std::{fs, path::PathBuf};

pub fn setup_components(root_dir: &PathBuf, components: &mut Vec<(String, String)>) {
    for current_entry in fs::read_dir(root_dir.join("components")).unwrap() {
        let current_entry = current_entry.unwrap();
        let current_entry_path = current_entry.path();

        let mut buf: String = String::new();
        translate_file(&mut buf, &current_entry_path, &Vec::with_capacity(0));

        components.push((
            current_entry_path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            buf,
        ));
    }
}
