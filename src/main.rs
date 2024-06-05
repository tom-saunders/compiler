use std::{
    fs::{remove_file, File},
    io::Write,
    path::PathBuf,
    process::Command,
    str::FromStr,
};

use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[command(flatten)]
    output_control: OutputControl,

    /// path to file to compile
    file: String,
}

#[derive(Args, Debug)]
#[group(multiple = false)]
struct OutputControl {
    /// we should stop after the lexer
    #[arg(long)]
    lex: bool,

    /// we should stop after the parser
    #[arg(long)]
    parse: bool,

    /// we should stop after assembly generation
    #[arg(long)]
    codegen: bool,

    /// emit assembly file but don't assemble or link it
    #[arg(short = 'S')]
    skip_assembly: bool,
}

fn main() {
    let cli: Cli = Cli::parse();

    println!("output_control: {:?}", cli.output_control);
    println!("file: {:?}", cli.file);

    let source_path = PathBuf::from_str(&cli.file).unwrap();

    match source_path.extension() {
        Some(_) => (),
        None => panic!("input file does not have an extension: [{}]", cli.file),
    }

    let mut preprocessed_path = source_path.clone();
    preprocessed_path.set_extension("i");

    let mut assembly_path = source_path.clone();
    assembly_path.set_extension("s");

    let mut binary_path = source_path.clone();
    binary_path.set_extension("");

    preprocess(&source_path, &preprocessed_path);
    compile(&preprocessed_path, &assembly_path, &cli.output_control);
    if !cli.output_control.skip_assembly {
        assemble(&assembly_path, &binary_path);
    }
}

fn preprocess(source_path: &PathBuf, preprocessed_path: &PathBuf) {
    let source_name = source_path.to_str().unwrap();
    let preprocessed_name = preprocessed_path.to_str().unwrap();

    println!("running preprocessor child process");
    println!("preprocessor in : {}", source_name);
    println!("preprocessor out: {}", preprocessed_name);

    let mut preprocess = match Command::new("gcc")
        .args(["-E", "-P", source_name, "-o", preprocessed_name])
        .spawn()
    {
        Ok(p) => p,
        Err(err) => panic!("error running preprocessor: {}", err),
    };

    let preprocess_exit = match preprocess.wait() {
        Ok(status) => status,
        Err(err) => panic!("error waiting on preprocessor child process: {}", err),
    };

    match preprocess_exit.code() {
        Some(c) => {
            println!("preprocessor exited with code: {}", c);
            if c != 0 {
                panic!("preprocessor exited with nonzero code: {}", c);
            }
        }
        None => panic!("preprocessor terminated due to signal"),
    };
}

fn assemble(assembly_path: &PathBuf, binary_path: &PathBuf) {
    let assembly_name = assembly_path.to_str().unwrap();
    let binary_name = binary_path.to_str().unwrap();

    println!("running assembler child process");
    println!("assembler in : {}", assembly_name);
    println!("assembler out: {}", binary_name);

    let mut preprocess = match Command::new("gcc")
        .args([assembly_name, "-o", binary_name])
        .spawn()
    {
        Ok(p) => p,
        Err(err) => panic!("error running assembler: {}", err),
    };

    let preprocess_exit = match preprocess.wait() {
        Ok(status) => status,
        Err(err) => panic!("error waiting on assembler child process: {}", err),
    };

    match preprocess_exit.code() {
        Some(c) => {
            println!("assembler exited with code: {}", c);
            if c != 0 {
                panic!("assembler exited with nonzero code: {}", c);
            }
        }
        None => panic!("assembler terminated due to signal"),
    };

    match remove_file(assembly_path) {
        Ok(_) => (),
        Err(err) => panic!(
            "failed to remove assembly file [{}]: {}",
            assembly_name, err
        ),
    };
}

fn compile(preprocessed_path: &PathBuf, assembly_path: &PathBuf, output_control: &OutputControl) {
    let preprocessed_name = preprocessed_path.to_str().unwrap();

    compile_inner(preprocessed_path, assembly_path, output_control);

    match remove_file(preprocessed_path) {
        Ok(_) => (),
        Err(err) => panic!(
            "failed to remove preprocessed input file [{}]: {}",
            preprocessed_name, err
        ),
    };
}

fn compile_inner(
    preprocessed_path: &PathBuf,
    assembly_path: &PathBuf,
    output_control: &OutputControl,
) {
    let tokens = lexer(&preprocessed_path);
    if output_control.lex {
        println!("terminating after lexer");
        return;
    }

    let ast = parser(&tokens);
    if output_control.parse {
        println!("terminating after parser");
        return;
    }

    let assembly = generator(&ast);
    if output_control.codegen {
        println!("terminating after code generation");
        return;
    }

    emitter(&assembly, assembly_path);
}

struct Token {}

struct AST {}

fn lexer(_preprocessed_path: &PathBuf) -> Vec<Token> {
    return Vec::new();
}

fn parser(_tokens: &Vec<Token>) -> AST {
    return AST {};
}

fn generator(_ast: &AST) -> String {
    return String::from("");
}

fn emitter(_assembly: &String, assembly_path: &PathBuf) {
    let assembly_name = assembly_path.to_str().unwrap();
    let mut assembly_file = match File::create(assembly_path) {
        Ok(f) => f,
        Err(err) => panic!(
            "unable to open assembly file [{}] for writing: {}",
            assembly_name, err
        ),
    };

    assembly_file.write_all(b"\t.globl  main\nmain:\n\tmovl    $2, %eax\n\tret\n");
}
