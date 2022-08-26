use core_foundation::base::{CFIndex, CFRange};
use core_foundation::string::CFString;
use std::{env, ptr};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueDCSDictionary {
    _unused: [u8; 0],
}

pub type DCSDictionary = *mut OpaqueDCSDictionary;

#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    pub fn DCSGetTermRangeInString(
        dictionary: DCSDictionary,
        textString: CFString,
        offset: CFIndex,
    ) -> CFRange;

    pub fn DCSCopyTextDefinition(
        dictionary: DCSDictionary,
        textString: CFString,
        range: CFRange,
    ) -> CFString;
}

fn format_output(text: String) {
    let keywords = vec!["noun", "verb", "adjective", "ORIGIN", "DERIVATIVES", "PHRASES", "VERBS"];
    let mut result_string = String::new();
    for word in text.split_ascii_whitespace() {
        if keywords.contains(&word) {
            result_string.push('\n');
            result_string.push_str(word);
            result_string.push('\n');
        } else if word.ends_with('.') || word.ends_with(')') {
            result_string.push_str(word);
            result_string.push('\n');
        }
        else {
            result_string.push_str(word);
            result_string.push(' ');
        }
    }
    println!("{result_string}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: define [word]");
        return;
    }
    let word = args.get(1).expect("usage: define [word]").trim();

    let search_term = CFString::new(word);
    let dictionary: DCSDictionary = ptr::null_mut();
    let offset: CFIndex = 0;

    let range = unsafe { DCSGetTermRangeInString(dictionary, search_term.clone(), offset) };
    if range.location >= 0 {
        let definition = unsafe { DCSCopyTextDefinition(dictionary, search_term, range) };
        let definition = definition.to_string();
        format_output(definition);
    } else {
        println!("word not found.");
    }
}
