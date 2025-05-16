use std::path::PathBuf;
use txtlib::*;

pub fn get_components(
    root_dir: &PathBuf,
    original_name: String,
    components: &mut Vec<(String, String)>,
) -> EmptyTxtResult {
    for current_entry in get_entries(root_dir)? {
        let current_path = current_entry.path();
        let current_entry_name = get_file_stem(&current_path)?;

        if !current_entry_name.starts_with('_') {
            return Err(TxtError::Custom(format!(
                "names for components should start with `_`. the component at {} does not.",
                current_path.display()
            )));
        }

        if current_path.is_dir() {
            get_components(
                &current_path,
                original_name.clone() + current_entry_name,
                components,
            )?;

            continue;
        }

        let mut content = String::new();
        read_file(&mut content, &current_path)?;
        components.push((
            original_name.clone() + current_entry_name,
            content.split_whitespace().collect::<Vec<_>>().join(" "),
        ));
    }

    Ok(())
}

pub fn get_pages(
    from: &PathBuf,
    to: &PathBuf,
    components: &Vec<(String, String)>,
) -> EmptyTxtResult {
    for current_entry in get_entries(from)? {
        let current_path = current_entry.path();

        if current_path.is_file() {
            continue;
        }

        let current_entry_name = get_whole_file_name(&current_path)?;
        let des = to.join(current_entry_name);

        create_new_folder(&des)?;
        get_pages(&from.join(current_entry_name), &des, components)?;
        continue;
    }

    let mut result = String::new();
    let mut content = String::new();
    read_file(&mut content, &from.join("index.txt"))?;
    let _ = parse_page(&mut result, from, &content, components).map_err(|e| e.fire());

    write_new_file(&to.join("index.html"), &result)?;

    Ok(())
}

fn parse_page(
    buf: &mut String,
    from: &PathBuf,
    content: &str,
    components: &Vec<(String, String)>,
) -> EmptyTxtResult {
    let content = content.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut tag_stack: Vec<String> = Vec::new();
    let mut temp_storage: Vec<String> = Vec::new();

    for current_word in content.split_whitespace() {
        match current_word {
            ">" => {
                let tag = parse_tag(buf, from, &mut temp_storage, components, ">")?;
                tag_stack.push(tag);
            }
            "\\" => {
                parse_tag(buf, from, &mut temp_storage, components, " />")?;
            }
            "<" => {
                let last_tag = tag_stack
                    .pop()
                    .ok_or(TxtError::Custom(String::from("no closing tag exists.")))?;

                buf.push_str(&temp_storage.join(" "));
                buf.push_str("</");
                buf.push_str(&last_tag);
                buf.push('>');

                temp_storage.clear();
            }

            _ => {
                if current_word.len() != 1 && current_word.starts_with('|') {
                    buf.push_str(&temp_storage.join(" "));
                    temp_storage.clear();
                    temp_storage.push(current_word.strip_prefix('|').unwrap().to_string());
                } else {
                    temp_storage.push(current_word.to_string());
                }
            }
        };
    }

    Ok(())
}

fn parse_tag(
    buf: &mut String,
    from: &PathBuf,
    temp_storage: &mut Vec<String>,
    components: &Vec<(String, String)>,
    close_with: &str,
) -> TxtResult<String> {
    let tag = temp_storage.remove(0).to_string();

    if tag.starts_with('_') {
        let current_component = &components
            .iter()
            .find(|current_component| current_component.0 == tag)
            .ok_or(TxtError::Custom(format!("component not found: {tag}")))?
            .1;

        let mut final_value = current_component.to_string();

        let mut parameter_name = String::from("#");
        let mut value = String::new();
        let mut waiting_value = false;

        temp_storage.push(String::from("|"));
        for current_word in temp_storage {
            if !waiting_value {
                parameter_name.push_str(current_word);
                waiting_value = true;

                continue;
            }

            if current_word == "|" {
                if value.trim().starts_with('@') {
                    let mut result = String::new();
                    let mut content = String::new();
                    read_file(
                        &mut content,
                        &from
                            .join(value.trim().strip_prefix('@').unwrap())
                            .with_extension("txt"),
                    )?;

                    let _ =
                        parse_page(&mut result, from, &content, components).map_err(|e| e.fire());

                    final_value = final_value.replace(&parameter_name, &result)
                } else {
                    final_value = final_value.replace(&parameter_name, value.trim())
                }

                parameter_name.clear();
                parameter_name.push('#');
                value.clear();
                waiting_value = false;

                continue;
            }

            value.push(' ');
            value.push_str(&current_word);
        }

        parse_page(buf, from, &final_value, components)?;
    } else {
        buf.push('<');
        buf.push_str(&tag);

        let mut value = String::new();
        let mut waiting_value = false;
        for current_value in &mut *temp_storage {
            if !waiting_value {
                buf.push(' ');
                buf.push_str(current_value);
                buf.push_str("=\"");

                waiting_value = true;

                continue;
            }

            if *current_value == "|" {
                buf.push_str(value.trim_start());
                buf.push('\"');

                waiting_value = false;
                value.clear();

                continue;
            }

            value.push(' ');
            value.push_str(current_value);
        }

        if !value.is_empty() {
            buf.push_str(value.trim_start());
            buf.push('\"');
        }

        temp_storage.clear();
        buf.push_str(close_with);
    }

    Ok(tag)
}
