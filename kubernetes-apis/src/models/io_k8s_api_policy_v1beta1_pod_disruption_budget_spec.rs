/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiPolicyV1beta1PodDisruptionBudgetSpec : PodDisruptionBudgetSpec is a description of a PodDisruptionBudget.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiPolicyV1beta1PodDisruptionBudgetSpec {
  /// IntOrString is a type that can hold an int32 or a string.  When used in JSON or YAML marshalling and unmarshalling, it produces or consumes the inner type.  This allows you to have, for example, a JSON field that can accept a name or number.
  #[serde(rename = "maxUnavailable")]
  max_unavailable: Option<String>,
  /// IntOrString is a type that can hold an int32 or a string.  When used in JSON or YAML marshalling and unmarshalling, it produces or consumes the inner type.  This allows you to have, for example, a JSON field that can accept a name or number.
  #[serde(rename = "minAvailable")]
  min_available: Option<String>,
  #[serde(rename = "selector")]
  selector: Option<::models::IoK8sApimachineryPkgApisMetaV1LabelSelector>
}

impl IoK8sApiPolicyV1beta1PodDisruptionBudgetSpec {
  /// PodDisruptionBudgetSpec is a description of a PodDisruptionBudget.
  pub fn new() -> IoK8sApiPolicyV1beta1PodDisruptionBudgetSpec {
    IoK8sApiPolicyV1beta1PodDisruptionBudgetSpec {
      max_unavailable: None,
      min_available: None,
      selector: None
    }
  }

  pub fn set_max_unavailable(&mut self, max_unavailable: String) {
    self.max_unavailable = Some(max_unavailable);
  }

  pub fn with_max_unavailable(mut self, max_unavailable: String) -> IoK8sApiPolicyV1beta1PodDisruptionBudgetSpec {
    self.max_unavailable = Some(max_unavailable);
    self
  }

  pub fn max_unavailable(&self) -> Option<&String> {
    self.max_unavailable.as_ref()
  }

  pub fn reset_max_unavailable(&mut self) {
    self.max_unavailable = None;
  }

  pub fn set_min_available(&mut self, min_available: String) {
    self.min_available = Some(min_available);
  }

  pub fn with_min_available(mut self, min_available: String) -> IoK8sApiPolicyV1beta1PodDisruptionBudgetSpec {
    self.min_available = Some(min_available);
    self
  }

  pub fn min_available(&self) -> Option<&String> {
    self.min_available.as_ref()
  }

  pub fn reset_min_available(&mut self) {
    self.min_available = None;
  }

  pub fn set_selector(&mut self, selector: ::models::IoK8sApimachineryPkgApisMetaV1LabelSelector) {
    self.selector = Some(selector);
  }

  pub fn with_selector(mut self, selector: ::models::IoK8sApimachineryPkgApisMetaV1LabelSelector) -> IoK8sApiPolicyV1beta1PodDisruptionBudgetSpec {
    self.selector = Some(selector);
    self
  }

  pub fn selector(&self) -> Option<&::models::IoK8sApimachineryPkgApisMetaV1LabelSelector> {
    self.selector.as_ref()
  }

  pub fn reset_selector(&mut self) {
    self.selector = None;
  }

}


