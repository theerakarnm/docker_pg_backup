use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::{Client, Region};
use std::error::Error;
use std::path::Path;

/// Uploads the file at `file_path` to the specified S3 bucket and key.
pub async fn upload_file_to_s3(
    file_path: &str,
    bucket: &str,
    key: &str,
    aws_region: &str,
) -> Result<(), Box<dyn Error>> {
    // Set up the region provider and load AWS configuration.
    let region_provider = RegionProviderChain::first_try(Region::new(aws_region.to_string()));
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    // Read the file as a ByteStream for upload.
    let path = Path::new(file_path);
    let body = ByteStream::from_path(path).await?;

    // Upload the object to S3.
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body)
        .send()
        .await?;

    Ok(())
}
