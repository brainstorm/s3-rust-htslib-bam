use lambda_runtime::{handler_fn, Context};
use serde_json::Value;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler_fn(bam_header)).await?;
    Ok(())
}

async fn bam_header(_: Value, _: Context) -> Result<(), Error> {
    Ok(())
}