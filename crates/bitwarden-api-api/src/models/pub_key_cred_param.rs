/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PubKeyCredParam {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::PublicKeyCredentialType>,
    #[serde(rename = "alg", skip_serializing_if = "Option::is_none")]
    pub alg: Option<crate::models::Algorithm>,
}

impl PubKeyCredParam {
    pub fn new() -> PubKeyCredParam {
        PubKeyCredParam {
            r#type: None,
            alg: None,
        }
    }
}
