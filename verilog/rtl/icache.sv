`include "verilog/include/defs.sv"

module icache(
    output[15:0] imem_bus,
    input[15:0] imem_adr
);

    reg[15:0] memory['h10000]; 

    initial begin
        $readmemh("target/verilog_out/test.chiper8", memory);
    end

    assign imem_bus = memory[{1'd0, imem_adr[15:1]}];

endmodule