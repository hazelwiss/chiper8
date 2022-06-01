#[macro_export]
macro_rules! VERILOG_DIR {
    ($dir:literal) => {
        concat!("verilog/", $dir)
    };
}

#[macro_export]
macro_rules! VERILOG_OUT {
    ($dir:literal) => {
        concat!("target/verilog_out/", $dir)
    };
}
