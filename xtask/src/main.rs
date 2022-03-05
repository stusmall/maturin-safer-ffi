use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process::Command;

macro_rules! check_command {
    ($cmd:tt, $msg:tt) => {
        let output = $cmd.output().unwrap();
        if !output.status.success() {
            let stderr = std::str::from_utf8(&output.stderr).unwrap();
            eprint!($msg, format!("\n\n{}", stderr));
            eprint!("Try again with command: {:?}", $cmd);
            std::process::exit(1);
        }
    };
}

fn main() {
    build_rust();
    build_python();
}

fn build_rust() {
    //TODO: Make colors auto detected so CI output doesn't look like shit
    println!("Building rust packages");
    let mut build_cmd = Command::new("cargo");
    build_cmd.arg("build").arg("--color=always");
    build_cmd.arg("--release");
    check_command!(build_cmd, "Failed to build rust project: {}");

    println!("Generating headers");
    let mut gen_header_cmd = Command::new("cargo");
    gen_header_cmd.args(&[
        "--color=always",
        "test",
        "--features",
        "c-headers",
        "--",
        "generate_headers",
    ]);
    check_command!(gen_header_cmd, "Failed to build headers: {}");
}



fn build_python() {
    let mut manifest = project_root();
    manifest.push("pythonsdk/Cargo.toml");


    create_dir_all("./pythonsdk/target/").expect("Failed to set up destination for python header file");
    println!("Prepping the header for cffi");
    let mut preprocessor_cmd = Command::new("gcc");
    preprocessor_cmd.args(&[
        "-E",
        &project_root()
            .join("target/centralcrate_no_includes.h")
            .to_string_lossy(),
        "-o",
        &project_root()
            .join("pythonsdk/target/header.h")
            .to_string_lossy(),
    ]);
    check_command!(preprocessor_cmd, "Running C preprocessor failed:  {}");

    println!("Building python wheels");
    let mut maturin_cmd = Command::new("maturin");
    maturin_cmd.args(&["build", "--release", "-m", manifest.to_str().unwrap()]);
    check_command!(maturin_cmd, "Python build failed:  {}");
}

pub fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}