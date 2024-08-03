use build_util::build;

fn main() {
    let pkg_names = ["aws-c-common", "aws-c-cal", "aws-c-io"];
    let pkg_name = "aws-c-io";
    build(&pkg_name, &pkg_names);
}
