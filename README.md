# Read BAM header on an AWS lambda with rust-htslib

This small Bioinformatics proof of concept that bundles [htslib](http://github.com/samtools/htslib) into
 an AWS Lambda for massive distributed computing. This only prints a BAM header, but I hope you see the
  massive scaling potential, hitting an S3 bucket with millions of concurrent lambdas will be interesting to see ;)

To make this work, this README assumes the following prerequisites:

1. You are already authenticated against AWS (with either environment credentials or AWS_PROFILE set) - in an
     account that you can deploy CloudFormation stacks/lambdas.
2. [AWS SAM](https://aws.amazon.com/sam/) is properly installed.
3. You have a [functioning Rust(up) installation](https://rustup.rs/),
     docker and [`cross`](https://github.com/rust-embedded/cross).
4. You have jq installed (for the lambda invoke - feel free to just replace with cat)

If that is in order, clone this repository.

Building the Rust binaries can be done via `cross`:

```
$ cross build --release --target x86_64-unknown-linux-musl
```

And then invoke or deploy the resulting `bootstrap` binary:

```
$ cp ./target/x86_64-unknown-linux-musl/release/bootstrap .
$ sam local invoke -e event.json
```

The output will show both the status of the lambda invoke - and if successful, the list of target names extracted from the BAM file, i.e:

```
Invoking foo (provided)
Skip pulling image and use local one: amazon/aws-sam-cli-emulation-image-provided:rapid-1.2.0.

Mounting /Users/rvalls/dev/umccr/htsget/s3-rust-htslib-bam as /var/task:ro,delegated inside runtime container
START RequestId: ad132130-ef6b-1eec-4e40-a76ac5054a4d Version: $LATEST
Testing header of BAM file s3://umccr-research-dev/htsget/htsnexus_test_NA12878.bam
[D::init_add_plugin] Loaded "knetfile"
[D::init_add_plugin] Loaded "mem"
[D::init_add_plugin] Loaded "crypt4gh-needed"
[D::init_add_plugin] Loaded "libcurl"
[D::init_add_plugin] Loaded "s3"
[D::init_add_plugin] Loaded "s3w"
*   Trying 52.95.134.108:443...
* Connected to umccr-research-dev.s3.amazonaws.com (52.95.134.108) port 443 (#0)
* ALPN, offering http/1.1
* SSL certificate problem: unable to get local issuer certificate
* Closing connection 0
[E::easy_errno] Libcurl reported error 60 (SSL peer certificate or SSH remote key was not OK)
[E::hts_open_format] Failed to open file "s3://umccr-research-dev/htsget/htsnexus_test_NA12878.bam" : I/O error
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: BamOpen { source: Open { target: "s3://umccr-research-dev/htsget/htsnex
us_test_NA12878.bam" } }', src/main.rs:42:20
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Function 'htsgetAws' timed out after 3 seconds
```