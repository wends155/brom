use brom_auth::password::hash_password;
use brom_db::{DbPool, MigrationRunner};
use rand::{Rng as _, distributions::Alphanumeric};

fn resolve_db_path() -> String {
    std::env::var("DATABASE_URL").unwrap_or_else(|_| "brom.db".to_string())
}

fn resolve_email() -> String {
    std::env::var("BROM_ADMIN_EMAIL").unwrap_or_else(|_| "admin@example.com".to_string())
}

fn resolve_password() -> String {
    match std::env::var("BROM_ADMIN_PASSWORD") {
        Ok(p) if !p.trim().is_empty() => p,
        _ => rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(24)
            .map(char::from)
            .collect(),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = resolve_db_path();
    let email = resolve_email();
    let password = resolve_password();
    let password_hash = hash_password(&password)?;

    let pool = DbPool::new(&db_path)?;
    let runner = MigrationRunner::new(&pool);
    runner.ensure_internal_tables()?;

    let conn = pool.get()?;
    conn.execute(
        "INSERT OR IGNORE INTO _brom_user (email, password_hash, created_at, updated_at) \
         VALUES (?1, ?2, datetime('now'), datetime('now'))",
        (&email, &password_hash),
    )?;

    println!("Seeded admin user (or already existed).");
    println!("DB: {db_path}");
    println!("Email: {email}");
    println!("Password: {password}");
    println!("Login URL: http://localhost:3000/admin");

    Ok(())
}
