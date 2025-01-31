$version: "1.0"

namespace com.amazonaws.s3
use smithy.test#httpResponseTests
use smithy.test#httpRequestTests

apply NotFound @httpResponseTests([
    {
        id: "HeadObjectEmptyBody",
        documentation: "This test case validates https://github.com/awslabs/smithy-rs/issues/456",
        params: {
        },
        bodyMediaType: "application/xml",
        body: "",
        protocol: "aws.protocols#restXml",
        code: 404,
        headers: {
            "x-amz-request-id": "GRZ6BZ468DF52F2E",
            "x-amz-id-2": "UTniwu6QmCIjVeuK2ZfeWBOnu7SqMQOS3Vac6B/K4H2ZCawYUl+nDbhGTImuyhZ5DFiojR3Kcz4=",
            "content-type": "application/xml",
            "date": "Thu, 03 Jun 2021 04:05:52 GMT",
            "server": "AmazonS3"
        }
    }
])

apply GetBucketLocation @httpResponseTests([
    {
        id: "GetBucketLocation",
        documentation: "This test case validates https://github.com/awslabs/aws-sdk-rust/issues/116",
        code: 200,
        body: "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<LocationConstraint xmlns=\"http://s3.amazonaws.com/doc/2006-03-01/\">us-west-2</LocationConstraint>",
        params: {
            "LocationConstraint": "us-west-2"
        },
        bodyMediaType: "application/xml",
        protocol: "aws.protocols#restXml"
    }
])

apply PutBucketLifecycleConfiguration @httpRequestTests([
    {
        id: "PutBucketLifecycleConfiguration",
        documentation: "This test validates that the content md5 header is set correctly",
        method: "PUT",
        protocol: "aws.protocols#restXml",
        uri: "/test-bucket",
        headers: {
            // we can assert this, but when this test is promoted, it can't assert
            // on the exact contents
            "content-md5": "b14bbeb8064f913b40c4975a03ef6e4a",
        },
        bodyMediaType: "application/xml",
        body: """
        <LifecycleConfiguration xmlns=\"http://s3.amazonaws.com/doc/2006-03-01/\">
            <Rule xmlns=\"http://s3.amazonaws.com/doc/2006-03-01/\">
                <Expiration xmlns=\"http://s3.amazonaws.com/doc/2006-03-01/\">
                    <Days xmlns=\"http://s3.amazonaws.com/doc/2006-03-01/\">1</Days>
                </Expiration>
                <ID xmlns=\"http://s3.amazonaws.com/doc/2006-03-01/\">Expire</ID>
                <Status xmlns=\"http://s3.amazonaws.com/doc/2006-03-01/\">Enabled</Status>
            </Rule>
        </LifecycleConfiguration>
        """,
        params: {
            "Bucket": "test-bucket",
            "LifecycleConfiguration": {
                "Rules": [
                    {"Expiration": { "Days": 1 }, "Status": "Enabled", "ID": "Expire" },
                ]
            }
        }
    }
])

apply CreateMultipartUpload @httpRequestTests([
    {
        id: "CreateMultipartUploadUriConstruction",
        documentation: "This test validates that the URI for CreateMultipartUpload is created correctly",
        method: "POST",
        protocol: "aws.protocols#restXml",
        uri: "/test-bucket/object.txt",
        queryParams: [
            "uploads",
            "x-id=CreateMultipartUpload"
        ],
        params: {
            "Bucket": "test-bucket",
            "Key": "object.txt"
        }
    }
])
