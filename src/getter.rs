use crate::parse_page;
use std::path::PathBuf;
use txtlib::*;

pub fn get_public(
    root_dir: &PathBuf,
    original_name: String,
    public: &mut Vec<(String, String)>,
) -> EmptyTxtResult {
    for current_entry in get_entries(root_dir)? {
        let current_path = current_entry.path();
        let current_entry_name = get_whole_file_name(&current_path)?;
        let current_name = original_name.clone() + "/" + current_entry_name;

        if current_path.is_dir() {
            get_public(&current_path, current_name, public)?;
            continue;
        }

        let mut file_content = String::new();
        if let Ok(_) = read_from_file(&mut file_content, &mut open_file(&current_path)?) {
            let file_content_words: Vec<&str> = file_content.split_whitespace().collect();

            public.push((current_name, file_content_words.join(" ")));
        }
    }

    Ok(())
}

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
    public: &Vec<(String, String)>,
) -> EmptyTxtResult {
    for current_entry in get_entries(from)? {
        let current_path = current_entry.path();
        let current_entry_name = get_whole_file_name(&current_path)?;
        let start = &from.join(current_entry_name);

        if current_path.is_dir() {
            let des = &to.join(current_entry_name);
            create_new_folder(des)?;
            get_pages(start, des, components, public)?;

            continue;
        }

        parse_page(start, &to, current_entry_name, components, public)?;
    }

    Ok(())
}
