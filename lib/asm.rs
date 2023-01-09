/* asm.rs
 * By: John Jekel
 * Copyright (C) 2023 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Human-readable text -> DecodedInstruction
 *
*/

/* Imports */

use crate::{DecodedInstruction, DecodedOpcode};

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

pub fn assemble_raw(instruction_string: &str) -> Option<DecodedInstruction> {//Does not support labels
    let preprocessed_instruction_string = instruction_string.trim().to_string().to_uppercase();
    let mut tokens = Vec::<&str>::with_capacity(5);//Max number of tokens we expect
    for token in preprocessed_instruction_string.split(&[' ', '\t', '\n', '\r', ',', '[', ']']) {//TODO ensure the syntax is correct instead of just stripping out these symbols when splitting
        if !token.is_empty() {
            tokens.push(token);
        }
    }
    if tokens.len() < 2 {//Basic sanity check; every instruction has at least 1 operand
        return None;
    }
    eprintln!("{:?}", tokens);//TESTING

    //Determine the instruction type
    let mut decoded_instruction;
    match tokens[0] {
        "ADD" | "SUB" | "ADDS" | "SUBS" | "AND" | "ORR" | "EOR" | "LSL" | "LSR" | "BR" => {
            decoded_instruction = DecodedInstruction::R{
                opcode: DecodedOpcode::Invalid,
                rm: 0xFF,
                shamt: 0xFF,
                rn: 0xFF,
                rd: 0xFF
            };
        },
        "ADDI" | "SUBI" | "ADDIS" | "SUBIS" | "ANDI" | "ORRI" | "EORI" => {
            decoded_instruction = DecodedInstruction::I{
                opcode: DecodedOpcode::Invalid,
                imm12: 0xFFFF,
                rn: 0xFF,
                rd: 0xFF
            };
        },
        "LDUR" | "STUR" | "LDURSW" | "STURW" | "LDURH" | "STURH" | "LDURB" | "STURB" | "LDXR" | "STXR" => {
            decoded_instruction = DecodedInstruction::D{
                opcode: DecodedOpcode::Invalid,
                addr9: 0xFFFF,
                op2: 0xFF,
                rn: 0xFF,
                rt: 0xFF
            };
        },
        "MOVZ" | "MOVK" => {
            decoded_instruction = DecodedInstruction::IW{
                opcode: DecodedOpcode::Invalid,
                lsl: 0xFF,
                imm16: 0xFFFF,
                rd: 0xFF
            };
        },
        "B" | "BL" => {
            decoded_instruction = DecodedInstruction::B{
                opcode: DecodedOpcode::Invalid,
                addr26: 0xFFFFFFFF
            };
        },
        "CBZ" | "CBNZ"  => {
            decoded_instruction = DecodedInstruction::CB {
                opcode: DecodedOpcode::Invalid,
                addr19: 0xFFFFFFFF,
                rt: 0xFF
            };
        },
        token => {
            if token.starts_with("B.") {//B.cond
                decoded_instruction = DecodedInstruction::CB {
                    opcode: DecodedOpcode::Invalid,
                    addr19: 0xFFFFFFFF,
                    rt: 0xFF
                };
            } else {//Invalid
                return None;
            }
        }
    }
    //Ensure, based on the instruction type, that there are the correct number of operands
    match decoded_instruction {
        DecodedInstruction::R{..} | DecodedInstruction::I{..} | DecodedInstruction::D{..} => {
            //For R, this is the instruction followed by three registers (except for BR, which only has 1 register after it)
            //For I, this is the instruction followed by two registers and an immediate
            //For D, this is the instruction followed by a register, another register, and an offset
            if tokens.len() != 4 {
                if !((tokens[0] == "BR") && (tokens.len() == 2)) {
                    return None;
                }
            }
        },
        DecodedInstruction::IW{..} => { if tokens.len() != 5 { return None; }},//Instruction, register, immediate, "LSL", and LSL amount
        DecodedInstruction::B{..} => { if tokens.len() != 2 { return None; }},//Instruction and offset
        DecodedInstruction::CB{..} => {
            //Instruction, register and offset, unless the is B.cond, in which case there is only an offset
            if (tokens.len() != 3) || (tokens[0].starts_with("B.") && (tokens.len() != 2)) {
                if !(tokens[0].starts_with("B.") && (tokens.len() == 2)) {
                    return None;
                }
            }
        },
    }

    //At this point, we can assume the instruction type, as well as the number of tokens is valid
    //However, the tokens themselves, as well as the condition for B.cond, will still need checking

    //Begin to construct the instruction
    /*eprintln!("TESTING: {:?}", decoded_instruction);//TESTING
    match &mut decoded_instruction {//This works!
        DecodedInstruction::R{opcode, ..} => {
            *opcode = DecodedOpcode::ADD;
        }
        _ => {panic!("testing");}
    }
    */

    //Determine the opcode
    match &mut decoded_instruction {
        DecodedInstruction::R{opcode, ..} | DecodedInstruction::D{opcode, ..} => {
            //Opcode is 11 bits
            match tokens[0] {
                "ADD" =>    { *opcode = DecodedOpcode::ADD; },
                "SUB" =>    { *opcode = DecodedOpcode::SUB; },
                "ADDS" =>   { *opcode = DecodedOpcode::ADDS; },
                "SUBS" =>   { *opcode = DecodedOpcode::SUBS; },
                "AND" =>    { *opcode = DecodedOpcode::AND; },
                "ORR" =>    { *opcode = DecodedOpcode::ORR; },
                "EOR" =>    { *opcode = DecodedOpcode::EOR; },
                "LSL" =>    { *opcode = DecodedOpcode::LSL; },
                "LSR" =>    { *opcode = DecodedOpcode::LSR; },
                "BR" =>     { *opcode = DecodedOpcode::BR; },
                "LDUR" =>   { *opcode = DecodedOpcode::LDUR; }
                "STUR" =>   { *opcode = DecodedOpcode::STUR; }
                "LDURSW" => { *opcode = DecodedOpcode::LDURSW; }
                "STURW" =>  { *opcode = DecodedOpcode::STURW; }
                "LDURH" =>  { *opcode = DecodedOpcode::LDURH; }
                "STURH" =>  { *opcode = DecodedOpcode::STURH; }
                "LDURB" =>  { *opcode = DecodedOpcode::LDURB; }
                "STURB" =>  { *opcode = DecodedOpcode::STURB; }
                "LDXR" =>   { *opcode = DecodedOpcode::LDXR; }
                "STXR" =>   { *opcode = DecodedOpcode::STXR; }
                _ => { panic!("This should never occur"); }
            }
        },
        DecodedInstruction::I{opcode, ..} => {
            //Opcode is 10 bits
            match tokens[0] {
                "ADDI" =>  { *opcode = DecodedOpcode::ADDI; },
                "SUBI" =>  { *opcode = DecodedOpcode::SUBI; },
                "ADDIS" => { *opcode = DecodedOpcode::ADDIS; },
                "SUBIS" => { *opcode = DecodedOpcode::SUBIS; },
                "ANDI" =>  { *opcode = DecodedOpcode::ANDI; },
                "ORRI" =>  { *opcode = DecodedOpcode::ORRI; },
                "EORI" =>  { *opcode = DecodedOpcode::EORI; },
                _ => { panic!("This should never occur"); }
            }
        },
        DecodedInstruction::IW{opcode, ..} => {
            //Opcode is 9 bits
            match tokens[0] {
                "MOVZ" =>  { *opcode = DecodedOpcode::MOVZ; },
                "MOVK" =>  { *opcode = DecodedOpcode::MOVK; },
                _ => { panic!("This should never occur"); }
            }
        },
        DecodedInstruction::CB{opcode, ..} => {
            //Opcode is 8 bits
            match tokens[0] {
                "CBZ" =>  { *opcode = DecodedOpcode::CBZ; },
                "CBNZ" => { *opcode = DecodedOpcode::CBNZ; },
                token => {
                    if token.starts_with("B.") {//B.cond
                        *opcode = DecodedOpcode::B_cond;
                    } else {
                        panic!("This should never occur");
                    }
                }
            }
        },
        DecodedInstruction::B{opcode, ..} => {
            //Opcode is 6 bits
            match tokens[0] {
                "B" => { *opcode = DecodedOpcode::B; },
                "BL" => { *opcode = DecodedOpcode::BL; },
                _ => { panic!("This should never occur"); }
            }
        }
    }

    //Determine register fields

    //Determine Rd/Rt/neither depending on the instruction type
    match &mut decoded_instruction {
        DecodedInstruction::R{rd: rd_rt, ..} | DecodedInstruction::I{rd: rd_rt, ..} | DecodedInstruction::D{rt: rd_rt, ..} | DecodedInstruction::CB{rt: rd_rt, ..} | DecodedInstruction::IW{rd: rd_rt, ..} => {
            if let Some(parsed_rd_rt) = parse_register(tokens[1]) {
                *rd_rt = parsed_rd_rt;
            } else {
                return None;
            }
        }
        DecodedInstruction::B{..} => {}//It does not have this field
    }

/*
    //Determine Rn depending on the instruction type
    match instruction_type {
        DecodedInstruction::R | DecodedInstruction::I | DecodedInstruction::D => {
            if tokens[0] != "BR" {//This one does not have the Rn operand
                if let Some(rn) = parse_register(tokens[2]) {
                    instruction.set_bits(rn, 9, 5);
                } else {
                    return None;
                }
            }
        }
        DecodedInstruction::CB | DecodedInstruction::IM | DecodedInstruction::B => {}//They do not have this field
    }

    //Determine Rm if the instruction type is R
    if matches!(instruction_type, DecodedInstruction::R) {
        if tokens[0] != "BR" {//This one does not have the Rn operand
            if let Some(rm) = parse_register(tokens[3]) {
                instruction.set_bits(rm, 20, 16);
            } else {
                return None;
            }
        }
    }

    //Determine immediate fields

    //Determine the immediate/address to use from the 4th token depending on the instruction_type
    match instruction_type {
        DecodedInstruction::I => {
            if let Some(immediate) = smart_parse_immediate(tokens[3]) {
                //The field is 12 bits, so immediate must be between -2048 and +2047
                if (immediate < -2048) || (immediate > 2047) {//Too large for the field to hold
                    return None;
                }
                let masked_immediate = (immediate as u32) & 0b111111111111;
                instruction.set_bits(masked_immediate as u32, 21, 10);
            } else {
                return None;
            }
        },
        DecodedInstruction::D => {
            if let Some(address) = smart_parse_immediate(tokens[3]) {
                //The field is 9 bits, so address must be between -512 and +511
                if (address < -512) || (address > 511) {//Too large for the field to hold
                    return None;
                }
                let masked_address = (address as u32) & 0b111111111;
                instruction.set_bits(masked_address, 20, 12);
            } else {
                return None;
            }
        },
        DecodedInstruction::R | DecodedInstruction::CB | DecodedInstruction::IM | DecodedInstruction::B => {}//They do not have this field
    }

    //Determine the branch address to use from the second token if the instruction type is B
    if matches!(instruction_type, DecodedInstruction::B) {
        if let Some(address) = smart_parse_immediate(tokens[1]) {
            //The field is 26 bits, so address must be between -33554432 and 33554431
            if (address < -33554432) || (address > 33554431) {//Too large for the field to hold
                return None;
            }
            let masked_address = (address as u32) & 0b11111111111111111111111111;
            instruction.set_bits(masked_address, 25, 0);
        } else {
            return None;
        }
    }

    //Determine the branch address/immediate to use from the third token if the instruction type is CB or IM respectively
    match instruction_type {
        DecodedInstruction::CB => {
            if let Some(address) = smart_parse_immediate(tokens[2]) {
                //The field is 19 bits, so address must be between -262144 and 262143
                if (address < -262144) || (address > 262143) {//Too large for the field to hold
                    return None;
                }
                let masked_address = (address as u32) & 0b1111111111111111111;
                instruction.set_bits(masked_address, 23, 5);
            } else {
                return None;
            }
        },
        DecodedInstruction::IM => {
            if let Some(immediate) = smart_parse_immediate(tokens[2]) {
                //The field is 16 bits, so immediate must be between -32768 and 32767
                if (immediate < -32768) || (immediate > 32767) {//Too large for the field to hold
                    return None;
                }
                let masked_immediate = (immediate as u32) & 0b1111111111111111;
                instruction.set_bits(masked_immediate, 20, 5);
            } else {
                return None;
            }
        },
        DecodedInstruction::R | DecodedInstruction::I | DecodedInstruction::D | DecodedInstruction::B => {}//They do not have this operand
    }

    //Deal with the shift amount for IM-type instructions
    if matches!(instruction_type, DecodedInstruction::IM) {
        //Ensure the user put lsl before the shift amount
        if tokens[3] != "LSL" {
            return None;
        }

        //There are only 4 valid shift amounts that can come after
        match smart_parse_immediate(tokens[4]) {
            Some(0) =>  { instruction.set_bits(0b00, 22, 21); },
            Some(16) => { instruction.set_bits(0b01, 22, 21); },
            Some(32) => { instruction.set_bits(0b10, 22, 21); },
            Some(48) => { instruction.set_bits(0b11, 22, 21); },
            _ => { return None; }
        }
    }

    //NOTE: For LEGv2, shamt for the R-type instructions, as well as op2 for D-type instructions, are always all zeroes

    return Some((instruction_type, instruction));
    */

    return Some(decoded_instruction);
}

