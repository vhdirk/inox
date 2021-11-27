use capnpc;
use std::process::Command;

fn main() {
    // Rerun the build script when files in the resources folder are changed.
    println!("cargo:rerun-if-changed=resources");
    println!("cargo:rerun-if-changed=resources/*");

    Command::new("sassc")
        .args(&[
            "-t",
            "compressed",
            "html/thread_view.scss",
            "html/thread_view.css",
        ])
        .current_dir("resources")
        .status()
        .unwrap();

    Command::new("glib-compile-resources")
        .args(&["--generate", "resources.xml"])
        .current_dir("resources")
        .status()
        .unwrap();

    // capnpc::CompilerCommand::new()
    //     .file("resources/webext.capnp")
    //     .run()
    //     .expect("compiling schema");
}
