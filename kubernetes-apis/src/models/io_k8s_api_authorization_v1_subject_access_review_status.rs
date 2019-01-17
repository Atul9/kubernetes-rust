/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiAuthorizationV1SubjectAccessReviewStatus : SubjectAccessReviewStatus

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiAuthorizationV1SubjectAccessReviewStatus {
  /// Allowed is required. True if the action would be allowed, false otherwise.
  #[serde(rename = "allowed")]
  allowed: bool,
  /// Denied is optional. True if the action would be denied, otherwise false. If both allowed is false and denied is false, then the authorizer has no opinion on whether to authorize the action. Denied may not be true if Allowed is true.
  #[serde(rename = "denied")]
  denied: Option<bool>,
  /// EvaluationError is an indication that some error occurred during the authorization check. It is entirely possible to get an error and be able to continue determine authorization status in spite of it. For instance, RBAC can be missing a role, but enough roles are still present and bound to reason about the request.
  #[serde(rename = "evaluationError")]
  evaluation_error: Option<String>,
  /// Reason is optional.  It indicates why a request was allowed or denied.
  #[serde(rename = "reason")]
  reason: Option<String>
}

impl IoK8sApiAuthorizationV1SubjectAccessReviewStatus {
  /// SubjectAccessReviewStatus
  pub fn new(allowed: bool) -> IoK8sApiAuthorizationV1SubjectAccessReviewStatus {
    IoK8sApiAuthorizationV1SubjectAccessReviewStatus {
      allowed: allowed,
      denied: None,
      evaluation_error: None,
      reason: None
    }
  }

  pub fn set_allowed(&mut self, allowed: bool) {
    self.allowed = allowed;
  }

  pub fn with_allowed(mut self, allowed: bool) -> IoK8sApiAuthorizationV1SubjectAccessReviewStatus {
    self.allowed = allowed;
    self
  }

  pub fn allowed(&self) -> &bool {
    &self.allowed
  }


  pub fn set_denied(&mut self, denied: bool) {
    self.denied = Some(denied);
  }

  pub fn with_denied(mut self, denied: bool) -> IoK8sApiAuthorizationV1SubjectAccessReviewStatus {
    self.denied = Some(denied);
    self
  }

  pub fn denied(&self) -> Option<&bool> {
    self.denied.as_ref()
  }

  pub fn reset_denied(&mut self) {
    self.denied = None;
  }

  pub fn set_evaluation_error(&mut self, evaluation_error: String) {
    self.evaluation_error = Some(evaluation_error);
  }

  pub fn with_evaluation_error(mut self, evaluation_error: String) -> IoK8sApiAuthorizationV1SubjectAccessReviewStatus {
    self.evaluation_error = Some(evaluation_error);
    self
  }

  pub fn evaluation_error(&self) -> Option<&String> {
    self.evaluation_error.as_ref()
  }

  pub fn reset_evaluation_error(&mut self) {
    self.evaluation_error = None;
  }

  pub fn set_reason(&mut self, reason: String) {
    self.reason = Some(reason);
  }

  pub fn with_reason(mut self, reason: String) -> IoK8sApiAuthorizationV1SubjectAccessReviewStatus {
    self.reason = Some(reason);
    self
  }

  pub fn reason(&self) -> Option<&String> {
    self.reason.as_ref()
  }

  pub fn reset_reason(&mut self) {
    self.reason = None;
  }

}


