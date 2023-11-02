#![allow(non_snake_case)] // haha got rid of the dumb warning
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::prelude::*;
// use std::ops::Deref;
use std::process;

// Main function
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Argument expected in file. Please give argument.");
        process::exit(0x0100);
    }
    let contents = getContents(args);
    let processedContents = processFile(contents);
    let binary = compile(processedContents);
    writeBinary(binary);
}

// Output file
fn writeBinary(bin: [u16; 32]) {
    let file = fs::File::create("out.bin");
    let mut binArray: [u8; 64] = [0; 64];
    let mut i = 0;
    for byt in bin.iter() {
        binArray[i] = byt.to_be_bytes()[0];
        binArray[i + 1] = byt.to_be_bytes()[1];
        i += 2;
    }
    file.unwrap().write_all(&binArray.as_slice());
}

// Compile files
fn compile(buf: Vec<Vec<String>>) -> [u16; 32] {
    let mut regA: u16;
    let mut regB: u16;

    let mut bin: [u16; 32] = [0; 32];
    let mut labels: HashMap<String, u16> = HashMap::new();
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    for line in buf.iter() {
        let instruction = line.get(0).unwrap();

        if instruction.chars().last().unwrap() == ':' {
            let chopped = &instruction[0..instruction.len() - 1].to_string(); // Out of bounds on zero error
            labels.insert(chopped.clone(), j);
            j -= 1;
        }
        j += 1;
    }

    // Main instruction parser
    for line in buf.iter() {
        // Get lines
        let instruction = line.get(0).unwrap();
        
        // INSTRUCTION: NOP
        if instruction == "NOP" || instruction == "nop" {
            bin[i as usize] = 0x0000;
        } 

        // INSTRUCTION: ADD
        else if instruction == "ADD" || instruction == "add" {
            regA = 0;
            regB = 0;
            
            setRegister(line.get(1).unwrap().to_uppercase());
            setRegister(line.get(2).unwrap().to_uppercase());
            
            bin[i as usize] = (0x0001 << 12) | (regA << 8) | (regB << 4);
        } 

        // INSTRUCTION: LDI
        else if instruction == "LDI" || instruction == "ldi" {
            regA = 0;
            
            let mut immediate: u16 = line.get(2).unwrap().parse().unwrap();
            immediate = immediate & 0x00FF;

            setRegister(line.get(1).unwrap().to_uppercase());

            bin[i as usize] = (0x0002 << 12) | (regA << 8) | immediate;
        } 
        
        // INSTRUCTION: SUB
        else if instruction == "SUB" || instruction == "sub" {
            regA = 0;
            regB = 0;

            setRegister(line.get(1).unwrap().to_uppercase());
            setRegister(line.get(2).unwrap().to_uppercase());

            bin[i as usize] = (0x0003 << 12) | (regA << 8) | (regB << 4);
        } 

        // INSTRUCTION: INV
        else if instruction == "INV" || instruction == "inv" {
            regA = 0;

            setRegister(line.get(1).unwrap().to_uppercase());

            bin[i as usize] = (0x0004 << 12) | (regA << 8);
        } 
        
        // INSTRUCTION: AND
        else if instruction == "AND" || instruction == "and" {
            regA = 0;
            regB = 0;

            setRegister(line.get(1).unwrap().to_uppercase());
            setRegister(line.get(2).unwrap().to_uppercase());

            bin[i as usize] = (0x0005 << 12) | (regA << 8) | (regB << 4);
        } 
        
        // INSTRUCTION: OR
        else if instruction == "OR" || instruction == "or" {
            regA = 0;
            regB = 0;

            setRegister(line.get(1).unwrap().to_uppercase());
            setRegister(line.get(2).unwrap().to_uppercase());

            bin[i as usize] = (0x0006 << 12) | (regA << 8) | (regB << 4);
        } 
        
        // INSTRUCTION: XOR
        else if instruction == "XOR" || instruction == "xor" {
            regA = 0;
            regB = 0;

            setRegister(line.get(1).unwrap().to_uppercase());
            setRegister(line.get(2).unwrap().to_uppercase());

            bin[i as usize] = (0x0007 << 12) | (regA << 8) | (regB << 4);
        } 
        
        // INSTRUCTION: MOV
        else if instruction == "MOV" || instruction == "mov" {
            regA = 0;
            regB = 0;

            setRegister(line.get(1).unwrap().to_uppercase());
            setRegister(line.get(2).unwrap().to_uppercase());

            bin[i as usize] = (0x0008 << 12) | (regA << 8) | (regB << 4);
        } 
        
        // INSTRUCTION: SR
        else if instruction == "SR" || instruction == "sr" {
            regA = 0;
            regB = 0;

            setRegister(line.get(1).unwrap().to_uppercase());
            setRegister(line.get(2).unwrap().to_uppercase());

            bin[i as usize] = (0x0009 << 12) | (regA << 8) | (regB << 4);
        } 
        
        // INSTRUCTION: SL
        else if instruction == "SL" || instruction == "sl" {
            regA = 0;
            regB = 0;

            setRegister(line.get(1).unwrap().to_uppercase());
            setRegister(line.get(2).unwrap().to_uppercase());
            
            bin[i as usize] = (0x000A << 12) | (regA << 8) | (regB << 4);
        } 
        
        // INSTRUCTION: IN
        else if instruction == "IN" || instruction == "in" {
            regA = 0;

            setRegister(line.get(1).unwrap().to_uppercase());
            
            let mut immediate: u16 = line.get(2).unwrap().parse().unwrap();
            immediate = immediate & 0x00FF;
            bin[i as usize] = (0x000B << 12) | (regA << 8) | (immediate << 4);
        } 
        
        // INSTRUCTION: OUT
        else if instruction == "OUT" || instruction == "out" {
            regA = 0;

            setRegister(line.get(1).unwrap().to_uppercase());
            
            let mut immediate: u16 = line.get(2).unwrap().parse().unwrap();
            immediate = immediate & 0x00FF;
            bin[i as usize] = (0x000C << 12) | (regA << 8) | immediate;
        } 
        
        // INSTRUCTION: JZ
        else if instruction == "JZ" || instruction == "jz" {
            regA = 0;

            setRegister(line.get(1).unwrap().to_uppercase());

            let label = labels.get(line.get(2).unwrap()).unwrap();
            bin[i as usize] = (0x000D << 12) | (regA << 8) | label;
        } 
        
        // INSTRUCTION: JLT
        else if instruction == "JLT" || instruction == "jlt" {
            regA = 0;
            let label = labels.get(line.get(2).unwrap()).unwrap();
            
            setRegister(line.get(1).unwrap().to_uppercase());

            bin[i as usize] = (0x000E << 12) | (regA << 8) | label;
        } 
        
        // INSTRUCTION: J
        else if instruction == "J" || instruction == "j" {
            let label = labels.get(line.get(1).unwrap()).unwrap();
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

// File I/O
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

// Get file contents
fn getContents(args: Vec<String>) -> String {
    let contents = fs::read_to_string(args.get(1).unwrap()).expect("could not open file");
    contents
}

// Retruns register based on string
fn setRegister(buf: String) -> u8 {
    let regA: u8;
    let strBuff = buf.as_str();

    match strBuff {
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

    regA
}
