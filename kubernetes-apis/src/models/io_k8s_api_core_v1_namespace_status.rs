/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiCoreV1NamespaceStatus : NamespaceStatus is information about the current status of a Namespace.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiCoreV1NamespaceStatus {
  /// Phase is the current lifecycle phase of the namespace. More info: https://kubernetes.io/docs/tasks/administer-cluster/namespaces/
  #[serde(rename = "phase")]
  phase: Option<String>
}

impl IoK8sApiCoreV1NamespaceStatus {
  /// NamespaceStatus is information about the current status of a Namespace.
  pub fn new() -> IoK8sApiCoreV1NamespaceStatus {
    IoK8sApiCoreV1NamespaceStatus {
      phase: None
    }
  }

  pub fn set_phase(&mut self, phase: String) {
    self.phase = Some(phase);
  }

  pub fn with_phase(mut self, phase: String) -> IoK8sApiCoreV1NamespaceStatus {
    self.phase = Some(phase);
    self
  }

  pub fn phase(&self) -> Option<&String> {
    self.phase.as_ref()
  }

  pub fn reset_phase(&mut self) {
    self.phase = None;
  }

}


