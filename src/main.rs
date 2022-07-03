use crate::bytecode::OpCode;
use crate::chunk::Chunk;
use crate::vm::Vm;

mod bytecode;
mod chunk;
mod debug;
mod vm;

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.push_op(
        OpCode::LOAD_CONST {
            constant_index: constant,
        },
        125,
    );

    let constant = chunk.add_constant(3.4);
    chunk.push_op(
        OpCode::LOAD_CONST {
            constant_index: constant,
        },
        125,
    );

    chunk.push_op(OpCode::ADD, 125);

    let constant = chunk.add_constant(5.6);
    chunk.push_op(
        OpCode::LOAD_CONST {
            constant_index: constant,
        },
        125,
    );
    chunk.push_op(OpCode::DIVIDE, 125);
    chunk.push_op(OpCode::NEGATE, 125);
    chunk.push_op(OpCode::RETURN, 125);

    /*
    #[cfg(feature = "vm_debug_trace")]
    chunk.disassemble_chunk();
    */

    let mut vm = Vm::new(&chunk);
    let _ = vm.exec();
}
