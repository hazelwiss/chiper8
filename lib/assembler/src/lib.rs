mod assembly;
mod operands;
mod tokenizer;

use std::{fs, path::Path};

pub fn assemble(files: Vec<&Path>, _out: &Path) {
    if files.len() != 1 {
        panic!("can only assemble 1 file for now.")
    }
    let file = files.get(0).unwrap();
    let contents = fs::read_to_string(file).expect(&format!("Unable to read file {file:?}"));
    let tokenized = tokenizer::tokenize_string(&contents);
    let _ = assembly::assemble(&tokenized);
}

pub fn assemble_for_verilog(files: Vec<&Path>, out: &Path) {
    if files.len() != 1 {
        panic!("can only assemble 1 file for now.")
    }
    let file = files.get(0).unwrap();
    let contents = fs::read_to_string(file).expect(&format!("Unable to read file {file:?}"));
    let tokenized = tokenizer::tokenize_string(&contents);
    let output = assembly::assemble_for_verilog(&tokenized);
    fs::write(out, output).expect(&format!("error writing to file '{out:?}'"));
}

#[test]
fn test() {
    let test_str = "
        mov r1, $9
        mov r2, $9h
        mov r0, $15
        mov r0, r1
        mov r1, r0 
        add r1, r0
        mov r2, r1    
    ";
    tokenizer::tokenize_string(test_str);
}
