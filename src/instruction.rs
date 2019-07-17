use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::collections::BTreeMap;

const ZERO:usize = '0' as usize;
const NINE:usize = '9' as usize;
const TENN:usize = 'A' as usize;
const FIVETEEN:usize = 'F' as usize;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Instruction {
    code:String,
    operation:usize,
    pointer:usize,
    signed:usize
}

impl Instruction {
    pub fn new(code:String, operation:usize, pointer:usize, signed:usize) ->Instruction {
        Instruction{
            code: code,
            operation: operation,
            pointer: pointer,
            signed: signed,
        }
    }
}

fn char_to_int(c:char) -> Option<usize> {
    let num = c as usize;
    if num >= ZERO && num <= NINE {
        return Some(num - ZERO);
    }
    else if num >= TENN && num <= FIVETEEN {
        return Some((num - TENN) + 0xA);
    }
    None
}

fn hex_to_int<'a>(code:&'a str) -> Option<usize> {
    let mut val:usize = 0x0;
    for c in code.chars() {
        match char_to_int(c) {
            Some(re) => val = re + (0x10 * val),
            None => return None
        }
    }
    Some(val)
}

pub fn load(filename:&'static str) -> BTreeMap<usize, Instruction>{
    let data = read_file(&filename); 
    gen_instructions(&data)
}

fn save_instruction<'a>(map:&mut BTreeMap<usize, Instruction>, instruction:&'a str) {
    let signed_str = instruction.get(0x0..0x1).unwrap();
    let operation_str = instruction.get(0x1..0x3).unwrap();
    let direction_str = instruction.get(0x3..0xB).unwrap();
    let pointer_str = instruction.get(0xB..).unwrap();
    
    let signed = hex_to_int(&signed_str).unwrap();
    let operation = hex_to_int(&operation_str).unwrap();
    let pointer = hex_to_int(&pointer_str).unwrap();
    let direction = hex_to_int(&direction_str).unwrap();

    map.insert(direction, Instruction::new(
        instruction.to_string().clone(), 
        operation, 
        pointer, 
        signed
    ));
}

fn gen_instructions<'a>(data:&'a str) ->BTreeMap<usize, Instruction> {
    let mut instructions:BTreeMap<usize, Instruction> = BTreeMap::new();
    let invalid_chars = ['\n', ' ', '\r'];
    let mut instruction = String::new();

    for c in data.chars() {
        if instruction.len() > 7 && invalid_chars.contains(&c) {
            save_instruction(&mut instructions, &instruction);
            instruction.clear();
        } 
        else if !invalid_chars.contains(&c) {
            instruction.push(c);
        }
    }

    if instruction.len() > 0 {
        save_instruction(&mut instructions, &mut instruction);
        instruction.clear();
    }
    instructions
}

fn read_file(filename:&'static str) -> String {
    let path = Path::new(filename);
    let mut file = match File::open(path) {
        Err(why) => panic!("{}", why),
        Ok(file) => file
    };

    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Err(why) => panic!("{}", why),
        Ok(_) => data
    }
}