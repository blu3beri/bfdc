use std::error::Error;
use std::{env, fs};

const USAGE: &str = "USAGE: bfdc <SOURCE> <LANG>";

fn main() {
    match start() {
        Ok(a) => println!("{:?}", a),
        Err(e) => eprintln!("{}", e),
    }
}

fn start() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err(USAGE.into());
    }
    let file = fs::read_to_string(&args[1])?;
    let lang = fs::read_to_string(&args[2])?.replace("\n", "");
    let vec_lang: Vec<char> = lang.chars().collect();
    let vec_file: Vec<char> = file.chars().filter(|c| vec_lang.contains(c)).collect();

    compile(vec_file, vec_lang)
}

fn compile(program: Vec<char>, lang: Vec<char>) -> Result<String, Box<dyn Error>> {
    assert!(
        program.iter().filter(|p| **p == lang[6]).count()
            == program.iter().filter(|p| **p == lang[7]).count(),
        "INVALID CHAR COUNT FOR JUMPS"
    );
    let mut arr: Vec<u8> = vec![0u8; 100000];
    let mut result: Vec<u8> = vec![];
    let mut data_pointer = 0;
    let mut instruction_pointer = 0;

    loop {
        if let Some(instruction) = program.get(instruction_pointer) {
            if *instruction == lang[0] {
                //
                data_pointer = data_pointer + 1;
                //
            } else if *instruction == lang[1] {
                //
                data_pointer = data_pointer - 1;
                //
            } else if *instruction == lang[2] {
                arr[data_pointer] = arr[data_pointer] + 1;
            } else if *instruction == lang[3] {
                arr[data_pointer] = arr[data_pointer] - 1;
            } else if *instruction == lang[4] {
                result.push(arr[data_pointer]);
            } else if *instruction == lang[5] {
                todo!();
            } else if *instruction == lang[6] {
                if arr[data_pointer] == 0 {
                    let next_seven = program[instruction_pointer..]
                        .iter()
                        .position(|i| *i == lang[7])
                        .expect("COULD NOT FIND NEXT JUMP-TO POINT");

                    instruction_pointer = next_seven;
                }
            } else if *instruction == lang[7] {
                if arr[data_pointer] != 0 {
                    let prev_six = program[..instruction_pointer]
                        .iter()
                        .rposition(|i| *i == lang[6])
                        .expect("COULD NOT FIND NEXT JUMP-BACK-TO POINT");

                    instruction_pointer = prev_six;
                }
            } else {
                panic!("UNEXPECTED CHARACTER: {}", instruction);
            }
            instruction_pointer = instruction_pointer + 1;
        } else {
            break;
        }
    }

    String::from_utf8(result).map_err(|e| e.into())
}
