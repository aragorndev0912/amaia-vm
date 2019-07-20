//!Operaciones aceptadas por la virtual machine
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io;

use crate::operation::Operation;
use crate::instruction::Instruction;

#[derive(Debug)]
struct Memory<'a> {
    accumulator:usize,
    counter:usize,
    map:&'a BTreeMap<usize, Instruction>,
    mem:HashMap<usize, usize>
}

impl Memory<'_> {
    fn new <'a>(map:&'a BTreeMap<usize, Instruction>) ->Memory {
        Memory {
            accumulator: 0,
            counter:1,
            map:map,
            mem:HashMap::new()
        }
    }

    fn next(&mut self) -> Operation {
        self.map[&self.counter].operation.clone()
    }

    fn read(&mut self) -> bool {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                let key = self.map[&self.counter].pointer;
                let string:String = String::from(buffer.get(0..(buffer.len()-2)).unwrap());
                let value = string.parse::<usize>().unwrap();
                self.mem.insert(key, value);
                self.counter +=1;
                return true;
            },
            Err(err) => println!("err: {}", err)
        }

        false
    }

    fn write(&mut self) -> bool {
        let key = self.map[&self.counter].pointer;
        match self.mem.get(&key) {
            Some(value) => {
                println!("{}", value);
                self.counter += 1;
                return true;
            },
            None => println!("err: key: {:?} value no exist.", key)
        }

        false
    }

    fn load(&mut self) -> bool {
        let dir = self.map[&self.counter].pointer;
        match self.mem.get(&dir) {
            Some(value) => {
                self.accumulator = value.clone();
                self.counter += 1;
                return true;
            },
            None => println!("error: load value in pos {}", dir)
        }

        false
    }

    fn store(&mut self) -> bool {
        let dir = self.map[&self.counter].pointer;
        match self.mem.get(&dir) {
            Some(_) => {*self.mem.get_mut(&dir).unwrap() = self.accumulator;},
            None => {self.mem.insert(dir, self.accumulator);}
        }
        self.counter += 1;
        true
    }

    fn operation(&mut self, ope:&Operation) -> bool {
        let dir = self.map[&self.counter].pointer;
        match self.mem.get(&dir) {
            Some(value) => {
                match ope {
                    Operation::Add => self.accumulator += value,
                    Operation::Sub => self.accumulator -= value,
                    Operation::Div => self.accumulator /= value,
                    Operation::Mul => self.accumulator *= value,
                    _ => { return false }
                }

                *self.mem.get_mut(&dir).unwrap() = self.accumulator;
                self.counter +=1;
                return true;
            },
            None => println!("error: operation {:?}", ope)
        }

        false
    }
}

pub fn run<'a>(map:&'a BTreeMap<usize, Instruction>) {
    let mut memory = Memory::new(&map);
    let mut is_running = true;

    while is_running {
        match memory.next() {
            Operation::Read => is_running = memory.read(),
            Operation::Write => is_running = memory.write(),

            Operation::Load => is_running = memory.load(),
            Operation::Store => is_running = memory.store(),

            Operation::Add => is_running = memory.operation(&Operation::Add),
            Operation::Sub => is_running = memory.operation(&Operation::Sub),
            Operation::Div => is_running = memory.operation(&Operation::Div),
            Operation::Mul => is_running = memory.operation(&Operation::Mul),

            Operation::Jump => memory.counter +=1,
            Operation::JumpNeg => memory.counter +=1,
            Operation::JumpZero => memory.counter +=1,
            Operation::Stop => is_running = false
        }
    }
}
