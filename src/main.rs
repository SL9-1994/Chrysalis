use std::path::Path;

use chrysalis::{
    input_args::{Cli, Commands},
    utils::{
        // io::read_dir,
        io::{read_files_in_dir, read_single_file},
        validation::{is_dir_or_file, PathType},
    },
};

//     constants--

// end constants--

fn main() {
    let cli = Cli::default();

    let sei_file_extensions: Vec<&str> = vec!["V2", "csv", "V1A", "asc", "UD", "NS", "EW"];
    let met_file_extensions: Vec<&str> = vec![];
    let ast_file_extensions: Vec<&str> = vec![];

    match &cli.command {
        Commands::Sei(val) => common_process_command(val.path.as_path(), sei_file_extensions),
        Commands::Met(val) => common_process_command(val.path.as_path(), met_file_extensions),
        Commands::Ast(val) => common_process_command(val.path.as_path(), ast_file_extensions),
    };
}

fn common_process_command(val: &Path, extensions: Vec<&str>) {
    match is_dir_or_file(val) {
        PathType::File => match read_single_file(val) {
            Ok(content) => println!("{:?}", content),
            Err(err) => eprintln!("Error reading file: {}", err),
        },
        PathType::Directory => match read_files_in_dir(val, extensions) {
            Ok(contents) => println!("{:?}", contents),
            Err(err) => eprintln!("Error reading directory: {}", err),
        },
        PathType::NotFound => eprintln!("Error determining if path is a file or directory"),
    }
}
