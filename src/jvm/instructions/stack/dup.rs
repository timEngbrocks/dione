use crate::{
    class_loader::parser::{Parser, U2},
    jvm::{
        frame::Frame,
        instructions::{Instruction, InstructionResult},
        types::Types, runtime_constant_pool::RuntimeConstantPool,
    },
    opcodes,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DUP {}
impl Instruction for DUP {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::DUP));
        DUP {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context.stack.pop();
        match value {
            Types::Double(_) | Types::Long(_) => panic!("Can not DUP a Double or Long"),
            _ => {
                execution_context.stack.push(value.clone());
                execution_context.stack.push(value);
            }
        }
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("dup")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DUP_X1 {}
impl Instruction for DUP_X1 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::DUP_X1));
        DUP_X1 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value1 = execution_context.stack.pop();
        let value2 = execution_context.stack.pop();
        match value1 {
            Types::Double(_) | Types::Long(_) => panic!("Can not DUP_X1 a Double or Long"),
            _ => {
                execution_context.stack.push(value1.clone());
                execution_context.stack.push(value2);
                execution_context.stack.push(value1);
            }
        }
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("dup_x1")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DUP_X2 {}
impl Instruction for DUP_X2 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::DUP_X2));
        DUP_X2 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value1 = execution_context.stack.pop();
        let value2 = execution_context.stack.pop();
        match value1 {
            Types::Double(_) | Types::Long(_) => {
                panic!("DUP_X2: value1 can not be a Double or Long")
            }
            _ => {
                let value3 = execution_context.stack.pop();
                match value2 {
                    Types::Double(_) | Types::Long(_) => {
                        execution_context.stack.push(value3);
                        execution_context.stack.push(value1.clone());
                        execution_context.stack.push(value2);
                        execution_context.stack.push(value1);
                    }
                    _ => {
                        execution_context.stack.push(value1.clone());
                        execution_context.stack.push(value3);
                        execution_context.stack.push(value2);
                        execution_context.stack.push(value1);
                    }
                }
            }
        }
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("dup_x2")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DUP2 {}
impl Instruction for DUP2 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::DUP2));
        DUP2 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value1 = execution_context.stack.pop();
        match value1 {
            Types::Double(_) | Types::Long(_) => {
                execution_context.stack.push(value1.clone());
                execution_context.stack.push(value1);
            }
            _ => {
                let value2 = execution_context.stack.pop();
                match value2 {
					Types::Double(_) | Types::Long(_) => panic!("DUP2: If value1 is not a Double or Long, value2 can not be a Double or Long"),
					_ => {
						execution_context.stack.push(value2.clone());
						execution_context.stack.push(value1.clone());
						execution_context.stack.push(value2);
						execution_context.stack.push(value1);
					}
				}
            }
        }
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("dup2")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DUP2_X1 {}
impl Instruction for DUP2_X1 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::DUP2_X1));
        DUP2_X1 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value1 = execution_context.stack.pop();
        let value2 = execution_context.stack.pop();
        match value1 {
            Types::Double(_) | Types::Long(_) => match value2 {
                Types::Double(_) | Types::Long(_) => panic!(
                    "DUP2_X1: If value1 is a Double or Long, value2 can not be a Double or Long"
                ),
                _ => {
                    execution_context.stack.push(value1.clone());
                    execution_context.stack.push(value2);
                    execution_context.stack.push(value1);
                }
            },
            _ => {
                let value3 = execution_context.stack.pop();
                match value2 {
					Types::Double(_) | Types::Long(_) => panic!("DUP2_X1: If value1 is not a Double or Long, value2 can not be a Double or Long"),
					_ => {
						match value3 {
							Types::Double(_) | Types::Long(_) => panic!("DUP2_X1: If value1 and value2 are not a Double or Long, value3 can not be a Double or Long"),
							_ => {
								execution_context.stack.push(value2.clone());
								execution_context.stack.push(value1.clone());
								execution_context.stack.push(value3);
								execution_context.stack.push(value2);
								execution_context.stack.push(value1);
							}
						}
					}
				}
            }
        }
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("dup2_x1")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DUP2_X2 {}
impl Instruction for DUP2_X2 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::DUP2_X2));
        DUP2_X2 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value1 = execution_context.stack.pop();
        let value2 = execution_context.stack.pop();
        match value1 {
			Types::Double(_) | Types::Long(_) => {
				match value2 {
					Types::Double(_) | Types::Long(_) => {
						execution_context.stack.push(value1.clone());
						execution_context.stack.push(value2);
						execution_context.stack.push(value1);
					},
					_ => {
						let value3 = execution_context.stack.pop();
						match value3 {
							Types::Double(_) | Types::Long(_) => panic!("DUP2_X2: If value1 is a Double or Long and value2 is not a Double or Long, then value3 can not be a Double or Long"),
							_ => {
								execution_context.stack.push(value1.clone());
								execution_context.stack.push(value3);
								execution_context.stack.push(value2);
								execution_context.stack.push(value1);
							}
						}
					}
				}
			},
			_ => {
				match value2 {
					Types::Double(_) | Types::Long(_) => panic!("DUP2_X2: If value1 is not a Double or Long, value2 can not be a Double or Long"),
					_ => {
						let value3 = execution_context.stack.pop();
						match value3 {
							Types::Double(_) | Types::Long(_) => {
								execution_context.stack.push(value2.clone());
								execution_context.stack.push(value1.clone());
								execution_context.stack.push(value3);
								execution_context.stack.push(value2);
								execution_context.stack.push(value1);
							}
							_ => {
								let value4 = execution_context.stack.pop();
								match value4 {
									Types::Double(_) | Types::Long(_) => panic!("DUP2_X2: If value1, value2 and value3 are not a Double or Long, value4 can not be a Double or Long"),
									_ => {
										execution_context.stack.push(value2.clone());
										execution_context.stack.push(value1.clone());
										execution_context.stack.push(value4);
										execution_context.stack.push(value3);
										execution_context.stack.push(value2);
										execution_context.stack.push(value1);
									}
								}
							}
						}
					}
				}
			}
		}
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("dup2_x2")
    }
}
