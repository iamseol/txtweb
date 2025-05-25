use crate::parse_page;
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

    let mut file_content = String::new();
    read_file(&mut file_content, &from.join("index.txt"))?;

    let (content, page_components): (&str, Vec<(String, String)>) =
        if let Some((content, str_page_components)) = file_content.split_once("\n\n@") {
            let mut page_components: Vec<(String, String)> = Vec::new();

            str_page_components
                .split("\n\n@")
                .for_each(|current_page_component| {
                    page_components.push(
                        current_page_component
                            .split_once("\n")
                            .map(|(a, b)| (a.to_string(), b.to_string()))
                            .unwrap(),
                    );
                });

            (content, page_components)
        } else {
            (&file_content, Vec::new())
        };

    let mut result = String::new();
    let _ =
        parse_page(&mut result, from, content, components, &page_components).map_err(|e| e.fire());

    write_new_file(&to.join("index.html"), &result)?;

    Ok(())
}
