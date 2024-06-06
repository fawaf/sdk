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
pub struct WebAuthnCredentialResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "prfStatus", skip_serializing_if = "Option::is_none")]
    pub prf_status: Option<models::WebAuthnPrfStatus>,
}

impl WebAuthnCredentialResponseModel {
    pub fn new() -> WebAuthnCredentialResponseModel {
        WebAuthnCredentialResponseModel {
            object: None,
            id: None,
            name: None,
            prf_status: None,
        }
    }
}
