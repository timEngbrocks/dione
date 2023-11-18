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
    attribute_name_index: U2,
    attribute_length: U4,
    max_stack: U2,
    max_locals: U2,
    code_length: U4,
    code: Vec<U1>,
    exception_table_length: U2,
    exception_table: Vec<ExceptionTableEntry>,
    attributes_count: U2,
    attributes: Vec<AttributeInfo>,
}

impl AttributeCode {
    pub fn get_exception_handler_table(&self) -> ExceptionHandlerTable {
        let mut exception_handler_table = ExceptionHandlerTable::new();
        for exception_table_entry in &self.exception_table {
            exception_handler_table.add(ExceptionHandler::new(exception_table_entry.clone()));
        }
        exception_handler_table
    }

    pub fn attribute_name_index(&self) -> &U2 {
        &self.attribute_name_index
    }

    pub fn attribute_length(&self) -> &U4 {
        &self.attribute_length
    }

    pub fn max_stack(&self) -> &U2 {
        &self.max_stack
    }

    pub fn max_locals(&self) -> &U2 {
        &self.max_locals
    }

    pub fn code_length(&self) -> &U4 {
        &self.code_length
    }

    pub fn code(&self) -> &Vec<U1> {
        &self.code
    }

    pub fn exception_table_length(&self) -> &U2 {
        &self.exception_table_length
    }

    pub fn exception_table(&self) -> &Vec<ExceptionTableEntry> {
        &self.exception_table
    }

    pub fn attributes_count(&self) -> &U2 {
        &self.attributes_count
    }

    pub fn attributes(&self) -> &Vec<AttributeInfo> {
        &self.attributes
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
            attributes: attribute_info,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExceptionTableEntry {
    start_pc: U2,
    end_pc: U2,
    handler_pc: U2,
    catch_type: U2,
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

    pub fn start_pc(&self) -> U2 {
        self.start_pc
    }

    pub fn end_pc(&self) -> U2 {
        self.end_pc
    }

    pub fn handler_pc(&self) -> U2 {
        self.handler_pc
    }

    pub fn catch_type(&self) -> U2 {
        self.catch_type
    }
}
