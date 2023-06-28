//! A todo list item until a get a working api for the rustc Ast

// use std::collections::VecDeque;

use crate::prelude::*;

pub struct Ast {
    args: syn::File, 
}

// impl Iterator for Ast {
//     type Item = syn::File;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         Some(self.args.clone())
//     }
// }
    
impl Ast {
    pub fn parse(s: &str) -> Result<Ast> {
        let parse = format!("fn main() {{
            {s}
        }}");

        match rushi::ast_from_str(&parse) {
            Some(s) => Ok(Ast {
                args: s,
            }),
            None => Err(anyhow::anyhow!("Failed to parse ast").into()),
        }
        
        // Ok(Ast {
        //     args: s.split_whitespace().collect::<VecDeque<&str>>(),
        // })
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
