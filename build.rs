extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/sketchybar.c")
        .compile("libsketchybar.a");
}
