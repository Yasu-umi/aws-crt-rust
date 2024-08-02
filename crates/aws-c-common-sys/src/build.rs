use build_util::{exc_bindgen, exc_build_native};

fn main() {
    let pkg_names = ["aws-c-common"];
    let pkg_name = "aws-c-common";

    let mut make_out_dirs = vec![];
    for pkg_name in pkg_names.iter() {
        make_out_dirs.push(exc_build_native(pkg_name, &pkg_names));
    }

    exc_bindgen(make_out_dirs.iter().map(|path| path.to_str().unwrap()));

    println!("cargo:rustc-link-lib={}", pkg_name);
    for make_out_dir in &make_out_dirs {
        println!("cargo:rustc-link-search={}", make_out_dir.to_str().unwrap(),);
    }
    println!("cargo::rerun-if-changed=../../crt/{}", pkg_name);
}
