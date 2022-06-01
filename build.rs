#![feature(exit_status_error)]

use paths::{VERILOG_DIR, VERILOG_OUT};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    let verilog_out = VERILOG_OUT!("");
    fs::create_dir_all(verilog_out).expect(&format!(
        "unable to create verilog output folder {verilog_out}"
    ));
    let out_asm = VERILOG_OUT!("test.chiper8");
    assembler::assemble_for_verilog(vec![Path::new("asm/test")], Path::new(out_asm));
    compile_verilog();
    println!("cargo:rerun-if-changed={}", verilog_out);
    println!("cargo:rerun-if-changed={}", out_asm);
}

fn compile_verilog() {
    let verilog_dir = VERILOG_DIR!("");
    assert!(
        fs::read_dir(verilog_dir).is_ok(),
        "Unable to locate directory '{verilog_dir}'"
    );
    let verilog_dir_contents = walkdir::WalkDir::new(VERILOG_DIR!("rtl"));
    let verilog_sources: Vec<PathBuf> = verilog_dir_contents
        .into_iter()
        .map(|e| {
            let err = format!("Unable to read file {e:?}");
            e.expect(&err)
        })
        .filter(|e| match e.path().extension() {
            Some(os_str) => match os_str.to_str() {
                Some("v") => true,
                Some("sv") => true,
                _ => false,
            },
            _ => false,
        })
        .map(|e| e.path().to_path_buf())
        .collect();
    let output_file = VERILOG_OUT!("chiper8");
    let mut cmd = Command::new("iverilog");
    cmd.arg("-g2005-sv")
        .args(verilog_sources)
        .arg(concat!("-I", VERILOG_DIR!("include")))
        .arg("-o")
        .arg(output_file);
    cmd.status()
        .expect("Error during iverilog compliation. Aborting...")
        .exit_ok()
        .expect(&format!("'{cmd:?}'\nexited with error. Aborting..."));
    println!("cargo:rerun-if-changed={}", verilog_dir);
}
