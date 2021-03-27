use std::io::Cursor;

use lambda_runtime::{ handler_fn, Context, Error };
//use serde::{Deserialize, Serialize};
use serde_json::{ json, Value };
use simple_logger::SimpleLogger;
use log::LevelFilter;

use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;

use noodles_bam as bam;

struct Storage {
    region: Region,
    credentials: Credentials,
    bucket: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler_fn(s3_read_bam_header)).await?;
    Ok(())
}

async fn s3_read_bam_header(_event: Value, _ctx: Context) -> Result<Value, Error> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    let s3_object = stream_s3_object().await?;
    let output = read_bam_header(s3_object).await?;
    dbg!(&output);
    Ok(json!({ "message": &output }))
}

/// Fetches S3 object
async fn stream_s3_object() -> Result<Cursor<Vec<u8>>, Error> {
    let mut s3_obj_buffer = Cursor::new(Vec::new());
    let aws = Storage {
        region: Region::ApSoutheast2,
        credentials: Credentials::default()?,
        bucket: "umccr-research-dev".to_string(),
    };

    let bucket = Bucket::new(&aws.bucket, aws.region, aws.credentials)?;
    bucket.get_object_stream("/htsget/htsnexus_test_NA12878.bam", &mut s3_obj_buffer).await?;
    // Rewind buffer Cursor after writing, so that next reader can consume data 
    s3_obj_buffer.set_position(0);
    return Ok(s3_obj_buffer);
}

/// Reads BAM S3 object header
async fn read_bam_header(bam_bytes: Cursor<Vec<u8>>) -> Result<Value, Error> {
    let mut reader = bam::Reader::new(bam_bytes);
    let header = reader.read_header()?;
    Ok(json!({ "header": header,
               "message": "success" }))
}