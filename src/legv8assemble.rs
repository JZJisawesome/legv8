/* legv8assembles.rs
 * By: John Jekel
 * Copyright (C) 2023 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Assembles individual LEGv8 instructions
 *
*/

//TODO move more things to the library

/* Imports */

use legv8::{asm, encode, ConvenientlyBitAccessible};

/* Constants */

//TODO

/* Macros */

//TODO

/* Static Variables */

//TODO

/* Types */

#[derive(Copy, Clone, Debug)]
pub enum InstructionType {
    R,
    I,
    D,
    B,
    CB,
    IM//TODO fix to be IW
}

/* Associated Functions and Methods */

//TODO

/* Functions */

fn main() {
    eprintln!("\x1b[1m\x1b[35mlegv8assemble\x1b[0m, by \x1b[96mJZJ\x1b[0m :)");//TODO replace this with figlet
    eprintln!("Copyright (C) 2023 John Jekel");
    eprintln!("See the LICENSE file at the root of the project for licensing info.\n");

    eprintln!("\x1b[90mAt the prompt, enter a LEGv8 instruction to assemble, or press Ctrl+C to exit...\x1b[0m");
    let stdin = std::io::stdin();
    let mut line_buffer = String::new();
    loop {
        eprint!("\x1b[1m\x1b[35mlegv8assemble\x1b[97m>\x1b[0m ");
        stdin.read_line(&mut line_buffer).unwrap();
        let trimmed_line = line_buffer.trim();
        let nice_line = trimmed_line.to_string().to_uppercase();

        if let Some((instruction_type, instruction)) = assemble_to_raw_instruction(&nice_line) {
            match instruction_type {
                InstructionType::R => {
                    eprintln!("  The instruction \"\x1b[1m{}\x1b[0m\" is \x1b[94mR\x1b[0m-type", nice_line);
                    eprintln!("    ________________________________________________");
                    eprintln!("    |          \x1b[93m11\x1b[0m |     \x1b[93m5\x1b[0m |      \x1b[93m6\x1b[0m |     \x1b[93m5\x1b[0m |     \x1b[93m5\x1b[0m | \x1b[90m<- Field length in bits\x1b[0m");
                    eprintln!("    |----------------------------------------------|");
                    eprintln!("    | \x1b[93m31       21\x1b[0m | \x1b[93m20 16\x1b[0m | \x1b[93m15  10\x1b[0m | \x1b[93m9   5\x1b[0m | \x1b[93m4   0\x1b[0m | \x1b[90m<- Field start and end bit indexes (inclusive)\x1b[0m");
                    eprintln!("    |----------------------------------------------|");
                    eprintln!("    | \x1b[93mopcode\x1b[0m      | \x1b[93mRm\x1b[0m    | \x1b[93mshamt\x1b[0m  | \x1b[93mRn\x1b[0m    | \x1b[93mRd\x1b[0m    | \x1b[90m<- Field name\x1b[0m");
                    eprintln!("    |----------------------------------------------|");
                    eprintln!("    | \x1b[96m{:0>11b}\x1b[0m | \x1b[96m{:0>5b}\x1b[0m | \x1b[96m{:0>6b}\x1b[0m | \x1b[96m{:0>5b}\x1b[0m | \x1b[96m{:0>5b}\x1b[0m | \x1b[90m<- Field contents\x1b[0m",
                        instruction.get_bits(31, 21),
                        instruction.get_bits(20, 16),
                        instruction.get_bits(15, 10),
                        instruction.get_bits(9, 5),
                        instruction.get_bits(4, 0)
                    );
                    eprintln!("    ------------------------------------------------");
                },
                InstructionType::I => {
                    eprintln!("  The instruction \"\x1b[1m{}\x1b[0m\" is \x1b[94mI\x1b[0m-type", nice_line);
                    eprintln!("    _____________________________________________");
                    eprintln!("    |         \x1b[93m10\x1b[0m |           \x1b[93m12\x1b[0m |     \x1b[93m5\x1b[0m |     \x1b[93m5\x1b[0m | \x1b[90m<- Field length in bits\x1b[0m");
                    eprintln!("    |-------------------------------------------|");
                    eprintln!("    | \x1b[93m31      22\x1b[0m | \x1b[93m21        10\x1b[0m | \x1b[93m9   5\x1b[0m | \x1b[93m4   0\x1b[0m | \x1b[90m<- Field start and end bit indexes (inclusive)\x1b[0m");
                    eprintln!("    |-------------------------------------------|");
                    eprintln!("    | \x1b[93mopcode\x1b[0m     | \x1b[93mimmediate\x1b[0m    | \x1b[93mRn\x1b[0m    | \x1b[93mRd\x1b[0m    | \x1b[90m<- Field name\x1b[0m");
                    eprintln!("    |-------------------------------------------|");
                    eprintln!("    | \x1b[96m{:0>10b}\x1b[0m | \x1b[96m{:0>12b}\x1b[0m | \x1b[96m{:0>5b}\x1b[0m | \x1b[96m{:0>5b}\x1b[0m | \x1b[90m<- Field contents\x1b[0m",
                        instruction.get_bits(31, 22),
                        instruction.get_bits(21, 10),
                        instruction.get_bits(9, 5),
                        instruction.get_bits(4, 0)
                    );
                    eprintln!("    ---------------------------------------------");
                },
                InstructionType::D => {
                    eprintln!("  The instruction \"\x1b[1m{}\x1b[0m\" is \x1b[94mD\x1b[0m-type", nice_line);
                    eprintln!("    ___________________________________________________");
                    eprintln!("    |          \x1b[93m11\x1b[0m |         \x1b[93m9\x1b[0m |     \x1b[93m2\x1b[0m |     \x1b[93m5\x1b[0m |     \x1b[93m5\x1b[0m | \x1b[90m<- Field length in bits\x1b[0m");
                    eprintln!("    |-------------------------------------------------|");
                    eprintln!("    | \x1b[93m31       21\x1b[0m | \x1b[93m20     12\x1b[0m | \x1b[93m11 10\x1b[0m | \x1b[93m9   5\x1b[0m | \x1b[93m4   0\x1b[0m | \x1b[90m<- Field start and end bit indexes (inclusive)\x1b[0m");
                    eprintln!("    |-------------------------------------------------|");
                    eprintln!("    | \x1b[93mopcode\x1b[0m      | \x1b[93maddress\x1b[0m   | \x1b[93mop2\x1b[0m   | \x1b[93mRn\x1b[0m    | \x1b[93mRt\x1b[0m    | \x1b[90m<- Field name\x1b[0m");
                    eprintln!("    |-------------------------------------------------|");
                    eprintln!("    | \x1b[96m{:0>11b}\x1b[0m | \x1b[96m{:0>9b}\x1b[0m | \x1b[96m{:0>1b}\x1b[0m   \x1b[96m{:0>1b}\x1b[0m | \x1b[96m{:0>5b}\x1b[0m | \x1b[96m{:0>5b}\x1b[0m | \x1b[90m<- Field contents\x1b[0m",
                        instruction.get_bits(31, 21),
                        instruction.get_bits(20, 12),
                        instruction.get_bit(11), instruction.get_bit(10),
                        instruction.get_bits(9, 5),
                        instruction.get_bits(4, 0)
                    );
                    eprintln!("    ------------------------------------------");
                },
                InstructionType::B => {
                    eprintln!("  The instruction \"\x1b[1m{}\x1b[0m\" is \x1b[94mB\x1b[0m-type", nice_line);
                    eprintln!("    _______________________________________");
                    eprintln!("    |      \x1b[93m6\x1b[0m |                         \x1b[93m26\x1b[0m | \x1b[90m<- Field length in bits\x1b[0m");
                    eprintln!("    |-------------------------------------|");
                    eprintln!("    | \x1b[93m31  26\x1b[0m | \x1b[93m25                       0\x1b[0m | \x1b[90m<- Field start and end bit indexes (inclusive)\x1b[0m");
                    eprintln!("    |-------------------------------------|");
                    eprintln!("    | \x1b[93mopcode\x1b[0m | \x1b[93maddress\x1b[0m                    | \x1b[90m<- Field name\x1b[0m");
                    eprintln!("    |-------------------------------------|");
                    eprintln!("    | \x1b[96m{:0>6b}\x1b[0m | \x1b[96m{:0>26b}\x1b[0m | \x1b[90m<- Field contents\x1b[0m",
                        instruction.get_bits(31, 26),
                        instruction.get_bits(25, 0),
                    );
                    eprintln!("    ---------------------------------------");
                },
                InstructionType::CB => {
                    eprintln!("  The instruction \"\x1b[1m{}\x1b[0m\" is \x1b[94mCB\x1b[0m-type", nice_line);
                    eprintln!("    __________________________________________");
                    eprintln!("    |        \x1b[93m8\x1b[0m |                  \x1b[93m19\x1b[0m |     \x1b[93m5\x1b[0m | \x1b[90m<- Field length in bits\x1b[0m");
                    eprintln!("    |----------------------------------------|");
                    eprintln!("    | \x1b[93m31    24\x1b[0m | \x1b[93m23                5\x1b[0m | \x1b[93m4   0\x1b[0m | \x1b[90m<- Field start and end bit indexes (inclusive)\x1b[0m");
                    eprintln!("    |----------------------------------------|");
                    eprintln!("    | \x1b[93mopcode\x1b[0m   | \x1b[93maddress\x1b[0m             | \x1b[93mRt\x1b[0m    | \x1b[90m<- Field name\x1b[0m");
                    eprintln!("    |----------------------------------------|");
                    eprintln!("    | \x1b[96m{:0>8b}\x1b[0m | \x1b[96m{:0>19b}\x1b[0m | \x1b[96m{:0>5b}\x1b[0m | \x1b[90m<- Field contents\x1b[0m",
                        instruction.get_bits(31, 24),
                        instruction.get_bits(23, 5),
                        instruction.get_bits(4, 0)
                    );
                    eprintln!("    ------------------------------------------");
                },
                InstructionType::IM => {
                    eprintln!("  The instruction \"\x1b[1m{}\x1b[0m\" is \x1b[94mIW\x1b[0m-type", nice_line);
                    eprintln!("    ________________________________________________");
                    eprintln!("    |         \x1b[93m9\x1b[0m |     \x1b[93m2\x1b[0m |               \x1b[93m16\x1b[0m |     \x1b[93m5\x1b[0m | \x1b[90m<- Field length in bits\x1b[0m");
                    eprintln!("    |----------------------------------------------|");
                    eprintln!("    | \x1b[93m31     22\x1b[0m | \x1b[93m23 21\x1b[0m | \x1b[93m20             5\x1b[0m | \x1b[93m4   0\x1b[0m | \x1b[90m<- Field start and end bit indexes (inclusive)\x1b[0m");
                    eprintln!("    |----------------------------------------------|");
                    eprintln!("    | \x1b[93mopcode\x1b[0m    | \x1b[93mshamt\x1b[0m | \x1b[93mimmediate\x1b[0m        | \x1b[93mRd\x1b[0m    | \x1b[90m<- Field name\x1b[0m");
                    eprintln!("    |----------------------------------------------|");
                    eprintln!("    | \x1b[96m{:0>9b}\x1b[0m | \x1b[96m{:0>1b}\x1b[0m   \x1b[96m{:0>1b}\x1b[0m | \x1b[96m{:0>16b}\x1b[0m | \x1b[96m{:0>5b}\x1b[0m | \x1b[90m<- Field contents\x1b[0m",
                        instruction.get_bits(31, 23),
                        instruction.get_bit(22), instruction.get_bit(21),
                        instruction.get_bits(20, 5),
                        instruction.get_bits(4, 0)
                    );
                    eprintln!("    ------------------------------------------------");
                }
            }

            eprintln!("  \x1b[92mAlternatively, here is the instruction in a few, potentially more convenient formats:\x1b[0m");
            eprintln!("    Hex (BE): \x1b[96m{:#X}\x1b[0m", instruction);
            eprintln!("    Bin (BE): \x1b[96m{:#b}\x1b[0m", instruction);
            eprintln!("    Oct (BE): \x1b[96m{:#o}\x1b[0m", instruction);
            eprintln!("    Dec (BE): \x1b[96m{}\x1b[0m", instruction);
            let instruction_le = instruction.swap_bytes();
            eprintln!("    Hex (LE): \x1b[96m{:#X}\x1b[0m", instruction_le);
            eprintln!("    Bin (LE): \x1b[96m{:#b}\x1b[0m", instruction_le);
            eprintln!("    Oct (LE): \x1b[96m{:#o}\x1b[0m", instruction_le);
            eprintln!("    Dec (LE): \x1b[96m{}\x1b[0m", instruction_le);
        } else {
            eprintln!("  Hmm, it seems that the instruction you entered \x1b[91misn't valid\x1b[0m. Give it another go!");
        }

        line_buffer.truncate(0);
    }
}

fn assemble_to_raw_instruction(instruction_string: &str) -> Option<(InstructionType, u32)> {
    //TODO replace this function with a wrapper over library functions
    /*
    let assembly_result = asm::assemble_raw(instruction_string)?;
    let instruction_type;
    match assembly_result {
        //TODO convert to InstructionType
    }

    let raw_instruction = encode::encode(assembly_result)?;
    return Some(instruction_type, raw_instruction);
    */

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

    //Determine register fields

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

    //Determine immediate fields

    //Determine the immediate/address to use from the 4th token depending on the instruction_type
    match instruction_type {
        InstructionType::I => {
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
        InstructionType::D => {
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
        InstructionType::R | InstructionType::CB | InstructionType::IM | InstructionType::B => {}//They do not have this field
    }

    //Determine the branch address to use from the second token if the instruction type is B
    if matches!(instruction_type, InstructionType::B) {
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
        InstructionType::CB => {
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
        InstructionType::IM => {
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
        InstructionType::R | InstructionType::I | InstructionType::D | InstructionType::B => {}//They do not have this operand
    }

    //Deal with the shift amount for IM-type instructions
    if matches!(instruction_type, InstructionType::IM) {
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
