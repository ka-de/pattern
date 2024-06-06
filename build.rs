// Used to embed resources like files or binaries into the executable.
extern crate embed_resource;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        // Sets the game icon in the executable for Windows.
        embed_resource::compile("build/windows/icon.rc");
    }
}
