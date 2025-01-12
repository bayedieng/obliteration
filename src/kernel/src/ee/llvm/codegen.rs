use crate::disasm::Disassembler;
use crate::llvm::module::LlvmModule;
use thiserror::Error;

/// Contains states for lifting a module.
pub(super) struct Codegen<'a> {
    input: Disassembler<'a>,
    output: &'a mut LlvmModule,
}

impl<'a> Codegen<'a> {
    pub fn new(input: Disassembler<'a>, output: &'a mut LlvmModule) -> Self {
        Self { input, output }
    }

    pub fn lift(&mut self, _offset: usize) -> Result<(), LiftError> {
        Ok(())
    }
}

/// Represents an error for [`Codegen::lift()`].
#[derive(Debug, Error)]
pub enum LiftError {}
