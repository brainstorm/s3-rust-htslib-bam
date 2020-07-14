use url::Url;

use serde_json::json;
use lambda_http::{
    handler,
    lambda::{self, Context},
    IntoResponse, Request//, RequestExt, Response,
};
use crate::reader::BamReader;
use rust_htslib::htslib;

pub mod reader;
pub mod errors;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(bam_header)).await?;
    Ok(())
}

async fn bam_header(_event: Request, _: Context) -> Result<impl IntoResponse, Error> {
    const BUCKET: &str = "umccr-research-dev";
    const KEY: &str = "htsget/htsnexus_test_NA12878.bam";

    // Get some lowlevel libcurl action on hfile_curl/s3 from htslib
    // WARNING: Disable for production use as it prints out secret tokens on CloudWatch!
    // hts_set_log_level(10);
    let bam_head = bam_header_s3(BUCKET, KEY).await;

    Ok(json!(bam_head))
}

async fn bam_header_s3(bucket: &str, key: &str) -> Vec<String> {
    let s3_url = Url::parse(&("s3://".to_string() + &bucket + "/" + &key)).unwrap();
    let reader = BamReader::new(s3_url);
    return reader.unwrap().target_names();
}

pub fn hts_set_log_level(level: u32) {
    unsafe {
        htslib::hts_set_log_level(level);
    }
}