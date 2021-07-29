# Read BAM header on an AWS lambda with rust-htslib

DEPRECATED: Please see the [C-less version of this proof of concept instead](https://github.com/umccr/s3-rust-noodles-bam). Safer, better.

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

The output will show both the status of the lambda invoke - and if successful, the header records from the BAM file, i.e:

```
END RequestId: dbd528c7-858d-15e7-6067-9723ce1e643f
REPORT RequestId: dbd528c7-858d-15e7-6067-9723ce1e643f  Init Duration: 139.11 ms        Duration: 2251.32 ms    Billed Duration: 2300 ms      M
emory Size: 128 MB      Max Memory Used: 13 MB

[
(...)
"@SQ\tSN:HLA-DRB1*04:03:01\tLN:15246\tAS:GRCh38\tM5:ce0de8afd561fb1fb0d3acce94386a27\tUR:ftp://ftp.1000genomes.ebi.ac.uk/vol1/ftp/technical/reference/GRCh38_reference_genome/GRCh38_full_analysis_set_plus_decoy_hla.fa\tSP:Human",
"@SQ\tSN:HLA-DRB1*07:01:01:01\tLN:16110\tAS:GRCh38\tM5:4063054a8189fbc81248b0f37b8273fd\tUR:ftp://ftp.1000genomes.ebi.ac.uk/vol1/ftp/technical/reference/GRCh38_reference_genome/GRCh38_full_analysis_set_plus_decoy_hla.fa\tSP:Human",
"@SQ\tSN:HLA-DRB1*07:01:01:02\tLN:16120\tAS:GRCh38\tM5:a4b1a49cfe8fb2c98c178c02b6c64ed4\tUR:ftp://ftp.1000genomes.ebi.ac.uk/vol1/ftp/technical/reference/GRCh38_reference_genome/GRCh38_full_analysis_set_plus_decoy_hla.fa\tSP:Human",
"@CO\t$known_indels_file(s) = ftp://ftp.1000genomes.ebi.ac.uk/vol1/ftp/technical/reference/GRCh38_reference_genome/other_mapping_resources/ALL.wgs.1000G_phase3.GRCh38.ncbi_remapper.20150424.shapeit2_indels.vcf.gz",
"@CO\tFASTQ=ERR009378_1.fastq.gz",
(...)
```
