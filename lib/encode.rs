/* encode.rs
 * By: John Jekel
 * Copyright (C) 2023 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Decoded LEGv8 instruction -> actual 32-bit raw instruction
 *
*/

/* Imports */

use crate::{DecodedOpcode, DecodedInstruction, ConvenientlyBitAccessible};

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

pub fn encode(decoded_instruction: DecodedInstruction) -> Option<u32> {
    //Decode the opcode first
    let raw_opcode = encode_opcode(decoded_instruction)?;

    let mut raw_instruction = 0u32;
    match decoded_instruction {
        DecodedInstruction::R{rm, shamt, rn, rd, ..} => {
            debug_assert!(raw_opcode <= 0b11111111111);
            raw_instruction.set_bits(raw_opcode.into(), 31, 21);

            if rm > 31 { return None; }
            raw_instruction.set_bits(rm.into(), 20, 16);

            if shamt > 63 { return None; }
            raw_instruction.set_bits(shamt.into(), 15, 10);

            if rn > 31 { return None; }
            raw_instruction.set_bits(rn.into(), 9, 5);

            if rd > 31 { return None; }
            raw_instruction.set_bits(rd.into(), 4, 0);
        },
        DecodedInstruction::I{imm12, rn, rd, ..} => {
            debug_assert!(raw_opcode <= 0b1111111111);
            raw_instruction.set_bits(raw_opcode.into(), 31, 22);

            if imm12 > 0b111111111111 { return None; }
            raw_instruction.set_bits(imm12.into(), 21, 10);

            if rn > 31 { return None; }
            raw_instruction.set_bits(rn.into(), 9, 5);

            if rd > 31 { return None; }
            raw_instruction.set_bits(rd.into(), 4, 0);
        },
        DecodedInstruction::D{addr9, op2, rn, rt, ..} => {
            debug_assert!(raw_opcode <= 0b11111111111);
            raw_instruction.set_bits(raw_opcode.into(), 31, 21);

            if addr9 > 0b111111111 { return None; }
            raw_instruction.set_bits(addr9.into(), 20, 12);

            if op2 > 0b11 { return None; }
            raw_instruction.set_bits(op2.into(), 11, 10);

            if rn > 31 { return None; }
            raw_instruction.set_bits(rn.into(), 9, 5);

            if rt > 31 { return None; }
            raw_instruction.set_bits(rt.into(), 4, 0);
        },
        DecodedInstruction::B{addr26, ..} => {
            debug_assert!(raw_opcode <= 0b111111);
            raw_instruction.set_bits(raw_opcode.into(), 31, 26);

            if addr26 > 0b11111111111111111111111111 { return None; }
            raw_instruction.set_bits(addr26.into(), 25, 0);
        },
        DecodedInstruction::CB{addr19, rt, ..} => {
            debug_assert!(raw_opcode <= 0b11111111);
            raw_instruction.set_bits(raw_opcode.into(), 31, 24);

            if addr19 > 0b1111111111111111111 { return None; }
            raw_instruction.set_bits(addr19.into(), 23, 5);

            if rt > 31 { return None; }
            raw_instruction.set_bits(rt.into(), 4, 0);
        },
        DecodedInstruction::IW{lsl, imm16, rd, ..} => {
            debug_assert!(raw_opcode <= 0b111111111);
            raw_instruction.set_bits(raw_opcode.into(), 31, 23);

            raw_instruction.set_bits(
                match lsl {
                    0 => { 0 },
                    16 => { 1 },
                    32 => { 2 },
                    48 => { 3 }
                    _ => { return None; }
                },

                22, 21
            );

            raw_instruction.set_bits(imm16.into(), 20, 5);

            if rd > 31 { return None; }
            raw_instruction.set_bits(rd.into(), 4, 0);
        }
    }

    return Some(raw_instruction);
}

pub fn encode_opcode(decoded_instruction: DecodedInstruction) -> Option<u16> {
    match decoded_instruction {
        DecodedInstruction::R{opcode, ..} => {
            //Opcode is 11 bits
            match opcode {
                DecodedOpcode::ADD =>  { return Some(0b10001011000); },
                DecodedOpcode::SUB =>  { return Some(0b11001011000); },
                DecodedOpcode::ADDS => { return Some(0b10101011000); },
                DecodedOpcode::SUBS => { return Some(0b11101011000); },
                DecodedOpcode::AND =>  { return Some(0b10001010000); },
                DecodedOpcode::ORR =>  { return Some(0b10101010000); },
                DecodedOpcode::EOR =>  { return Some(0b11001010000); },
                DecodedOpcode::LSL =>  { return Some(0b11010011011); },
                DecodedOpcode::LSR =>  { return Some(0b11010011010); },
                DecodedOpcode::BR =>   { return Some(0b11010110000); }
                _ => { return None; }
            }
        },
        DecodedInstruction::D{opcode, ..} => {
            //Opcode is 11 bits
            match opcode {
                DecodedOpcode::LDUR =>   { return Some(0b11111000010); },
                DecodedOpcode::STUR =>   { return Some(0b11111000000); },
                DecodedOpcode::LDURSW => { return Some(0b10111000100); },
                DecodedOpcode::STURW =>  { return Some(0b10111000000); },
                DecodedOpcode::LDURH =>  { return Some(0b01111000010); },
                DecodedOpcode::STURH =>  { return Some(0b01111000000); },
                DecodedOpcode::LDURB =>  { return Some(0b00111000010); },
                DecodedOpcode::STURB =>  { return Some(0b00111000000); },
                DecodedOpcode::LDXR =>   { return Some(0b11001000010); },
                DecodedOpcode::STXR =>   { return Some(0b11001000000); },
                _ => { return None; }
            }
        },
        DecodedInstruction::I{opcode, ..} => {
            //Opcode is 10 bits
            match opcode {
                DecodedOpcode::ADDI =>  { return Some(0b1001000100); },
                DecodedOpcode::SUBI =>  { return Some(0b1101000100); },
                DecodedOpcode::ADDIS => { return Some(0b1011000100); },
                DecodedOpcode::SUBIS => { return Some(0b1111000100); },
                DecodedOpcode::ANDI =>  { return Some(0b1001001000); },
                DecodedOpcode::ORRI =>  { return Some(0b1011001000); },
                DecodedOpcode::EORI =>  { return Some(0b1101001000); },
                _ => { return None; }
            }
        },
        DecodedInstruction::IW{opcode, ..} => {
            //Opcode is 9 bits
            match opcode {
                DecodedOpcode::MOVZ =>  { return Some(0b110100101); },
                DecodedOpcode::MOVK =>  { return Some(0b111100101); },
                _ => { return None; }
            }
        },
        DecodedInstruction::CB{opcode, ..} => {
            //Opcode is 8 bits
            match opcode {
                DecodedOpcode::CBZ =>    { return Some(0b10110100); },
                DecodedOpcode::CBNZ =>   { return Some(0b10110101); },
                DecodedOpcode::B_cond => { return Some(0b01010100); },
                _ => { return None; }
            }
        },
        DecodedInstruction::B{opcode, ..} => {
            //Opcode is 6 bits
            match opcode {
                DecodedOpcode::B =>  { return Some(0b000101); },
                DecodedOpcode::BL => { return Some(0b100101); },
                _ => { return None; }
            }
        }
    }
}
