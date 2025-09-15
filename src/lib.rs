mod json_structure;

use crate::json_structure::{DefaultDevice, Device, EditDeviceVars, FritzResponse};
use regex::Regex;

#[derive(Debug)]
pub enum Error {
    RequestChallenge(reqwest::Error),
    RequestSid(reqwest::Error),
    ParseChallenge,
    ParseSid,
    BadRequest(reqwest::Error),
    ParsingError(serde_json::Error),
    ConversionError,
}


pub async fn auth(base_url: &str, user: &str, password: &str) -> Result<String, Error> {
    let challenge_regex: Regex = Regex::new(r"<Challenge>([a-z0-9]{8})").unwrap();
    let sid_regex: Regex = Regex::new(r"<SID>([a-z0-9]{16})").unwrap();

    let url = format!("{base_url}/login_sid.lua");

    let challenge_response = reqwest::get(&url)
        .await.map_err(|e| Error::RequestChallenge(e))?
        .text()
        .await.map_err(|e| Error::RequestChallenge(e))?;

    let challenge_token = challenge_regex
        .captures(&challenge_response)
        .and_then(|cap| cap.get(1).map(|token| token.as_str())).ok_or(Error::ParseChallenge)?;

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
        .send().await.map_err(|e| Error::RequestSid(e))?
        .text()
        .await.map_err(|e| Error::RequestSid(e))?;

    let sid = sid_regex
        .captures(&sid_respose)
        .and_then(|cap| cap.get(1).map(|sid| sid.as_str())).ok_or(Error::ParseSid)?;

    Ok(sid.to_string())
}

pub async fn get_network_devices(base_url: &str, sid: &str) -> Result<Vec<DefaultDevice>, Error> {
    let client = reqwest::Client::new();

    let json = client
        .post(format!("{base_url}/data.lua"))
        .form(&[("page", "homeNet"), ("sid", sid)])
        .send()
        .await.map_err(|e| Error::BadRequest(e))?
        .text()
        .await.map_err(|e| Error::BadRequest(e))?;

    let resp: FritzResponse = serde_json::from_str(&json).map_err(|e| Error::ParsingError(e))?;
    let data = resp.data.to_home_net_data()?;
    
    let mut devices = Vec::new();


    for d in data.devices {
        match d {
            Device::RouterDevice(_) => {}
            Device::PLCDevice(_) => {}
            Device::DefaultDevice(device) => devices.push(device)
        }
    }

    Ok(devices)
}

pub async fn get_device_infos(base_url: &str, sid: &str, uid: &str) -> Result<EditDeviceVars, Error> {
    let client = reqwest::Client::new();

    let json = client
        .post(format!("{base_url}/data.lua"))
        .form(&[("page", "edit_device"), ("sid", sid), ("dev", uid)])
        .send()
        .await.map_err(|e| Error::BadRequest(e))?
        .text()
        .await.map_err(|e| Error::BadRequest(e))?;

    let resp: FritzResponse = serde_json::from_str(&json).map_err(|e| Error::ParsingError(e))?;
    Ok(resp.data.to_edit_device_data()?.vars)
}