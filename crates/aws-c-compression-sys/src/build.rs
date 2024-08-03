use build_util::build;

fn main() {
    let pkg_names = ["aws-c-common", "aws-c-compression"];
    let pkg_name = "aws-c-compression";
    build(&pkg_name, &pkg_names);
}
