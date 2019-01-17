/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiCoreV1Taint : The node this Taint is attached to has the \"effect\" on any pod that does not tolerate the Taint.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiCoreV1Taint {
  /// Required. The effect of the taint on pods that do not tolerate the taint. Valid effects are NoSchedule, PreferNoSchedule and NoExecute.
  #[serde(rename = "effect")]
  effect: String,
  /// Required. The taint key to be applied to a node.
  #[serde(rename = "key")]
  key: String,
  /// Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.
  #[serde(rename = "timeAdded")]
  time_added: Option<String>,
  /// Required. The taint value corresponding to the taint key.
  #[serde(rename = "value")]
  value: Option<String>
}

impl IoK8sApiCoreV1Taint {
  /// The node this Taint is attached to has the \"effect\" on any pod that does not tolerate the Taint.
  pub fn new(effect: String, key: String) -> IoK8sApiCoreV1Taint {
    IoK8sApiCoreV1Taint {
      effect: effect,
      key: key,
      time_added: None,
      value: None
    }
  }

  pub fn set_effect(&mut self, effect: String) {
    self.effect = effect;
  }

  pub fn with_effect(mut self, effect: String) -> IoK8sApiCoreV1Taint {
    self.effect = effect;
    self
  }

  pub fn effect(&self) -> &String {
    &self.effect
  }


  pub fn set_key(&mut self, key: String) {
    self.key = key;
  }

  pub fn with_key(mut self, key: String) -> IoK8sApiCoreV1Taint {
    self.key = key;
    self
  }

  pub fn key(&self) -> &String {
    &self.key
  }


  pub fn set_time_added(&mut self, time_added: String) {
    self.time_added = Some(time_added);
  }

  pub fn with_time_added(mut self, time_added: String) -> IoK8sApiCoreV1Taint {
    self.time_added = Some(time_added);
    self
  }

  pub fn time_added(&self) -> Option<&String> {
    self.time_added.as_ref()
  }

  pub fn reset_time_added(&mut self) {
    self.time_added = None;
  }

  pub fn set_value(&mut self, value: String) {
    self.value = Some(value);
  }

  pub fn with_value(mut self, value: String) -> IoK8sApiCoreV1Taint {
    self.value = Some(value);
    self
  }

  pub fn value(&self) -> Option<&String> {
    self.value.as_ref()
  }

  pub fn reset_value(&mut self) {
    self.value = None;
  }

}


