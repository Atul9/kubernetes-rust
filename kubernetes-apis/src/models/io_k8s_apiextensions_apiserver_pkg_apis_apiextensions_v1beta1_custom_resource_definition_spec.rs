/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionSpec : CustomResourceDefinitionSpec describes how a user wants their resource to appear

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionSpec {
  /// AdditionalPrinterColumns are additional columns shown e.g. in kubectl next to the name. Defaults to a created-at column. Optional, the global columns for all versions. Top-level and per-version columns are mutually exclusive.
  #[serde(rename = "additionalPrinterColumns")]
  additional_printer_columns: Option<Vec<::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceColumnDefinition>>,
  #[serde(rename = "conversion")]
  conversion: Option<::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceConversion>,
  /// Group is the group this resource belongs in
  #[serde(rename = "group")]
  group: String,
  #[serde(rename = "names")]
  names: ::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionNames,
  /// Scope indicates whether this resource is cluster or namespace scoped.  Default is namespaced
  #[serde(rename = "scope")]
  scope: String,
  #[serde(rename = "subresources")]
  subresources: Option<::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceSubresources>,
  #[serde(rename = "validation")]
  validation: Option<::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceValidation>,
  /// Version is the version this resource belongs in Should be always first item in Versions field if provided. Optional, but at least one of Version or Versions must be set. Deprecated: Please use `Versions`.
  #[serde(rename = "version")]
  version: Option<String>,
  /// Versions is the list of all supported versions for this resource. If Version field is provided, this field is optional. Validation: All versions must use the same validation schema for now. i.e., top level Validation field is applied to all of these versions. Order: The version name will be used to compute the order. If the version string is \"kube-like\", it will sort above non \"kube-like\" version strings, which are ordered lexicographically. \"Kube-like\" versions start with a \"v\", then are followed by a number (the major version), then optionally the string \"alpha\" or \"beta\" and another number (the minor version). These are sorted first by GA > beta > alpha (where GA is a version with no suffix such as beta or alpha), and then by comparing major version, then minor version. An example sorted list of versions: v10, v2, v1, v11beta2, v10beta3, v3beta1, v12alpha1, v11alpha2, foo1, foo10.
  #[serde(rename = "versions")]
  versions: Option<Vec<::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionVersion>>
}

impl IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionSpec {
  /// CustomResourceDefinitionSpec describes how a user wants their resource to appear
  pub fn new(group: String, names: ::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionNames, scope: String) -> IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionSpec {
    IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionSpec {
      additional_printer_columns: None,
      conversion: None,
      group: group,
      names: names,
      scope: scope,
      subresources: None,
      validation: None,
      version: None,
      versions: None
    }
  }

  pub fn set_additional_printer_columns(&mut self, additional_printer_columns: Vec<::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceColumnDefinition>) {
    self.additional_printer_columns = Some(additional_printer_columns);
  }

  pub fn with_additional_printer_columns(mut self, additional_printer_columns: Vec<::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceColumnDefinition>) -> IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionSpec {
    self.additional_printer_columns = Some(additional_printer_columns);
    self
  }

  pub fn additional_printer_columns(&self) -> Option<&Vec<::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceColumnDefinition>> {
    self.additional_printer_columns.as_ref()
  }

  pub fn reset_additional_printer_columns(&mut self) {
    self.additional_printer_columns = None;
  }

  pub fn set_conversion(&mut self, conversion: ::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceConversion) {
    self.conversion = Some(conversion);
  }

  pub fn with_conversion(mut self, conversion: ::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceConversion) -> IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionSpec {
    self.conversion = Some(conversion);
    self
  }

  pub fn conversion(&self) -> Option<&::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceConversion> {
    self.conversion.as_ref()
  }

  pub fn reset_conversion(&mut self) {
    self.conversion = None;
  }

  pub fn set_group(&mut self, group: String) {
    self.group = group;
  }

  pub fn with_group(mut self, group: String) -> IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionSpec {
    self.group = group;
    self
  }

  pub fn group(&self) -> &String {
    &self.group
  }


  pub fn set_names(&mut self, names: ::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionNames) {
    self.names = names;
  }

  pub fn with_names(mut self, names: ::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionNames) -> IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionSpec {
    self.names = names;
    self
  }

  pub fn names(&self) -> &::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionNames {
    &self.names
  }


  pub fn set_scope(&mut self, scope: String) {
    self.scope = scope;
  }

  pub fn with_scope(mut self, scope: String) -> IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionSpec {
    self.scope = scope;
    self
  }

  pub fn scope(&self) -> &String {
    &self.scope
  }


  pub fn set_subresources(&mut self, subresources: ::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceSubresources) {
    self.subresources = Some(subresources);
  }

  pub fn with_subresources(mut self, subresources: ::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceSubresources) -> IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionSpec {
    self.subresources = Some(subresources);
    self
  }

  pub fn subresources(&self) -> Option<&::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceSubresources> {
    self.subresources.as_ref()
  }

  pub fn reset_subresources(&mut self) {
    self.subresources = None;
  }

  pub fn set_validation(&mut self, validation: ::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceValidation) {
    self.validation = Some(validation);
  }

  pub fn with_validation(mut self, validation: ::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceValidation) -> IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionSpec {
    self.validation = Some(validation);
    self
  }

  pub fn validation(&self) -> Option<&::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceValidation> {
    self.validation.as_ref()
  }

  pub fn reset_validation(&mut self) {
    self.validation = None;
  }

  pub fn set_version(&mut self, version: String) {
    self.version = Some(version);
  }

  pub fn with_version(mut self, version: String) -> IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionSpec {
    self.version = Some(version);
    self
  }

  pub fn version(&self) -> Option<&String> {
    self.version.as_ref()
  }

  pub fn reset_version(&mut self) {
    self.version = None;
  }

  pub fn set_versions(&mut self, versions: Vec<::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionVersion>) {
    self.versions = Some(versions);
  }

  pub fn with_versions(mut self, versions: Vec<::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionVersion>) -> IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionSpec {
    self.versions = Some(versions);
    self
  }

  pub fn versions(&self) -> Option<&Vec<::models::IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionVersion>> {
    self.versions.as_ref()
  }

  pub fn reset_versions(&mut self) {
    self.versions = None;
  }

}


