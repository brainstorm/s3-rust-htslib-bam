# Read BAM header on an AWS lambda with rust-htslib

This small PoC assumes that:

1. You are already authenticated against AWS
2. [AWS CDK](https://aws.amazon.com/cdk/) is properly installed.
3. You have a [functioning Rust(up) installation](https://rustup.rs/), docker and [`cross`](https://github.com/rust-embedded/cross).

If that is in order, clone this repository and build away:

```
$ cross build --release --target x86_64-unknown-linux-musl
$ cd deploy && cdk deploy
rust-htslib-lambda: deploying...
[0%] start: Publishing c59cf9536e04f460efe5bf09a3e7404d2f0dbf43be6a353e09e46c4e0b574d37:current
[100%] success: Published c59cf9536e04f460efe5bf09a3e7404d2f0dbf43be6a353e09e46c4e0b574d37:current
rust-htslib-lambda: creating CloudFormation changeset...



 ✅  rust-htslib-lambda

Stack ARN:
arn:aws:cloudformation:ap-southeast-2:<ACCT_ID>:stack/rust-htslib-lambda/33990140-a619-11ea-98e1-0a1a04ef0eac
```

Then once deployed, invoke the lambda:

```
$ aws lambda invoke --function-name "arn:aws:lambda:ap-southeast-2:<ACCT_ID>:function:rust-htslib-lambda" response.json
{
    "StatusCode": 200,
    "FunctionError": "Unhandled",
    "ExecutedVersion": "$LATEST"
}

$ jq . response.json 
{
  "errorType": "Runtime.ExitError",
  "errorMessage": "RequestId: 1d98e743-00c8-4535-ac8b-0d4c7f2ee4a3 Error: Runtime exited with error: exit status 101"
}
```

On another (tmux) terminal, before invoking the lambda function:

```
$ cw tail -f /aws/lambda/rust-htslib-lambda
(...)
< HTTP/1.1 200 OK
< x-amz-id-2: hash 
< x-amz-request-id: hash
< Date: Tue, 30 Jun 2020 05:46:22 GMT
< Last-Modified: Thu, 07 May 2020 06:36:24 GMT
< ETag: "6dc47e886b9f2ecef870af88da3ebdd6"
< Accept-Ranges: bytes
< Content-Type: binary/octet-stream
< Content-Length: 2596799
< Server: AmazonS3
< 
* Closing connection 0
[src/main.rs:20] "BAM header targets are: {}" = "BAM header targets are: {}"
[src/main.rs:20] res = [
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "10",
    "11",
    "12",
    "13",
    "14",
    "15",
    "16",
    "17",
    "18",
    "19",
    "20",
    "21",
    "22",
    "X",
    "Y",
    "MT",
    "GL000207.1",
    "GL000226.1",
    "GL000229.1",
    "GL000231.1",
    "GL000210.1",
    "GL000239.1",
    "GL000235.1",
    "GL000201.1",
    "GL000247.1",
    "GL000245.1",
    "GL000197.1",
    "GL000203.1",
    "GL000246.1",
    "GL000249.1",
    "GL000196.1",
    "GL000248.1",
    "GL000244.1",
    "GL000238.1",
    "GL000202.1",
    "GL000234.1",
    "GL000232.1",
    "GL000206.1",
    "GL000240.1",
    "GL000236.1",
    "GL000241.1",
    "GL000243.1",
    "GL000242.1",
    "GL000230.1",
    "GL000237.1",
    "GL000233.1",
    "GL000204.1",
    "GL000198.1",
    "GL000208.1",
    "GL000191.1",
    "GL000227.1",
    "GL000228.1",
    "GL000214.1",
    "GL000221.1",
    "GL000209.1",
    "GL000218.1",
    "GL000220.1",
    "GL000213.1",
    "GL000211.1",
    "GL000199.1",
    "GL000217.1",
    "GL000216.1",
    "GL000215.1",
    "GL000205.1",
    "GL000219.1",
    "GL000224.1",
    "GL000223.1",
    "GL000195.1",
    "GL000212.1",
    "GL000222.1",
    "GL000200.1",
    "GL000193.1",
    "GL000194.1",
    "GL000225.1",
    "GL000192.1",
    "NC_007605",
    "hs37d5",
]
END RequestId: fdd85a16-f4e0-4d35-9cca-7f35c696bf21
REPORT RequestId: fdd85a16-f4e0-4d35-9cca-7f35c696bf21    Duration: 470.63 ms    Billed Duration: 500 ms    Memory Size: 128 MB    Max Memory Used: 7 MB
```
