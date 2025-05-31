use std::path::PathBuf;

use txtlib::*;

pub fn parse_page(
    from: &PathBuf,
    to: &PathBuf,
    name: &str,
    components: &Vec<(String, String)>,
    public: &Vec<(String, String)>,
) -> EmptyTxtResult {
    let mut file_content = String::new();
    read_file(&mut file_content, &from)?;

    let mut buf = String::new();
    parse_page_content(&mut buf, &file_content, components, public)?;

    write_new_file(&to.join(name).with_extension("html"), &buf)?;

    Ok(())
}

fn parse_page_content(
    buf: &mut String,
    content: &str,
    components: &Vec<(String, String)>,
    public: &Vec<(String, String)>,
) -> EmptyTxtResult {
    let (content, page_components): (&str, Vec<(String, String)>) =
        if let Some((content, str_page_components)) = content.split_once("\n\n@") {
            let mut page_components: Vec<(String, String)> = Vec::new();

            str_page_components
                .split("\n\n@")
                .for_each(|current_page_component| {
                    page_components.push({
                        current_page_component
                            .split_once("\n")
                            .map(|(a, b)| (a.to_string(), b.to_string()))
                            .unwrap_or((current_page_component.to_string(), String::new()))
                    });
                });

            (content, page_components)
        } else {
            (&content, Vec::new())
        };

    let mut tag_stack: Vec<String> = Vec::new();
    let mut temp_storage: Vec<String> = Vec::new();

    for current_word in content.split_whitespace() {
        match current_word {
            ">" => {
                let tag = parse_tag(
                    buf,
                    &mut temp_storage,
                    components,
                    &page_components,
                    public,
                    ">",
                )?;
                tag_stack.push(tag);
            }
            "\\" => {
                parse_tag(
                    buf,
                    &mut temp_storage,
                    components,
                    &page_components,
                    public,
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

fn parse_tag(
    buf: &mut String,
    temp_storage: &mut Vec<String>,
    components: &Vec<(String, String)>,
    page_components: &Vec<(String, String)>,
    public: &Vec<(String, String)>,
    close_with: &str,
) -> TxtResult<String> {
    let tag = &temp_storage.remove(0);

    if tag.starts_with('_') {
        parse_component(buf, tag, temp_storage, components, page_components, public)?;
    } else if tag.starts_with('!') {
        parse_embedded(buf, temp_storage, public)?;
    } else {
        parse_normal_tag(buf, tag, temp_storage, close_with);
    }

    Ok(tag.to_string())
}

fn parse_component(
    buf: &mut String,
    tag: &str,
    temp_storage: &mut Vec<String>,
    components: &Vec<(String, String)>,
    page_components: &Vec<(String, String)>,
    public: &Vec<(String, String)>,
) -> EmptyTxtResult {
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
    for current_word in &mut *temp_storage {
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

                let mut content = String::new();
                parse_page_content(&mut content, &found_page_component.1, components, public)?;

                final_value = final_value.replace(&parameter_name, &content);
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

    parse_page_content(buf, &final_value, components, public)?;
    temp_storage.clear();

    Ok(())
}

fn parse_embedded(
    buf: &mut String,
    temp_storage: &mut Vec<String>,
    public: &Vec<(String, String)>,
) -> EmptyTxtResult {
    let url = temp_storage.get(0).ok_or(TxtError::Custom(String::from(
        "!embed should have one single parameter for the url to the file.",
    )))?;

    let content = public
        .iter()
        .find(|c| *url == c.0)
        .ok_or(TxtError::Custom(format!("the url `{url}` is not found.")))?;

    buf.push_str(&content.1);
    temp_storage.clear();

    Ok(())
}

fn parse_normal_tag(buf: &mut String, tag: &str, temp_storage: &mut Vec<String>, close_with: &str) {
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

    buf.push_str(close_with);
    temp_storage.clear();
}
