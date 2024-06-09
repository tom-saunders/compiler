use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
    process::{Child, Command, Stdio},
    thread,
};

trait IoTrait {
    fn write(&self, path: &Path, contents: &str) -> io::Result<()>;
}

trait CompilerProc {
    fn wrap_and_spawn(&self, src_path: &Path, out_path: &Path) -> Result<Child, io::Error>;
}

struct DefaultImpl;

impl IoTrait for DefaultImpl {
    fn write(&self, path: &Path, contents: &str) -> io::Result<()> {
        fs::write(path, contents)
    }
}

impl CompilerProc for DefaultImpl {
    fn wrap_and_spawn(&self, src_path: &Path, out_path: &Path) -> Result<Child, io::Error> {
        Command::new("gcc")
            .args(["-E", src_path.to_str().unwrap(), "-o", out_path.to_str().unwrap()])
            .spawn()
    }
}

fn preprocess_internal(
    src_path: &Path,
    out_path: &Path,
    io_handle: &impl IoTrait,
    compiler: &impl CompilerProc,
) -> io::Result<String> {

    let mut preprocess = compiler
        .wrap_and_spawn(src_path, out_path)
        .expect("Error spawning preprocessor");

    let exit = preprocess
        .wait()
        .expect("Failed to wait on preprocessor");

    match &exit.code() {
        Some(0) => println!("Preprocessor ran successfully"),
        Some(c) => panic!("Preprocessor exited with nonzero code: {c}"),
        None => panic!("Preprocessor terminated due to signal"),
    };

    let result = fs::read_to_string(out_path).expect("Unable to read written preprocessor file");

    if ! result.is_ascii() {
        panic!("Naive implementation only handles input in ASCII");
    }

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

    use tempfile::{self, NamedTempFile};

    struct IncludeCompiler;
    impl CompilerProc for IncludeCompiler {
        fn wrap_and_spawn(&self, src_path: &Path, out_path: &Path) -> Result<Child, io::Error> {
            Command::new("gcc")
                .stdout(Stdio::piped())
                .args(["-I", "src/asset", "-E", src_path.to_str().unwrap(), "-o", out_path.to_str().unwrap()])
                .spawn()
        }
    }

    #[test]
    fn test_no_include() {
        struct IoTest;
        impl IoTrait for IoTest {
            fn write(&self, path: &Path, contents: &str) -> io::Result<()> {
                Ok(())
            }
        }

        let tmp = tempfile::NamedTempFile::new().expect("Unable to create temp file for test");

        let src_path: PathBuf = PathBuf::from("src/asset/return_0.c");
        let out_path: PathBuf = PathBuf::from(tmp.path());
        let io_test: IoTest = IoTest;
        let compiler = DefaultImpl;
        let result = preprocess_internal(&src_path, &out_path, &io_test, &compiler)
            .expect("preprocessor should return");
        let exp = include_str!("asset/return_0.i");

        assert_eq!(exp, &result);
    }

    #[test]
    fn test_include() {
        struct IoTest;
        impl IoTrait for IoTest {
            fn write(&self, path: &Path, contents: &str) -> io::Result<()> {
                Ok(())
            }
        }
        
        let tmp = tempfile::NamedTempFile::new().expect("Unable to create temp file for test");

        let src_path: PathBuf = PathBuf::from("src/asset/include_a.c");
        let out_path: PathBuf = PathBuf::from(tmp.path());
        let io_test: IoTest = IoTest;
        let compiler = IncludeCompiler;
        let result = preprocess_internal(&src_path, &out_path, &io_test, &compiler)
            .expect("preprocessor should return");
        let exp = include_str!("asset/include_a.i");

        assert_eq!(exp, &result);
    }
}
