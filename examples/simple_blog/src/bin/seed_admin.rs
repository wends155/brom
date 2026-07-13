use brom_auth::password::hash_password;
use brom_db::{DbPool, MigrationRunner};
use rand::{Rng as _, distributions::Alphanumeric};

struct Config {
    db_path: String,
    email: String,
    password: String,
}

impl Config {
    #[rustfmt::skip]
    fn load() -> Self {
        // ast-grep-ignore
        let db_path = std::env::var("DATABASE_URL").unwrap_or_else(|_| "brom.db".to_string());
        // ast-grep-ignore
        let email = std::env::var("BROM_ADMIN_EMAIL").unwrap_or_else(|_| "admin@example.com".to_string());
        // ast-grep-ignore
        let password = match std::env::var("BROM_ADMIN_PASSWORD") {
            Ok(p) if !p.trim().is_empty() => p,
            _ => rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(24)
                .map(char::from)
                .collect(),
        };

        Config {
            db_path,
            email,
            password,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load();
    let password_hash = hash_password(&config.password)?;

    let pool = DbPool::new(&config.db_path)?;
    let runner = MigrationRunner::new(&pool);
    runner.ensure_internal_tables()?;

    let conn = pool.get()?;
    conn.execute(
        "INSERT OR IGNORE INTO _brom_user (email, password_hash, created_at, updated_at) \
         VALUES (?1, ?2, datetime('now'), datetime('now'))",
        (&config.email, &password_hash),
    )?;

    println!("Seeded admin user (or already existed).");
    println!("DB: {}", config.db_path);
    println!("Email: {}", config.email);
    println!("Password: {}", config.password);
    // ast-grep-ignore
    println!("Login URL: http://localhost:3000/admin");

    Ok(())
}
