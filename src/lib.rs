mod json_structure;

use std::fmt::format;
use regex::Regex;
use crate::json_structure::{DefaultDevice, FritzHomeNetResponse};

#[derive(Debug)]
pub enum AuthError {
    RequestChallenge(reqwest::Error),
    RequestSid(reqwest::Error),
    ParseChallenge,
    ParseSid
}

#[derive(Debug)]
pub enum DataError {
    BadRequest(reqwest::Error),
    ParsingError(serde_json::Error)
}


pub async fn auth(base_url: &str, user: &str, password: &str) -> Result<String, AuthError> {
    let challenge_regex: Regex = Regex::new(r"<Challenge>([a-z0-9]{8})").unwrap();
    let sid_regex: Regex = Regex::new(r"<SID>([a-z0-9]{16})").unwrap();

    let url = format!("{base_url}/login_sid.lua");

    let challenge_response = reqwest::get(&url)
        .await.map_err(|e| AuthError::RequestChallenge(e))?
        .text()
        .await.map_err(|e| AuthError::RequestChallenge(e))?;

    let challenge_token = challenge_regex
        .captures(&challenge_response)
        .and_then(|cap| cap.get(1).map(|token| token.as_str())).ok_or(AuthError::ParseChallenge)?;

    let joined = format!("{}-{}", challenge_token, password);

    // Insert a NUL byte after every byte.
    let mut buf = Vec::with_capacity(joined.len() * 2);
    for &b in joined.as_bytes() {
        buf.push(b);
        buf.push(0u8);
    }

    let digest = md5::compute(&buf);

    let client = reqwest::Client::new();
    let params = [
        ("response", format!("{challenge_token}-{digest:x}")),
        ("username", user.to_string())
    ];
    let sid_respose = client.post(url)
        .form(&params)
        .send().await.map_err(|e| AuthError::RequestSid(e))?
        .text()
        .await.map_err(|e| AuthError::RequestSid(e))?;

    let sid = sid_regex
        .captures(&sid_respose)
        .and_then(|cap| cap.get(1).map(|sid| sid.as_str())).ok_or(AuthError::ParseSid)?;

    Ok(sid.to_string())
}

pub async fn get_network_devices(base_url: &str, sid: &str) -> Result<Vec<DefaultDevice>, DataError> {
    let client = reqwest::Client::new();

    // Perform the POST
    let json = client
        .post(format!("{base_url}/data.lua"))
        .form(&[("lang", "de"), ("page", "homeNet"), ("sid", sid)])
        .send()
        .await.map_err(|e| DataError::BadRequest(e))?
        .text()
        .await.map_err(|e| DataError::BadRequest(e))?;

    let resp: FritzHomeNetResponse = serde_json::from_str(&json).map_err(|e| DataError::ParsingError(e))?;
    
    let devices = resp.data

    
    for d in resp {
        if !d.is_plc() && !d.is_router() {
            devices.push(d.to_device().map_err(|e| DataError::ConversionError(e))?)
        }
    }
    
}