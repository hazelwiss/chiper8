use std::process::Command;

fn main() {
    Command::new("target/verilog_out/chiper8")
        .status()
        .expect("");
}
