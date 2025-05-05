use std::{env, io::Write, path::PathBuf};
use txtlib::*;
use txtweb::{get_user_input, setup_components, translate_page};

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
    let mut project_name: String = String::new();
    get_user_input(&mut project_name, "project name > ");

    let root_dir: PathBuf = get_cwd()?;

    create_new_folder(&root_dir)?;

    create_new_folder(&root_dir.join("components"))?;
    create_new_folder(&root_dir.join("contents"))?;
    create_new_folder(&root_dir.join("public"))?;
    create_new_folder(&root_dir.join("public/css"))?;
    create_new_folder(&root_dir.join("public/js"))?;

    let mut index_file = create_new_file(&root_dir.join("contents").join("index.txt"))?;
    let mut readme = create_new_file(&root_dir.join("README.md"))?;

    index_file.write_all(format!("html lang en >\n  head >\n    title > {project_name} <\n  <\n  body >\n    main > h1 > under constructoin... < <\n  <\n<").as_bytes()).unwrap();
    readme
        .write_all(format!("# {project_name}").as_bytes())
        .unwrap();

    Ok(())
}

fn build() -> EmptyTxtResult {
    let root_dir: PathBuf = env::current_dir().unwrap();
    let mut components: Vec<(String, String)> = Vec::new();

    create_clean_folder(&root_dir.join("dist"))?;
    copy_all_folder(&root_dir.join("contents"), &root_dir.join("dist"))?;

    setup_components(&root_dir, &mut components);
    translate_page(&root_dir.join("dist"), &components);

    copy_all_folder(&root_dir.join("public"), &root_dir.join("dist/public"))?;

    Ok(())
}
