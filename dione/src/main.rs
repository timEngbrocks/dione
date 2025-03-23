use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
	#[command(subcommand)]
	command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
	ParseClassFile(ParseClassFileArgs),
}

#[derive(Args, Debug)]
struct ParseClassFileArgs {
	file: String
}

fn main() {
    let cli_args = CliArgs::parse();
    match cli_args.command {
        Commands::ParseClassFile(ParseClassFileArgs { file }) => {
            let class_file = class_file_parser::parse(file);
            println!("{class_file}");
        },
    }
}
