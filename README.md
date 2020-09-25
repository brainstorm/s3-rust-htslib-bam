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

And then **invoke** or **deploy** the resulting `bootstrap` binary:

```
$ cp ./target/x86_64-unknown-linux-musl/release/bootstrap .
$ sam local invoke # TODO: -e a suitable .json
```

The output will show both the status of the lambda invoke - and if successful, the list of target names extracted from the BAM file.