use std::fmt;
use std::fmt::Formatter;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum OpCode {
    RETURN,

    LOAD_CONST { constant_index: i8 },
    LOAD_CONST_LARGE { constant_index: i32 },

    NEGATE,
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}
