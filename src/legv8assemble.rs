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
                    eprintln!("    _______________________________________________");
                    eprintln!("    |           10 |           12 |     5 |     5 | <- Field length in bits");
                    eprintln!("    |---------------------------------------------|");
                    eprintln!("    | 31        22 | 21        10 | 9   5 | 4   0 | <- Field start and end bit indexes (inclusive)");
                    eprintln!("    |---------------------------------------------|");
                    eprintln!("    | opcode       | immediate    | Rn    | Rd    | <- Field name");
                    eprintln!("    |---------------------------------------------|");
                    eprintln!("    | {:0>10b} | {:0>12b} | {:0>5b} | {:0>5b} | <- Field contents",
                        instruction.get_bits(31, 20),
                        instruction.get_bits(21, 10),
                        instruction.get_bits(9, 5),
                        instruction.get_bits(4, 0)
                    );
                    eprintln!("    -----------------------------------------------");
                },
                InstructionType::D => {
                    eprintln!("  The instruction \"{}\" is D type", nice_line);
                    eprintln!("    ___________________________________________________");
                    eprintln!("    |          11 |         9 |     2 |     5 |     5 | <- Field length in bits");
                    eprintln!("    |-------------------------------------------------|");
                    eprintln!("    | 31       21 | 20     12 | 11 10 | 9   5 | 4   0 | <- Field start and end bit indexes (inclusive)");
                    eprintln!("    |-------------------------------------------------|");
                    eprintln!("    | opcode      | address   | op2   | Rn    | Rd    | <- Field name");
                    eprintln!("    |-------------------------------------------------|");
                    eprintln!("    | {:0>11b} | {:0>9b} | {:0>1b}   {:0>1b} | {:0>5b} | {:0>5b} | <- Field contents",
                        instruction.get_bits(31, 21),
                        instruction.get_bits(20, 12),
                        instruction.get_bit(11), instruction.get_bit(10),
                        instruction.get_bits(9, 5),
                        instruction.get_bits(4, 0)
                    );
                    eprintln!("    ---------------------------------------------------");
                }
                _ => { todo!(); }//TODO other instruction types
            }

            eprintln!("  Alternatively, here is the instruction in a few, potentially more convenient formats:");
            eprintln!("    Hex: {:#X}", instruction);
            eprintln!("    Bin: {:#b}", instruction);
            eprintln!("    Oct: {:#o}", instruction);
            eprintln!("    Dec: {}", instruction);
        } else {
            eprintln!("  Hmm, it seems that the instruction you entered isn't valid. Give it another go!");
        }

        line_buffer.truncate(0);
    }
}

fn assemble(instruction_string: &str) -> Option<(InstructionType, u32)> {
    let tokens: Vec<&str> = instruction_string.split_whitespace().collect();
    if tokens.len() < 2 {
        return None;
    }

    //Determine the instruction type
    let instruction_type;
    match tokens[0] {
        "ADD" | "SUB" | "ADDS" | "SUBS" | "AND" | "ORR" | "EOR" | "LSL" | "LSR" | "BR" => { instruction_type = InstructionType::R; },
        "ADDI" | "SUBI" | "ADDIS" | "SUBIS" | "ANDI" | "ORRI" | "EORI" => { instruction_type = InstructionType::I; },
        "LDUR" | "STUR" | "LDURSW" | "STURW" | "LDURH" | "STURH" | "LDURB" | "STURB" | "LDXR" | "STXR" => { instruction_type = InstructionType::D; },
        "MOVZ" | "MOVK" => { instruction_type = InstructionType::IW; },
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

    //Determine the opcode
    /*let opcode;
    match tokens[0] {

    }
    */

    return Some((instruction_type, 0xDEADBEEF));//TODO
}
