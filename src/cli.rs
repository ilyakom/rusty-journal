use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Add task to ToDo List
    Add {
        #[structopt()] 
        text: String
    },

    /// Remove task from ToDo list (by position)
    Done { position: usize},

    /// Print all the tasks 
    List
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Journal",
    about = "Command Line ToDo List"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>
}