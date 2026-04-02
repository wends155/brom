use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "brom",
    about = "Headless CMS macro engine and toolkit",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Applies database migrations
    Migrate,
    /// Scaffolds a new brom project
    New { name: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Migrate => {
            println!("STUB(Phase 2): Run migrations");
        }
        Commands::New { name } => {
            println!("STUB(Phase 2): Scaffold new project '{name}'");
        }
    }
}
