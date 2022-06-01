use std::fmt::{format, Display};

use crate::{
    operands::{Operand, Register},
    tokenizer::{Token, TokenType},
};

enum ALUOperation {
    MOV,
    ADD,
    SUB,
    LSL,
    LSR,
}

impl ALUOperation {
    fn from_token(token: &Token) -> Self {
        match token.tt {
            TokenType::MOV => Self::MOV,
            TokenType::ADD => Self::ADD,
            TokenType::SUB => Self::SUB,
            TokenType::LSL => Self::LSL,
            TokenType::LSR => Self::LSR,
            _ => panic!(
                "Invalid token for converting to ALU operation on line {}.",
                token.line
            ),
        }
    }

    fn to_number(&self) -> usize {
        match self {
            ALUOperation::MOV => 0b000,
            ALUOperation::ADD => 0b001,
            ALUOperation::SUB => 0b010,
            ALUOperation::LSL => 0b011,
            ALUOperation::LSR => 0b100,
        }
    }
}

impl Display for ALUOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::MOV => "mov",
            Self::ADD => "add",
            Self::SUB => "sub",
            Self::LSL => "lsl",
            Self::LSR => "lsr",
        })
    }
}

enum Instr {
    ALU {
        dst: Register,
        src: Operand,
        alu_opc: ALUOperation,
    },
    LOAD {
        dst: Register,
        adr_reg: Register,
        offset: u8,
    },
    STORE {
        dst: Register,
        adr_reg: Register,
        offset: u8,
    },
}

impl Instr {
    fn to_hex_string(&self) -> String {
        match self {
            Self::ALU { dst, src, alu_opc } => {
                let alu_bit = match src {
                    Operand::Register(_) => 0,
                    _ => 1,
                };
                let dst = dst.to_number();
                let src = src.to_number();
                format!("0{dst:1X}{src:1X}{}", alu_opc.to_number() | (alu_bit << 3))
            }
            Self::LOAD {
                dst,
                adr_reg,
                offset,
            } => {
                let dst = dst.to_number();
                let adr_reg = adr_reg.to_number();
                format!("1{dst}{adr_reg}{offset}")
            }
            Self::STORE {
                dst,
                adr_reg,
                offset,
            } => {
                let dst = dst.to_number();
                let adr_reg = adr_reg.to_number();
                format!("2{dst}{adr_reg}{offset}")
            }
        }
    }
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::ALU { dst, src, alu_opc } => format!("{alu_opc} {dst}, {src}"),
            Self::LOAD {
                dst,
                adr_reg,
                offset,
            } => format!("load {dst}, {adr_reg}, {offset}"),
            Self::STORE {
                dst,
                adr_reg,
                offset,
            } => format!("store {dst}, {adr_reg}, {offset}"),
        })
    }
}

pub fn assemble(_tokenized_input: &[Token]) -> Vec<u8> {
    todo!()
}

pub fn assemble_for_verilog(tokenized_input: &[Token]) -> String {
    let mut instructions = vec![];
    let mut iter = tokenized_input.iter();
    while let Some(token) = iter.next() {
        match token.tt {
            TokenType::ADD | TokenType::SUB | TokenType::MOV | TokenType::LSR | TokenType::LSL => {
                // Type 1
                let dst = match iter.next() {
                    Some(Token {
                        tt: TokenType::Operand(Operand::Register(reg)),
                        ..
                    }) => *reg,
                    _ => panic!(
                        "Invalid destination operand '{:?}' at line {}",
                        token.tt, token.line
                    ),
                };
                let src = match iter.next() {
                    Some(Token {
                        tt: TokenType::Operand(operand),
                        ..
                    }) => *operand,
                    _ => panic!(
                        "Invalid source operand '{:?}' at line {}",
                        token.tt, token.line
                    ),
                };
                instructions.push(Instr::ALU {
                    dst,
                    src,
                    alu_opc: ALUOperation::from_token(token),
                })
            }
            _ => panic!("Invalid first token in sequence at line {}", token.line),
        }
    }
    let mut string = String::new();
    for instr in instructions {
        string.push_str(&format!("{} // {instr}\n", instr.to_hex_string()));
    }
    string
}
