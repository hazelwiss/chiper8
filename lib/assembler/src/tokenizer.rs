use crate::operands::{Operand, Register};

#[derive(Debug)]
pub enum TokenType {
    MOV,
    ADD,
    SUB,
    LSR,
    LSL,
    Operand(Operand),
}

#[derive(Debug)]
pub struct Token {
    pub tt: TokenType,
    pub line: usize,
}

fn parse_literal(literal: &str, index: usize) -> u8 {
    let literal = match literal.chars().last() {
        Some('h') => u8::from_str_radix(&literal[0..literal.len() - 1], 16),
        Some('d') => u8::from_str_radix(&literal[0..literal.len() - 1], 10),
        Some(c) if c.is_numeric() => literal.parse::<u8>(),
        _ => panic!("Invalid literal at line {index}"),
    }
    .expect(&format!("Unable to parse literal at line {index}"));
    if literal > 0xF {
        panic!("literal too large to fit 4 bits")
    } else {
        literal
    }
}

fn tokenize_line(line: &str, index: usize, tokens: &mut Vec<Token>) {
    let mut iter = line.split(&[' ', ',']).filter(|e| *e != "");
    while let Some(string) = iter.next() {
        tokens.push(Token {
            tt: match string {
                "mov" => TokenType::MOV,
                "add" => TokenType::ADD,
                "sub" => TokenType::SUB,
                "lsr" => TokenType::LSR,
                "lsl" => TokenType::LSL,
                "r0" => TokenType::Operand(Operand::Register(Register::R0)),
                "r1" => TokenType::Operand(Operand::Register(Register::R1)),
                "r2" => TokenType::Operand(Operand::Register(Register::R2)),
                "r3" => TokenType::Operand(Operand::Register(Register::R3)),
                "r4" => TokenType::Operand(Operand::Register(Register::R4)),
                "r5" => TokenType::Operand(Operand::Register(Register::R5)),
                "r6" => TokenType::Operand(Operand::Register(Register::R6)),
                "r7" => TokenType::Operand(Operand::Register(Register::R7)),
                "r8" => TokenType::Operand(Operand::Register(Register::R8)),
                "r9" => TokenType::Operand(Operand::Register(Register::R9)),
                "r10" => TokenType::Operand(Operand::Register(Register::R10)),
                "r11" => TokenType::Operand(Operand::Register(Register::R11)),
                "r12" => TokenType::Operand(Operand::Register(Register::R12)),
                "r13" => TokenType::Operand(Operand::Register(Register::R13)),
                "r14" => TokenType::Operand(Operand::Register(Register::R14)),
                "r15" => TokenType::Operand(Operand::Register(Register::R15)),
                string if string[0..1].contains('$') => TokenType::Operand(Operand::Imm({
                    let literal = parse_literal(&string[1..], index);
                    literal
                })),
                _ => panic!("Unable to tokenize '{string}' on line {index}"),
            },
            line: index,
        });
    }
}

pub fn tokenize_string(contents: &str) -> Vec<Token> {
    let mut tokens = vec![];
    for (index, line) in contents.lines().enumerate() {
        tokenize_line(line, index + 1, &mut tokens);
    }
    tokens
}
