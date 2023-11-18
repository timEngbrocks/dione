use crate::util::{sized_array::SizedArray, stack::Stack};

use super::{
    descriptor::parse_method_descriptor,
    execution_context::{ExecutionContext, ExecutionContextStepResult},
    frame::Frame,
    object_manager::ObjectManager,
    types::Types,
};

static mut INTERPRETER_COUNT: usize = 0;

pub struct Interpreter {
    call_stack: Vec<ExecutionContext>,
    current: Option<ExecutionContext>,
    identifier: usize,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            call_stack: Vec::new(),
            current: None,
            identifier: unsafe {
                INTERPRETER_COUNT += 1;
                INTERPRETER_COUNT
            },
        }
    }

    pub fn start(&mut self) {
        self.run("Main", "main", "([Ljava/lang/String;)V");
    }

    pub fn run(&mut self, class_name: &str, method_name: &str, descriptor: &str) {
        let execution_context = self.setup_method_execution(class_name, method_name, descriptor);
        self.current = Some(execution_context);

        while self.current.is_some() {
            let execution_context = self.current.as_mut().unwrap();
            // execution_context.trace_parsed_method();

            // execution_context.trace_instruction(&self.identifier, &instruction_index, instruction);
            match execution_context.step() {
                ExecutionContextStepResult::Completed => {
                    if self.call_stack.is_empty() {
                        break;
                    }
                    self.current = self.call_stack.pop();
                    if self.current.is_none() {
                        break;
                    }
                    continue;
                }
                ExecutionContextStepResult::Continue => {}
                ExecutionContextStepResult::Call(new_execution_context) => {
                    self.call_stack.push(self.current.take().unwrap());
                    self.current = Some(new_execution_context);
                    continue;
                }
                ExecutionContextStepResult::Return(value) => {
                    self.current = self.call_stack.pop();
                    if self.current.is_none() {
                        break;
                    }
                    self.current.as_mut().unwrap().set_return_value(value);
                    continue;
                }
                ExecutionContextStepResult::Throw(exception) => {
                    if !execution_context.try_handle(&exception) {
                        // Unwind call stack
                        if self.call_stack.is_empty() {
                            break;
                        }
                        self.current = self.call_stack.pop();
                        if self.current.is_none() {
                            break;
                        }
                        continue;
                    }
                    continue;
                }
            }
        }
    }

    // pub fn run(&mut self, class_name: &str, method_name: &str, descriptor: &str) {
    //     let execution_context = self.setup_method_execution(class_name, method_name, descriptor);
    //     let mut global_exception: Option<()> = None;
    //     self.current = Some(execution_context);

    //     while self.current.is_some() {
    //         let execution_context = self.current.as_mut().unwrap();
    //         execution_context.trace_parsed_method();

    //         if let Some(exception) = global_exception {
    //             if !execution_context
    //                 .instruction_stream()
    //                 .try_handle(&exception)
    //             {
    //                 // Unwind call stack
    //                 if self.call_stack.is_empty() {
    //                     break;
    //                 }
    //                 self.current = self.call_stack.pop();
    //                 if self.current.is_none() {
    //                     break;
    //                 }
    //                 continue;
    //             }
    //             global_exception = None;
    //             continue;
    //         }

    //         let instruction_index = *execution_context.instruction_stream().cursor();
    //         if !execution_context.instruction_stream().has_next() {
    //             if self.call_stack.is_empty() {
    //                 break;
    //             }
    //             self.current = self.call_stack.pop();
    //             if self.current.is_none() {
    //                 break;
    //             }
    //             continue;
    //         }

    //         let instruction = execution_context.instruction_stream().next();
    //         execution_context.trace_instruction(&self.identifier, &instruction_index, instruction);

    //         let result = instruction.execute(execution_context.frame());
    //         global_exception = *result.get_exception();

    //         if let Some(new_execution_context) = result.get_call().clone() {
    //             self.call_stack.push(self.current.take().unwrap());
    //             self.current = Some(new_execution_context);
    //             continue;
    //         }

    //         if let Some(branch) = result.get_branch().clone() {
    //             match branch {
    //                 BranchKind::Absolute(value) => {
    //                     execution_context.instruction_stream().absolute_jump(*value);
    //                 }
    //                 BranchKind::Relative(value) => {
    //                     execution_context.instruction_stream().relative_jump(*value);
    //                 }
    //             }
    //             continue;
    //         }

    //         if let Some(ret) = result.get_ret().clone() {
    //             match ret {
    //                 ReturnKind::Value(value) => {
    //                     self.current
    //                         .as_ref()
    //                         .unwrap()
    //                         .frame()
    //                         .assert_matches_return_type(&value);
    //                     self.current = self.call_stack.pop();
    //                     if self.current.is_none() {
    //                         break;
    //                     }
    //                     self.current
    //                         .as_mut()
    //                         .unwrap()
    //                         .frame()
    //                         .stack()
    //                         .push(value.clone());
    //                 }
    //                 ReturnKind::Void => {
    //                     self.current = self.call_stack.pop();
    //                     if self.current.is_none() {
    //                         break;
    //                     }
    //                 }
    //             }
    //             continue;
    //         }
    //     }
    // }

    fn setup_method_execution(
        &mut self,
        class_name: &str,
        method_name: &str,
        descriptor: &str,
    ) -> ExecutionContext {
        if let Some((object, method)) =
            ObjectManager::get(class_name).get_method(method_name, descriptor)
        {
            let return_type = match parse_method_descriptor(method.descriptor()) {
                Some((_, return_type)) => return_type,
                None => panic!("Invalid method descriptor: {}", method.descriptor()),
            };

            let (max_locals, max_stack) = if method.is_native() {
                (None, None)
            } else {
                (Some(*method.max_locals()), Some(*method.max_stack()))
            };
            let frame = Frame::new(
                SizedArray::<Types>::new(max_locals),
                Stack::<Types>::new(max_stack),
                object.class_file(),
                object.name().clone(),
                method.name().clone(),
                method.descriptor().clone(),
                return_type,
            );

            let instruction_stream = method.instruction_stream().clone();

            ExecutionContext::new(frame, instruction_stream)
        } else {
            panic!(
                "Method not found: {}{} on {}",
                method_name, descriptor, class_name
            );
        }
    }
}
