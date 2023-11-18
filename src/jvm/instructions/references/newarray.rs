use crate::{
    class_loader::parser::{Parser, U1, U2},
    jvm::{
        frame::Frame,
        instructions::{Instruction, InstructionResult},
        runtime_constant_pool::RuntimeConstantPool,
        types::{
            array::{Array, PrimitiveArray},
            boolean::Boolean,
            byte::Byte,
            char::Char,
            double::Double,
            float::Float,
            int::Int,
            long::Long,
            short::Short,
            Types, Value,
        },
    },
    opcodes,
    util::heap::Heap,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct NEWARRAY {
    atype: U1,
}
impl Instruction for NEWARRAY {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::NEWARRAY));
        let atype = parser.consume_u1();
        Self { atype }
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let count = match execution_context.stack().pop() {
            Types::Int(value) => value.get(),
            _ => panic!("Expected Int"),
        };
        let array = match self.atype {
            4 => PrimitiveArray::new(Types::Boolean(Boolean::new()), count as usize),
            5 => PrimitiveArray::new(Types::Char(Char::new()), count as usize),
            6 => PrimitiveArray::new(Types::Float(Float::new()), count as usize),
            7 => PrimitiveArray::new(Types::Double(Double::new()), count as usize),
            8 => PrimitiveArray::new(Types::Byte(Byte::new()), count as usize),
            9 => PrimitiveArray::new(Types::Short(Short::new()), count as usize),
            10 => PrimitiveArray::new(Types::Int(Int::new()), count as usize),
            11 => PrimitiveArray::new(Types::Long(Long::new()), count as usize),
            _ => panic!("Invalid atype"),
        };
        let reference = Heap::allocate_primitive_array(array);
        execution_context.stack().push(Types::Reference(reference));
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        2
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        format!("newarray {}", self.atype)
    }
}
