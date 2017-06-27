use std::fs;
use std::process;
use std::io::Write;
use std::path::Path;

pub fn setup_plugins() -> Vec<String>{

    let plugin_files = fs::read_dir("plugins").unwrap();
    let plugin_target = Path::new("plugin_bin");
    let mut plugin_lib_paths : Vec<String> = Vec::new();

    if !plugin_target.exists() {
        fs::create_dir(plugin_target).unwrap();
    }

    for plugin_file in plugin_files {
        let file = plugin_file.unwrap();
        setup_plugin(&file, &plugin_target, &mut plugin_lib_paths);
    }

    plugin_lib_paths
}

fn setup_plugin(
    file : &fs::DirEntry,
    plugin_target : &Path,
    plugin_lib_paths : &mut Vec<String>) {

    let os_file_name = file.file_name();
    let file_name = os_file_name.to_str().unwrap();
    if file_name.ends_with(".rs") {
        let plugin_name = String::from(&file_name[..file_name.len() - 3]);
        println!("Loading {} from {}", plugin_name, file.path().display());

        // create dir
        let plugin_dir = plugin_target.to_str().unwrap().to_owned() + "/" + &plugin_name + "/";
        let plugin_path = Path::new(&plugin_dir);
        let src_dir = plugin_dir.clone() + "/src";

        // TODO: would it be better to just use rustc?
        // TODO: have a way to update the cargo file
        // TODO: include custom dependency specification
        if !plugin_path.exists() {
            create_plugin_crate(&plugin_path, &plugin_dir, &plugin_name);

            fs::create_dir(&src_dir).unwrap();
        }

        update_plugin_file(file, &src_dir);

        run_cargo(plugin_path, &plugin_dir, &plugin_name, plugin_lib_paths);
    }
}

fn update_plugin_file(
    file : &fs::DirEntry,
    src_dir : &String) {
    // TODO: don't update if the file wasn't changed.
    fs::copy(file.path(), src_dir.to_owned() + "/lib.rs").unwrap();
}

fn run_cargo(
    plugin_path : &Path,
    plugin_dir : &String,
    plugin_name : &String,
    plugin_lib_paths : &mut Vec<String>) {

    let output = process::Command::new("cargo")
        .current_dir(plugin_path)
        .args(&["build"])
        .output()
        .expect("failed to execute process");
    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        println!("{}", String::from_utf8_lossy(&output.stderr));
        panic!("Could not compile {}", plugin_name);
    } else {
        let lib_ext = if cfg!(target_os = "windows") { ".dll" } else { ".so" };
        let lib_path = plugin_dir.clone() + "target/debug/" + &plugin_name + lib_ext;
        plugin_lib_paths.push(lib_path);
    }
}

fn create_plugin_crate(plugin_path : &Path, plugin_dir : &String, plugin_name : &String) {
    fs::create_dir(plugin_path).unwrap();
    // write cargo file
    let cargo_path = plugin_dir.clone() + "Cargo.toml";
    let mut cargo_file = fs::File::create(cargo_path).unwrap();
    write!(cargo_file, "
        [package]
        name = \"{}\"
        version = \"0.1.0\"
        authors = [\"Richard Warburton <richard.warburton@gmail.com>\"]

        [lib]
        crate-type = [\"dylib\"]

        [dependencies]
        irc = {{ git = \"https://github.com/RichardWarburton/irc.git\" }}
        regex = \"0.2\"

        [dependencies.modules]
        path = \"../../libs/modules\"
        ", plugin_name).unwrap();
    cargo_file.flush().unwrap();
}
