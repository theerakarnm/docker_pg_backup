mod backup;
mod settings;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = settings::Settings::new()?;

    let backup_file = backup::run_backup(
        &config.container_name,
        &config.db_name,
        &config.db_user,
        &config.backup_dir,
    )?;
    println!("Backup completed: {}", backup_file);

    Ok(())
}
