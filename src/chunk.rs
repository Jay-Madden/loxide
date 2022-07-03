use crate::bytecode::OpCode;
use crate::debug::generate_debug_line;

pub type Value = f64;

pub struct Chunk {
    pub lines: Vec<i32>,
    pub code: Vec<OpCode>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            lines: vec![],
            code: vec![],
            constants: vec![],
        }
    }

    pub fn push_op(&mut self, op: OpCode, line_num: i32) {
        self.code.push(op);
        self.lines.push(line_num);
    }

    pub fn add_constant(&mut self, value: Value) -> i8 {
        self.constants.push(value);
        self.constants.len() as i8 - 1
    }

    pub fn add_constant_large(&mut self, value: Value) -> i32 {
        self.constants.push(value);
        self.constants.len() as i32 - 1
    }

    pub fn read_op(&self, ip: usize) -> OpCode {
        self.code[ip]
    }

    pub fn get_constant(&self, index: i8) -> Value {
        self.constants[index as usize]
    }

    pub fn get_constant_large(&self, index: i32) -> Value {
        self.constants[index as usize]
    }

    pub fn disassemble_chunk(&self) {
        println!("====== Chunk Debug =======");
        for (i, op) in self.code.iter().enumerate() {
            println!("{}", generate_debug_line(self, i, *op));
        }
        println!("==========================");
        println!();
    }
}
