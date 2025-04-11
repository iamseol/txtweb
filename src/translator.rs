use crate::util::is_status_code;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

pub fn translate_page(
    folder_path: &PathBuf,
    components: &Vec<(String, String)>,
    previous_base: Option<&str>,
) {
    let mut final_base: String = String::new();

    if folder_path.join("base.txt").try_exists().unwrap() {
        translate_file(&mut final_base, &folder_path.join("base.txt"), components);
        fs::remove_file(folder_path.join("base.txt")).unwrap();
    } else {
        final_base = previous_base.unwrap().to_string();
    }

    let mut content: String = final_base.clone();

    let entries = folder_path.read_dir().unwrap();
    for current_entry in entries {
        let current_entry = current_entry.unwrap();

        if current_entry.path().is_dir() {
            translate_page(&current_entry.path(), components, Some(&final_base));
            continue;
        }

        let current_entry_name = current_entry
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let mut current_entry_content: String = String::new();
        translate_file(
            &mut current_entry_content,
            &current_entry.path(),
            components,
        );

        content = content.replace(
            &format!("<#{current_entry_name} />"),
            &current_entry_content,
        );

        fs::remove_file(current_entry.path()).unwrap();
    }

    let folder_name = folder_path.file_name().unwrap().to_str().unwrap();
    let mut new_html = File::create_new(if is_status_code(folder_name) {
        fs::remove_dir_all(&folder_path).unwrap();
        folder_path
            .parent()
            .unwrap()
            .join(&format!("{folder_name}.html"))
    } else {
        folder_path.join("index.html")
    })
    .unwrap();

    new_html.write_all(b"<!DOCTYPE html>").unwrap();
    new_html.write_all(content.as_bytes()).unwrap();
}

pub fn translate_file(buf: &mut String, file_path: &PathBuf, components: &Vec<(String, String)>) {
    let mut raw: String = String::new();

    File::open(file_path)
        .unwrap()
        .read_to_string(&mut raw)
        .unwrap();

    translate(buf, &raw, &components);
}

pub fn translate(buf: &mut String, from: &str, components: &Vec<(String, String)>) {
    let from = from.replace("\n", " ").replace("\t", " ");

    if !from.contains('>') {
        buf.push_str(&from);
    }

    let mut storage: String = String::new();
    let mut close_memory: Vec<String> = Vec::with_capacity(from.matches('<').count());

    from.chars().for_each(|current_char| match current_char {
        '>' => {
            let (tag, attr) = storage
                .trim()
                .split_once(' ')
                .map(|(a, b)| {
                    (
                        a.trim(),
                        b.trim()
                            .split('|')
                            .map(|c| {
                                c.trim()
                                    .split_once(' ')
                                    .map(|(a, b)| (a.trim(), b.trim()))
                                    .unwrap_or_else(|| (c.trim(), ""))
                            })
                            .collect(),
                    )
                })
                .unwrap_or_else(|| (storage.trim(), Vec::with_capacity(0)));

            if tag.starts_with('_') {
                let mut current_component =
                    components.iter().find(|c| c.0 == tag).unwrap().1.clone();

                for (current_attr_num, current_attr) in attr.iter().enumerate() {
                    let current_attr_num = current_attr_num + 1;
                    current_component =
                        current_component.replace(&format!("#{current_attr_num}"), current_attr.0)
                }

                buf.push_str(&current_component);
            } else {
                buf.push('<');
                buf.push_str(tag);

                for current_attr in attr {
                    buf.push(' ');
                    buf.push_str(current_attr.0);
                    buf.push_str("=\"");
                    buf.push_str(current_attr.1);
                    buf.push('"');
                }

                buf.push('>');
                close_memory.push(String::from(tag));
            }

            storage.clear();
        }
        '<' => {
            if storage.is_empty() {
                buf.pop();
                buf.push_str(" />");
            } else {
                translate(buf, &storage.trim(), components);

                buf.push_str("</");
                buf.push_str(close_memory.last().unwrap());
                buf.push('>');
            }

            close_memory.pop();
            storage.clear();
        }
        _ => storage.push(current_char),
    });
}
