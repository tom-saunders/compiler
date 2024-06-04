use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[command(flatten)]
    output_control: Option<OutputControl>,

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
    emit_assembly: bool,
}

fn main() {
    let cli = Cli::parse();

    println!("output_control: {:?}", cli.output_control);
    println!("file: {:?}", cli.file);
}
