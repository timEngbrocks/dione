use crate::{
    class_loader::parser::{Parser, U1, U2},
    jvm::{
        frame::Frame,
        instructions::{Instruction, InstructionResult},
        object_manager::ObjectManager,
        runtime_constant_pool::{
            numeric_constant::{NumericConstant, NumericConstantKind},
            RuntimeConstants, RuntimeConstantPool,
        },
        types::{Types, Value},
    },
    opcodes,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LDC {
    index: U1,
}
impl Instruction for LDC {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::LDC));
        let index = parser.consume_u1();
        LDC { index }
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        ldc_impl(execution_context, self.index as u16);
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        2
    }

    fn to_string(&self, runtime_constant_pool: &RuntimeConstantPool) -> String {
        ldc_to_string_impl(runtime_constant_pool, self.index as u16, "ldc".to_string())
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LDC_W {
    indexbyte1: U1,
    indexbyte2: U1,
}
impl Instruction for LDC_W {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::LDC_W));
        let indexbyte1 = parser.consume_u1();
        let indexbyte2 = parser.consume_u1();
        LDC_W {
            indexbyte1,
            indexbyte2,
        }
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let index = (self.indexbyte1 as u16) << 8 | self.indexbyte2 as u16;
        ldc_impl(execution_context, index);
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        3
    }

    fn to_string(&self, runtime_constant_pool: &RuntimeConstantPool) -> String {
        let index = (self.indexbyte1 as u16) << 8 | self.indexbyte2 as u16;
        ldc_to_string_impl(runtime_constant_pool, index, "ldc_w".to_string())
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LDC2_W {
    indexbyte1: U1,
    indexbyte2: U1,
}
impl Instruction for LDC2_W {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::LDC2_W));
        let indexbyte1 = parser.consume_u1();
        let indexbyte2 = parser.consume_u1();
        LDC2_W {
            indexbyte1,
            indexbyte2,
        }
    }

    // TODO: https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-6.html#jvms-6.5.ldc2_w
    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let index = (self.indexbyte1 as u16) << 8 | self.indexbyte2 as u16;
        match execution_context.runtime_constant_pool.get(index) {
            RuntimeConstants::NumericConstant(NumericConstant {
                value: NumericConstantKind::Long(value),
            }) => {
                execution_context.stack.push(Types::Long(value.clone()));
            }
            RuntimeConstants::NumericConstant(NumericConstant {
                value: NumericConstantKind::Double(value),
            }) => {
                execution_context.stack.push(Types::Double(value.clone()));
            }
            // TODO: Dynamically computed long/double constant
            _ => {
                panic!("LDC2_W::execute: Unknown constant at index {}", index);
            }
        }
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        3
    }

    fn to_string(&self, runtime_constant_pool: &RuntimeConstantPool) -> String {
        let index = (self.indexbyte1 as u16) << 8 | self.indexbyte2 as u16;
        match runtime_constant_pool.get(index) {
            RuntimeConstants::NumericConstant(NumericConstant {
                value: NumericConstantKind::Long(value),
            }) => {
                format!("ldc2_w: Long({})", value.get())
            }
            RuntimeConstants::NumericConstant(NumericConstant {
                value: NumericConstantKind::Double(value),
            }) => {
                format!("ldc2_w: Double({})", value.get())
            }
            // TODO: Dynamically computed long/double constant
            _ => {
                panic!("LDC2_W::to_string: Unknown constant at index {}", index);
            }
        }
    }
}

fn ldc_to_string_impl(runtime_constant_pool: &RuntimeConstantPool, index: u16, kind: String) -> String {
    match runtime_constant_pool.get(index) {
        RuntimeConstants::NumericConstant(NumericConstant {
            value: NumericConstantKind::Integer(value),
        }) => {
            format!("{}: Int({})", kind, value.get())
        }
        RuntimeConstants::NumericConstant(NumericConstant {
            value: NumericConstantKind::Float(value),
        }) => {
            format!("{}: Float({})", kind, value.get())
        }
        RuntimeConstants::StringConstant(constant) => {
            format!("{}: String({})", kind, constant.text())
        }
        RuntimeConstants::SymRefClassOrInterface(class_ref) => {
            format!("{}: Object({})", kind, class_ref.name)
        }
        // TODO: method type, method handle, dynamically computed constant
        _ => {
            panic!("LDC::to_string: Unknown constant at index {}", index);
        }
    }
}

// TODO: https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-6.html#jvms-6.5.ldc
fn ldc_impl(execution_context: &mut Frame, index: u16) {
    match execution_context.runtime_constant_pool.get(index) {
        RuntimeConstants::NumericConstant(NumericConstant {
            value: NumericConstantKind::Integer(value),
        }) => {
            execution_context.stack.push(Types::Int(value.clone()));
        }
        RuntimeConstants::NumericConstant(NumericConstant {
            value: NumericConstantKind::Float(value),
        }) => {
            execution_context.stack.push(Types::Float(value.clone()));
        }
        RuntimeConstants::StringConstant(constant) => {
            execution_context.stack.push(Types::Reference(constant.get()))
        }
        RuntimeConstants::SymRefClassOrInterface(class_ref) => {
            let reference = ObjectManager::get_associated_class_object(&class_ref.name);
            execution_context.stack.push(Types::Reference(reference));
        }
        // TODO: method type, method handle, dynamically computed constant
        _ => {
            panic!("LDC::execute: Unknown constant at index {}", index);
        }
    }
}
