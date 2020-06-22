# Read BAM header on an AWS lambda with rust-htslib

This small PoC assumes that:

1. You are already authenticated against AWS
2. [AWS CDK](https://aws.amazon.com/cdk/) is properly installed.
3. You have a [functioning Rust(up) installation](https://rustup.rs/), docker and [`cross`](https://github.com/rust-embedded/cross).

If that is in order, clone this repository and build away:

```
$ cross build --release --target x86_64-unknown-linux-musl
$ cdk deploy
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
START RequestId: 4a75c2f1-7a35-4a68-9ab8-105dd0d10f9a Version: $LATEST

[E::hts_open_format] Failed to open file "s3://gatk-test-data/wgs_bam/NA12878_24RG_hg38/NA12878_24RG_small.hg38.bam" : I/O error

thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: BamOpen { source: Open { target: "s3://gatk-test-data/wgs_bam/NA12878_24RG_hg38/NA12878_24RG_small.hg38.bam" } }', src/main.rs:17:12

stack backtrace:

0:           0x641674 - backtrace::backtrace::libunwind::trace::h234d741a55b60f88
at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86

1:           0x641674 - backtrace::backtrace::trace_unsynchronized::h350b2c8c65b00d1d

at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66

2:           0x641674 - std::sys_common::backtrace::_print_fmt::h4a536ea1c8e8e74a

at src/libstd/sys_common/backtrace.rs:78

3:           0x641674 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::had63074188e24509

at src/libstd/sys_common/backtrace.rs:59

4:           0x67b09c - core::fmt::write::h0f3ca38b916f7bdd

at src/libcore/fmt/mod.rs:1069

5:           0x63f6d3 - std::io::Write::write_fmt::h904ea4dad7931404

at src/libstd/io/mod.rs:1504

6:           0x643b95 - std::sys_common::backtrace::_print::h5b567d4903ca6eb3

at src/libstd/sys_common/backtrace.rs:62

7:           0x643b95 - std::sys_common::backtrace::print::hf98b9b1b18a4dc81

at src/libstd/sys_common/backtrace.rs:49

8:           0x643b95 - std::panicking::default_hook::{{closure}}::h5fbf8e21242992f2

at src/libstd/panicking.rs:198

9:           0x6438d2 - std::panicking::default_hook::hb4d89e36502020cd

at src/libstd/panicking.rs:218

10:           0x6441a2 - std::panicking::rust_panic_with_hook::hc36f90fb81cc1268

at src/libstd/panicking.rs:511

11:           0x643d8b - rust_begin_unwind

at src/libstd/panicking.rs:419

12:           0x67a481 - core::panicking::panic_fmt::h31cb4ec4ac5347b3

at src/libcore/panicking.rs:111

13:           0x67a2a3 - core::option::expect_none_failed::h3e3ee4886fcb0833

at src/libcore/option.rs:1268

14:           0x402134 - bootstrap::main::h4cfb5e1da07e4c36

15:           0x401903 - std::rt::lang_start::{{closure}}::h71ce4b28a2a11ce2

16:           0x6444d1 - std::rt::lang_start_internal::{{closure}}::ha24276d619b0834a

at src/libstd/rt.rs:52

17:           0x6444d1 - std::panicking::try::do_call::ha58b8718efdbddf5

at src/libstd/panicking.rs:331

18:           0x6444d1 - std::panicking::try::h2d6d423bf379e813

at src/libstd/panicking.rs:274

19:           0x6444d1 - std::panic::catch_unwind::h45b4b6133cb33025

at src/libstd/panic.rs:394

20:           0x6444d1 - std::rt::lang_start_internal::h47125699e3ec3d7e

at src/libstd/rt.rs:51

21:           0x402222 - main

END RequestId: 4a75c2f1-7a35-4a68-9ab8-105dd0d10f9a

REPORT RequestId: 4a75c2f1-7a35-4a68-9ab8-105dd0d10f9a  Duration: 911.60 ms     Billed Duration: 1000 ms        Memory Size: 128 MB     Max Memory Used: 16 MB

RequestId: 4a75c2f1-7a35-4a68-9ab8-105dd0d10f9a Error: Runtime exited with error: exit status 101
Runtime.ExitError

[E::hts_open_format] Failed to open file "s3://gatk-test-data/wgs_bam/NA12878_24RG_hg38/NA12878_24RG_small.hg38.bam" : I/O error

thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: BamOpen { source: Open { target: "s3://gatk-test-data/wgs_bam/NA12878_24RG_hg38/NA12878_24RG_small.hg38.bam" } }', src/main.rs:17:12

stack backtrace:

0:           0x641674 - backtrace::backtrace::libunwind::trace::h234d741a55b60f88

at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86

1:           0x641674 - backtrace::backtrace::trace_unsynchronized::h350b2c8c65b00d1d

at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66

2:           0x641674 - std::sys_common::backtrace::_print_fmt::h4a536ea1c8e8e74a

at src/libstd/sys_common/backtrace.rs:78

3:           0x641674 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::had63074188e24509

at src/libstd/sys_common/backtrace.rs:59

4:           0x67b09c - core::fmt::write::h0f3ca38b916f7bdd

at src/libcore/fmt/mod.rs:1069

5:           0x63f6d3 - std::io::Write::write_fmt::h904ea4dad7931404

at src/libstd/io/mod.rs:1504

6:           0x643b95 - std::sys_common::backtrace::_print::h5b567d4903ca6eb3

at src/libstd/sys_common/backtrace.rs:62

7:           0x643b95 - std::sys_common::backtrace::print::hf98b9b1b18a4dc81

at src/libstd/sys_common/backtrace.rs:49

8:           0x643b95 - std::panicking::default_hook::{{closure}}::h5fbf8e21242992f2

at src/libstd/panicking.rs:198

9:           0x6438d2 - std::panicking::default_hook::hb4d89e36502020cd

at src/libstd/panicking.rs:218

10:           0x6441a2 - std::panicking::rust_panic_with_hook::hc36f90fb81cc1268

at src/libstd/panicking.rs:511

11:           0x643d8b - rust_begin_unwind

at src/libstd/panicking.rs:419

12:           0x67a481 - core::panicking::panic_fmt::h31cb4ec4ac5347b3

at src/libcore/panicking.rs:111

13:           0x67a2a3 - core::option::expect_none_failed::h3e3ee4886fcb0833

at src/libcore/option.rs:1268

14:           0x402134 - bootstrap::main::h4cfb5e1da07e4c36

15:           0x401903 - std::rt::lang_start::{{closure}}::h71ce4b28a2a11ce2

16:           0x6444d1 - std::rt::lang_start_internal::{{closure}}::ha24276d619b0834a

at src/libstd/rt.rs:52

17:           0x6444d1 - std::panicking::try::do_call::ha58b8718efdbddf5

at src/libstd/panicking.rs:331

18:           0x6444d1 - std::panicking::try::h2d6d423bf379e813

at src/libstd/panicking.rs:274

19:           0x6444d1 - std::panic::catch_unwind::h45b4b6133cb33025

at src/libstd/panic.rs:394

20:           0x6444d1 - std::rt::lang_start_internal::h47125699e3ec3d7e

at src/libstd/rt.rs:51

21:           0x402222 - main
```