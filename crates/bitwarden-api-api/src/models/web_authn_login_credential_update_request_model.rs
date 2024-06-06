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
pub struct WebAuthnLoginCredentialUpdateRequestModel {
    #[serde(rename = "deviceResponse")]
    pub device_response: Box<models::AuthenticatorAssertionRawResponse>,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "encryptedUserKey")]
    pub encrypted_user_key: String,
    #[serde(rename = "encryptedPublicKey")]
    pub encrypted_public_key: String,
    #[serde(rename = "encryptedPrivateKey")]
    pub encrypted_private_key: String,
}

impl WebAuthnLoginCredentialUpdateRequestModel {
    pub fn new(
        device_response: models::AuthenticatorAssertionRawResponse,
        token: String,
        encrypted_user_key: String,
        encrypted_public_key: String,
        encrypted_private_key: String,
    ) -> WebAuthnLoginCredentialUpdateRequestModel {
        WebAuthnLoginCredentialUpdateRequestModel {
            device_response: Box::new(device_response),
            token,
            encrypted_user_key,
            encrypted_public_key,
            encrypted_private_key,
        }
    }
}
