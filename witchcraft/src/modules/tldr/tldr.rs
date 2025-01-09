use crate::{core::types::Closure, search_value};
use termimad::*;

pub fn read_markdown_from_path(argsv: &[String]) -> i32 {
    let skin = MadSkin::default();
    skin.print_text(&std::fs::read_to_string("../README.md").unwrap());
    return 0;
}

pub fn api() -> Closure {
    vec![("tldr", read_markdown_from_path)]
}
