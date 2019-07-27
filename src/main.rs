/**
 * author: Franklin Morales (Aragorn)
 * July 2019
 */
use std::env;
use std::path::Path;

mod instruction;
mod memory;
mod operation;

fn run(args:& Vec<String>) -> Result<bool, String> {
    if args.len() != 2 {
        return Err("error: avm need one param for execute.".to_string());
    }
    
    let file_dir = args[1].clone();
    let path = Path::new(&file_dir);
    let extension = match path.extension() {
        Some(value) => {
            value
        },
        None => return Err("error: most exist one extension file".to_string())
    };

    if extension != "avm" {
        return Err("error: the file most be extension '.avm'".to_string());
    }

    let display = path.display();
    match instruction::load(&display.to_string()) {
        Ok(memory) => memory::run(&memory),
        Err(why) => println!("{}", why)
    }

    Ok(true)
}

fn main() {
    let args:Vec<String> = env::args().collect();
    match run(&args) {
        Ok(_) => println!("compilation success!"),
        Err(e) => println!("{}", e)
    }
}
