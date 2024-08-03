use build_util::{exc_bindgen, exc_build_native, get_make_out_dir};

fn main() {
    let pkg_names = ["aws-c-common", "aws-c-cal"];
    let pkg_name = "aws-c-cal";

    for pkg_name in pkg_names {
        exc_build_native(pkg_name);
    }

    exc_bindgen();

    println!("cargo:rustc-link-lib={}", pkg_name);
    println!(
        "cargo:rustc-link-search={}",
        get_make_out_dir().to_str().unwrap(),
    );
    println!("cargo::rerun-if-changed=../../crt/{}", pkg_name);
}