fn parse_register(register_token: &str) -> Option<u8> {
    if let Some(relevant_part_of_token) = register_token.strip_prefix("X") {
        if let Some(parsed_register_number) = relevant_part_of_token.parse::<u8>().ok() {
            if parsed_register_number < 32 {
                return Some(parsed_register_number);
            } else {
                return None;
            }
        } else if relevant_part_of_token == "ZR" {//The zero register
            return Some(31);
        } else {
            return None;
        }
    } else {//Register did not start with X
        return None;
    }
}

fn smart_parse_immediate(uint_string: &str) -> Option<i32> {
    if let Some(binary_uint_string) = uint_string.strip_prefix("0b") {
        return i32::from_str_radix(binary_uint_string, 2).ok();
    } else if let Some(binary_uint_string) = uint_string.strip_prefix("0B") {
        return i32::from_str_radix(binary_uint_string, 2).ok();
    } else if let Some(hex_uint_string) = uint_string.strip_prefix("0x") {
        return i32::from_str_radix(hex_uint_string, 16).ok();
    } else if let Some(hex_uint_string) = uint_string.strip_prefix("0X") {
        return i32::from_str_radix(hex_uint_string, 16).ok();
    } else if let Some(oct_uint_string) = uint_string.strip_prefix("0o") {
        return i32::from_str_radix(oct_uint_string, 8).ok();
    } else if let Some(oct_uint_string) = uint_string.strip_prefix("0O") {
        return i32::from_str_radix(oct_uint_string, 8).ok();
    } else if let Some(dec_uint) = uint_string.parse::<i32>().ok() {
        return Some(dec_uint);
    } else {
        return None;
    }
}
