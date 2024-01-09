use std::fs::{read, read_to_string};
use std::{env, fs};

mod programs;

const PROGRAM: &'static str = include_str!("main.rs");
const HELLO_WORLD: &'static str = include_str!("programs/hello_world.rs");
const COOL: &'static str = include_str!("programs/cool.rs");
const FIBONACCI: &'static str = include_str!("programs/fibonacci.rs");

fn segfault() -> ! {
    let mut x = 0;
    loop {
        x += x;
        unsafe {
            let p = x as *mut u8;
            *p.add(x) = 0xFF;
        }
    }
}

fn exe_bytes() -> Vec<u8> {
    let path = std::env::current_exe();

    if let Ok(v) = path {
        if let Some(p) = v.to_str() {
            if let Ok(bytes) = read(p) {
                return bytes;
            }
        }
    }

    segfault();
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    'outer: for x in 0..haystack.len() {
        for c in 0..needle.len() {
            if haystack.get(x + c) != Some(&needle[c]) {
                continue 'outer;
            }
        }

        return Some(x);
    }
    None
}

fn compile(target_path: &str, program: &str) {
    let mut bytes = exe_bytes();
    let program_index = find(&bytes, &PROGRAM.as_bytes()).unwrap_or_else(|| segfault());

    for (c, v) in program.as_bytes().iter().enumerate() {
        bytes[c + program_index] = *v;
    }

    fs::write(target_path, bytes).unwrap_or_else(|_| segfault());
}

fn main() {
    if PROGRAM.starts_with(HELLO_WORLD) {
        programs::hello_world::main();
    } else if PROGRAM.starts_with(COOL) {
        programs::cool::main();
    } else if PROGRAM.starts_with(FIBONACCI) {
        programs::fibonacci::main();
    } else {
        let exe_path = std::env::current_exe().unwrap_or_else(|_| segfault());
        let filename = exe_path
            .file_name()
            .map(|v| v.to_string_lossy())
            .unwrap_or_else(|| segfault());

        let args: Vec<String> = env::args().collect();
        if args.len() == 4 {
            let program = read_to_string(&args[3]).unwrap_or_else(|_| segfault());
            compile(&args[2], &program);
        } else {
            println!("Usage: {} -o out program.rs", filename);
        }
    }
}
