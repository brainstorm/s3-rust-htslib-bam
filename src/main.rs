//use std::path::Path;
use url::Url;
use crate::reader::BamReader;
use rust_htslib::htslib;

pub mod reader;
pub mod errors;

fn main() {
//    const BUCKET: &str = "gatk-test-data";
//    const KEY: &str = "wgs_bam/NA12878_24RG_hg38/NA12878_24RG_small.hg38.bam";
    const BUCKET: &str = "umccr-research-dev";
    const KEY: &str = "htsget/htsnexus_test_NA12878.bam";
    hts_set_log_level(10);
    //hts_set_log_level(10); 
    dbg!(bam_header_s3(BUCKET, KEY));
//   dbg!(bam_header_local(Path::new("data/sample.bam")));
}

pub fn hts_set_log_level(level: u32) {
    unsafe {
        htslib::hts_set_log_level(level);
    }
}

pub fn bam_header_s3(bucket: &str, key: &str) -> Vec<String> {
    let s3_url = Url::parse(&("s3://".to_string() + &bucket + "/" + &key)).unwrap();
    let reader = BamReader::new(s3_url);
    return reader.unwrap().target_names();
}

// pub fn bam_header_local(fname: &Path) -> Vec<String> {
//     let reader = BamReader::new(fname);
//     return reader.unwrap().target_names();
// }
