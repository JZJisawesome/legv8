# legv8

"LEGv8" Instruction Set Tools (Minimized version of ARMv8 for my ECE 222 course)

The purpose of this project is to make it easier for people taking ECE 222 to practice ARMv8 assembly, specifically the LEGv8 subset of the ISA.
In this repo, you'll find tools to assemble and disassemble LEGv8 instructions, as well as an emulator to actually test your code.

# Okay, that's awesome, but how do I use them?

I'm glad you asked! To start off, you'll need to have a copy of Cargo installed, since the LEGv8 tools are written in Rust.
You can install Cargo on all of the major platforms, though since I use Linux that is the only platform that is well-tested.
Check out https://doc.rust-lang.org/cargo/getting-started/installation.html for how to do that.

While you're waiting for Cargo to install, if you have Git installed, go ahead and clone this repository:

```
$ git clone https://git.jekel.ca/JZJ/legv8.git
Cloning into 'legv8'...
remote: Enumerating objects: 77, done.
remote: Counting objects: 100% (77/77), done.
remote: Compressing objects: 100% (72/72), done.
remote: Total 77 (delta 45), reused 0 (delta 0), pack-reused 0
Receiving objects: 100% (77/77), 13.56 KiB | 13.56 MiB/s, done.
Resolving deltas: 100% (45/45), done.
$
```

If you don't have Git, then you can download a .zip of the code from https://git.jekel.ca/JZJ/legv8/archive/main.zip.
Then extract it to a convenient directory.

Next open a terminal in the folder where you extracted or cloned the repository, and type the following command:

```
$ cargo build --release
   Compiling legv8 v0.1.0 (/ram/legv8)
    Finished release [optimized] target(s) in 7.37s
$
```

Congratulations, you've built the LEGv8 tools and are ready to use them!

## Using the Assembler

To use the assembler, from the project's root directory, type the following command:

```
$ cargo run --release --bin legv8assemble
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/legv8assemble`
legv8assemble, by JZJ :)
Copyright (C) 2023 John Jekel
See the LICENSE file at the root of the project for licensing info.

At the prompt, enter a LEGv8 instruction to assemble, or press Ctrl+C to exit...
legv8assemble>
```

You can now type in a LEGv8 instruction you'd like to assemble, for example:

```
legv8assemble> movz x1, 1234, lsl 48
  The instruction "MOVZ X1, 1234, LSL 48" is IM-type
    ________________________________________________
    |         9 |     2 |               16 |     5 | <- Field length in bits
    |----------------------------------------------|
    | 31     22 | 23 21 | 20             5 | 4   0 | <- Field start and end bit indexes (inclusive)
    |----------------------------------------------|
    | opcode    | shamt | immediate        | Rd    | <- Field name
    |----------------------------------------------|
    | 110100101 | 1   1 | 0000010011010010 | 00001 | <- Field contents
    ------------------------------------------------
  Alternatively, here is the instruction in a few, potentially more convenient formats:
    Hex (BE): 0xD2E09A41
    Bin (BE): 0b11010010111000001001101001000001
    Oct (BE): 0o32270115101
    Dec (BE): 3537934913
    Hex (LE): 0x419AE0D2
    Bin (LE): 0b1000001100110101110000011010010
    Oct (LE): 0o10146560322
    Dec (LE): 1100669138
legv8assemble>
```

The assembler is fairly intelligent, and it will catch mistakes. For example, the following instruction is valid:

```
legv8assemble> addi x1, x2, 1234
  The instruction "ADDI X1, X2, 1234" is I-type
    _____________________________________________
    |         10 |           12 |     5 |     5 | <- Field length in bits
    |-------------------------------------------|
    | 31      22 | 21        10 | 9   5 | 4   0 | <- Field start and end bit indexes (inclusive)
    |-------------------------------------------|
    | opcode     | immediate    | Rn    | Rd    | <- Field name
    |-------------------------------------------|
    | 1001000100 | 010011010010 | 00010 | 00001 | <- Field contents
    ---------------------------------------------
  Alternatively, here is the instruction in a few, potentially more convenient formats:
    Hex (BE): 0x91134841
    Bin (BE): 0b10010001000100110100100001000001
    Oct (BE): 0o22104644101
    Dec (BE): 2433960001
    Hex (LE): 0x41481391
    Bin (LE): 0b1000001010010000001001110010001
    Oct (LE): 0o10122011621
    Dec (LE): 1095242641
legv8assemble>
```

However, since I-type instructions only have 12 bit signed immediate fields, if you specify an immediate greater than
2047 or less than -2048, it will refuse to assemble the instruction:

```
legv8assemble> addi x1, x2, 9000
  Hmm, it seems that the instruction you entered isn't valid. Give it another go!
legv8assemble>
```

When you've finished using the assembler, just press Ctrl+C to exit!

```
legv8assemble> ^C
$
```

## Disassembler

Coming soon!

## Emulator

Coming soon!

# Useful Links

<a href="https://git.jekel.ca/JZJ/legv8">Click here to visit the LEGv8 Git repository!</a>.

You can also visit the <a href="https://github.com/JZJisawesome/legv8">Github</a> or <a href="https://gitlab.com/JZJisawesome/legv8">GitLab</a> mirrors to leave issues!

# Dependencies

None other than the standard libraries!

# Code and Documentation Licence

Copyright (c) 2023 John Jekel

MIT Licensed (see the LICENSE file for details)
