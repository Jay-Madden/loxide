use crate::{Chunk, OpCode};

pub fn disassemble_op(chunk: &Chunk, op: OpCode) -> String {
    match op {
        OpCode::RETURN => String::from("RETURN"),
        OpCode::LOAD_CONST { constant_index } => format!(
            "LOAD_CONST {:>4} (Value: {})",
            constant_index,
            chunk.get_constant(constant_index)
        ),
        OpCode::LOAD_CONST_LARGE { constant_index } => format!(
            "LOAD_CONST_LARGE {:>4} (Value: {})",
            constant_index,
            chunk.get_constant_large(constant_index)
        ),
        OpCode::NEGATE => String::from("NEGATE"),
        OpCode::ADD => String::from("ADD"),
        OpCode::SUBTRACT => String::from("SUBTRACT"),
        OpCode::MULTIPLY => String::from("MULTIPLY"),
        OpCode::DIVIDE => String::from("DIVIDE"),
    }
}

pub fn get_line_info(chunk: &Chunk, offset: usize) -> String {
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        return String::from("  |");
    }

    return chunk.lines[offset].to_string();
}

pub fn generate_debug_line(chunk: &Chunk, i: usize, op: OpCode) -> String {
    format!(
        "{:#08x} {:>4} {}",
        i,
        get_line_info(chunk, i),
        disassemble_op(chunk, op)
    )
}
