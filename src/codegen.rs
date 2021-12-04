use inkwell::{context::Context, builder::Builder, module::Module};

use crate::parser::Opcode;

pub struct Compiler<'ctx> {
    pub ctx: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub module: Module<'ctx>,
    code: Vec<Opcode>
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx Context, code: Vec<Opcode>) -> Self {
        Self {
            ctx: context,
            module: context.create_module("length"),
            builder: context.create_builder(),
            code
        }
    }

    pub fn compile(&mut self) {
        
        let main_type = self.ctx.i32_type().fn_type(&[], false);
        let main = self.module.add_function("main", main_type, None);

        self.init_external_functions();

    }

    fn init_external_functions(&mut self) {
        let int64_type = self.ctx.i64_type();

        let init_stack_type = int64_type.fn_type(&[], false);
        let free_stack_type = self.ctx.void_type().fn_type(&[int64_type.into()], false);

        let push_type = self.ctx.void_type().fn_type(&[int64_type.into(), int64_type.into()], false);
        let pop_type = int64_type.fn_type(&[int64_type.into()], false);

        let stack_op_type = self.ctx.void_type().fn_type(&[int64_type.into()], false);

        self.module.add_function("initStack", init_stack_type, None);
        self.module.add_function("freeStack", free_stack_type, None);
        self.module.add_function("push", push_type, None);
        self.module.add_function("pop", pop_type, None);

        self.module.add_function("lengthInp", stack_op_type, None);
        self.module.add_function("lengthAdd", stack_op_type, None);
        self.module.add_function("lengthSub", stack_op_type, None);
        self.module.add_function("lengthDup", stack_op_type, None);
        self.module.add_function("lengthOutn", stack_op_type, None);
        self.module.add_function("lengthOuta", stack_op_type, None);
        self.module.add_function("lengthRol", stack_op_type, None);
        self.module.add_function("lengthSwap", stack_op_type, None);
        self.module.add_function("lengthMul", stack_op_type, None);
        self.module.add_function("lengthDiv", stack_op_type, None);
        self.module.add_function("lengthRor", stack_op_type, None);
    }
}