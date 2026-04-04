use clap::{Parser, Subcommand};
mod config;

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

fn init_tracing() {
    use tracing_subscriber::EnvFilter;
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_env("BROM_LOG").unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
}

fn main() {
    init_tracing();
    let cli = Cli::parse();

    match cli.command {
        Commands::Migrate => {
            let config = config::AppConfig::load();
            let db_path = &config.db_path;

            let pool = match brom_db::DbPool::new(db_path) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Failed to connect to database '{db_path}': {e}");
                    std::process::exit(1);
                }
            };

            let runner = brom_db::MigrationRunner::new(&pool);

            if let Err(e) = runner.ensure_internal_tables() {
                eprintln!("Failed to initialize internal tables: {e}");
                std::process::exit(1);
            }

            let migrations_dir = std::path::Path::new("migrations");
            match runner.run_pending(migrations_dir) {
                Ok(applied) => {
                    if applied.is_empty() {
                        println!("No pending migrations.");
                    } else {
                        for version in applied {
                            println!("Applied migration: {version}");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Migration failed: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::New { name } => {
            println!("STUB(Phase 2): Scaffold new project '{name}'");
        }
    }
}
