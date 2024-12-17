use std::env;
use std::fs;
use std::io::Write;

use codegen::CodeGenerator;
use scanner::LexingError;

pub mod codegen;
pub mod lowering;
pub mod parser;
pub mod regalloc;
pub mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    run_file(file_path);
}

fn run_file(file_path: &String) {
    let contents = fs::read_to_string(file_path).expect("Unable to read file");
    run(&contents);
}

fn run(source: &str) {
    let mut source_scanner = scanner::Scanner::new(source);
    let tokens = match source_scanner.scan_tokens() {
        Ok(x) => x,

        Err(LexingError::UnexpectedCharacter { line }) => error(line, "Unexpected Char"),
        Err(LexingError::UnterminatedString { line }) => error(line, "Unterminated Str"),
    };

    //dbg!(&tokens);

    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse().unwrap();

    dbg!(&ast);

    let low_ir = lowering::lower(ast);

    dbg!(&low_ir);

    let reg = regalloc::allocate_registers(low_ir);

    dbg!(&reg);

    let mut generator = CodeGenerator::new();
    let code = generator.generate(reg);

    dbg!(&code);

    let mut file = fs::File::create("build/out.asm").unwrap();
    file.write_all(code.as_bytes()).unwrap();
}

fn error(line: i32, message: &str) -> ! {
    report(line, "", message);
}

fn report(line: i32, position: &str, message: &str) -> ! {
    println!("[line {line}] Error {position}: {message}");
    std::process::exit(1);
}
