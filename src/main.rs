use chrysalis::{
    input_args::{Cli, Commands},
    utils::validation::is_dir_or_file,
};

fn main() {
    let cli = Cli::default();
    match &cli.command {
        Commands::Sei(val) => {
            println!("{:?}", is_dir_or_file(&val.path));
        }
        Commands::Met(val) => {
            println!("{:?}", is_dir_or_file(&val.path));
        }
        Commands::Ast(val) => {
            println!("{:?}", is_dir_or_file(&val.path));
        }
    };
}
