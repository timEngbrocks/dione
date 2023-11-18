use crate::class_loader::attribute_info::attribute_code::ExceptionTableEntry;

#[derive(Clone)]
pub struct ExceptionHandler {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl ExceptionHandler {
    pub fn new(exception_table_entry: ExceptionTableEntry) -> ExceptionHandler {
        ExceptionHandler {
            start_pc: exception_table_entry.start_pc(),
            end_pc: exception_table_entry.end_pc(),
            handler_pc: exception_table_entry.handler_pc(),
            catch_type: exception_table_entry.catch_type(),
        }
    }

    pub fn start_pc(&self) -> u16 {
        self.start_pc
    }

    pub fn end_pc(&self) -> u16 {
        self.end_pc
    }

    pub fn handler_pc(&self) -> u16 {
        self.handler_pc
    }

    pub fn catch_type(&self) -> u16 {
        self.catch_type
    }
}

#[derive(Clone)]
pub struct ExceptionHandlerTable {
    exception_handler: Vec<ExceptionHandler>,
}

impl Default for ExceptionHandlerTable {
    fn default() -> Self {
        Self::new()
    }
}

impl ExceptionHandlerTable {
    pub fn new() -> Self {
        ExceptionHandlerTable {
            exception_handler: Vec::new(),
        }
    }

    pub fn add(&mut self, exception_handler: ExceptionHandler) {
        self.exception_handler.push(exception_handler);
    }

    pub fn try_handle(&mut self, _: &()) -> Option<ExceptionHandler> {
        // 1. Find handlers responsible for start_pc -> end_pc code block
        // 2. Find handler responsible for catch_type
        None
    }
}
