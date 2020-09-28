# Read BAM header on an AWS lambda with rust-htslib

This small Bioinformatics proof of concept that bundles [htslib](http://github.com/samtools/htslib) into
 an AWS Lambda for massive distributed computing. This only prints a BAM header, but I hope you see the
  massive scaling potential, hitting an S3 bucket with millions of concurrent lambdas will be interesting to see ;)

To make this work, this README assumes the following prerequisites:

1. You are already authenticated against AWS (with either environment credentials or AWS_PROFILE set) - in an
     account that you can deploy CloudFormation stacks/lambdas.
2. [AWS SAM](https://aws.amazon.com/serverless/sam/) is properly installed.
3. You have a [functioning Rust(up) installation](https://rustup.rs/),
     docker and [`cross`](https://github.com/rust-embedded/cross).

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
END RequestId: dbd528c7-858d-15e7-6067-9723ce1e643f
REPORT RequestId: dbd528c7-858d-15e7-6067-9723ce1e643f  Init Duration: 139.11 ms        Duration: 2251.32 ms    Billed Duration: 2300 ms      M
emory Size: 128 MB      Max Memory Used: 13 MB

["1","2","3","4","5","6","7","8","9","10","11","12","13","14","15","16","17","18","19","20","21","22","X","Y","MT","GL000207.1","GL000226.1","G
L000229.1","GL000231.1","GL000210.1","GL000239.1","GL000235.1","GL000201.1","GL000247.1","GL000245.1","GL000197.1","GL000203.1","GL000246.1",
(...)
```
