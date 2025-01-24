use clap::Subcommand;

#[derive(Subcommand, PartialEq, Debug)]
enum CompletionCommand {
    #[clap(about = "Generate auto-completions for Bash")]
    Bash,

    #[clap(about = "Generate auto-completions for Zsh")]
    Zsh,

    #[clap(about = "Generate auto-completions for Fish")]
    Fish,

    #[clap(name = "powershell", about = "Generate auto-completions for PowerShell")]
    PowerShell,
}
