use lambda_runtime::{ handler_fn, Context };
use serde_json::{ json, Value };

use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use s3::S3Error;

use noodles_bam as bam;
use noodles_sam as sam;

struct Storage {
    region: Region,
    credentials: Credentials,
    bucket: String,
}

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler_fn(cloud_lambda_bioinfo)).await;
    Ok(())
}

async fn cloud_lambda_bioinfo(_: Value, _: Context) -> Result<Value, Error> {
    let s3_object = stream_s3_object().await?;
    let output = bam_header(s3_object).await?;
    Ok(json!({ "message": output }))
}

async fn stream_s3_object() -> Result<u16, S3Error> {
    let mut output = std::io::stdout();
    let aws = Storage {
        region: "ap-southeast-2".parse()?,
        credentials: Credentials::from_instance_metadata()?,
        bucket: "umccr-primary-data-dev".to_string(),
    };

    let bucket = Bucket::new(&aws.bucket, aws.region, aws.credentials)?;
    return bucket.get_object_stream("sample-file.bam", &mut output).await;
}

async fn bam_header(bam_bytes: u16) -> Result<Value, Error> {
    let mut reader = bam::Reader::new(u16::from(bam_bytes));
    let header = reader.read_header()?;

    let res;

    if header.is_empty() {
        let reference_sequences = reader.read_reference_sequences()?;
        let mut builder = sam::Header::builder();

        for reference_sequence in reference_sequences {
            builder = builder.add_reference_sequence(reference_sequence);
        }

        res = builder.build();
    } else {
        res = header;
    }

    Ok(json!({ "header": res.to_string() }))
}