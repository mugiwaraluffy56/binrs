mod encode;
mod decode;

use encode::{str_to_ascii, to_binary_vec};
use decode::{binary_to_num, ascii_to_str};
use std::{fs::{self, File}, io::{self, Write}};

trait Codec {
    fn encode(&self, input: &String) -> Vec<u32>;
    // fn decode(&self, input: &Vec<u32>) -> String;
}

enum Methods {
    Encode,
    // Decode,
}
// enum CodecResult {
//     Encoded(Vec<u32>),
//     Decoded(String),
// }

struct Converter;

impl Codec for Converter {
    fn encode(&self, input: &String) -> Vec<u32> {
        let ascii_values = str_to_ascii(&input);
        let binary_values = to_binary_vec(&ascii_values);

        return binary_values;
    }

    // fn decode(&self, input: &Vec<u32>) -> String {
    //     let ascii_values: Vec<u32> = input.iter().map(|&b| binary_to_num(b)).collect();
    //     ascii_to_str(&ascii_values)
    // }

}

impl Converter {
    fn run(&self, method: Methods, input: &String) -> Vec<u32>{
        match method {
            Methods::Encode => self.encode(input),
            // Methods::Decode => self.decode(input),
        }
    }
}

// impl Converter {
//     fn run(&self, method: Methods, input: &String, encoded: Option<&Vec<u32>>) -> CodecResult {
//         match method {
//             Methods::Encode => CodecResult::Encoded(self.encode(input)),
//             Methods::Decode => CodecResult::Decoded(self.decode(encoded.unwrap())),
//         }
//     }
// }



fn main () {

    print!("Enter text to convert to binary: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    // let input = fs::read_to_string("./src/input.txt").expect("Failed to read line");

    let binary_values = Converter.run(Methods::Encode, &input);

    // let mut file = File::create("./src/numbers.txt").unwrap();
    // for n in &binary_values {
        // file.write_all(&n.to_le_bytes()).unwrap();
    // }
    // for n in &binary_values {
        // if *n == 1010 {
            // writeln!(file, "{} ", n).unwrap();
            // continue;
        // }
        // write!(file, "{} ", n).unwrap();
    // }

    for (idx, i) in binary_values.iter().enumerate() {
        if idx == binary_values.len() - 1 {
            break;
        }

        print!("{} ",i);
        io::stdout().flush().unwrap();
    }
    print!("\n")

}