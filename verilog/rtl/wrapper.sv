`include "verilog/include/defs.sv"

module wrapper;

    reg clk;

    wire[15:0]   imem_bus;
    wire[15:0]   imem_adr;

    wire[7:0]    dmem_wbus;
    wire[7:0]    dmem_rbus;
    wire[15:0]   dmem_adr;
    wire         dmem_signal;

    //dcache dc(
    //    
    //);

    icache ic(
        .imem_bus(imem_bus),
        .imem_adr(imem_adr)
    );

    microa ma(
        .clk(clk),

        .imem_bus(imem_bus),
        .imem_adr(imem_adr),

        .dmem_wbus(dmem_wbus),
        .dmem_rbus(dmem_rbus),
        .dmem_adr(dmem_adr)
    );

    int i;
    initial begin
        //$dumpfile("wave.vcd");
        //$dumpvars(0, clk, ma.pc);
        clk = 0;
        for (i = 0; i < 256; i = i + 1) begin
            $display("t=%-4d, pc: %h, state: %d, instr: %h, imem_adr: %h", $time, ma.pc, ma.state, ma.instr, ma.imem_adr);
            $display("regs: 0[%h] 1[%h] 2[%h] 3[%h] 4[%h] 5[%h] 6[%h] 7[%h] 8[%h] 9[%h] A[%h] B[%h] C[%h] D[%h] E[%h] F[%h]\n", 
                ma.regs[0], ma.regs[1], ma.regs[2], ma.regs[3], ma.regs[4], ma.regs[5], ma.regs[6], ma.regs[7], 
                ma.regs[8], ma.regs[9], ma.regs[10], ma.regs[11], ma.regs[12], ma.regs[13], ma.regs[14], ma.regs[15]);
            #2;
        end
        $finish();
    end

    always #1 clk = ~clk; 

endmodule
