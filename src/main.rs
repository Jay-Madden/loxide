use crate::bytecode::OpCode;
use crate::chunk::Chunk;

mod bytecode;
mod chunk;

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.push_op(OpCode::LOAD_CONST { constant });
    chunk.print();
}
