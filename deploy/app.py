from zipfile import ZipFile
import shutil
import boto3
from aws_cdk import (
    aws_lambda as lambda_,
    aws_s3 as s3,
    core,
)

s3c = boto3.client('s3')

CA_BUNDLE="cert.pem"
BUCKET="umccr-research-dev"
ASSET="bootstrap.zip"
KEY="htsget/app/{}".format(ASSET)
KEY_ACL="htsget/*" # gives the lambda only access to resources prefixed by this S3 key
TARGET_PATH="../target/x86_64-unknown-linux-musl/release/bootstrap"

class rustHtslibLambda(core.Stack):
    def __init__(self, app: core.App, id: str) -> None:
        super().__init__(app, id)

        lambda_bucket = s3.Bucket.from_bucket_attributes(
            self, 'LambdaCodeBucket',
            bucket_name=BUCKET
        )

        external_bucket = s3.Bucket.from_bucket_attributes(
            self, 'ExternalBucket',
            bucket_name="gatk-test-data"
        )

        lambdaFn = lambda_.Function(
            self, 'rust_htslib_lambda',
            handler='main',
            code=lambda_.Code.asset(ASSET),
            runtime=lambda_.Runtime.PROVIDED,
            timeout=core.Duration.seconds(10)
        )
        
        lambdaFn.add_environment("CURL_CA_BUNDLE", CA_BUNDLE)
        lambdaFn.add_event_source
        lambda_bucket.grant_read(lambdaFn, KEY_ACL)
        external_bucket.grant_read(lambdaFn)

app = core.App()

# Pack for lambda PROVIDED runtime (must be a .zip)...
with ZipFile(ASSET, 'w') as fzip:
    fzip.write(TARGET_PATH, "bootstrap")
    fzip.write(CA_BUNDLE)

# ... and ship it!
rustHtslibLambda(app, "rust-htslib-lambda")
app.synth()
