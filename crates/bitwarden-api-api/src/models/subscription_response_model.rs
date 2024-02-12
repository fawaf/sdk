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
pub struct SubscriptionResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "storageName", skip_serializing_if = "Option::is_none")]
    pub storage_name: Option<String>,
    #[serde(rename = "storageGb", skip_serializing_if = "Option::is_none")]
    pub storage_gb: Option<f64>,
    #[serde(rename = "maxStorageGb", skip_serializing_if = "Option::is_none")]
    pub max_storage_gb: Option<i32>,
    #[serde(rename = "upcomingInvoice", skip_serializing_if = "Option::is_none")]
    pub upcoming_invoice: Option<Box<crate::models::BillingSubscriptionUpcomingInvoice>>,
    #[serde(rename = "subscription", skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Box<crate::models::BillingSubscription>>,
    #[serde(rename = "license", skip_serializing_if = "Option::is_none")]
    pub license: Option<Box<crate::models::UserLicense>>,
    #[serde(rename = "expiration", skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
}

impl SubscriptionResponseModel {
    pub fn new() -> SubscriptionResponseModel {
        SubscriptionResponseModel {
            object: None,
            storage_name: None,
            storage_gb: None,
            max_storage_gb: None,
            upcoming_invoice: None,
            subscription: None,
            license: None,
            expiration: None,
        }
    }
}
