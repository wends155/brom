//! CLI entrypoint for the brom framework.
//! Provides commands to generate schemas, run migrations, and bootstrap apps.

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
    /// Shows the difference between migrations and current database state
    Diff,
    /// Scaffolds a new brom project
    New { name: String },
}

fn init_tracing() -> tracing_appender::non_blocking::WorkerGuard {
    use tracing_subscriber::EnvFilter;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    let file_appender = tracing_appender::rolling::daily("logs", "brom.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let env_filter = EnvFilter::try_from_env("BROM_LOG").unwrap_or_else(|_| EnvFilter::new("info"));

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false);

    let stdout_layer = tracing_subscriber::fmt::layer().with_ansi(true);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(stdout_layer)
        .with(file_layer)
        .init();

    guard
}

fn main() {
    let _guard = init_tracing();
    let cli = Cli::parse();

    match cli.command {
        Commands::Migrate => {
            let config = config::AppConfig::load();
            let db_path = &config.db_path;
            tracing::info!(%db_path, "Executing database migrations");

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
        Commands::Diff => {
            println!("STUB(Phase 4): Diff database schema against migrations");
        }
        Commands::New { name } => {
            println!("STUB(Phase 3+): Scaffold new project '{name}'");
        }
    }
}
