# Rust AWS IoT Pre-Provisioning Hook Example

## TL;DR

- during IoT fleet provisioning, the IoT Core can optionally call a lambda to validate the provisioning request
  (pre-provisioning hook)
- the current implementation deserializes the input into a serde_json::Value type to enable arbitrary JSON payload to be
  parsed by the handler
- this shifts the burden of validating the input into the handler (but no serialization error is thrown on any
  technically valid JSON input sent by the IoT Core even if say the parameters of the provisioning template change)

## Building the project

The project can be built with the `cargo lambda build --release` and deployed with the `cargo lambda deploy` commands
respectively in an appropriately authenticated terminal.

## Using the lambda

Please make sure that the lambda is attached to the provisioning template (for example
via `IoT Core/Connect/Connect many devices/<YOUR_PROVISIONING_TEMPLATE>/Edit details`)

Afterward, please check that a resource-based policy was indeed attached to the lambda function that enables the IoT
Core to call the lambda:

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": {
        "Service": "iot.amazonaws.com"
      },
      "Action": "lambda:InvokeFunction",
      "Resource": "arn:aws:lambda:YOUR_REGION:YOUR_ACCOUNT:function:YOUR_FUNCTION",
      "Condition": {
        "StringEquals": {
          "AWS:SourceAccount": "YOUR_ACCOUNT"
        },
        "ArnLike": {
          "AWS:SourceArn": "arn:aws:iot:YOUR_REGION:YOUR_ACCOUNT:provisioningtemplate/YOUR_PROVISIONING_TEMPLATE"
        }
      }
    }
  ]
}
```
