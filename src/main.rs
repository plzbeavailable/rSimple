mod compiler;
mod scanner;
use std::env;

fn main() {

    // read and scan the command line arguments
    let args: Vec<String> = env::args().collect();

    let mut config_source_file = String::new();
    let mut output_bytecode_file = String::new();
    let mut run_bytecode_file = String::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" => {
                println!("Usage: rSimple.exe [options]");
                println!("-c <file> : Compile source file");
                println!("-o <exename> : Output executable name");
                println!("-i : Interactive mode");
                println!("-r <file> : Run file");
                return;
            }
            "-c" => {
                if i + 1 < args.len() {
                    config_source_file = args[i + 1].clone();
                    i += 1;
                } else {
                    eprintln!("Error: -c requires a file argument");
                    return;
                }
            }
            "-o" => {
                if i + 1 < args.len() {
                    output_bytecode_file = args[i + 1].clone();
                    i += 1;
                } else {
                    eprintln!("Error: -o requires an exename argument");
                    return;
                }
            }
            "-i" => {
                println!("Interactive mode");
                return;
            }
            "-r" => {
                println!("Run file");
                if i + 1 < args.len() {
                    run_bytecode_file = args[i + 1].clone();
                    i += 1;
                } else {
                    eprintln!("Error: -o requires an exename argument");
                    return;
                }
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                eprintln!("use rSimple.exe -h for help");
                return;
            }
        }
        i += 1;
    }


    // act with the arguments
    println!("Config file: {}", config_source_file);
    println!("Output executable name: {}", output_bytecode_file);
    if !config_source_file.is_empty() && !output_bytecode_file.is_empty() {
        println!("Compile file: {}", config_source_file);
        compiler::compile(&config_source_file, &output_bytecode_file);
    }
    if !run_bytecode_file.is_empty() {
        println!("Run file: {}", run_bytecode_file);
    }

    return;
}