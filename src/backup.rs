use chrono::Local;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process::Command;

pub fn run_backup(
    container_name: &str,
    db_name: &str,
    db_user: &str,
    backup_dir: &str,
) -> Result<String, Box<dyn Error>> {
    // Ensure the backup directory exists
    std::fs::create_dir_all(backup_dir)?;

    // Create a timestamped filename
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let backup_file = format!("{}/backup_{}_{}.sql", backup_dir, db_name, timestamp);

    println!("Running pg_dump and saving to: {}", backup_file);

    // Run pg_dump using Docker exec
    let output = Command::new("docker")
        .args(&[
            "exec",
            "-t",
            container_name,
            "pg_dump",
            "-U",
            db_user,
            db_name,
        ])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "pg_dump failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let mut file = File::create(&backup_file)?;
    file.write_all(&output.stdout)?;

    Ok(backup_file)
}
