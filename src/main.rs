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
use noodles_sam as sam;

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
    Ok(json!({ "message": output }))
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
    return Ok(s3_obj_buffer);
}

/// Reads BAM S3 object header
async fn read_bam_header(bam_bytes: Cursor<Vec<u8>>) -> Result<Value, Error> {
    let mut reader = bam::Reader::new(bam_bytes);
    let header = reader.read_header()?;

    if header.is_empty() {
        let reference_sequences = reader.read_reference_sequences()?;
        let mut builder = sam::Header::builder();

        for reference_sequence in reference_sequences {
            builder = builder.add_reference_sequence(reference_sequence);
        }

        Ok(json!({ "header": builder.build().to_string(),
                   "message": "success" }))
    } else {
        Ok(json!({ "header": header,
                   "message": "success" }))
    }
}