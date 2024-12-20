extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/cfun.c")
        .compile("libcfun.a");
}