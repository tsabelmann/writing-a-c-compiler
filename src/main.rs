use std::{os::unix::process::CommandExt, path::PathBuf, str::FromStr};
use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("wacc - Writing a C compiler")
        .version("0.1.0")
        .about("My own little C compiler")
        .arg(Arg::new("file").required(true))
        .arg(
            Arg::new("lex")
                .help("Abort after lexer phase")
                .short('l')
                .long("lex")
                .action(ArgAction::SetTrue)
                .required(false)
        )   
        .arg(
            Arg::new("parse")
                .help("Abort after parser phase")
                .short('p')
                .long("parse")
                .action(ArgAction::SetTrue)
                .required(false)
        )       
        .arg(
            Arg::new("codegen")
                .help("Abort after codgen phase")
                .short('c')
                .long("codegen")
                .action(ArgAction::SetTrue)
                .required(false)
        )   
        .get_matches();

    println!("file: {:?}", matches.get_one::<String>("file"));
   
    let should_abort_after_lex = matches.get_one::<bool>("lex").unwrap_or(&false).to_owned();
    let should_abort_after_parse = matches.get_one::<bool>("parse").unwrap_or(&false).to_owned();
    let should_abort_after_codegen = matches.get_one::<bool>("codegen").unwrap_or(&false).to_owned();

    println!("{}, {}, {}", should_abort_after_lex, should_abort_after_parse, should_abort_after_codegen);


    

    let input_file_path = match matches.get_one::<String>("file") {
        Some(path_string) => match PathBuf::from_str(path_string) {
            Ok(path) => match path.canonicalize() {
                Ok(path) => path,
                _ => std::process::exit(-1)
            },
            _ => std::process::exit(-2)
        },
        None => std::process::exit(-3),
    };

    let folder_path = match input_file_path.parent() {
        Some(path) => path,
        None => std::process::exit(-4),
    };

    let input_file_name = match input_file_path.file_name() {
        Some(file_name) => file_name,
        None => std::process::exit(-5),
    };

    let mut output_file_path = folder_path.join(input_file_name);
    let _ = output_file_path.set_extension("i");

    println!("Folder: {}", folder_path.display());

    // Preprocessor
    let result = std::process::Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(input_file_path)
        .arg("-o")
        .arg(output_file_path)
        .exec();
    println!("Error: {:?}", result);



    // println!("Path: {:?}", file_path);
}
