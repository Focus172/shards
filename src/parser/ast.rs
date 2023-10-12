use crate::prelude::*;
use libloading::{Library, Symbol};

// impl Iterator for Ast {
//     type Item = syn::File;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         Some(self.args.clone())
//     }
// }

// pub struct Ast<'a> {
//     args: VecDeque<&'a str>,
// }
//
// impl<'a> Ast<'a> {
//     pub fn parse(s: &'a str, _sys: &SystemState) -> anyhow::Result<Ast<'a>> {
//         Ok(Ast {
//             args: s.split_whitespace().collect::<VecDeque<&str>>(),
//         })
//     }
//
//     pub fn next(&mut self) -> Option<&'a str> {
//         self.args.pop_front()
//     }
//
//     pub fn get_args(&mut self) -> Vec<&'a str> {
//         let ret = self.args.iter().map(|s| *s).collect();
//         self.args.clear();
//         ret
//     }
// }

pub trait FromExternalAst {
    fn parse(s: &str) -> Option<Ast>;
}

impl FromExternalAst for Ast {
    fn parse(s: &str) -> Option<Ast> {
        let crate_path = env!("PWD");
        let library_path = format!("{}/{}", crate_path, "libs/librushi.so");
        println!("Loading add() from {}", library_path);

        let ast = unsafe {
            let ptr = s.as_ptr();
            let len = s.len();
            let lib = Library::new(library_path).unwrap();
            let func: Symbol<libshards::ParseFuncSig> = lib.get(b"parse").unwrap();

            func(ptr, len)
        };

        ast.try_into().ok()

        // let parse = format!(
        //     "fn main() {{
        //     {s}
        // }}"
        // );

        // let syntax = syn::parse_file(&parse).expect("Unable to parse file");
        // let Some(shebang) = syntax.shebang else {
        //     panic!("No shebang found");
        // };

        // match rushi::ast_from_str(&parse) {
        //     Some(s) => Ok(Ast {
        //         args: s,
        //     }),
        //     None => Err(anyhow::anyhow!("Failed to parse ast").into()),
        // }

        // Ok(Ast {
        //     args: s.split_whitespace().collect::<VecDeque<&str>>(),
        // })

        // Ok(Self { args: syntax })
    }

    // pub fn next(&mut self) -> Option<&'a str> {
    //     self.args.pop_front()
    // }
    //
    // pub fn get_args(&mut self) -> Vec<&'a str> {
    //     let ret = self.args.iter().map(|s| *s).collect();
    //     self.args.clear();
    //     ret
    // }
}
