extern crate cc;

fn main() {
    cc::Build::new().file("lz4.c").compile("lz4");
}
