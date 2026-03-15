mod encode;
mod decode;

use encode::{encode};
use decode::{decode};
use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(name = "bincodec", about = "Binary encoder/decoder")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Encode {
        #[arg(trailing_var_arg = true)]
        input: Vec<String>,
    },
    Decode {
        #[arg(trailing_var_arg = true)]
        input: Vec<String>,
    },
}
trait Codec {
    fn encode(&self, input: &str) -> String;
    fn decode(&self, input: &str) -> Result<String, String>;
}

struct Converter;

impl Codec for Converter {
    fn encode(&self, input: &str) -> String {
        encode(input)
    }

    fn decode(&self, input: &str) -> Result<String, String> {
        decode(input)
    }
}

// impl Converter {
//     fn run(&self, method: Methods, input: &str) {
//         match method {
//             Methods::Encode => {
//                 println!("{}", self.encode(input))
//             },

//             Methods::Decode => match self.decode(input) {
//                 Ok(s) => println!("{}", s),
//                 Err(e) => eprintln!("Error: {}", e),
//             }
//         }
//     }
// }

fn main () {

    let cli = Cli::parse();

    // print!("Mode: ");
    // io::stdout().flush().unwrap();
    // let mut mode = String::new();
    // io::stdin().read_line(&mut mode).expect("Failed to read line");

    // print!("Enter text to convert to binary: ");
    // io::stdout().flush().unwrap();
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("Failed to read line");

    // match mode.trim() {
    //     "encode" => Converter.run(Methods::Encode, input.trim()),
    //     "decode" => Converter.run(Methods::Decode, input.trim()),
    //     _ => println!("Unknown mode"),
    // }


    match cli.command {
        Command::Encode { input } => println!("{}", Converter.encode(&input.join(" "))),
        Command::Decode { input } => match Converter.decode(&input.join(" ")) {
            Ok(s)  => println!("{}", s),
            Err(e) => eprintln!("Error: {}", e),
        },
    }
}