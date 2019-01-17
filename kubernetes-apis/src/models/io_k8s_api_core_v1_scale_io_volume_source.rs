/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiCoreV1ScaleIoVolumeSource : ScaleIOVolumeSource represents a persistent ScaleIO volume

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiCoreV1ScaleIoVolumeSource {
  /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Default is \"xfs\".
  #[serde(rename = "fsType")]
  fs_type: Option<String>,
  /// The host address of the ScaleIO API Gateway.
  #[serde(rename = "gateway")]
  gateway: String,
  /// The name of the ScaleIO Protection Domain for the configured storage.
  #[serde(rename = "protectionDomain")]
  protection_domain: Option<String>,
  /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
  #[serde(rename = "readOnly")]
  read_only: Option<bool>,
  #[serde(rename = "secretRef")]
  secret_ref: ::models::IoK8sApiCoreV1LocalObjectReference,
  /// Flag to enable/disable SSL communication with Gateway, default false
  #[serde(rename = "sslEnabled")]
  ssl_enabled: Option<bool>,
  /// Indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned. Default is ThinProvisioned.
  #[serde(rename = "storageMode")]
  storage_mode: Option<String>,
  /// The ScaleIO Storage Pool associated with the protection domain.
  #[serde(rename = "storagePool")]
  storage_pool: Option<String>,
  /// The name of the storage system as configured in ScaleIO.
  #[serde(rename = "system")]
  system: String,
  /// The name of a volume already created in the ScaleIO system that is associated with this volume source.
  #[serde(rename = "volumeName")]
  volume_name: Option<String>
}

impl IoK8sApiCoreV1ScaleIoVolumeSource {
  /// ScaleIOVolumeSource represents a persistent ScaleIO volume
  pub fn new(gateway: String, secret_ref: ::models::IoK8sApiCoreV1LocalObjectReference, system: String) -> IoK8sApiCoreV1ScaleIoVolumeSource {
    IoK8sApiCoreV1ScaleIoVolumeSource {
      fs_type: None,
      gateway: gateway,
      protection_domain: None,
      read_only: None,
      secret_ref: secret_ref,
      ssl_enabled: None,
      storage_mode: None,
      storage_pool: None,
      system: system,
      volume_name: None
    }
  }

  pub fn set_fs_type(&mut self, fs_type: String) {
    self.fs_type = Some(fs_type);
  }

  pub fn with_fs_type(mut self, fs_type: String) -> IoK8sApiCoreV1ScaleIoVolumeSource {
    self.fs_type = Some(fs_type);
    self
  }

  pub fn fs_type(&self) -> Option<&String> {
    self.fs_type.as_ref()
  }

  pub fn reset_fs_type(&mut self) {
    self.fs_type = None;
  }

  pub fn set_gateway(&mut self, gateway: String) {
    self.gateway = gateway;
  }

  pub fn with_gateway(mut self, gateway: String) -> IoK8sApiCoreV1ScaleIoVolumeSource {
    self.gateway = gateway;
    self
  }

  pub fn gateway(&self) -> &String {
    &self.gateway
  }


  pub fn set_protection_domain(&mut self, protection_domain: String) {
    self.protection_domain = Some(protection_domain);
  }

  pub fn with_protection_domain(mut self, protection_domain: String) -> IoK8sApiCoreV1ScaleIoVolumeSource {
    self.protection_domain = Some(protection_domain);
    self
  }

  pub fn protection_domain(&self) -> Option<&String> {
    self.protection_domain.as_ref()
  }

  pub fn reset_protection_domain(&mut self) {
    self.protection_domain = None;
  }

  pub fn set_read_only(&mut self, read_only: bool) {
    self.read_only = Some(read_only);
  }

  pub fn with_read_only(mut self, read_only: bool) -> IoK8sApiCoreV1ScaleIoVolumeSource {
    self.read_only = Some(read_only);
    self
  }

  pub fn read_only(&self) -> Option<&bool> {
    self.read_only.as_ref()
  }

  pub fn reset_read_only(&mut self) {
    self.read_only = None;
  }

  pub fn set_secret_ref(&mut self, secret_ref: ::models::IoK8sApiCoreV1LocalObjectReference) {
    self.secret_ref = secret_ref;
  }

  pub fn with_secret_ref(mut self, secret_ref: ::models::IoK8sApiCoreV1LocalObjectReference) -> IoK8sApiCoreV1ScaleIoVolumeSource {
    self.secret_ref = secret_ref;
    self
  }

  pub fn secret_ref(&self) -> &::models::IoK8sApiCoreV1LocalObjectReference {
    &self.secret_ref
  }


  pub fn set_ssl_enabled(&mut self, ssl_enabled: bool) {
    self.ssl_enabled = Some(ssl_enabled);
  }

  pub fn with_ssl_enabled(mut self, ssl_enabled: bool) -> IoK8sApiCoreV1ScaleIoVolumeSource {
    self.ssl_enabled = Some(ssl_enabled);
    self
  }

  pub fn ssl_enabled(&self) -> Option<&bool> {
    self.ssl_enabled.as_ref()
  }

  pub fn reset_ssl_enabled(&mut self) {
    self.ssl_enabled = None;
  }

  pub fn set_storage_mode(&mut self, storage_mode: String) {
    self.storage_mode = Some(storage_mode);
  }

  pub fn with_storage_mode(mut self, storage_mode: String) -> IoK8sApiCoreV1ScaleIoVolumeSource {
    self.storage_mode = Some(storage_mode);
    self
  }

  pub fn storage_mode(&self) -> Option<&String> {
    self.storage_mode.as_ref()
  }

  pub fn reset_storage_mode(&mut self) {
    self.storage_mode = None;
  }

  pub fn set_storage_pool(&mut self, storage_pool: String) {
    self.storage_pool = Some(storage_pool);
  }

  pub fn with_storage_pool(mut self, storage_pool: String) -> IoK8sApiCoreV1ScaleIoVolumeSource {
    self.storage_pool = Some(storage_pool);
    self
  }

  pub fn storage_pool(&self) -> Option<&String> {
    self.storage_pool.as_ref()
  }

  pub fn reset_storage_pool(&mut self) {
    self.storage_pool = None;
  }

  pub fn set_system(&mut self, system: String) {
    self.system = system;
  }

  pub fn with_system(mut self, system: String) -> IoK8sApiCoreV1ScaleIoVolumeSource {
    self.system = system;
    self
  }

  pub fn system(&self) -> &String {
    &self.system
  }


  pub fn set_volume_name(&mut self, volume_name: String) {
    self.volume_name = Some(volume_name);
  }

  pub fn with_volume_name(mut self, volume_name: String) -> IoK8sApiCoreV1ScaleIoVolumeSource {
    self.volume_name = Some(volume_name);
    self
  }

  pub fn volume_name(&self) -> Option<&String> {
    self.volume_name.as_ref()
  }

  pub fn reset_volume_name(&mut self) {
    self.volume_name = None;
  }

}


