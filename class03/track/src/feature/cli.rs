use clap::Parser;

#[derive(Debug, thiserror::Error)]
#[error("CLI error occurred.")]
pub struct CliError;

#[derive(Debug, Clone, Copy, clap::Subcommand)]
pub enum Command {
    /// Start tracking time.
    Start,
    // Stop,
    // Report,
}

#[derive(Debug, clap::Parser)]
#[command(version, about, arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

pub fn run() -> Result<(), CliError> {
    // match command_builder().subcommand() {
    // 	Some(("start", sub_matches)) => {
    // 		// todo!()
    // 	}
    // 	_ => unreachable!(),
    // }
    let args = Cli::parse();
    match args.command {
        Command::Start => {
            todo!();
        }
    }

    // Ok(())
}

// fn command_builder() -> clap::ArgMatches {
// 	clap::Command::new("track")
// 		.subcommand(clap::Command::new("start").about("Start tracking time"))
// 		.subcommand(clap::Command::new("stop").about("Stop tracking time"))
// 		.subcommand(clap::Command::new("report").about("Generate a report"))
// 		.arg(
// 			clap::Arg::new("fluffy")
// 				.short('f')
// 				.long("fluffy")
// 				.required(true)
// 				.help("Good boy"),
// 		)
// 		.subcommand_required(true)
// 		.arg_required_else_help(true)
// 		.get_matches()
// }
