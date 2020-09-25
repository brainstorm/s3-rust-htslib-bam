use lambda::{handler_fn, Context};
use rust_htslib::htslib;
use serde_json::json;
use serde_json::Value;
use url::Url;

use crate::reader::BamReader;

pub mod errors;
pub mod reader;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler_fn(bam_header)).await?;
    Ok(())
}

async fn bam_header(event: Value, _: Context) -> Result<Value, Error> {
    //
    // we expect an event context like
    // {
    //   "bam": "s3://mybucket/mybam.bam"
    // }

    let s3_url = match event.get("bam") {
        Some(a) => Url::parse(&a.as_str().unwrap()).unwrap(),
        None => {
            return Ok(
                json!({"error": "Must pass the S3 location of a BAM in as the 'bam' field of the lambda event"}),
            );
        }
    };

    println!("Testing header of BAM file {}", s3_url);

    // WARNING: Disable for production use as it prints out secret tokens to CloudWatch!
    hts_set_log_level(10);

    // Get some lowlevel libcurl action on hfile_curl/s3 from htslib
    let reader = BamReader::new(s3_url);

    let bam_head = reader.unwrap().target_names();

    // we both print out the bam header target names (will appear in lambda cloud watch logs)
    for i in &bam_head {
        println!("{}", i);
    }

    // and return the array as a JSON object
    Ok(json!(bam_head))
}

pub fn hts_set_log_level(level: u32) {
    unsafe {
        htslib::hts_set_log_level(level);
    }
}
