use crate::{env::EnvState, ast::Ast};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn eval(&self, ast: &mut Ast, env: &mut EnvState) -> anyhow::Result<()> {

        match ast.next().unwrap() {
            "exit" => return Err(anyhow::anyhow!("exit")),
            "echo" => {
                println!("{}", ast.get_args().join(" "));
                return Ok(());
            },
            "set" => {
                // let args = ast.get_args();
                // builtins::set(key, args)
            },
            l if l.contains('=') => {
                // let (key, val) = l.split_once('=').unwrap();
                // builtins::set(key, val, env)
            }
            _ => {}
        };


            /*
            let mut f = std::fs::File::create("temp.rs").unwrap();
            let write = format!("fn main() {{\n\t{}\n}}", line);
            f.write_all(write.as_bytes()).unwrap();

            let info = std::process::Command::new("rustc")
                // .arg("--edition=2021")
                .arg("temp.rs")
                // .arg("--out-dir")
                // .arg(".")
                // .arg("--error-format=json")
                //   "-L",
                //   "dependency=/home/focus/code/rushi/target/debug/deps",
                //   "--extern",
                //   "anyhow=/home/focus/code/rushi/target/debug/deps/libanyhow-f199e6f943aad02b.rlib",
                //   "--extern",
                //   "clap=/home/focus/code/rushi/target/debug/deps/libclap-c5ef4992a3fcaf5a.rlib",
                //   "--extern",
                //   "rustyline=/home/focus/code/rushi/target/debug/deps/librustyline-9d36abc44d2ad736.rlib"
                .output()
                .expect("failed to execute process");

            let out = std::process::Command::new("./temp")
                .output()
                .expect("failed to execute process");

            std::fs::remove_file("temp.rs").unwrap();
            std::fs::remove_file("temp").unwrap();

            println!("{}", String::from_utf8_lossy(&out.stdout));
            log::info!("{}", String::from_utf8_lossy(&info.stderr));
            */

        Err(anyhow::anyhow!("Function should have returned by now"))
    }
}
