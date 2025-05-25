use std::path::PathBuf;
use txtlib::*;

pub fn parse_page(
    buf: &mut String,
    from: &PathBuf,
    content: &str,
    components: &Vec<(String, String)>,
    page_components: &Vec<(String, String)>,
) -> EmptyTxtResult {
    let content = content.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut tag_stack: Vec<String> = Vec::new();
    let mut temp_storage: Vec<String> = Vec::new();

    for current_word in content.split_whitespace() {
        match current_word {
            ">" => {
                let tag = parse_tag(
                    buf,
                    from,
                    &mut temp_storage,
                    components,
                    page_components,
                    ">",
                )?;
                tag_stack.push(tag);
            }
            "\\" => {
                parse_tag(
                    buf,
                    from,
                    &mut temp_storage,
                    components,
                    page_components,
                    " />",
                )?;
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

pub fn parse_tag(
    buf: &mut String,
    from: &PathBuf,
    temp_storage: &mut Vec<String>,
    components: &Vec<(String, String)>,
    page_components: &Vec<(String, String)>,
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
                    let page_component_name = value.trim().strip_prefix('@').unwrap();

                    let found_page_component = page_components
                        .iter()
                        .find(|c| c.0 == page_component_name)
                        .ok_or(TxtError::Custom(format!(
                            "the page component {page_component_name} is not found."
                        )))?;

                    final_value = final_value.replace(&parameter_name, &found_page_component.1);
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

        parse_page(buf, from, &final_value, components, page_components)?;
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
