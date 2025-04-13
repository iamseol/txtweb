use std::{env, fs, io::Write, path::PathBuf};
use txtweb::{components::setup_components, translator::translate_page, util::copy_dir_all};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.first().unwrap().as_ref() {
        "new" => new(&args.get(1).unwrap().as_ref()),
        "build" => build(),
        _ => help(),
    }
}

fn new(project_name: &str) {
    let root_dir: PathBuf = env::current_dir().unwrap().join(project_name);

    fs::create_dir(&root_dir).unwrap();
    fs::create_dir(&root_dir.join("components")).unwrap();
    fs::create_dir(&root_dir.join("contents")).unwrap();
    fs::create_dir(&root_dir.join("public")).unwrap();
    fs::create_dir(&root_dir.join("css")).unwrap();
    fs::create_dir(&root_dir.join("js")).unwrap();

    let mut base_file = fs::File::create_new(&root_dir.join("contents").join("base.txt")).unwrap();
    base_file.write_all(format!("html lang en >\n  head >\n    title > {project_name} <\n  <\n  body >\n    main > h1 > under constructoin... < <\n  <\n<").as_bytes()).unwrap();
}

fn build() {
    let root_dir: PathBuf = env::current_dir().unwrap();
    let mut components: Vec<(String, String)> = Vec::new();

    fs::remove_dir_all(root_dir.join("dist")).unwrap_or(());
    fs::create_dir(root_dir.join("dist")).unwrap();

    copy_dir_all(&root_dir.join("contents"), &root_dir.join("dist"));
    setup_components(&root_dir, &mut components);
    translate_page(&root_dir.join("dist"), &components);

    copy_dir_all(&root_dir.join("public"), &root_dir.join("dist/public"));
    copy_dir_all(&root_dir.join("css"), &root_dir.join("dist/css"));
    copy_dir_all(&root_dir.join("js"), &root_dir.join("dist/js"));
}

fn help() {
    println!("you can get help from `https://github.com/iamseol/txtweb`");
}
