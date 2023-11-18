use log::trace;

use super::{
    frame::Frame,
    instructions::{BranchKind, Instruction, InstructionStream, Instructions, ReturnKind},
    types::Types,
};

#[derive(Clone)]
pub struct ExecutionContext {
    frame: Frame,
    instruction_stream: InstructionStream,
}

pub enum ExecutionContextStepResult {
    Completed,
    Continue,
    Call(ExecutionContext),
    Return(Types),
    Throw(()),
}

impl ExecutionContext {
    pub fn new(frame: Frame, instruction_stream: InstructionStream) -> Self {
        ExecutionContext {
            frame,
            instruction_stream,
        }
    }

    pub fn step(&mut self) -> ExecutionContextStepResult {
        if !self.instruction_stream.has_next() {
            return ExecutionContextStepResult::Completed;
        }

        let instruction = self.instruction_stream.next();
        let result = instruction.execute(&mut self.frame);

        if let Some(branch_kind) = result.get_branch() {
            match branch_kind {
                BranchKind::Absolute(value) => {
                    self.instruction_stream.absolute_jump(*value);
                }
                BranchKind::Relative(value) => {
                    self.instruction_stream.relative_jump(*value);
                }
            }
        }

        if let Some(new_execution_context) = result.get_call() {
            return ExecutionContextStepResult::Call(new_execution_context.clone());
        }

        if let Some(return_kind) = result.get_ret() {
            match return_kind {
                ReturnKind::Value(value) => {
                    self.frame.assert_matches_return_type(value);
                    return ExecutionContextStepResult::Return(value.clone());
                }
                ReturnKind::Void => {
                    return ExecutionContextStepResult::Completed;
                }
            }
        }

        if let Some(exception) = result.get_exception() {
            return ExecutionContextStepResult::Throw(*exception);
        }

        ExecutionContextStepResult::Continue
    }

    pub fn try_handle(&mut self, exception: &()) -> bool {
        self.instruction_stream.try_handle(exception)
    }

    pub fn set_return_value(&mut self, value: Types) {
        self.frame.stack().push(value);
    }

    fn trace_parsed_method(&self) {
        trace!(
            target: "parsed_methods",
            "Object: {} Method: {} - {}",
            &self.frame.object_name(),
            &self.frame.method_name(),
            &self.instruction_stream.len()
        );
        trace!(
            target: "parsed_methods",
            "{}",
            &self.instruction_stream.to_string(self.frame.runtime_constant_pool())
        );
    }

    fn trace_instruction(
        &self,
        identifier: &usize,
        instruction_index: &usize,
        instruction: &Instructions,
    ) {
        trace!(
            target: "instructions",
            "({}) {}.{} @ {} -> {:?}",
            identifier,
            &self.frame.object_name(),
            &self.frame.method_name(),
            instruction_index + instruction.length() as usize,
            instruction.to_string(self.frame.runtime_constant_pool())
        );
    }
}
