use std::{fs, io, path::PathBuf, str::FromStr};

use clap::{Args, Parser};

use preprocessor;
use lexer;
use parser;
use generator;
use assembler;

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

fn change_extension(orig_path: &PathBuf, extension: &str) -> PathBuf {
    let mut copy_path = orig_path.clone();
    copy_path.set_extension(extension);
    copy_path
}

fn main() {
    let cli: Cli = Cli::parse();
    let output_control = cli.output_control;
    
    let src_path = PathBuf::from_str(&cli.file).expect("Unable to get PathBuf from file argument: {&cli.file}");

    src_path.extension().expect("Input file argument does not have an extension: {&cli.file}");

    let preprocessed_path = change_extension(&src_path, "i");
    let assembly_path = change_extension(&src_path, "s");
    let binary_path = change_extension(&src_path, "");

    let preprocessed = preprocessor::preprocess(&src_path, &preprocessed_path).expect("Error in preprocessing");

    let tokens = lexer::lex(&preprocessed).expect("Failed to lex input");
    fs::remove_file(&preprocessed_path).expect("Failed to remove preprocessed input");
    if output_control.lex {
        println!("Terminating after lex");
        return
    }

    let ast = parser::parse(&tokens).expect("Failed to parse");
    if output_control.parse {
        println!("Terminating after parse");
        return
    }

    let generated = generator::generate(&ast).expect("Failed code generation");
    if output_control.codegen {
        println!("Terminating after codegen");
        return
    }

    emitter::emit(&generated, &assembly_path).expect("Failed code emission");
    if output_control.skip_assembly {
        println!("Terminating after code emission");
        return;
    }

    assembler::assemble(&assembly_path, &binary_path).expect("Failed assembly");
    fs::remove_file(assembly_path).expect("Failed to remove assembly file");

}
