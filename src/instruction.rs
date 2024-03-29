use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::collections::BTreeMap;

use crate::operation::Operation;

const ZERO:usize = '0' as usize;
const NINE:usize = '9' as usize;
const TENN:usize = 'A' as usize;
const FIVETEEN:usize = 'F' as usize;

#[allow(dead_code)]
#[derive(Debug)]
/// Estrutura donde se almacena los datos de cada instrucción.
pub struct Instruction {
    pub code:String,
    pub operation:Operation,
    pub pointer:usize,
    pub signed:usize
}

impl Instruction {
    pub fn new(code:String, operation:Operation, pointer:usize, signed:usize) ->Instruction {
        Instruction{
            code: code,
            operation: operation,
            pointer: pointer,
            signed: signed
        }
    }
}
/// Se encarga del llamado a la función de lectura del fichero
/// y la función que permita la construción del árbol de instrucciones.
pub fn load<'a>(filename:&'a str) -> Result<BTreeMap<usize, Instruction>, String>{
    match read_file(&filename) {
        Ok(data) => Ok(gen_instructions(&data)),
        Err(why) => Err(why)
    }
}

/// Se encarga de convertir un número hexadecimal a un entero
/// sin signo de 8 bytes.
///
/// # Ejemplo
/// ```
/// let h:char = 'A'; //10
/// let i:usize = char_to_int(h); //i == 10
/// ```
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

/// Convierte un slide de String en un numero
/// haxedecimal.
/// 
/// # Ejemplo
/// ```
/// let h_num = "14"; // valor en hexadecimal
/// let d_num = hex_to_int(&hex_num); // d_num == 20
/// ```
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

/// Devuelve el valor correspondiente de un entero sin
/// signo a un enum de tipo `Operation`.
/// 
/// # Ejemplo
/// ```
/// let ope:Operation = int_to_operation(10 as usize); // ope == Read
/// ```
fn int_to_operation(value:& usize) -> Operation {
    match value {
        0x0A => return Operation::Read,
        0x0B => return Operation::Write,

        0x14 => return Operation::Load,
        0x15 => return Operation::Store,

        0x1E => return Operation::Add,
        0x1F => return Operation::Sub,
        0x20 => return Operation::Div,
        0x21 => return Operation::Mul,

        0x28 => return Operation::Jump,
        0x29 => return Operation::JumpNeg,
        0x2A => return Operation::JumpZero,
        _ => return Operation::Stop
    }
}

/// Permite almacenar los datos necesarios para la construción de la
/// structura Instruction y lo almacena en el árbol de instrución.
fn save_instruction<'a>(map:&mut BTreeMap<usize, Instruction>, instruction:&'a str, index:&mut usize) {
    let signed_str = instruction.get(0x0..0x1).unwrap();
    let operation_str = instruction.get(0x1..0x3).unwrap();
    let pointer_str = instruction.get(0x3..).unwrap();

    let signed = hex_to_int(&signed_str).unwrap();
    let operation = int_to_operation(&hex_to_int(&operation_str).unwrap());
    let pointer = hex_to_int(&pointer_str).unwrap();

    map.insert(*index, Instruction::new(
        instruction.to_string().clone(),
        operation,
        pointer,
        signed
    ));

    *index += 1;
}

fn gen_instructions<'a>(data:&'a str) ->BTreeMap<usize, Instruction> {
    let mut instructions:BTreeMap<usize, Instruction> = BTreeMap::new();
    let invalid_chars = ['\n', ' ', '\r'];
    let mut instruction = String::new();
    let mut index:usize = 1;

    for c in data.chars() {
        if instruction.len() > 7 && invalid_chars.contains(&c) {
            save_instruction(&mut instructions, &instruction, &mut index);
            instruction.clear();
        }
        else if !invalid_chars.contains(&c) {
            instruction.push(c);
        }
    }

    if instruction.len() > 0 {
        save_instruction(&mut instructions, &mut instruction, &mut index);
        instruction.clear();
    }
    instructions
}

fn read_file<'a>(filename:&'a str) -> Result<String, String> {
    let path = Path::new(filename);
    let mut file = match File::open(path) {
        Err(_) => return Err(format!("Error: is not possible open file: {}.", filename)),
        Ok(file) => file
    };

    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Err(_) => return Err(format!("Error: is not possible read line.")),
        Ok(_) => Ok(data)
    }
}
