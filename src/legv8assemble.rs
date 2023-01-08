/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

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

        if let Some((instruction_type, instruction)) = assemble(trimmed_line) {
            match instruction_type {
                InstructionType::R => {
                    eprintln!("  The instruction \"{}\" is R type", trimmed_line);
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
                    eprintln!("  The instruction \"{}\" is I type", trimmed_line);
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
                    eprintln!("  The instruction \"{}\" is D type", trimmed_line);
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
            }

            eprintln!("  Alternatively, here is the instruction in a few, potentially more convenient formats:");
            eprintln!("    Hex: {:#X}", instruction);
            eprintln!("    Bin: {:#b}", instruction);
            eprintln!("    Oct: {:#o}", instruction);
            eprintln!("    Dec: {}", instruction);
        } else {
            eprintln!("\tHmm, it seems that the instruction you entered isn't valid. Give it another go!");
        }

        line_buffer.truncate(0);
    }
}

fn assemble(string: &str) -> Option<(InstructionType, u32)> {
    return Some((InstructionType::D, 0xDEADBEEF));//TODO
}
