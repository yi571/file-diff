use structopt::StructOpt;

mod cli;
pub mod hashing;
use cli::CommandLineArgs;

mod compare;
mod init;
mod list;
mod record;

fn main() {
    let command_line_args = CommandLineArgs::from_args();

    match command_line_args.action {
        cli::Action::Init { path } => init::init_record(path),
        cli::Action::Record { message, path } => record::record(message, path),
        cli::Action::List { path } => list::get_csv_list(path),
        cli::Action::Compare {
            path,
            origin_item,
            target_item,
            export_path,
        } => compare::compare(path, origin_item, target_item, export_path),
    }
}
