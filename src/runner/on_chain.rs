use std::collections::HashMap;
use std::time::Duration;

use anyhow::Result;
use reqwest::blocking::Client;
use serde::Deserialize;

use crate::common;
use crate::error::CrateError;

#[derive(Deserialize)]
struct JsonError {
    code: i16,
    message: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct JsonRpcError {
    jsonrpc: String,
    error: JsonError,
    id: String,
}

pub fn run_program(_code: Vec<u8>, address: &str) -> Result<()> {
    common::print("Sending", "`rpc_methods` RPC request");
    let response = post_json(address, &json_rpc("rpc_methods"))?;
    log::debug!("Response: {}", response);
    Err(CrateError::UnimplementedCommand.into())
}

fn json_rpc(method: &str) -> HashMap<&str, String> {
    let mut json = HashMap::new();
    json.insert("id", "1".to_string());
    json.insert("jsonrpc", "2.0".to_string());
    json.insert("method", method.to_string());
    json
}

fn post_json(address: &str, json: &HashMap<&str, String>) -> Result<String> {
    let client = Client::builder().timeout(Duration::from_secs(5)).build()?;

    let response = client
        .post(&format!("http://{}", address))
        .json(json)
        .send()?
        .error_for_status()?;
    let json_str = response.text()?;

    if let Ok(JsonRpcError { error, .. }) = serde_json::from_str::<JsonRpcError>(&json_str) {
        Err(CrateError::JsonRpcError(error.code, error.message).into())
    } else {
        Ok(json_str)
    }
}