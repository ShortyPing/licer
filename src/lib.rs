use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub local_version: Option<String>
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseManifest {
    pub license_list_version: String,
    pub licenses: Vec<License>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub reference: String,
    pub is_deprecated_license_id: bool,
    pub details_url: String,
    pub name: String,
    pub license_id: String
}



#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseDetail {
    pub license_text: String
}