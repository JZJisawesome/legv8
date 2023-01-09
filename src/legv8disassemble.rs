/* legv8disassemble.rs
 * By: John Jekel
 * Copyright (C) 2023 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Disassembles individual LEGv8 instructions
 *
*/

/* Imports */

//TODO (include "use" and "mod" here)

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

fn main() {
    //TESTING
    use legv8::asm::assemble_raw;
    let test = assemble_raw("add x1, x2, x3");
    eprintln!("Result: {:?}", test);

    eprintln!("Coming soon!");
    todo!();
}
