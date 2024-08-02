use bindgen;
use num_cpus;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{exit, Command};
use std::{env, process};

pub fn exc_build_native(pkg_name: &str, pkg_names: &[&str]) -> PathBuf {
    exc_cmake(pkg_name, pkg_names);
    exc_make(pkg_name, pkg_names);
    exc_make_install(pkg_name);
    get_make_out_dir(pkg_name)
}

pub fn exc_cmake(pkg_name: &str, pkg_names: &[&str]) {
    let cmake_out_dir = get_cmake_out_dir(pkg_name);
    let mut command = Command::new("cmake");
    let mut _command = command.args([
        "-S",
        get_pkg_dir(pkg_name).to_str().unwrap(),
        "-B",
        &cmake_out_dir.to_str().unwrap(),
    ]);
    for pkg_name in pkg_names {
        let make_out_dir = get_make_out_dir(pkg_name);
        let cmake_out_dir = get_cmake_out_dir(pkg_name);
        _command = _command.args([
            format!(
                "-DCMAKE_MODULE_PATH={}/build/lib/cmake",
                cmake_out_dir.to_str().unwrap()
            )
            .as_str(),
            format!("-DCMAKE_INSTALL_PREFIX={}", make_out_dir.to_str().unwrap()).as_str(),
            format!(
                "-DCMAKE_PREFIX_PATH={}/include",
                make_out_dir.to_str().unwrap()
            )
            .as_str(),
        ]);
    }
    let output = _command.output().unwrap();
    show_output(&output);
}

pub fn exc_make(pkg_name: &str, pkg_names: &[&str]) {
    let cmake_out_dir = get_cmake_out_dir(pkg_name);
    let mut command = Command::new("make");
    let command = command
        .current_dir(&cmake_out_dir)
        .args([
            "-j",
            &num_cpus::get().to_string(),
            "V=1",
        ]);
    for pkg_name in pkg_names {
        let make_out_dir = get_make_out_dir(pkg_name);
        command.arg(format!("-I{}/include", make_out_dir.to_str().unwrap()).as_str());
    }
    println!("--------------------------------");
    println!("{:?}", command.get_args());
    println!("--------------------------------");
    let output = command.output().unwrap();
    show_output(&output);
}

pub fn exc_make_install(pkg_name: &str) {
    let cmake_out_dir = get_cmake_out_dir(pkg_name);
    let output = Command::new("make")
        .current_dir(&cmake_out_dir)
        .args([
            "V=1",
            "install",
        ])
        .output()
        .unwrap();
    show_output(&output);
}

pub fn exc_bindgen<I>(make_out_dirs: I)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        });
    for make_out_dir in make_out_dirs {
        builder = builder.clang_args([format!(
            "-I{}",
            format!("{}/include", make_out_dir.as_ref()),
        )]);
    }
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

fn get_pkg_dir(pkg_name: &str) -> PathBuf {
    PathBuf::from("../../crt/").join(pkg_name)
}
fn get_out_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}
fn get_cmake_out_dir(pkg_name: &str) -> PathBuf {
    get_out_dir().join(pkg_name)
}
fn get_make_out_dir(pkg_name: &str) -> PathBuf {
    get_cmake_out_dir(pkg_name).join("build")
}
