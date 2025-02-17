mod backup;
mod encryption;
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

    let encrypted_file = encryption::encrypt_file(&backup_file, config.encryption_key.as_bytes())?;
    println!("Encrypted backup saved: {}", encrypted_file);

    let file_name = std::path::Path::new(&encrypted_file)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    Ok(())
}
