use std::fmt;
use std::fmt::Formatter;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum OpCode {
    RETURN,
    LOAD_CONST { constant: i8 },
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            OpCode::RETURN => write!(f, "RETURN"),
            OpCode::LOAD_CONST { constant } => write!(f, "LOAD_CONST {}", constant),
        }
    }
}
