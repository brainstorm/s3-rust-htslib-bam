import * as JSZip from "jszip";
import * as fs from "fs";
import {AssetCode, Function, Runtime} from "@aws-cdk/aws-lambda";
import {Duration} from "@aws-cdk/core";
import {PolicyStatement} from "@aws-cdk/aws-iam";
import cdk = require('@aws-cdk/core');

const LAMBDA_FUNCTION_NAME='s3-rust-htslib-bam-lambda'

const CA_BUNDLE_FILENAME='cert.pem'
const BOOTSTRAP_ZIP_FILENAME='bootstrap.zip'

// these are the two binaries that go into the bootstrap.zip to make our lambda function

// this build location is set based on where the Rust cross compiler will place the release build
const RUST_RELEASE_BINARY_CONTENT=fs.readFileSync(`./target/x86_64-unknown-linux-musl/release/bootstrap`)
const CA_BUNDLE_BINARY_CONTENT=fs.readFileSync(`./${CA_BUNDLE_FILENAME}`)

const BOOTSTRAP_ZIP_PATH=`./target/${BOOTSTRAP_ZIP_FILENAME}`

class S3RustHtslibBamStack extends cdk.Stack {

    constructor(scope: cdk.App, id: string, props?: cdk.StackProps) {
        super(scope, id, props);

        // our demo is of something that reads BAM files from S3 so we give it broad permissions
        // to read S3 locations.. a production setup would have tighter permissions
        const lambdaPolicy = new PolicyStatement()
        lambdaPolicy.addActions("s3:List*", "s3:Get*")
        lambdaPolicy.addResources("*")

        new Function(this, LAMBDA_FUNCTION_NAME, {
            functionName: LAMBDA_FUNCTION_NAME,
            handler: "main",
            runtime: Runtime.PROVIDED,
            code: new AssetCode(`./target/${BOOTSTRAP_ZIP_FILENAME}`),
            memorySize: 2048,
            timeout: Duration.seconds(10),
            environment: {
                CURL_CA_BUNDLE: CA_BUNDLE_FILENAME,
                RUST_BACKTRACE: "full"
            },
            initialPolicy: [lambdaPolicy],
        })
    }
}

// construct a bootstrap.zip (expected by the lambda infrastructure)
// that contains the Rust executable + certs
new JSZip()
    // this path is the fixed zip entry name that AWS lambda expects to execute
    .file("bootstrap", RUST_RELEASE_BINARY_CONTENT, {
        unixPermissions: "0755"
    })
    .file(CA_BUNDLE_FILENAME, CA_BUNDLE_BINARY_CONTENT, {
        unixPermissions: "0544"
    })
    .generateNodeStream({type:'nodebuffer', platform: 'UNIX'})
    .pipe(fs.createWriteStream(BOOTSTRAP_ZIP_PATH))
    .on('finish', () => {
        console.log(`${BOOTSTRAP_ZIP_PATH} written`);

        // construct the CDK stack
        const app = new cdk.App();
        new S3RustHtslibBamStack(app, 's3-rust-htslib-bam-stack');
        app.synth();
    });
