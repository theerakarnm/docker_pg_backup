mod settings; // if you placed the Settings struct in settings.rs
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = settings::Settings::new()?;

    Ok(())
}
