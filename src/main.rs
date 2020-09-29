use lambda::{handler_fn, Context};
use rust_htslib::{bam, bam::Read, htslib};
use serde_json::json;
use serde_json::Value;
use url::Url;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler_fn(bam_header)).await?;
    Ok(())
}

async fn bam_header(event: Value, _: Context) -> Result<Value, Error> {
    // we expect an event context like
    // {
    //   "bam": "s3://mybucket/mybam.bam"
    // }
    dbg!("Incoming lambda event is: ", &event);
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
    // hts_set_log_level(10);

    // Get some lowlevel libcurl action on hfile_curl/s3 from htslib and fetch the header
    let bam = bam::Reader::from_url(&s3_url).unwrap();
    let header = bam::Header::from_template(bam.header()).to_bytes();

    // and return the array as a JSON object
    Ok(json!(header))
}

pub fn _hts_set_log_level(level: u32) {
    unsafe {
        htslib::hts_set_log_level(level);
    }
}
