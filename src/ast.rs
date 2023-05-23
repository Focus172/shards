//! A todo list item until a get a working api for the rustc Ast

use std::collections::VecDeque;

pub struct Ast<'a> {
    args: VecDeque<&'a str>,
}

impl<'a> Ast<'a> {
    pub fn parse(s: &'a str) -> anyhow::Result<Ast<'a>> {
        Ok(Ast {
            args: s.split_whitespace().collect::<VecDeque<&str>>(),
        })
    }

    pub fn next(&mut self) -> Option<&'a str> {
        self.args.pop_front()
    }

    pub fn get_args(&mut self) -> Vec<&'a str> {
        let ret = self.args.iter().map(|s| *s).collect();
        self.args.clear();
        ret
    }
}
