use crate::chunk::Value;
use crate::debug::generate_debug_line;
use crate::{Chunk, OpCode};

macro_rules! binary_op {
    ($self:ident, $op:tt) => {
        let b = $self.stack.pop().unwrap();
        let a = $self.stack.pop().unwrap();

        $self.stack.push(a $op b);
    };
}

pub enum ExecStatus {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

pub struct Vm<'a> {
    chunk: &'a Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl<'a> Vm<'a> {
    pub fn new(chunk: &'a Chunk) -> Vm {
        Vm {
            chunk,
            ip: 0,
            stack: vec![],
        }
    }

    pub fn exec(&mut self) -> ExecStatus {
        self.run()
    }

    fn run(&mut self) -> ExecStatus {
        #[cfg(feature = "vm_debug_trace")]
        println!("====== Interpreter Execute =======");

        loop {
            let curr_op = self.chunk.read_op(self.ip);

            #[cfg(feature = "vm_debug_trace")]
            println!(
                "{}   {:?}",
                generate_debug_line(self.chunk, self.ip, curr_op),
                self.stack
            );

            self.ip += 1;

            match curr_op {
                OpCode::RETURN => {
                    println!("{}", self.stack.pop().unwrap());
                    return ExecStatus::InterpretOk;
                }
                OpCode::LOAD_CONST { constant_index } => {
                    let val = self.chunk.get_constant(constant_index);
                    self.stack.push(val);
                }
                OpCode::LOAD_CONST_LARGE { constant_index } => {
                    let val = self.chunk.get_constant_large(constant_index);
                    self.stack.push(val);
                }
                OpCode::NEGATE => {
                    let negated = self.stack.last_mut().unwrap();
                    *negated = -*negated;
                }
                OpCode::ADD => {
                    binary_op!(self, +);
                }
                OpCode::SUBTRACT => {
                    binary_op!(self, -);
                }
                OpCode::MULTIPLY => {
                    binary_op!(self, *);
                }
                OpCode::DIVIDE => {
                    binary_op!(self, /);
                }
            }
        }
        ExecStatus::InterpretOk
    }
}
