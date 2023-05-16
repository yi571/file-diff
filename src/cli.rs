use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// init
    #[structopt()]
    Init {
        /// folder path
        #[structopt(short, long, parse(from_os_str))]
        path: Option<PathBuf>,
    },
    /// record update
    Record {
        /// record update message
        #[structopt(short, long)]
        message: String,

        /// folder path
        #[structopt(short, long, parse(from_os_str))]
        path: Option<PathBuf>,
    },
    /// get records
    List {
        /// folder path
        #[structopt(short, long, parse(from_os_str))]
        path: Option<PathBuf>,
    },
    /// compare two records
    Compare {
        /// folder path
        #[structopt(short, long, parse(from_os_str))]
        path: Option<PathBuf>,

        /// origin item
        #[structopt(short, long)]
        origin_item: String,

        /// target item
        #[structopt(short, long)]
        target_item: String,

        /// export folder path
        #[structopt(short, long, parse(from_os_str))]
        export_path: Option<PathBuf>,
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "check file hash", about = "calculate and record file hash")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
}
