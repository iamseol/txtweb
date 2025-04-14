use std::{env, fs, io::Write, path::PathBuf};
use txtweb::{
    components::setup_components,
    translator::translate_page,
    util::{copy_dir_all, get_user_input},
};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let first = args.first();

    if let Some(first) = first {
        match first.as_ref() {
            "new" => new(),
            "build" => build(),
            _ => help(),
        }
    } else {
        help();
    }
}

fn new() {
    let mut project_name: String = String::new();
    get_user_input(&mut project_name, "project name > ");

    let root_dir: PathBuf = env::current_dir().unwrap().join(&project_name);

    fs::create_dir(&root_dir).unwrap();
    fs::create_dir(root_dir.join("components")).unwrap();
    fs::create_dir(root_dir.join("contents")).unwrap();
    fs::create_dir(root_dir.join("public")).unwrap();
    fs::create_dir(root_dir.join("public/css")).unwrap();
    fs::create_dir(root_dir.join("public/js")).unwrap();

    let mut index_file = fs::File::create_new(root_dir.join("contents").join("index.txt")).unwrap();
    let mut readme = fs::File::create_new(root_dir.join("README.md")).unwrap();

    index_file.write_all(format!("html lang en >\n  head >\n    title > {project_name} <\n  <\n  body >\n    main > h1 > under constructoin... < <\n  <\n<").as_bytes()).unwrap();
    readme
        .write_all(format!("# {project_name}").as_bytes())
        .unwrap();
}

fn build() {
    let root_dir: PathBuf = env::current_dir().unwrap();
    let mut components: Vec<(String, String)> = Vec::new();

    fs::remove_dir_all(root_dir.join("dist")).unwrap_or(());
    fs::create_dir(root_dir.join("dist")).unwrap();

    copy_dir_all(root_dir.join("contents"), root_dir.join("dist"));
    setup_components(&root_dir, &mut components);
    translate_page(root_dir.join("dist"), &components);

    copy_dir_all(root_dir.join("public"), root_dir.join("dist/public"));
}

fn help() {
    println!("you can read documentation from `https://github.com/iamseol/txtweb`");
}
