mod backup;
mod encryption;
mod s3_upload;
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
    let s3_key = format!("backups/{}", file_name);

    s3_upload::upload_file_to_s3(
        &encrypted_file,
        &config.s3_bucket,
        &s3_key,
        &config.aws_region,
    )
    .await?;
    println!(
        "Encrypted backup uploaded to S3 bucket '{}' with key '{}'",
        config.s3_bucket, s3_key
    );

    Ok(())
}
