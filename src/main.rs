use std::path::Path;

use url::Url;
use crate::reader::BamReader;

pub mod reader;
pub mod errors;

fn main() {
    const BUCKET: &str = "gatk-test-data";
    const KEY: &str = "wgs_bam/NA12878_24RG_hg38/NA12878_24RG_small.hg38.bam";

    dbg!(bam_header_s3(BUCKET, KEY));
//   dbg!(bam_header_local(Path::new("data/sample.bam")));
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