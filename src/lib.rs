use std::{io, fs, error::Error, io::Read};

#[derive(Debug)]
enum Types {
    LeftPointer,
    RightPointer,
    Add,
    Subtract,
    Print,
    Input,
    LoopBegin,
    LoopEnd,
}

pub struct Interpreter {
    type_array: Vec<Types>,
    array: [u8; 30000],
    pointer: usize,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            type_array: Vec::new(),
            array: [0u8; 30000],
            pointer: 0,
        }
    }

    pub fn run(mut self, arguments: Vec<String>) -> Result<(), Box<dyn Error>> {
        let filename = &arguments[1];
        let lines = fs::read_to_string(filename)?;
        
        self.parse(&lines);
        self.interpret()?;
        Ok(())
    }

    fn parse(&mut self, lines: &str) {
        for c in lines.chars() {
            self.type_array.push(
                match c {
                '<' => Types::LeftPointer,
                '>' => Types::RightPointer,
                '+' => Types::Add,
                '-' => Types::Subtract,
                '.' => Types::Print,
                ',' => Types::Input,
                '[' => Types::LoopBegin,
                ']' => Types::LoopEnd,
                _ => continue,
            });
        }
    }

    fn interpret(&mut self) -> Result<(), Box<dyn Error>> {
        let mut loop_vec: Vec<usize> = Vec::new();
        let mut loop_in: usize = 0;
        let len = self.type_array.len();
        let mut i: usize = 0;
        while i < len {
            match self.type_array[i] {
                Types::LeftPointer => self.pointer -= 1,
                Types::RightPointer => self.pointer += 1,
                Types::Add => self.array[self.pointer] = self.array[self.pointer].wrapping_add(1),
                Types::Subtract => self.array[self.pointer] = self.array[self.pointer].wrapping_sub(1),
                Types::Print => print!("{}", self.array[self.pointer] as char),
                Types::Input => io::stdin().read_exact(&mut self.array[self.pointer..self.pointer + 1])?,
                Types::LoopBegin => { 
                    loop_vec.push(i);
                    loop_in += 1;
                },
                Types::LoopEnd => {
                    if self.array[self.pointer] != 0 {
                        i = loop_vec[loop_in - 1];
                    }
                    else {
                        loop_in -= 1;
                        loop_vec.pop();
                    }
                },
            }
            i += 1;
        }
        if loop_in != 0 {
            return Err("Bracket pair left unclosed".into());
        }

        Ok(())
    }
}