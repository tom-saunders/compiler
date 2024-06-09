use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
    process::{Child, Command, Stdio},
    thread,
};

trait CompilerProc {
    fn wrap_and_spawn(&self, assembled_path: &Path, out_path: &Path) -> Result<Child, io::Error>;
}

struct DefaultImpl;

impl CompilerProc for DefaultImpl {
    fn wrap_and_spawn(&self, assembled_path: &Path, out_path: &Path) -> Result<Child, io::Error> {
        Command::new("gcc")
            .args([
                assembled_path.to_str().unwrap(),
                "-o",
                out_path.to_str().unwrap(),
            ])
            .spawn()
    }
}

fn assemble_internal(
    assembled_path: &Path,
    out_path: &Path,
    compiler: &impl CompilerProc,
) -> io::Result<()> {
    let mut assembler = compiler
        .wrap_and_spawn(assembled_path, out_path)
        .expect("Error spawning assembler");

    let exit = assembler.wait().expect("Failed to wait on preprocessor");

    match &exit.code() {
        Some(0) => println!("Preprocessor ran successfully"),
        Some(c) => panic!("Preprocessor exited with nonzero code: {c}"),
        None => panic!("Preprocessor terminated due to signal"),
    };

    Ok(())
}

pub fn assemble(assembled_path: &Path, out_path: &Path) -> io::Result<()> {
    let default_impl = DefaultImpl;
    assemble_internal(assembled_path, out_path, &default_impl)
}
