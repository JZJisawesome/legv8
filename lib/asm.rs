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
    eprintln!("TESTING: {:?}", decoded_instruction);//TESTING
    match &mut decoded_instruction {//This works!
        DecodedInstruction::R{opcode, ..} => {
            *opcode = DecodedOpcode::ADD;
        }
        _ => {panic!("testing");}
    }

    //Determine the opcode
    /*match instruction_type {
        DecodedInstruction::R{} | DecodedInstruction::D => {
            //Opcode is 11 bits
            let opcode;
            match tokens[0] {
                "ADD" =>    { opcode = 0b10001011000; },
                "SUB" =>    { opcode = 0b11001011000; },
                "ADDS" =>   { opcode = 0b10101011000; },
                "SUBS" =>   { opcode = 0b11101011000; },
                "AND" =>    { opcode = 0b10001010000; },
                "ORR" =>    { opcode = 0b10101010000; },
                "EOR" =>    { opcode = 0b11001010000; },
                "LSL" =>    { opcode = 0b11010011011; },
                "LSR" =>    { opcode = 0b11010011010; },
                "BR" =>     { opcode = 0b11010110000; },
                "LDUR" =>   { opcode = 0b11111000010; }
                "STUR" =>   { opcode = 0b11111000000; }
                "LDURSW" => { opcode = 0b10111000100; }
                "STURW" =>  { opcode = 0b10111000000; }
                "LDURH" =>  { opcode = 0b01111000010; }
                "STURH" =>  { opcode = 0b01111000000; }
                "LDURB" =>  { opcode = 0b00111000010; }
                "STURB" =>  { opcode = 0b00111000000; }
                "LDXR" =>   { opcode = 0b11001000010; }
                "STXR" =>   { opcode = 0b11001000000; }
                _ => { panic!("This should never occur"); }
            }
            debug_assert!(opcode <= 0b11111111111);
            instruction.set_bits(opcode, 31, 21);
        },
        DecodedInstruction::I => {
            //Opcode is 10 bits
            let opcode;
            match tokens[0] {
                "ADDI" =>  { opcode = 0b1001000100; },
                "SUBI" =>  { opcode = 0b1101000100; },
                "ADDIS" => { opcode = 0b1011000100; },
                "SUBIS" => { opcode = 0b1111000100; },
                "ANDI" =>  { opcode = 0b1001001000; },
                "ORRI" =>  { opcode = 0b1011001000; },
                "EORI" =>  { opcode = 0b1101001000; },
                _ => { panic!("This should never occur"); }
            }
            debug_assert!(opcode <= 0b1111111111);
            instruction.set_bits(opcode, 31, 22);
        },
        DecodedInstruction::IM => {
            //Opcode is 9 bits
            let opcode;
            match tokens[0] {
                "MOVZ" =>  { opcode = 0b110100101; },
                "MOVK" =>  { opcode = 0b111100101; },
                _ => { panic!("This should never occur"); }
            }
            debug_assert!(opcode <= 0b111111111);
            instruction.set_bits(opcode, 31, 23);
        },
        DecodedInstruction::CB => {
            //Opcode is 8 bits
            let opcode;
            match tokens[0] {
                "CBZ" =>  { opcode = 0b10110100; },
                "CBNZ" => { opcode = 0b10110101; },
                token => {
                    if token.starts_with("B.") {//B.cond
                        opcode = 0b01010100;
                    } else {
                        panic!("This should never occur");
                    }
                }
            }
            debug_assert!(opcode <= 0b11111111);
            instruction.set_bits(opcode, 31, 24);
        },
        DecodedInstruction::B => {
            //Opcode is 6 bits
            let opcode;
            match tokens[0] {
                "B" => { opcode = 0b000101; },
                "BL" => { opcode = 0b100101; },
                _ => { panic!("This should never occur"); }
            }
            debug_assert!(opcode <= 0b111111);
            instruction.set_bits(opcode, 31, 26);
        }
    }
    */

/*
    //Determine register fields

    //Determine Rd/Rt/neither depending on the instruction type
    match instruction_type {
        DecodedInstruction::R | DecodedInstruction::I | DecodedInstruction::D | DecodedInstruction::CB | DecodedInstruction::IM => {
            if let Some(rd_rt) = parse_register(tokens[1]) {
                instruction.set_bits(rd_rt, 4, 0);
            } else {
                return None;
            }
        }
        DecodedInstruction::B => {}//It does not have this field
    }

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
