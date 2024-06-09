use std::{fs::{self, File}, io::{self, Read, Write}, path::Path, process::{Child, Command, Stdio}, thread};

trait IoTrait {
    fn read_to_string(&self, path: &Path) -> io::Result<String>;
    fn write(&self, path: &Path, contents: &str) -> io::Result<()>;
}

trait CompilerProc {
    fn wrap_and_spawn(&self) -> Result<Child, io::Error>;
}

struct DefaultImpl;

impl IoTrait for DefaultImpl {
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        fs::read_to_string(path)
    }
    fn write(&self, path: &Path, contents: &str) -> io::Result<()> {
        fs::write(path, contents)
    }
}

impl CompilerProc for DefaultImpl {
    fn wrap_and_spawn(&self) -> Result<Child, io::Error> {
        Command::new("gcc")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .args(["-E", "-"])
            .spawn()
    }
}

fn preprocess_internal(src_path: &Path, out_path: &Path, io_handle: &impl IoTrait, compiler: &impl CompilerProc) -> io::Result<String> {
    let input = io_handle.read_to_string(src_path)?;
    
    if ! input.is_ascii() {
        panic!("Naive implementation, only handles input in ASCII range for simplicity");
    }

    let mut preprocess = compiler.wrap_and_spawn()
        .expect("Error spawning preprocessor");

    let mut stdin = preprocess.stdin.take().unwrap();

    thread::spawn(move || {
        stdin.write_all(input.as_bytes()).expect("Failed to write input to preprocessor");
    });

    let exit = preprocess.wait_with_output().expect("Failed to wait on preprocessor");

    match &exit.status.code() {
        Some(0) => println!("Preprocessor ran successfully"),
        Some(c) => panic!("Preprocessor exited with nonzero code: {c}"),
        None => panic!("Preprocessor terminated due to signal"),
    };

    let result = String::from_utf8_lossy(&exit.stdout).to_string();
    
    io_handle.write(out_path, &result)?;

    Ok(result)
}

pub fn preprocess(src_path: &Path, out_path: &Path) -> io::Result<String> {
    let default_impl = DefaultImpl;
    preprocess_internal(src_path, out_path, &default_impl, &default_impl)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    struct IncludeCompiler;
    impl CompilerProc for IncludeCompiler  {
        fn wrap_and_spawn(&self) -> Result<Child, io::Error> {
            Command::new("gcc")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .args(["-I", "src/asset", "-E", "-"])
                .spawn()
        }
    }

    #[test]
    fn test_no_include() {
        struct IoTest;
        impl IoTrait for IoTest {
            fn read_to_string(&self, path: &Path) -> io::Result<String> {
                let contents = include_str!("asset/return_0.c");
                Ok(contents.to_string())
            }
        
            fn write(&self, path: &Path, contents: &str) -> io::Result<()> {
                Ok(())
            }
        }

        let src_path: PathBuf = PathBuf::new();
        let out_path: PathBuf = PathBuf::new();
        let io_test: IoTest = IoTest;
        let compiler = DefaultImpl;
        let result = preprocess_internal(&src_path, &out_path, &io_test, &compiler).expect("preprocessor should return");
        let exp = include_str!("asset/return_0.i");

        assert_eq!(exp, &result);
    }

    #[test]
    fn test_include() {
        struct IoTest;
        impl IoTrait for IoTest {
            fn read_to_string(&self, path: &Path) -> io::Result<String> {
                let contents = include_str!("asset/include_a.c");
                Ok(contents.to_string())
            }
        
            fn write(&self, path: &Path, contents: &str) -> io::Result<()> {
                Ok(())
            }
        }

        let src_path: PathBuf = PathBuf::new();
        let out_path: PathBuf = PathBuf::new();
        let io_test: IoTest = IoTest;
        let compiler = IncludeCompiler;
        let result = preprocess_internal(&src_path, &out_path, &io_test, &compiler).expect("preprocessor should return");
        let exp = include_str!("asset/include_a.i");

        assert_eq!(exp, &result);


    }
}
