use serde_json::json;

use super::*;

mod helpers;

#[test]
fn _010_rejects_invalid_input() {
    assert_with_pattern!(
        validate(json!({
            "foo": "bar"
        })),
        Err(_)
    );
}

#[test]
fn _020_accepts_valid_input() {
    assert_with_pattern!(
        validate(json!({
            "claimCertificateId" : "string",
            "certificateId" : "string",
            "certificatePem" : "string",
            "templateArn" : "arn:aws:iot:us-east-1:1234567890:provisioningtemplate/MyTemplate",
            "clientId" : "221a6d10-9c7f-42f1-9153-e52e6fc869c1",
            "parameters" : {
                "string" : "string",
            }
        })),
        Ok(_)
    );
}
