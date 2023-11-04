use crate::{
    class_loader::{
        constant_pool_info::ConstantPool,
        parser::{Parser, U1, U2, U4},
    },
    jvm::exception_handler::{ExceptionHandler, ExceptionHandlerTable},
};

use super::{Attribute, AttributeInfo};

#[derive(Debug, Clone)]
pub struct AttributeCode {
    pub attribute_name_index: U2,
    pub attribute_length: U4,
    pub max_stack: U2,
    pub max_locals: U2,
    pub code_length: U4,
    pub code: Vec<U1>,
    pub exception_table_length: U2,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attributes_count: U2,
    pub attribute_info: Vec<AttributeInfo>,
}

impl AttributeCode {
    pub fn get_exception_handler_table(&self) -> ExceptionHandlerTable {
        let mut exception_handler_table = ExceptionHandlerTable::new();
        for exception_table_entry in &self.exception_table {
            exception_handler_table.add(ExceptionHandler {
                start_pc: exception_table_entry.start_pc,
                end_pc: exception_table_entry.end_pc,
                handler_pc: exception_table_entry.handler_pc,
                catch_type: exception_table_entry.catch_type,
            });
        }
        exception_handler_table
    }
}

impl Attribute for AttributeCode {
    fn new(parser: &mut Parser, constant_pool: &ConstantPool) -> AttributeCode {
        let attribute_name_index = parser.consume_u2();
        let attribute_length = parser.consume_u4();
        let max_stack = parser.consume_u2();
        let max_locals = parser.consume_u2();
        let code_length = parser.consume_u4();
        let mut code = Vec::with_capacity(code_length as usize);
        for _ in 0..code_length {
            code.push(parser.consume_u1());
        }
        let exception_table_length = parser.consume_u2();
        let mut exception_table = Vec::with_capacity(exception_table_length as usize);
        for _ in 0..exception_table_length {
            exception_table.push(ExceptionTableEntry::new(parser));
        }
        let attributes_count = parser.consume_u2();
        let mut attribute_info = Vec::with_capacity(attribute_length as usize);
        for _ in 0..attributes_count {
            attribute_info.push(AttributeInfo::new(parser, constant_pool));
        }

        AttributeCode {
            attribute_name_index,
            attribute_length,
            max_stack,
            max_locals,
            code_length,
            code,
            exception_table_length,
            exception_table,
            attributes_count,
            attribute_info,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExceptionTableEntry {
    pub start_pc: U2,
    pub end_pc: U2,
    pub handler_pc: U2,
    pub catch_type: U2,
}

impl ExceptionTableEntry {
    pub fn new(parser: &mut Parser) -> ExceptionTableEntry {
        let start_pc = parser.consume_u2();
        let end_pc = parser.consume_u2();
        let handler_pc = parser.consume_u2();
        let catch_type = parser.consume_u2();

        ExceptionTableEntry {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        }
    }
}
