# IoK8sApiExtensionsV1beta1ReplicaSetStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available_replicas** | **i32** | The number of available replicas (ready for at least minReadySeconds) for this replica set. | [optional] 
**conditions** | [**Vec<::models::IoK8sApiExtensionsV1beta1ReplicaSetCondition>**](io.k8s.api.extensions.v1beta1.ReplicaSetCondition.md) | Represents the latest available observations of a replica set's current state. | [optional] 
**fully_labeled_replicas** | **i32** | The number of pods that have labels matching the labels of the pod template of the replicaset. | [optional] 
**observed_generation** | **i64** | ObservedGeneration reflects the generation of the most recently observed ReplicaSet. | [optional] 
**ready_replicas** | **i32** | The number of ready replicas for this replica set. | [optional] 
**replicas** | **i32** | Replicas is the most recently oberved number of replicas. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller/#what-is-a-replicationcontroller | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

