# Read BAM header on an AWS lambda with rust-htslib

This small Bioinformatics proof of concept that bundles [htslib](http://github.com/samtools/htslib) into
 an AWS Lambda for massive distributed computing. This only prints a BAM header, but I hope you see the
  massive scaling potential, hitting an S3 bucket with millions of concurrent lambdas will be interesting to see ;)

To make this work, this README assumes the following prerequisites:

1. You are already authenticated against AWS (with either environment credentials or AWS_PROFILE set) - in an
     account that you can deploy CloudFormation stacks/lambdas
2. [AWS CDK](https://aws.amazon.com/cdk/) is properly installed and has been bootstrapped in your
     AWS account from (1)
3. You have a [functioning Rust(up) installation](https://rustup.rs/),
     docker and [`cross`](https://github.com/rust-embedded/cross).
4. You have jq installed (for the lambda invoke - feel free to just replace with cat)

If that is in order, clone this repository.

The CDK deployment stack for the project is built using nodejs so we initialise this project for nodejs work

```
npm install
```

Building the Rust binaries can be done manually or via npm (rust build instructions have been added to this projects package.json)

```
$ cross build --release --target x86_64-unknown-linux-musl
```
or
```
$ npm run build-rust-release
```

Then deploy with [AWS CDK](https://aws.amazon.com/cdk/), again either manually or via npm:

```
$ cdk deploy -a "npx ts-node cdk.ts"
```
or
```
$ npm run deploy
```
outputs something like
```
s3-rust-htslib-bam-stack: deploying...
[0%] start: Publishing c59cf9536e04f460efe5bf09a3e7404d2f0dbf43be6a353e09e46c4e0b574d37:current
[100%] success: Published c59cf9536e04f460efe5bf09a3e7404d2f0dbf43be6a353e09e46c4e0b574d37:current
s3-rust-htslib-bam-stack: creating CloudFormation changeset...

 ✅  s3-rust-htslib-bam-stack

Stack ARN:
arn:aws:cloudformation:ap-southeast-2:<ACCT_ID>:stack/s3-rust-htslib-bam-stack/abcdefg
```

And finally, invoke the lambda (change DEMOBAM to use any S3 BAM file accessible from your account):

AWS cli 1
```
$ DEMOBAM=s3://mybucket/mybam.bam  aws lambda invoke --payload '{"bam": "'${DEMOBAM}'"}' --function-name "s3-rust-htslib-bam-lambda" response.json && jq < response.json .
```

AWS cli 2 (specifying input is not base64 encoded and piping to jq to remove paging)
```
$ DEMOBAM=s3://mybucket/mybam.bam  aws lambda invoke --payload '{"bam": "'${DEMOBAM}'"}' --function-name "s3-rust-htslib-bam-lambda" response.json --cli-binary-format raw-in-base64-out | jq . && jq < response.json .
```

npm (as above but easier to invoke!)
```
$ DEMOBAM=s3://mybucket/mybam.bam  npm run demo1
```

The output will show both the status of the lambda invoke - and if successful, the list of target names extracted
from the BAM file.
