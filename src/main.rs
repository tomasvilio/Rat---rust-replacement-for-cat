use std::fs::File; //to open and perform actions on files
use std::io::Read; //to convert the content of a file to string
use std::env; //to read arguments at compile time
use std::process::exit; //for closing the program


fn add_line_numbers(file_content:String) -> Vec<String>
{
    let mut lines = Vec::new(); 

    for (index, line) in file_content.lines().enumerate()
    {
        let updated_line = format!(" {:>4} │ {}", index + 1, line);
        lines.push(updated_line);
    }

    lines
}

fn main() 
{
    let mut arguments: Vec<String> = env::args().skip(1).collect();
    
    if arguments.is_empty()
    {
        println!("no file to read from! try the following syntax: rat <optional_flag> <your_file_name>.<your_file_extension>");
        exit(1);
    }

    for i in 0..arguments.len()
    {
        let mut file_data = File::open(&mut arguments[i]).unwrap();
        let mut file_content = String::new();
        file_data.read_to_string(&mut file_content).unwrap();
        
        if i != 0
        {
            println!("\nFile: {}\n", arguments[i]);
        } else
        {
            println!("File: {}\n", arguments[i]);
        }
        
        let file_content_formated = add_line_numbers(file_content);
            
        for item in &file_content_formated
        {
            println!("{}", item);
        }
    }
}
