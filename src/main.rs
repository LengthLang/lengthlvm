use std::{env::args, path::Path};

use inkwell::{
    context::Context,
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
    OptimizationLevel,
};
use lengthlvm::{codegen::Compiler, parser::parse};

fn main() -> Result<(), String> {
    let context = Context::create();

    if args().len() != 2 {
        return Err(String::from("Must pass exactly 1 length source file"))
    }

    let mut codegen = Compiler::new(&context, parse(&args().collect::<Vec<_>>()[1])?);

    codegen.compile();

    println!("Writing LLVM IR...");
    if let Err(string) = codegen.module.print_to_file("out.ir") {
        return Err(string.to_string());
    }

    let target = get_native_target_machine();

    codegen.module.set_triple(&target.get_triple());

    println!("Writing object file...");

    if let Err(string) = target.write_to_file(&codegen.module, FileType::Object, Path::new("out.o"))
    {
        return Err(string.to_string());
    }

    Ok(())
}

fn get_native_target_machine() -> TargetMachine {
    Target::initialize_native(&InitializationConfig::default())
        .expect("Failed to initialize native target");
    let target_triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&target_triple).unwrap();
    target
        .create_target_machine(
            &target_triple,
            &TargetMachine::get_host_cpu_name().to_string(),
            &TargetMachine::get_host_cpu_features().to_string(),
            OptimizationLevel::None,
            RelocMode::Default,
            CodeModel::Default,
        )
        .unwrap()
}
