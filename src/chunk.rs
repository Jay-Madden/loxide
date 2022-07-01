use crate::bytecode::OpCode;

type Value = f64;

pub struct Chunk {
    code: Vec<OpCode>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: vec![],
            constants: vec![],
        }
    }

    pub fn push_op(&mut self, op: OpCode) {
        self.code.push(op);
    }

    pub fn add_constant(&mut self, value: Value) -> i8 {
        self.constants.push(value);
        return self.constants.len() as i8;
    }

    pub fn print(self) {
        for (i, op) in self.code.iter().enumerate() {
            println!("{:#08x} {}", i, op);
        }
    }
}
