use std::env;

fn main() {
    let help_info = 
        CliInfo {
            version: String::from("Version: 0.1.0\n"),
            author: String::from("Author: pr0xyz\n"),
            about: String::from("Rust echo implementation\n"),
            usage: String::from("USAGE:\n\techor\n"),
            flags: String::from("FLAGS:\n\t-h, --help\t\tPrints help information\n\t-V, --version\t\tPrints version information"),
        };
        
    let args: Vec<String> = env::args().collect();
    
    let mut output = String::new();

    let mut check_vec: Vec<String> = Vec::new();

    for arg in args.iter().skip(1){
        if arg.starts_with('-') {
            check_vec.insert(0, arg.to_string());
        }
        check_vec.push(arg.to_string());
    }

    for arg in check_vec.iter() {
        match arg.as_str() {
            "-h" | "--help" => { println!("{}{}{}{}{}", 
            help_info.author, 
            help_info.version, 
            help_info.about,
            help_info.usage,
            help_info.flags); break},
            "-V" | "--version" => { println!("{}", help_info.version); break},
            arg if arg.starts_with('-') => { println!("{} is not a recognized flag. press -h or --help for information", arg); break;},
            arg => { output.push_str(arg); output.push_str(" ")},
        }
    }

    if !output.is_empty() {
        println!("{}", output.trim());
    }

}

#[derive(Debug)]
struct CliInfo {
    version: String,
    author: String,
    about: String,
    usage: String,
    flags: String,
}