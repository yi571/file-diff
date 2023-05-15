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
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "check file hash", about = "計算並紀錄檔案哈希")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
}
