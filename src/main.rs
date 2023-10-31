use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::ops::Deref;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("argument expected please give argument.");
        process::exit(0x0100);
    }
    let contents = getContents(args);
    let processedContents = processFile(contents);
    let binary = compile(processedContents);
    writeBinary(binary);
}
fn writeBinary(bin: [u16; 32]) {
    let mut file = fs::File::create("out.busm");
    let mut binArray: [u8; 64] = [0; 64];
    let mut i = 0;
    for byt in bin.iter() {
        binArray[i] = byt.to_be_bytes()[0];
        binArray[i + 1] = byt.to_be_bytes()[1];
        i += 2;
    }
    println!("{:?}", binArray);
    file.unwrap().write_all(&binArray.as_slice());
}
fn compile(buf: Vec<Vec<String>>) -> [u16; 32] {
    let mut bin: [u16; 32] = [0; 32];
    let mut labels: HashMap<String, u16> = HashMap::new();
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    for line in buf.iter() {
        let instruction = line.get(0).unwrap();

        if instruction.chars().last().unwrap() == ':' {
            let chopped = &instruction[0..instruction.len() - 1].to_string();
            labels.insert(chopped.clone(), j);
            j -= 1;
        }
        j += 1;
    }
    for line in buf.iter() {
        let instruction = line.get(0).unwrap();
        if instruction == "NOP" {
            bin[i as usize] = 0x0000;
        } else if instruction == "ADD" || instruction == "add" {
            let mut regA: u16 = 0;
            let mut regB: u16 = 0;
            match line.get(1).unwrap().as_str().to_uppercase().deref() {
                "R0" => regA = 0,
                "R1" => regA = 1,
                "R2" => regA = 2,
                "R3" => regA = 3,
                "R4" => regA = 4,
                "R5" => regA = 5,
                "R6" => regA = 6,
                "R7" => regA = 7,
                "R8" => regA = 8,
                "R9" => regA = 9,
                "R10" => regA = 10,
                "R11" => regA = 11,
                "R12" => regA = 12,
                "R13" => regA = 13,
                "R14" => regA = 14,
                "R15" => regA = 15,
                _ => {
                    println!("Not valid");
                    process::exit(0x02);
                }
            }
            match line.get(2).unwrap().as_str().to_uppercase().deref() {
                "R0" => regB = 0,
                "R1" => regB = 1,
                "R2" => regB = 2,
                "R3" => regB = 3,
                "R4" => regB = 4,
                "R5" => regB = 5,
                "R6" => regB = 6,
                "R7" => regB = 7,
                "R8" => regB = 8,
                "R9" => regB = 9,
                "R10" => regB = 10,
                "R11" => regB = 11,
                "R12" => regB = 12,
                "R13" => regB = 13,
                "R14" => regB = 14,
                "R15" => regB = 15,
                _ => {
                    println!("Not valid");
                    process::exit(0x02);
                }
            }
            bin[i as usize] = (0x0001 << 12) | (regA << 8) | (regB << 4);
        } else if instruction == "LDI" || instruction == "ldi" {
            let mut regA: u16 = 0;
            let mut immediate: u16 = line.get(2).unwrap().parse().unwrap();
            immediate = immediate & 0x00FF;

            match line.get(1).unwrap().as_str().to_uppercase().deref() {
                "R0" => regA = 0,
                "R1" => regA = 1,
                "R2" => regA = 2,
                "R3" => regA = 3,
                "R4" => regA = 4,
                "R5" => regA = 5,
                "R6" => regA = 6,
                "R7" => regA = 7,
                "R8" => regA = 8,
                "R9" => regA = 9,
                "R10" => regA = 10,
                "R11" => regA = 11,
                "R12" => regA = 12,
                "R13" => regA = 13,
                "R14" => regA = 14,
                "R15" => regA = 15,
                _ => {
                    println!("Not valid");
                    process::exit(0x02);
                }
            }
            bin[i as usize] = (0x0002 << 12) | (regA << 8) | immediate;
        } else if instruction == "SUB" || instruction == "sub" {
            let mut regA: u16 = 0;
            let mut regB: u16 = 0;
            match line.get(1).unwrap().as_str().to_uppercase().deref() {
                "R0" => regA = 0,
                "R1" => regA = 1,
                "R2" => regA = 2,
                "R3" => regA = 3,
                "R4" => regA = 4,
                "R5" => regA = 5,
                "R6" => regA = 6,
                "R7" => regA = 7,
                "R8" => regA = 8,
                "R9" => regA = 9,
                "R10" => regA = 10,
                "R11" => regA = 11,
                "R12" => regA = 12,
                "R13" => regA = 13,
                "R14" => regA = 14,
                "R15" => regA = 15,
                _ => {
                    println!("Not valid");
                    process::exit(0x02);
                }
            }
            match line.get(2).unwrap().as_str().to_uppercase().deref() {
                "R0" => regB = 0,
                "R1" => regB = 1,
                "R2" => regB = 2,
                "R3" => regB = 3,
                "R4" => regB = 4,
                "R5" => regB = 5,
                "R6" => regB = 6,
                "R7" => regB = 7,
                "R8" => regB = 8,
                "R9" => regB = 9,
                "R10" => regB = 10,
                "R11" => regB = 11,
                "R12" => regB = 12,
                "R13" => regB = 13,
                "R14" => regB = 14,
                "R15" => regB = 15,
                _ => {
                    println!("Not valid");
                    process::exit(0x02);
                }
            }
            bin[i as usize] = (0x0003 << 12) | (regA << 8) | (regB << 4);
        } else if instruction == "INV" || instruction == "inv" {
            let mut regA: u16 = 0;
            match line.get(1).unwrap().as_str().to_uppercase().deref() {
                "R0" => regA = 0,
                "R1" => regA = 1,
                "R2" => regA = 2,
                "R3" => regA = 3,
                "R4" => regA = 4,
                "R5" => regA = 5,
                "R6" => regA = 6,
                "R7" => regA = 7,
                "R8" => regA = 8,
                "R9" => regA = 9,
                "R10" => regA = 10,
                "R11" => regA = 11,
                "R12" => regA = 12,
                "R13" => regA = 13,
                "R14" => regA = 14,
                "R15" => regA = 15,
                _ => {
                    println!("Not valid");
                    process::exit(0x02);
                }
            }
            bin[i as usize] = (0x0004 << 12) | (regA << 8);
        } else if instruction == "AND" || instruction == "and" {
            let mut regA: u16 = 0;
            let mut regB: u16 = 0;
            match line.get(1).unwrap().as_str().to_uppercase().deref() {
                "R0" => regA = 0,
                "R1" => regA = 1,
                "R2" => regA = 2,
                "R3" => regA = 3,
                "R4" => regA = 4,
                "R5" => regA = 5,
                "R6" => regA = 6,
                "R7" => regA = 7,
                "R8" => regA = 8,
                "R9" => regA = 9,
                "R10" => regA = 10,
                "R11" => regA = 11,
                "R12" => regA = 12,
                "R13" => regA = 13,
                "R14" => regA = 14,
                "R15" => regA = 15,
                _ => {
                    println!("Not valid");
                    process::exit(0x02);
                }
            }
            match line.get(2).unwrap().as_str().to_uppercase().deref() {
                "R0" => regB = 0,
                "R1" => regB = 1,
                "R2" => regB = 2,
                "R3" => regB = 3,
                "R4" => regB = 4,
                "R5" => regB = 5,
                "R6" => regB = 6,
                "R7" => regB = 7,
                "R8" => regB = 8,
                "R9" => regB = 9,
                "R10" => regB = 10,
                "R11" => regB = 11,
                "R12" => regB = 12,
                "R13" => regB = 13,
                "R14" => regB = 14,
                "R15" => regB = 15,
                _ => {
                    println!("Not valid");
                    process::exit(0x02);
                }
            }
            bin[i as usize] = (0x0005 << 12) | (regA << 8) | (regB << 4);
        } else if instruction == "OR" || instruction == "or" {
            let mut regA: u16 = 0;
            let mut regB: u16 = 0;
            match line.get(1).unwrap().as_str().to_uppercase().deref() {
                "R0" => regA = 0,
                "R1" => regA = 1,
                "R2" => regA = 2,
                "R3" => regA = 3,
                "R4" => regA = 4,
                "R5" => regA = 5,
                "R6" => regA = 6,
                "R7" => regA = 7,
                "R8" => regA = 8,
                "R9" => regA = 9,
                "R10" => regA = 10,
                "R11" => regA = 11,
                "R12" => regA = 12,
                "R13" => regA = 13,
                "R14" => regA = 14,
                "R15" => regA = 15,
                _ => {
                    println!("Not valid");
                    process::exit(0x02);
                }
            }
            match line.get(2).unwrap().as_str().to_uppercase().deref() {
                "R0" => regB = 0,
                "R1" => regB = 1,
                "R2" => regB = 2,
                "R3" => regB = 3,
                "R4" => regB = 4,
                "R5" => regB = 5,
                "R6" => regB = 6,
                "R7" => regB = 7,
                "R8" => regB = 8,
                "R9" => regB = 9,
                "R10" => regB = 10,
                "R11" => regB = 11,
                "R12" => regB = 12,
                "R13" => regB = 13,
                "R14" => regB = 14,
                "R15" => regB = 15,
                _ => {
                    println!("Not valid");
                    process::exit(0x02);
                }
            }
            bin[i as usize] = (0x0006 << 12) | (regA << 8) | (regB << 4);
        } else if instruction == "XOR" || instruction == "xor" {
            let mut regA: u16 = 0;
            let mut regB: u16 = 0;
            match line.get(1).unwrap().as_str().to_uppercase().deref() {
                "R0" => regA = 0,
                "R1" => regA = 1,
                "R2" => regA = 2,
                "R3" => regA = 3,
                "R4" => regA = 4,
                "R5" => regA = 5,
                "R6" => regA = 6,
                "R7" => regA = 7,
                "R8" => regA = 8,
                "R9" => regA = 9,
                "R10" => regA = 10,
                "R11" => regA = 11,
                "R12" => regA = 12,
                "R13" => regA = 13,
                "R14" => regA = 14,
                "R15" => regA = 15,
                _ => {
                    println!("Not valid");
                    process::exit(0x02);
                }
            }
            match line.get(2).unwrap().as_str().to_uppercase().deref() {
                "R0" => regB = 0,
                "R1" => regB = 1,
                "R2" => regB = 2,
                "R3" => regB = 3,
                "R4" => regB = 4,
                "R5" => regB = 5,
                "R6" => regB = 6,
                "R7" => regB = 7,
                "R8" => regB = 8,
                "R9" => regB = 9,
                "R10" => regB = 10,
                "R11" => regB = 11,
                "R12" => regB = 12,
                "R13" => regB = 13,
                "R14" => regB = 14,
                "R15" => regB = 15,
                _ => {
                    println!("Not valid");
                    process::exit(0x02);
                }
            }
            bin[i as usize] = (0x0007 << 12) | (regA << 8) | (regB << 4);
        } else if instruction == "MOV" || instruction == "mov" {
            let mut regA: u16 = 0;
            let mut regB: u16 = 0;
            match line.get(1).unwrap().as_str().to_uppercase().deref() {
                "R0" => regA = 0,
                "R1" => regA = 1,
                "R2" => regA = 2,
                "R3" => regA = 3,
                "R4" => regA = 4,
                "R5" => regA = 5,
                "R6" => regA = 6,
                "R7" => regA = 7,
                "R8" => regA = 8,
                "R9" => regA = 9,
                "R10" => regA = 10,
                "R11" => regA = 11,
                "R12" => regA = 12,
                "R13" => regA = 13,
                "R14" => regA = 14,
                "R15" => regA = 15,
                _ => {
                    println!("Not valid");
                    process::exit(0x02);
                }
            }
            match line.get(2).unwrap().as_str().to_uppercase().deref() {
                "R0" => regB = 0,
                "R1" => regB = 1,
                "R2" => regB = 2,
                "R3" => regB = 3,
                "R4" => regB = 4,
                "R5" => regB = 5,
                "R6" => regB = 6,
                "R7" => regB = 7,
                "R8" => regB = 8,
                "R9" => regB = 9,
                "R10" => regB = 10,
                "R11" => regB = 11,
                "R12" => regB = 12,
                "R13" => regB = 13,
                "R14" => regB = 14,
                "R15" => regB = 15,
                _ => {
                    println!("Not valid");
                    process::exit(0x02);
                }
            }
            bin[i as usize] = (0x0008 << 12) | (regA << 8) | (regB << 4);
        } else if instruction == "SR" || instruction == "sr" {
            let mut regA: u16 = 0;
            let mut regB: u16 = 0;
            match line.get(1).unwrap().as_str().to_uppercase().deref() {
                "R0" => regA = 0,
                "R1" => regA = 1,
                "R2" => regA = 2,
                "R3" => regA = 3,
                "R4" => regA = 4,
                "R5" => regA = 5,
                "R6" => regA = 6,
                "R7" => regA = 7,
                "R8" => regA = 8,
                "R9" => regA = 9,
                "R10" => regA = 10,
                "R11" => regA = 11,
                "R12" => regA = 12,
                "R13" => regA = 13,
                "R14" => regA = 14,
                "R15" => regA = 15,
                _ => {
                    println!("Not valid");
                    process::exit(0x02);
                }
            }
            match line.get(2).unwrap().as_str().to_uppercase().deref() {
                "R0" => regB = 0,
                "R1" => regB = 1,
                "R2" => regB = 2,
                "R3" => regB = 3,
                "R4" => regB = 4,
                "R5" => regB = 5,
                "R6" => regB = 6,
                "R7" => regB = 7,
                "R8" => regB = 8,
                "R9" => regB = 9,
                "R10" => regB = 10,
                "R11" => regB = 11,
                "R12" => regB = 12,
                "R13" => regB = 13,
                "R14" => regB = 14,
                "R15" => regB = 15,
                _ => {
                    println!("Not valid");
                    process::exit(0x02);
                }
            }
            bin[i as usize] = (0x0009 << 12) | (regA << 8) | (regB << 4);
        } else if instruction == "SL" || instruction == "sl" {
            let mut regA: u16 = 0;
            let mut regB: u16 = 0;
            match line.get(1).unwrap().as_str().to_uppercase().deref() {
                "R0" => regA = 0,
                "R1" => regA = 1,
                "R2" => regA = 2,
                "R3" => regA = 3,
                "R4" => regA = 4,
                "R5" => regA = 5,
                "R6" => regA = 6,
                "R7" => regA = 7,
                "R8" => regA = 8,
                "R9" => regA = 9,
                "R10" => regA = 10,
                "R11" => regA = 11,
                "R12" => regA = 12,
                "R13" => regA = 13,
                "R14" => regA = 14,
                "R15" => regA = 15,
                _ => {
                    println!("Not valid");
                    process::exit(0x02);
                }
            }
            match line.get(2).unwrap().as_str().to_uppercase().deref() {
                "R0" => regB = 0,
                "R1" => regB = 1,
                "R2" => regB = 2,
                "R3" => regB = 3,
                "R4" => regB = 4,
                "R5" => regB = 5,
                "R6" => regB = 6,
                "R7" => regB = 7,
                "R8" => regB = 8,
                "R9" => regB = 9,
                "R10" => regB = 10,
                "R11" => regB = 11,
                "R12" => regB = 12,
                "R13" => regB = 13,
                "R14" => regB = 14,
                "R15" => regB = 15,
                _ => {
                    println!("Not valid");
                    process::exit(0x02);
                }
            }
            bin[i as usize] = (0x000A << 12) | (regA << 8) | (regB << 4);
        } else if instruction == "IN" || instruction == "in" {
            let mut regA: u16 = 0;
            match line.get(1).unwrap().as_str().to_uppercase().deref() {
                "R0" => regA = 0,
                "R1" => regA = 1,
                "R2" => regA = 2,
                "R3" => regA = 3,
                "R4" => regA = 4,
                "R5" => regA = 5,
                "R6" => regA = 6,
                "R7" => regA = 7,
                "R8" => regA = 8,
                "R9" => regA = 9,
                "R10" => regA = 10,
                "R11" => regA = 11,
                "R12" => regA = 12,
                "R13" => regA = 13,
                "R14" => regA = 14,
                "R15" => regA = 15,
                _ => {
                    println!("Not valid");
                    process::exit(0x02);
                }
            }
            let mut immediate: u16 = line.get(2).unwrap().parse().unwrap();
            immediate = immediate & 0x00FF;
            bin[i as usize] = (0x000B << 12) | (regA << 8) | immediate;
        } else if instruction == "OUT" || instruction == "out" {
            let mut regA: u16 = 0;
            match line.get(1).unwrap().as_str().to_uppercase().deref() {
                "R0" => regA = 0,
                "R1" => regA = 1,
                "R2" => regA = 2,
                "R3" => regA = 3,
                "R4" => regA = 4,
                "R5" => regA = 5,
                "R6" => regA = 6,
                "R7" => regA = 7,
                "R8" => regA = 8,
                "R9" => regA = 9,
                "R10" => regA = 10,
                "R11" => regA = 11,
                "R12" => regA = 12,
                "R13" => regA = 13,
                "R14" => regA = 14,
                "R15" => regA = 15,
                _ => {
                    println!("Not valid");
                    process::exit(0x02);
                }
            }
            let mut immediate: u16 = line.get(2).unwrap().parse().unwrap();
            immediate = immediate & 0x00FF;
            bin[i as usize] = (0x000C << 12) | (regA << 8) | immediate;
        } else if instruction == "JZ" || instruction == "jz" {
            let mut regA: u16 = 0;
            match line.get(1).unwrap().as_str().to_uppercase().deref() {
                "R0" => regA = 0,
                "R1" => regA = 1,
                "R2" => regA = 2,
                "R3" => regA = 3,
                "R4" => regA = 4,
                "R5" => regA = 5,
                "R6" => regA = 6,
                "R7" => regA = 7,
                "R8" => regA = 8,
                "R9" => regA = 9,
                "R10" => regA = 10,
                "R11" => regA = 11,
                "R12" => regA = 12,
                "R13" => regA = 13,
                "R14" => regA = 14,
                "R15" => regA = 15,
                _ => {
                    println!("Not valid");
                    process::exit(0x02);
                }
            }
            let label = labels.get(line.get(2).unwrap()).unwrap();
            bin[i as usize] = (0x000D << 12) | (regA << 8) | label;
        } else if instruction == "JLT" || instruction == "jlt" {
            let mut regA: u16 = 0;
            match line.get(1).unwrap().as_str().to_uppercase().deref() {
                "R0" => regA = 0,
                "R1" => regA = 1,
                "R2" => regA = 2,
                "R3" => regA = 3,
                "R4" => regA = 4,
                "R5" => regA = 5,
                "R6" => regA = 6,
                "R7" => regA = 7,
                "R8" => regA = 8,
                "R9" => regA = 9,
                "R10" => regA = 10,
                "R11" => regA = 11,
                "R12" => regA = 12,
                "R13" => regA = 13,
                "R14" => regA = 14,
                "R15" => regA = 15,
                _ => {
                    println!("Not valid");
                    process::exit(0x02);
                }
            }
            let label = labels.get(line.get(2).unwrap()).unwrap();
            bin[i as usize] = (0x000E << 12) | (regA << 8) | label;
        } else if instruction == "J" || instruction == "j" {
            let label = labels.get(line.get(1).unwrap()).unwrap();
            println!("{}", label);
            bin[i as usize] = (0x000F << 12) | label;
        } else {
            if instruction.chars().last().unwrap() != ':' {
                println!("Invalid instruction: {:?}", instruction);
                process::exit(0x1111);
            } else {
                i -= 1;
            }
        }
        i += 1;
    }
    bin
}

fn processFile(buf: String) -> Vec<Vec<String>> {
    let mut retValue: Vec<Vec<String>> = Vec::new();

    for line in buf.lines() {
        let linetemp: Vec<&str> = line.split(" ").collect();
        let mut tempVec: Vec<String> = Vec::new();
        for element in linetemp.iter() {
            tempVec.push(element.to_string());
        }
        retValue.push(tempVec.clone());
    }
    retValue
}

fn getContents(args: Vec<String>) -> String {
    let contents = fs::read_to_string(args.get(1).unwrap()).expect("could not open file");
    contents
}
