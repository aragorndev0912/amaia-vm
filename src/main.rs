/**
 * author: Franklin Morales (Aragorn)
 * July 2019
 */
use std::env;

mod instruction;
mod memory;
mod operation;

fn main() {
    let args:Vec<String> = env::args().collect(); 
    let path:String = if args.len() == 2 { args[1].clone() } else { String::from("avm/add.am") };

    match instruction::load(&path) {
        Ok(memory) => memory::run(&memory),
        Err(why) => println!("{}", why)
    }
}
