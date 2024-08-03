use bindgen;
use num_cpus;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{exit, Command};
use std::{env, process};

pub fn exc_build_native(pkg_name: &str) {
    exc_cmake(pkg_name);
    exc_make(pkg_name);
    exc_make_install(pkg_name);
}

pub fn exc_cmake(pkg_name: &str) {
    let make_out_dir = get_make_out_dir();
    let cmake_out_dir = get_cmake_out_dir(pkg_name);

    let mut command = Command::new("cmake");
    let command = command.current_dir(&cmake_out_dir).args([
        format!(
            "-DCMAKE_MODULE_PATH={}",
            get_cmake_module_path().to_str().unwrap()
        )
        .as_str(),
        format!("-DCMAKE_INSTALL_PREFIX={}", make_out_dir.to_str().unwrap()).as_str(),
        format!(
            "-DCMAKE_PREFIX_PATH={}",
            make_out_dir.join("include").to_str().unwrap()
        )
        .as_str(),
        // format!(
        //     "-DCMAKE_C_FLAGS={}",
        //     make_out_dir.join("include").to_str().unwrap()
        // )
        // .as_str(),
        "-S",
        get_pkg_dir(pkg_name).to_str().unwrap(),
        "-B",
        &cmake_out_dir.to_str().unwrap(),
    ]);

    let output = command.output().unwrap();
    show_output(&output);
}

pub fn exc_make(pkg_name: &str) {
    // let make_out_dir = get_make_out_dir();
    let cmake_out_dir = get_cmake_out_dir(pkg_name);

    let mut command = Command::new("make");
    let command = command.current_dir(&cmake_out_dir).args([
        "-j",
        &num_cpus::get().to_string(),
        "V=1",
        // "-I",
        // make_out_dir.join("include").to_str().unwrap(),
    ]);

    let output = command.output().unwrap();
    show_output(&output);
}

pub fn exc_make_install(pkg_name: &str) {
    // let make_out_dir = get_make_out_dir();
    let cmake_out_dir = get_cmake_out_dir(pkg_name);
    let output = Command::new("make")
        .current_dir(&cmake_out_dir)
        .args([
            "V=1",
            // "-I",
            // make_out_dir.join("include").to_str().unwrap(),
            "install",
        ])
        .output()
        .unwrap();
    show_output(&output);
}

pub fn exc_bindgen() {
    let make_out_dir = get_make_out_dir();
    let builder = bindgen::Builder::default()
        .header("wrapper.h")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .clang_args([format!(
            "-I{}",
            make_out_dir.join("include").to_str().unwrap(),
        )]);
    let bindings = builder
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .unwrap();
    bindings.write_to_file("src/bindings.rs").unwrap();
}

fn show_output(output: &process::Output) {
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    if output.stderr.len() > 0 {
        if !output.status.success() {
            exit(output.status.code().unwrap());
        }
    }
}

fn ensure_dir_exists(path: PathBuf) -> PathBuf {
    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }
    path
}
fn get_pkg_dir(pkg_name: &str) -> PathBuf {
    ensure_dir_exists(
        PathBuf::from(env::current_dir().unwrap())
            .join("../../crt/")
            .join(pkg_name),
    )
}
fn get_out_dir() -> PathBuf {
    ensure_dir_exists(PathBuf::from(env::var("OUT_DIR").unwrap()))
}
fn get_cmake_out_dir(pkg_name: &str) -> PathBuf {
    ensure_dir_exists(get_out_dir().join(pkg_name))
}
fn get_cmake_module_path() -> PathBuf {
    ensure_dir_exists(get_out_dir().join("build/lib/cmake"))
}
pub fn get_make_out_dir() -> PathBuf {
    ensure_dir_exists(get_out_dir().join("build"))
}
