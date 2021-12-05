use inkwell::{basic_block::BasicBlock, builder::Builder, context::Context, module::Module, values::{IntMathValue, IntValue}};

use crate::parser::Opcode;

pub struct Compiler<'ctx> {
    pub ctx: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub module: Module<'ctx>,
    code: Vec<Opcode>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx Context, code: Vec<Opcode>) -> Self {
        Self {
            ctx: context,
            module: context.create_module("length"),
            builder: context.create_builder(),
            code: code,
        }
    }

    pub fn compile(&mut self) {
        let main_type = self.ctx.i32_type().fn_type(&[], false);
        let main = self.module.add_function("main", main_type, None);

        self.init_external_functions();

        let mut blocks: Vec<BasicBlock> = vec![];
        let entry = self.ctx.append_basic_block(main, "entry");

        self.builder.position_at_end(entry);

        let stackptr = self
            .builder
            .build_call(
                self.module.get_function("initStack").unwrap(),
                &[],
                "stackptr",
            )
            .try_as_basic_value()
            .left()
            .unwrap();

        for _ in self.code.iter() {
            blocks.push(self.ctx.append_basic_block(main, ""));
        }

        let exit = self.ctx.append_basic_block(main, "exit");

        self.builder.build_unconditional_branch(blocks[0]);

        for (i, op) in self.code.iter().enumerate() {
            self.builder.position_at_end(blocks[i]);

            match op {
                Opcode::Inp => {
                    self.builder.build_call(
                        self.module.get_function("lengthInp").unwrap(),
                        &[stackptr.into()],
                        "",
                    );
                }
                Opcode::Add => {
                    self.builder.build_call(
                        self.module.get_function("lengthAdd").unwrap(),
                        &[stackptr.into()],
                        "",
                    );
                }
                Opcode::Sub => {
                    self.builder.build_call(
                        self.module.get_function("lengthSub").unwrap(),
                        &[stackptr.into()],
                        "",
                    );
                }
                Opcode::Dup => {
                    self.builder.build_call(
                        self.module.get_function("lengthDup").unwrap(),
                        &[stackptr.into()],
                        "",
                    );
                }
                Opcode::Gotou(location) => {
                    if *location >= self.code.len() {
                        println!("{} {}", location, self.code.len());
                        self.builder.build_unconditional_branch(exit);
                    } else {
                        self.builder
                            .build_unconditional_branch(blocks[location - 1]);
                    }
                }
                Opcode::Cond => {
                    let stack_top = self
                        .builder
                        .build_call(
                            self.module.get_function("pop").unwrap(),
                            &[stackptr.into()],
                            "",
                        )
                        .try_as_basic_value()
                        .left()
                        .unwrap();

                    let result: IntValue = self.builder.build_int_compare(
                        inkwell::IntPredicate::NE,
                        stack_top.into_int_value(),
                        self.ctx.i64_type().const_int(0, false).into(),
                        "",
                    );
                    if i + 1 == self.code.len() {
                        // do nothing, code exits either way
                    } else if i + 2 == self.code.len() {
                        self.builder.build_conditional_branch(result, blocks[i + 1], exit);
                    } else {
                        self.builder.build_conditional_branch(result, blocks[i + 1], blocks[i + 2]);

                    }
                }
                Opcode::Outn => {
                    self.builder.build_call(
                        self.module.get_function("lengthOutn").unwrap(),
                        &[stackptr.into()],
                        "",
                    );
                }
                Opcode::Outa => {
                    self.builder.build_call(
                        self.module.get_function("lengthOuta").unwrap(),
                        &[stackptr.into()],
                        "",
                    );
                }
                Opcode::Rol => {
                    self.builder.build_call(
                        self.module.get_function("lengthRol").unwrap(),
                        &[stackptr.into()],
                        "",
                    );
                }
                Opcode::Swap => {
                    self.builder.build_call(
                        self.module.get_function("lengthSwap").unwrap(),
                        &[stackptr.into()],
                        "",
                    );
                }
                Opcode::Mul => {
                    self.builder.build_call(
                        self.module.get_function("lengthMul").unwrap(),
                        &[stackptr.into()],
                        "",
                    );
                }
                Opcode::Div => {
                    self.builder.build_call(
                        self.module.get_function("lengthDiv").unwrap(),
                        &[stackptr.into()],
                        "",
                    );
                }
                Opcode::Pop => {
                    self.builder.build_call(
                        self.module.get_function("pop").unwrap(),
                        &[stackptr.into()],
                        "",
                    );
                }
                Opcode::Push(value) => {
                    self.builder.build_call(
                        self.module.get_function("push").unwrap(),
                        &[
                            stackptr.into(),
                            self.ctx.i64_type().const_int(*value as u64, false).into(),
                        ],
                        "",
                    );
                }
                Opcode::Ror => {
                    self.builder.build_call(
                        self.module.get_function("lengthRor").unwrap(),
                        &[stackptr.into()],
                        "",
                    );
                },
                Opcode::Nop => {}
            }

            if i + 1 == self.code.len() {
                self.builder.build_unconditional_branch(exit);
            } else {
                self.builder.build_unconditional_branch(blocks[i + 1]);
            }
        }

        self.builder.position_at_end(exit);

        self.builder.build_call(
            self.module.get_function("freeStack").unwrap(),
            &[stackptr.into()],
            "",
        );

        self.builder
            .build_return(Some(&self.ctx.i32_type().const_int(0, false)));
    }

    fn init_external_functions(&mut self) {
        let int64_type = self.ctx.i64_type();

        let init_stack_type = int64_type.fn_type(&[], false);
        let free_stack_type = self.ctx.void_type().fn_type(&[int64_type.into()], false);

        let push_type = self
            .ctx
            .void_type()
            .fn_type(&[int64_type.into(), int64_type.into()], false);
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
