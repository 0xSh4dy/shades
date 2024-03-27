use std::path::Path;
use std::process::Command;

use crate::codegen::irgenerator::IrGenerator;
use crate::lexer::scanner::tokens::TokenList;
use crate::llvm_wrappers::generators::funcgen::FuncGenerator;
use crate::llvm_wrappers::generators::mathgen::MathCodeGenerator;
use inkwell::context::Context;
use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::OptimizationLevel;
use crate::lexer::scanner::mainloop::handle_compound_statement;

pub fn generate_code(tokens: &mut TokenList) {
    let context = Context::create();
    let module = context.create_module("main");
    // Generate the shades_main function
    let void_type = context.void_type();
    let func_type = void_type.fn_type(&[], false);
    let function = module.add_function("shades_main", func_type, None);

    // Generate the main function
    let func_generator = FuncGenerator::new(&context, &module);
    func_generator.generate_c_main_function();

    // Match a compound statement
    let main_tree = handle_compound_statement(tokens);
    // Generate the IR
    let ir_generator = IrGenerator::new(&context,&module,"shades_main");

    ir_generator.generate_ir(main_tree.as_ref());

    module.print_to_file("/tmp/main.ll").unwrap();


    // // let ll_code = std::fs::read_to_string("/tmp/main.ll").expect("failed to read /tmp/main.ll");
    // Target::initialize_all(&InitializationConfig::default());

    // let triple = TargetMachine::get_default_triple();
    // let target = Target::from_triple(&triple).unwrap();
    // let cpu = TargetMachine::get_host_cpu_name().to_string();
    // let features = TargetMachine::get_host_cpu_features().to_string();
    // let target_machine_opt = target.create_target_machine(&triple, &cpu, &features, OptimizationLevel::Aggressive, RelocMode::Default, CodeModel::Default);
    
    // if let Some(target_machine) = target_machine_opt{
    //     let object_output_path = Path::new("/tmp/shadesbin.o");
    //     target_machine.write_to_file(&module, FileType::Object, object_output_path).expect("failed to generate object file");
    //     // clang -no-pie /tmp/shadesbin -o /tmp/shadesbinnew
    //     let output = Command::new("clang").args(&["-no-pie","/tmp/shadesbin.o","-o","/tmp/shadesbin"]).output().expect("failed to run linker command");
    //     if !output.status.success(){
    //         println!("linker command failed");
    //     }
    //     println!("Generated ELF at /tmp/shadesbin");
    // }
}
