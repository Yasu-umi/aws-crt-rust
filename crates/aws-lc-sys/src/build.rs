use build_util::build;

fn main() {
    let pkg_names = ["aws-lc"];
    let pkg_name = "aws-lc";
    build(&pkg_name, &pkg_names);
}
