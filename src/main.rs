/**
 * author: Franklin Morales (Aragorn)
 * July 2019
 */

mod instruction;

fn main() {
    let memory = instruction::load("sbasic/add.sb");
    println!("{:#?}", memory);
}