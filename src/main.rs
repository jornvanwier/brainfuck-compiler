#[cfg_attr(test, feature(test))]
#[cfg(test)] extern crate test;

use std::fs::File;
use std::io::{self, Read};
use vm::VM;
use compiler::RawToken;

mod compiler;
mod vm;

fn main() -> io::Result<()> {
//    let mut buf = String::new();
//    File::open("mandelbrot.b")?.read_to_string(&mut buf)?;
    let mut buf = "+[->,----------]<[+++++++++++.<]";
    let tokens = compiler::lex(&mut buf.chars());
//    println!("{}, {:?}", tokens.len(), tokens);

    let mut output = io::stdout();
//    let mut output = File::create("mandelbrot.txt")?;

    let mut vm = VM::new(tokens, &mut output);
    vm.execute();

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_hello(b: &mut Bencher) {
        let mut buf = String::new();
        File::open("hello.b").unwrap().read_to_string(&mut buf).unwrap();
        let tokens = compiler::lex(&mut buf.chars());

        b.iter(|| {
            let mut output = io::stdout();
            let mut vm = VM::new(tokens.clone(), &mut output);
            vm.execute();
        }
        );
    }
}