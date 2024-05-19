use chrysalis::input_args::{Cli, Commands};

fn main() {
    let cli = Cli::default();

    match &cli.command {
        Commands::Sei(val) => {
            println!("{:?}", val);
        }
        Commands::Met(val) => {
            println!("{:?}", val);
        }
        Commands::Ast(val) => {
            println!("{:?}", val);
        }
    };
}
