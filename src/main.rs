#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]

pub(crate) mod lexer;
pub(crate) mod token;

use std::{fs::File, io::Read};

fn main() {
    let buffer = open_file("main.fl");
    let lexed = lexer::lex(buffer);
}

fn open_file(file_name: &str) -> String {
    let file = File::open(file_name);
    let mut file = match file {
        Ok(file) => file,
        Err(err) => panic!("file ded boohoo err: {err}"),
    };

    let mut content_buffer = String::new();
    let bytes_read = match file.read_to_string(&mut content_buffer) {
        Ok(bytes_read) => bytes_read,
        Err(err) => panic!("error lololol : {err}"),
    };

    if bytes_read == 0 {
        panic!("bytesread == 0 thats prob bad??")
    }

    content_buffer
}