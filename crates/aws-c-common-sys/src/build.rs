use build_util::build;

fn main() {
    let pkg_names = ["aws-c-common"];
    let pkg_name = "aws-c-common";
    build(&pkg_name, &pkg_names);
}
