/**
 * author: Franklin Morales (Aragorn)
 * July 2019
 */

mod instruction;
mod memory;
mod operation;

fn main() {
    let memory = instruction::load("avm/add.am");
    memory::run(&memory);
}
