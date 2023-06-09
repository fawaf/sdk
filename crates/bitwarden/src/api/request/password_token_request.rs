use log::debug;
use serde::{Deserialize, Serialize};

use crate::sdk::auth::request::{TwoFactorProvider, TwoFactorRequest};
use crate::{api::response::IdentityTokenResponse, client::ApiConfigurations, error::Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordTokenRequest {
    scope: String,
    client_id: String,
    #[serde(rename = "deviceType")]
    device_type: u8,
    #[serde(rename = "deviceIdentifier")]
    device_identifier: String,
    #[serde(rename = "deviceName")]
    device_name: String,
    grant_type: String,
    #[serde(rename = "username")]
    email: String,
    #[serde(rename = "password")]
    master_password_hash: String,

    #[serde(rename = "twoFactorToken")]
    two_factor_token: Option<String>,
    #[serde(rename = "twoFactorProvider")]
    two_factor_provider: Option<TwoFactorProvider>,
    #[serde(rename = "twoFactorToken")]
    two_factor_remember: Option<bool>,
}

impl PasswordTokenRequest {
    pub fn new(email: &str, password_hash: &String, two_factor: &Option<TwoFactorRequest>) -> Self {
        let tf = two_factor.as_ref();
        let obj = Self {
            scope: "api offline_access".to_string(),
            client_id: "web".to_string(),
            device_type: 10,
            device_identifier: "b86dd6ab-4265-4ddf-a7f1-eb28d5677f33".to_string(),
            device_name: "firefox".to_string(),
            grant_type: "password".to_string(),
            master_password_hash: password_hash.to_string(),
            email: email.to_string(),
            two_factor_token: tf.map(|t| t.token.to_owned()),
            two_factor_provider: tf.map(|t| t.provider.clone()),
            two_factor_remember: tf.map(|t| t.remember),
        };
        debug!("initializing {:?}", obj);
        obj
    }

    pub(crate) async fn send(
        &self,
        configurations: &ApiConfigurations,
    ) -> Result<IdentityTokenResponse> {
        super::send_identity_connect_request(configurations, Some(&self.email), &self).await
    }
}
