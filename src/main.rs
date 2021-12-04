use inkwell::context::Context;
use lengthlvm::codegen::Compiler;

fn main() {
    let context = Context::create();

    let mut codegen = Compiler::new(&context, vec![]);

    codegen.compile();

    codegen.module.print_to_file("test.ir");
}
