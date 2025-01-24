mod commands;

#[cfg(debug_assertions)]
extern crate better_panic;
extern crate clap;
extern crate clap_complete;
#[cfg(not(debug_assertions))]
use human_panic::setup_panic;

extern crate imagenex831l;
use imagenex831l::Result;

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::shells::PowerShell;
use clap_complete::{
    generate,
    shells::{Bash, Fish, Zsh},
};

#[derive(Parser, Debug)]
#[command(
    name = "i831",
    author,
    about,
    long_about = "An unofficial interface for interacting with data from IMAGENEX 831L Pipe Profiling Sonars.",
    version
)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(ValueEnum, Copy, Clone, Debug, Eq, PartialEq)]
pub enum ToFormats {
    CSV,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(name = "gui", about = "Show the Graphical User Interface", long_about = None)]
    GUI,

    // #[clap(name = "completion", about = "Generation completion script.", long_about = None)]
    // Completion {
    //     #[clap(subcommand)]
    //     subcommand: CompletionCommand,
    // },
    #[clap(name = "convert", about = "Convert sonar files to other formats.")]
    Convert { to: ToFormats },
}

pub fn cli_match() -> Result<()> {
    // Parse the incoming command-line arguments
    let cli = Cli::parse();

    let app = Cli::command();
    let matches = app.get_matches();

    match &cli.command {
        Commands::GUI => panic!("Hello"),
        // Commands::Completion { subcommand } => {
        //     let mut app = Cli::command();
        //     match subcommand {
        //         CompletionCommand::Bash => {
        //             generate(Bash, &mut app, "i831", &mut std::io::stdout());
        //         },
        //         CompletionCommand::Zsh => {
        //             generate(Zsh, &mut app, "i831", &mut std::io::stdout());
        //         },
        //         CompletionCommand::Fish => {
        //             generate(Fish, &mut app, "i831", &mut std::io::stdout());
        //         },
        //         CompletionCommand::PowerShell => {
        //             generate(PowerShell, &mut app, "i831", &mut std::io::stdout());
        //         },
        //         _ => (),
        //     }
        // },
        Commands::Convert => {},
    }

    Ok(())
}

/// The main entry point of the application.
fn main() -> () {
    // Human Panic. Only enabled when *not* debugging.
    #[cfg(not(debug_assertions))]
    {
        setup_panic!();
    }

    // Better Panic. Only enabled *when* debugging.
    #[cfg(debug_assertions)]
    {
        better_panic::Settings::debug()
            .most_recent_first(false)
            .lineno_suffix(true)
            .verbosity(better_panic::Verbosity::Full)
            .install();
    }

    let cli = Cli::parse();
}
