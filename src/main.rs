use url::Url;
use crate::reader::BamReader;
use rust_htslib::htslib;

pub mod reader;
pub mod errors;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const BUCKET: &str = "umccr-research-dev";
    const KEY: &str = "htsget/htsnexus_test_NA12878.bam";
    // const BUCKET: &str = "gatk-test-data";
    // const KEY: &str = "wgs_bam/NA12878_24RG_hg38/NA12878_24RG_small.hg38.bam";

    // Get some lowlevel libcurl action on hfile_curl/s3 from htslib
    // Disable for production use as it prints out secret tokens on CloudWatch!
    hts_set_log_level(10);
    let res = bam_header_s3(BUCKET, KEY);

    // Show the BAM header targets on stdout
    dbg!("BAM header targets are: {}", res);
    Ok(())
}

pub fn bam_header_s3(bucket: &str, key: &str) -> Vec<String> {
    let s3_url = Url::parse(&("s3://".to_string() + &bucket + "/" + &key)).unwrap();
    let reader = BamReader::new(s3_url);
    return reader.unwrap().target_names();
}

pub fn hts_set_log_level(level: u32) {
    unsafe {
        htslib::hts_set_log_level(level);
    }
}