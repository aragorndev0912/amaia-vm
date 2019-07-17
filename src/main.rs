/**
 * author: Franklin Morales (Aragorn)
 * July 2019
 */

mod instruction;
mod memory;
use memory::Memory;

fn main() {
    let memory = instruction::load("avm/add.am");
    Memory::execute(&memory);
}