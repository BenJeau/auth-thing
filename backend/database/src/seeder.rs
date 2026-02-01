use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let seed_files = get_seed_files();
    let contents = read_seed_files(&seed_files)?;
    let pool = connect_to_db().await?;
    run_migrations(&pool).await?;
    create_seed_table(&pool).await?;
    run_seed(&pool, &contents, &seed_files).await?;
    Ok(())
}

fn get_seed_files() -> Vec<String> {
    std::env::args()
        .nth(1)
        .unwrap_or_else(|| {
            eprintln!("Usage: seeder <seed_file>");
            std::process::exit(1);
        })
        .split(',')
        .map(|s| s.to_string())
        .collect()
}

fn read_seed_files(seed_files: &[String]) -> Result<Vec<String>, Box<dyn Error>> {
    let mut contents = Vec::new();
    for seed_file in seed_files {
        println!("Opening seed file '{}'", seed_file);
        let seed_file = File::open(format!("{}/seeds/{seed_file}", env!("CARGO_MANIFEST_DIR")))?;
        let mut reader = BufReader::new(seed_file);
        let mut file_contents = String::new();
        reader.read_to_string(&mut file_contents)?;
        contents.push(file_contents);
    }
    Ok(contents)
}

async fn connect_to_db() -> Result<sqlx::SqlitePool, Box<dyn Error>> {
    println!("Connecting to database via the env var DATABASE_URL");
    let pool = database::connect_to_db(std::env::var("DATABASE_URL")?.as_str(), 1, 1).await?;
    Ok(pool)
}

async fn run_migrations(pool: &sqlx::SqlitePool) -> Result<(), Box<dyn Error>> {
    println!("Running migrations");
    database::run_migrations(pool).await?;
    Ok(())
}

async fn create_seed_table(pool: &sqlx::SqlitePool) -> Result<(), Box<dyn Error>> {
    println!("Creating seed table if it doesn't exist");
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS _seeds (id INTEGER PRIMARY KEY AUTOINCREMENT, timestamp DATETIME DEFAULT CURRENT_TIMESTAMP, seed_file TEXT)",
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn run_seed(
    pool: &sqlx::SqlitePool,
    contents: &[String],
    seed_files: &[String],
) -> Result<(), Box<dyn Error>> {
    for (index, (seed_file, content)) in seed_files.iter().zip(contents.iter()).enumerate() {
        let count = sqlx::query_scalar("SELECT COUNT(*) FROM _seeds WHERE seed_file = ?")
            .bind(seed_file)
            .fetch_optional(pool)
            .await?
            .unwrap_or(0);

        if count > 0 {
            println!(
                "Skipping seed file {} of {} '{}' because it has already been run",
                index + 1,
                seed_files.len(),
                seed_file
            );
            return Ok(());
        }

        println!(
            "Running seed file {} of {} '{}'",
            index + 1,
            seed_files.len(),
            seed_file
        );
        let mut tx = pool.begin().await?;
        sqlx::query(&content).execute(&mut *tx).await?;
        sqlx::query("INSERT INTO _seeds (seed_file) VALUES (?)")
            .bind(seed_file)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
    }
    Ok(())
}
