use std::{io::Write, path::PathBuf};
use txtlib::*;
use txtweb::{get_components, get_pages, get_public};

fn main() {
    let mut txtweb = TxtLib::new(
        "txtweb",
        "a text file-based webpage generator",
        "https://github.com/iamseol/txtweb",
    );

    txtweb.add_command("new", "for initialization", new);
    txtweb.add_command("build", "for building", build);

    txtweb.start();
}

fn new() -> EmptyTxtResult {
    let project_name = if let Ok(project_name) = get_user_arg(2) {
        project_name
    } else {
        let mut project_name: String = String::new();
        print!("project name >");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut project_name).unwrap();
        project_name.trim().to_string()
    };

    let root_dir: PathBuf = get_cwd()?.join(&project_name);

    create_new_folder(&root_dir)?;

    create_new_folder(&root_dir.join("components"))?;
    create_new_folder(&root_dir.join("pages"))?;
    create_new_folder(&root_dir.join("public"))?;
    create_new_folder(&root_dir.join("public/css"))?;
    create_new_folder(&root_dir.join("public/js"))?;

    write_new_file(
        &root_dir.join("pages").join("index.txt"),
        &format!(
            "html lang en >\n  head >\n    title > {project_name} <\n  <\n  body >\n    main > h1 > under constructoin... < <\n  <\n<"
        ),
    )?;

    write_new_file(&root_dir.join("README.md"), &format!("# {project_name}"))?;

    Ok(())
}

fn build() -> EmptyTxtResult {
    let root_dir: PathBuf = get_cwd()?;
    let mut public: Vec<(String, String)> = Vec::new();
    let mut components: Vec<(String, String)> = Vec::new();

    create_clean_folder(&root_dir.join("dist"))?;
    copy_all_folder(&root_dir.join("public"), &root_dir.join("dist/public"))?;
    get_public(&root_dir.join("public"), "/public".to_string(), &mut public)?;
    get_components(&root_dir.join("components"), String::new(), &mut components)?;
    get_pages(
        &root_dir.join("pages"),
        &root_dir.join("dist"),
        &components,
        &public,
    )?;

    Ok(())
}
