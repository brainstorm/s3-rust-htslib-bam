use lambda_runtime::{handler_fn, Context};
use serde_json::Value;

use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use s3::S3Error;

struct Storage {
    name: String,
    region: Region,
    credentials: Credentials,
    bucket: String,
    location_supported: bool,
}

#[tokio::main]
async fn main() -> Result<(), S3Error> {
    lambda_runtime::run(handler_fn(bam_header)).await;
    let mut output = std::io::stdout();
    let aws = Storage {
        name: "aws".into(),
        region: "ap-southeast-2".parse()?,
        // credentials: Credentials::from_profile(Some("rust-s3"))?,
        credentials: Credentials::from_instance_metadata()?,
        bucket: "umccr-primary-data-dev".to_string(),
        location_supported: true,
    };
    let bucket = Bucket::new(&aws.bucket, aws.region, aws.credentials)?;
    bucket.get_object_stream("sample-file.bam", &mut output);
    // let (data, code) = bucket.get_object_blocking("test_file")?;
    // let string = str::from_utf8(&data)?;
    Ok(())
}

async fn bam_header(_: Value, _: Context) -> Result<(), S3Error> {
    Ok(())
}