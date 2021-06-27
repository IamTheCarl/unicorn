<<<<<<< HEAD
// This stuff was disabled because it makes it hard for me to include the right dependencies when cross compiling.
=======
// This has all been commented out because it makes linking impossible when using a local build.
>>>>>>> 05a577b4ec56e644fba60807d74c8a48385463a1

// use std::{env, process::Command};

// use build_helper::rustc::{link_lib, link_search};

fn main() {
<<<<<<< HEAD
    // println!("cargo:rerun-if-changed=unicorn");
    // let out_dir = env::var("OUT_DIR").unwrap();
    // let unicorn = "libunicorn.a";
    // let _ = Command::new("cp")
    //     .current_dir("../..")
    //     .arg(&unicorn)
    //     .arg(&out_dir)
    //     .status()
    //     .unwrap();
    // link_search(
    //     Some(build_helper::SearchKind::Native),
    //     build_helper::out_dir(),
    // );
    // link_lib(Some(build_helper::LibKind::Static), "unicorn");
=======
    //     println!("cargo:rerun-if-changed=unicorn");
    //     let out_dir = env::var("OUT_DIR").unwrap();
    //     let unicorn = "libunicorn.a";
    //     let _ = Command::new("cp")
    //         .current_dir("../..")
    //         .arg(&unicorn)
    //         .arg(&out_dir)
    //         .status()
    //         .unwrap();
    //     link_search(
    //         Some(build_helper::SearchKind::Native),
    //         build_helper::out_dir(),
    //     );
    //     link_lib(Some(build_helper::LibKind::Static), "unicorn");
>>>>>>> 05a577b4ec56e644fba60807d74c8a48385463a1
}
