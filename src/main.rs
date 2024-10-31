use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut config_sourceFile = String::new();
    let mut output_bytecodeFile = String::new();
    let mut run_bytecodeFile = String::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-c" => {
                if i + 1 < args.len() {
                    config_sourceFile = args[i + 1].clone();
                    i += 1;
                } else {
                    eprintln!("Error: -c requires a file argument");
                    return;
                }
            }
            "-o" => {
                if i + 1 < args.len() {
                    output_bytecodeFile = args[i + 1].clone();
                    i += 1;
                } else {
                    eprintln!("Error: -o requires an exename argument");
                    return;
                }
            }
            "-i" => {
                println!("Interactive mode");
            }
            "-r" => {
                println!("Run file");
                if i + 1 < args.len() {
                    run_bytecodeFile = args[i + 1].clone();
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

    println!("Config file: {}", config_file);
    println!("Output executable name: {}", output_exe);
    return;
}