use std::io::{self, Write};

fn main () {

    print!("Enter text to convert to binary: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let ascii_values = str_to_ascii(&input);
    let binary_values = to_binary_vec(&ascii_values);

    for (idx, i) in binary_values.iter().enumerate() {
        if idx == binary_values.len() - 1 {
            break;
        }

        print!("{} ",i);
        io::stdout().flush().unwrap();
    }
    print!("\n")
}

fn str_to_ascii (s: &String) -> Vec<u32> {

    let mut ans = vec![];

    for c in s.chars() {
        ans.push(c as u32);
    }

    return ans;
}

fn to_binary_vec (vec: &Vec<u32>) -> Vec<u32> {

    let mut binary = vec![];

    for i in vec.iter() {
        binary.push(num_to_binary(&i));
    }

    return binary;
}

fn num_to_binary (num: &u32) -> u32 {
    
    if *num == 0 {
        return  0 as u32;
    }

    (*num % 2) + 10 * num_to_binary(&(*num / 2))
}