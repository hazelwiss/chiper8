`include "verilog/include/defs.sv"

`define STATE_FETCH     4'b0000
`define STATE_DECODE    4'b0001
`define STATE_EXECUTE   4'b0010
`define STATE_WRITEBACK 4'b0011

`define ALU_OPC_MOV 3'b000
`define ALU_OPC_ADD 3'b001
`define ALU_OPC_SUB 3'b010
`define ALU_OPC_SHR 3'b011
`define ALU_OPC_SHL 3'b100

`define OPC_ALU     4'b0000
`define OPC_LOAD    4'b0001
`define OPC_STORE   4'b0010
`define OPC_IMM     4'b0011

`define DST_REG  2'b00
`define DST_PC   2'b01

`define SRC_REG  2'b00
`define SRC_IMM4 2'b01
`define SRC_DEP  2'b10

`define CTRL(ALU_OP, D, S, VALID, STATE) \
    {alu_opc,       ctrl_dest,  ctrl_src,   valid_instr,    ctrl_nxt_state} = \
    {ALU_OP,        D,          S,          VALID,          STATE}

module microa (
    /* The basic clock signal. */
    input clk,

    /* Instruction memory bus. */
    input[15:0] imem_bus,
    /* Instruction memory address. */
    output reg[15:0] imem_adr,

    /* Data memory write bus. */
    output reg[7:0] dmem_wbus,
    /* Data memory read bus. */
    input[7:0] dmem_rbus,
    /* Data memory address. */
    output reg[15:0] dmem_adr,
    /* Data memory signal. */
    output reg dmem_signal

);
    /* Register file. */
    reg[15:0] pc = 0;
    reg[7:0] regs[0:15];
    
    /* Instruction state. */
    reg[15:0] instr;
    wire[3:0] opcode    = instr[15:12];
    wire[3:0] dest_adr  = instr[11:8];
    wire[3:0] src_adr   = instr[7:4];
    wire[3:0] imm4      = instr[7:4];
    wire      imm_bit   = instr[3];

    /* State machine. */
    reg[3:0] state;
    reg reset;

    /* Control logic */
    reg[1:0] ctrl_src;
    reg[1:0] ctrl_dest;
    reg[3:0] ctrl_nxt_state;
    reg[2:0] alu_opc;
    reg      valid_instr;

    /* ALU state. */
    wire[7:0] alu_a = 
        ctrl_dest == `DST_REG ? regs[dest_adr] : 
        ctrl_dest == `DST_PC ? pc[7:0] : 0;
    wire[7:0] alu_b = 
        ctrl_src == `SRC_REG    ? regs[src_adr] : 
        ctrl_src == `SRC_IMM4   ? {4'd0, imm4} : 
        imm_bit == 1'b1 ? {4'd0, imm4} : regs[src_adr];    
    reg[7:0] alu_out;   

    initial begin
        reset = `TRUE;
    end

    always @* begin 
        imem_adr = {pc[15:1], 1'b0};
        case(opcode)
            `OPC_ALU:       `CTRL(instr[2:0],   `DST_REG,   `SRC_DEP,   `TRUE,  `STATE_WRITEBACK);
            `OPC_LOAD:      `CTRL(`ALU_OPC_MOV, `DST_REG,   `SRC_REG,   `TRUE,  `STATE_WRITEBACK);
            `OPC_STORE:     `CTRL(`ALU_OPC_MOV, `DST_REG,   `SRC_REG,   `TRUE,  `STATE_FETCH);
            default:        `CTRL(`ALU_OPC_ADD, `DST_REG,   `SRC_REG,   `FALSE, `STATE_FETCH);
        endcase
    end

    always @(posedge clk) begin 
        if (reset) begin 
            state <= `STATE_FETCH;
            pc <= 16'd0;
            instr <= 16'd0;
            reset <= `FALSE;
        end
        else begin 
            case (state)
                `STATE_FETCH: begin
                    pc <= pc + 2;
                    instr <= imem_bus;
                    state <= `STATE_DECODE;
                end            
                `STATE_DECODE: begin
                    state <= `STATE_EXECUTE;
                end
                `STATE_EXECUTE: begin 
                    case (alu_opc)
                        `ALU_OPC_MOV: alu_out <= alu_b;
                        `ALU_OPC_ADD: alu_out <= alu_a + alu_b;
                        `ALU_OPC_SUB: alu_out <= alu_a - alu_b;
                        `ALU_OPC_SHR: alu_out <= alu_a >> alu_b;
                        `ALU_OPC_SHL: alu_out <= alu_a << alu_b;
                        default: alu_out <= 8'd0;
                    endcase
                    state <= ctrl_nxt_state;
                end
                `STATE_WRITEBACK: begin 
                    regs[dest_adr] <= alu_out;
                    state <= `STATE_FETCH;
                end
                default;
            endcase
        end
    end

endmodule
