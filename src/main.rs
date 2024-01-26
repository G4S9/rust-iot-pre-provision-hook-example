use std::collections::HashMap;
use std::sync::OnceLock;

use jsonschema::JSONSchema;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Response {
    allow_provisioning: bool,
    parameter_overrides: HashMap<String, String>,
}

#[derive(Error, Debug)]
enum AppError {
    #[error("Input validation error {0:?}")]
    ValidationError(Vec<String>),
}

async fn my_handler(
    LambdaEvent { payload: input, .. }: LambdaEvent<serde_json::Value>,
) -> anyhow::Result<Response, Error> {
    tracing::info!("Received input: {:?}", input);
    validate(input)?;
    Ok(Response {
        allow_provisioning: true,
        parameter_overrides: HashMap::new(),
    })
}

fn validate(input: serde_json::Value) -> anyhow::Result<()> {
    static SCHEMA: OnceLock<JSONSchema> = OnceLock::new();
    let schema = SCHEMA.get_or_init(|| {
        JSONSchema::options()
            .compile(&serde_json::from_str(include_str!("schemas/input.json")).unwrap())
            .unwrap()
    });
    if let Err(errors) = schema.validate(&input) {
        return Err(AppError::ValidationError(
            errors
                .map(|error| error.to_string())
                .collect::<Vec<String>>(),
        )
        .into());
    }
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let func = service_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

#[cfg(test)]
mod tests;
