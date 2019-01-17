/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiCoreV1LimitRangeSpec : LimitRangeSpec defines a min/max usage limit for resources that match on kind.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiCoreV1LimitRangeSpec {
  /// Limits is the list of LimitRangeItem objects that are enforced.
  #[serde(rename = "limits")]
  limits: Vec<::models::IoK8sApiCoreV1LimitRangeItem>
}

impl IoK8sApiCoreV1LimitRangeSpec {
  /// LimitRangeSpec defines a min/max usage limit for resources that match on kind.
  pub fn new(limits: Vec<::models::IoK8sApiCoreV1LimitRangeItem>) -> IoK8sApiCoreV1LimitRangeSpec {
    IoK8sApiCoreV1LimitRangeSpec {
      limits: limits
    }
  }

  pub fn set_limits(&mut self, limits: Vec<::models::IoK8sApiCoreV1LimitRangeItem>) {
    self.limits = limits;
  }

  pub fn with_limits(mut self, limits: Vec<::models::IoK8sApiCoreV1LimitRangeItem>) -> IoK8sApiCoreV1LimitRangeSpec {
    self.limits = limits;
    self
  }

  pub fn limits(&self) -> &Vec<::models::IoK8sApiCoreV1LimitRangeItem> {
    &self.limits
  }


}


