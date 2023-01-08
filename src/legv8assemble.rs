/* legv8assembles.rs
 * By: John Jekel
 * Copyright (C) 2022-2023 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Assembles individual LEGv8 instructions
 *
*/

//TODO move more things to the library

/* Imports */

use legv8::InstructionType;

/* Constants */

//TODO

/* Macros */

//TODO

/* Static Variables */

//TODO

/* Types */

trait ConvenientlyBitAccessible: Sized  {
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

fn main() {
    eprintln!("legv8assemble, by John Jekel (2023)\n");
    eprintln!("At the prompt, enter a LEGv8 instruction to assemble, or press Ctrl+C to exit...");

    let stdin = std::io::stdin();
    let mut line_buffer = String::new();
    loop {
        eprint!("legv8assemble> ");
        stdin.read_line(&mut line_buffer).unwrap();
        let trimmed_line = line_buffer.trim();
        let nice_line = trimmed_line.to_string().to_uppercase();

        if let Some((instruction_type, instruction)) = assemble(&nice_line) {
            match instruction_type {
                InstructionType::R => {
                    eprintln!("  The instruction \"{}\" is R type", nice_line);
                    eprintln!("    ________________________________________________");
                    eprintln!("    |          11 |     5 |      6 |     5 |     5 | <- Field length in bits");
                    eprintln!("    |----------------------------------------------|");
                    eprintln!("    | 31       21 | 20 16 | 15  10 | 9   5 | 4   0 | <- Field start and end bit indexes (inclusive)");
                    eprintln!("    |----------------------------------------------|");
                    eprintln!("    | opcode      | Rm    | shamt  | Rn    | Rd    | <- Field name");
                    eprintln!("    |----------------------------------------------|");
                    eprintln!("    | {:0>11b} | {:0>5b} | {:0>6b} | {:0>5b} | {:0>5b} | <- Field contents",
                        instruction.get_bits(31, 21),
                        instruction.get_bits(20, 16),
                        instruction.get_bits(15, 10),
                        instruction.get_bits(9, 5),
                        instruction.get_bits(4, 0)
                    );
                    eprintln!("    ------------------------------------------------");
                },
                InstructionType::I => {
                    eprintln!("  The instruction \"{}\" is I type", nice_line);
                    eprintln!("    _____________________________________________");
                    eprintln!("    |         10 |           12 |     5 |     5 | <- Field length in bits");
                    eprintln!("    |-------------------------------------------|");
                    eprintln!("    | 31      22 | 21        10 | 9   5 | 4   0 | <- Field start and end bit indexes (inclusive)");
                    eprintln!("    |-------------------------------------------|");
                    eprintln!("    | opcode     | immediate    | Rn    | Rd    | <- Field name");
                    eprintln!("    |-------------------------------------------|");
                    eprintln!("    | {:0>10b} | {:0>12b} | {:0>5b} | {:0>5b} | <- Field contents",
                        instruction.get_bits(31, 22),
                        instruction.get_bits(21, 10),
                        instruction.get_bits(9, 5),
                        instruction.get_bits(4, 0)
                    );
                    eprintln!("    ---------------------------------------------");
                },
                InstructionType::D => {
                    eprintln!("  The instruction \"{}\" is D type", nice_line);
                    eprintln!("    ___________________________________________________");
                    eprintln!("    |          11 |         9 |     2 |     5 |     5 | <- Field length in bits");
                    eprintln!("    |-------------------------------------------------|");
                    eprintln!("    | 31       21 | 20     12 | 11 10 | 9   5 | 4   0 | <- Field start and end bit indexes (inclusive)");
                    eprintln!("    |-------------------------------------------------|");
                    eprintln!("    | opcode      | address   | op2   | Rn    | Rt    | <- Field name");
                    eprintln!("    |-------------------------------------------------|");
                    eprintln!("    | {:0>11b} | {:0>9b} | {:0>1b}   {:0>1b} | {:0>5b} | {:0>5b} | <- Field contents",
                        instruction.get_bits(31, 21),
                        instruction.get_bits(20, 12),
                        instruction.get_bit(11), instruction.get_bit(10),
                        instruction.get_bits(9, 5),
                        instruction.get_bits(4, 0)
                    );
                    eprintln!("    ------------------------------------------");
                },
                InstructionType::B => {
                    eprintln!("  The instruction \"{}\" is B type", nice_line);
                    eprintln!("    _______________________________________");
                    eprintln!("    |      6 |                         26 | <- Field length in bits");
                    eprintln!("    |-------------------------------------|");
                    eprintln!("    | 31  26 | 25                       0 | <- Field start and end bit indexes (inclusive)");
                    eprintln!("    |-------------------------------------|");
                    eprintln!("    | opcode | address                    | <- Field name");
                    eprintln!("    |-------------------------------------|");
                    eprintln!("    | {:0>6b} | {:0>26b} | <- Field contents",
                        instruction.get_bits(31, 26),
                        instruction.get_bits(25, 0),
                    );
                    eprintln!("    ---------------------------------------");
                },
                InstructionType::CB => {
                    eprintln!("  The instruction \"{}\" is CB type", nice_line);
                    eprintln!("    __________________________________________");
                    eprintln!("    |        8 |                  19 |     5 | <- Field length in bits");
                    eprintln!("    |----------------------------------------|");
                    eprintln!("    | 31    24 | 23                5 | 4   0 | <- Field start and end bit indexes (inclusive)");
                    eprintln!("    |----------------------------------------|");
                    eprintln!("    | opcode   | address             | Rt    | <- Field name");
                    eprintln!("    |----------------------------------------|");
                    eprintln!("    | {:0>8b} | {:0>19b} | {:0>5b} | <- Field contents",
                        instruction.get_bits(31, 24),
                        instruction.get_bits(23, 5),
                        instruction.get_bits(4, 0)
                    );
                    eprintln!("    ------------------------------------------");
                },
                InstructionType::IM => {
                    eprintln!("  The instruction \"{}\" is IM type", nice_line);
                    eprintln!("    ________________________________________________");
                    eprintln!("    |         9 |     2 |               16 |     5 | <- Field length in bits");
                    eprintln!("    |----------------------------------------------|");
                    eprintln!("    | 31     22 | 23 21 | 20             5 | 4   0 | <- Field start and end bit indexes (inclusive)");
                    eprintln!("    |----------------------------------------------|");
                    eprintln!("    | opcode    | shamt | immediate        | Rd    | <- Field name");
                    eprintln!("    |----------------------------------------------|");
                    eprintln!("    | {:0>9b} | {:0>1b}   {:0>1b} | {:0>16b} | {:0>5b} | <- Field contents",
                        instruction.get_bits(31, 23),
                        instruction.get_bit(22), instruction.get_bit(21),
                        instruction.get_bits(20, 5),
                        instruction.get_bits(4, 0)
                    );
                    eprintln!("    ------------------------------------------------");
                }
            }

            eprintln!("  Alternatively, here is the instruction in a few, potentially more convenient formats:");
            eprintln!("    Hex: {:#X}", instruction);
            eprintln!("    Bin: {:#b}", instruction);
            eprintln!("    Oct: {:#o}", instruction);
            eprintln!("    Dec: {}", instruction);
            let instruction_le = instruction.swap_bytes();
            eprintln!("    Hex (Little-Endian): {:#X}", instruction_le);
            eprintln!("    Bin (Little-Endian): {:#b}", instruction_le);
            eprintln!("    Oct (Little-Endian): {:#o}", instruction_le);
            eprintln!("    Dec (Little-Endian): {}", instruction_le);
        } else {
            eprintln!("  Hmm, it seems that the instruction you entered isn't valid. Give it another go!");
        }

        line_buffer.truncate(0);
    }
}

fn assemble(instruction_string: &str) -> Option<(InstructionType, u32)> {
    let mut tokens = Vec::<&str>::with_capacity(5);//Max number of tokens we expect
    for token in instruction_string.split(&[' ', '\t', '\n', '\r', ',', '[', ']']) {//TODO ensure the syntax is correct instead of just stripping out these symbols when splitting
        if !token.is_empty() {
            tokens.push(token);
        }
    }
    if tokens.len() < 2 {//Basic sanity check; every instruction has at least 1 operand
        return None;
    }

    //Determine the instruction type
    let instruction_type;
    match tokens[0] {
        "ADD" | "SUB" | "ADDS" | "SUBS" | "AND" | "ORR" | "EOR" | "LSL" | "LSR" | "BR" => { instruction_type = InstructionType::R; },
        "ADDI" | "SUBI" | "ADDIS" | "SUBIS" | "ANDI" | "ORRI" | "EORI" => { instruction_type = InstructionType::I; },
        "LDUR" | "STUR" | "LDURSW" | "STURW" | "LDURH" | "STURH" | "LDURB" | "STURB" | "LDXR" | "STXR" => { instruction_type = InstructionType::D; },
        "MOVZ" | "MOVK" => { instruction_type = InstructionType::IM; },
        "B" | "BL" => { instruction_type = InstructionType::B; },
        "CBZ" | "CBNZ"  => { instruction_type = InstructionType::CB; },
        token => {
            if token.starts_with("B.") {//B.cond
                instruction_type = InstructionType::CB;
            } else {//Invalid
                return None;
            }
        }
    }

    //Ensure, based on the instruction type, that there are the correct number of operands
    match instruction_type {
        InstructionType::R | InstructionType::I | InstructionType::D => {
            //For R, this is the instruction followed by three registers (except for BR, which only has 1 register after it)
            //For I, this is the instruction followed by two registers and an immediate
            //For D, this is the instruction followed by a register, another register, and an offset
            if tokens.len() != 4 {
                if !((tokens[0] == "BR") && (tokens.len() == 2)) {
                    return None;
                }
            }
        },
        InstructionType::IM => { if tokens.len() != 5 { return None; }},//Instruction, register, immediate, "LSL", and LSL amount
        InstructionType::B => { if tokens.len() != 2 { return None; }},//Instruction and offset
        InstructionType::CB => {
            //Instruction, register and offset, unless the is B.cond, in which case there is only an offset
            if (tokens.len() != 3) || (tokens[0].starts_with("B.") && (tokens.len() != 2)) {
                if !(tokens[0].starts_with("B.") && (tokens.len() == 2)) {
                    return None;
                }
            }
        },
    }

    //At this point, we can assume the instruction, as well as the number of tokens is valid
    //However, the tokens themselves, as well as the condition for B.cond, will still need checking

    //Begin to construct the instruction
    let mut instruction = 0u32;//Fields are 0 to begin with

    //Determine the opcode
    match instruction_type {
        InstructionType::R | InstructionType::D => {
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
        InstructionType::I => {
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
        InstructionType::IM => {
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
        InstructionType::CB => {
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
        InstructionType::B => {
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

    //Determine Rd/Rt/neither depending on the instruction type
    match instruction_type {
        InstructionType::R | InstructionType::I | InstructionType::D | InstructionType::CB | InstructionType::IM => {
            if let Some(rd_rt) = parse_register(tokens[1]) {
                instruction.set_bits(rd_rt, 4, 0);
            } else {
                return None;
            }
        }
        InstructionType::B => {}//It does not have this field
    }

    //Determine Rn depending on the instruction type
    match instruction_type {
        InstructionType::R | InstructionType::I | InstructionType::D => {
            if tokens[0] != "BR" {//This one does not have the Rn operand
                if let Some(rn) = parse_register(tokens[2]) {
                    instruction.set_bits(rn, 9, 5);
                } else {
                    return None;
                }
            }
        }
        InstructionType::CB | InstructionType::IM | InstructionType::B => {}//They do not have this field
    }

    //Determine Rm if the instruction type is R
    if matches!(instruction_type, InstructionType::R) {
        if tokens[0] != "BR" {//This one does not have the Rn operand
            if let Some(rm) = parse_register(tokens[3]) {
                instruction.set_bits(rm, 20, 16);
            } else {
                return None;
            }
        }
    }

    //Determine the immediate/address to use from the 4th token depending on the instruction_type
    match instruction_type {
        InstructionType::I => {
            if let Some(immediate) = smart_parse_uint(tokens[3]) {
                if immediate > 0b111111111111 {//Too large for the field to hold
                    return None;
                }
                instruction.set_bits(immediate, 21, 10);
            }
        },
        InstructionType::D => {
            if let Some(address) = smart_parse_uint(tokens[3]) {
                if address > 0b111111111 {//Too large for the field to hold
                    return None;
                }
                instruction.set_bits(address, 20, 12);
            }
        },
        InstructionType::R | InstructionType::CB | InstructionType::IM | InstructionType::B => {}//They do not have this field
    }

    //TODO other fields

    return Some((instruction_type, instruction));
}

fn parse_register(register_token: &str) -> Option<u32> {
    if let Some(relevant_part_of_token) = register_token.strip_prefix("X") {
        if let Some(parsed_register_number) = relevant_part_of_token.parse::<u32>().ok() {
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

fn smart_parse_uint(uint_string: &str) -> Option<u32> {
    if let Some(binary_uint_string) = uint_string.strip_prefix("0b") {
        return u32::from_str_radix(binary_uint_string, 2).ok();
    } else if let Some(binary_uint_string) = uint_string.strip_prefix("0B") {
        return u32::from_str_radix(binary_uint_string, 2).ok();
    } else if let Some(hex_uint_string) = uint_string.strip_prefix("0x") {
        return u32::from_str_radix(hex_uint_string, 16).ok();
    } else if let Some(hex_uint_string) = uint_string.strip_prefix("0X") {
        return u32::from_str_radix(hex_uint_string, 16).ok();
    } else if let Some(oct_uint_string) = uint_string.strip_prefix("0o") {
        return u32::from_str_radix(oct_uint_string, 8).ok();
    } else if let Some(oct_uint_string) = uint_string.strip_prefix("0O") {
        return u32::from_str_radix(oct_uint_string, 8).ok();
    } else if let Some(dec_uint) = uint_string.parse::<u32>().ok() {
        return Some(dec_uint);
    } else {
        return None;
    }
}
