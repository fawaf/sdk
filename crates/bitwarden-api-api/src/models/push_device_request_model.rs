/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PushDeviceRequestModel {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: models::DeviceType,
}

impl PushDeviceRequestModel {
    pub fn new(id: String, r#type: models::DeviceType) -> PushDeviceRequestModel {
        PushDeviceRequestModel { id, r#type }
    }
}
