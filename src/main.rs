use structopt::StructOpt;

mod cli;
pub mod hashing;
use cli::CommandLineArgs;

mod init;
mod record;


fn main() {
    let command_line_args = CommandLineArgs::from_args();


    match command_line_args.action {
        cli::Action::Init { path } => init::init_record(path),
        cli::Action::Record { message, path } => record::record(message, path),
        
    }
    
}
