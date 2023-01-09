/* lib.rs
 * By: John Jekel
 * Copyright (C) 2023 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * LEGv8 Library
 *
*/

/* Imports */

pub mod asm;
pub mod disasm;
pub mod decode;
pub mod encode;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

#[derive(Copy, Clone, Debug)]
pub enum DecodedInstruction {
    R{opcode: DecodedOpcode, rm: u8, shamt: u8, rn: u8, rd: u8},
    I{opcode: DecodedOpcode, imm12: u16, rn: u8, rd: u8},
    D{opcode: DecodedOpcode, addr9: u16, op2: u8, rn: u8, rt: u8},
    B{opcode: DecodedOpcode, addr26: u32},
    CB{opcode: DecodedOpcode, addr19: u32, rt: u8},
    IW{opcode: DecodedOpcode, lsl: u8, imm16: u16, rd: u8}//NOTE lsl is not 0, 1, 2, or 3, it is 0, 16, 32, or 64 (since that is faster than having to translate it each time, at the expense of slowing down encode)
}

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum DecodedOpcode {
    ADD,
    SUB,
    ADDI,
    SUBI,
    ADDS,
    SUBS,
    ADDIS,
    SUBIS,
    LDUR,
    STUR,
    LDURSW,
    STURW,
    LDURH,
    STURH,
    LDURB,
    STURB,
    LDXR,
    STXR,
    MOVZ,
    MOVK,
    AND,
    ORR,
    EOR,
    ANDI,
    ORRI,
    EORI,
    LSL,
    LSR,
    CBZ,
    CBNZ,
    B_cond,
    B,
    BR,
    BL,
    Invalid
}

pub trait ConvenientlyBitAccessible: Sized  {
    fn get_bit(self: Self, index: u8) -> Self {
        return self.get_bits(index, index);
    }

    fn get_bits(self: Self, high: u8, low: u8) -> Self;

    fn set_bits(self: &mut Self, value: Self, high: u8, low: u8);
}

/* Associated Functions and Methods */

impl ConvenientlyBitAccessible for u32 {
    fn get_bits(self: Self, high: u8, low: u8) -> u32 {
        debug_assert!(high < 32);
        debug_assert!(low < 32);
        debug_assert!(low <= high);

        let num_bits_to_keep = high - low + 1;
        let mask = (1 << num_bits_to_keep) - 1;
        let unmasked_value = self >> low;
        return unmasked_value & mask;
    }

    fn set_bits(self: &mut Self, value: Self, high: u8, low: u8) {
        debug_assert!(high < 32);
        debug_assert!(low < 32);
        debug_assert!(low <= high);

        let num_bits_to_change = high - low + 1;
        let mask_unshifted = (1 << num_bits_to_change) - 1;
        let mask = mask_unshifted << low;
        debug_assert!(value <= mask_unshifted);
        let shifted_value = value << low;

        *self = (*self & !mask) | shifted_value;
    }
}

/* Functions */

//TODO
