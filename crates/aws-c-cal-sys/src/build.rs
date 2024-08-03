use build_util::build;

fn main() {
    let pkg_names = ["aws-c-common", "aws-c-cal"];
    let pkg_name = "aws-c-cal";
    build(&pkg_name, &pkg_names);
}
