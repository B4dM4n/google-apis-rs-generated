#![doc = "# Resources and Methods\n* [projects](resources/projects/struct.ProjectsActions.html)\n  * [*export*](resources/projects/struct.ExportRequestBuilder.html), [*import*](resources/projects/struct.ImportRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
    #[doc = "View and manage your Google Cloud Datastore data\n\n`https://www.googleapis.com/auth/datastore`"]
    pub const DATASTORE: &str = "https://www.googleapis.com/auth/datastore";
}
pub mod schemas {
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleDatastoreAdminV1Beta1CommonMetadata {
        #[doc = "The time the operation ended, either successfully or otherwise."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "The client-assigned labels which were provided when the operation was created. May also include additional labels."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The type of the operation. Can be used as a filter in ListOperationsRequest."]
        #[serde(
            rename = "operationType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation_type: ::std::option::Option<
            crate::schemas::GoogleDatastoreAdminV1Beta1CommonMetadataOperationType,
        >,
        #[doc = "The time that work began on the operation."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "The current state of the Operation."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Beta1CommonMetadataState>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1CommonMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1Beta1CommonMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleDatastoreAdminV1Beta1CommonMetadataOperationType {
        #[doc = "ExportEntities."]
        ExportEntities,
        #[doc = "ImportEntities."]
        ImportEntities,
        #[doc = "Unspecified."]
        OperationTypeUnspecified,
    }
    impl GoogleDatastoreAdminV1Beta1CommonMetadataOperationType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleDatastoreAdminV1Beta1CommonMetadataOperationType :: ExportEntities => "EXPORT_ENTITIES" , GoogleDatastoreAdminV1Beta1CommonMetadataOperationType :: ImportEntities => "IMPORT_ENTITIES" , GoogleDatastoreAdminV1Beta1CommonMetadataOperationType :: OperationTypeUnspecified => "OPERATION_TYPE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleDatastoreAdminV1Beta1CommonMetadataOperationType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleDatastoreAdminV1Beta1CommonMetadataOperationType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleDatastoreAdminV1Beta1CommonMetadataOperationType, ()>
        {
            Ok(match s {
                "EXPORT_ENTITIES" => {
                    GoogleDatastoreAdminV1Beta1CommonMetadataOperationType::ExportEntities
                }
                "IMPORT_ENTITIES" => {
                    GoogleDatastoreAdminV1Beta1CommonMetadataOperationType::ImportEntities
                }
                "OPERATION_TYPE_UNSPECIFIED" => {
                    GoogleDatastoreAdminV1Beta1CommonMetadataOperationType::OperationTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleDatastoreAdminV1Beta1CommonMetadataOperationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDatastoreAdminV1Beta1CommonMetadataOperationType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleDatastoreAdminV1Beta1CommonMetadataOperationType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXPORT_ENTITIES" => {
                    GoogleDatastoreAdminV1Beta1CommonMetadataOperationType::ExportEntities
                }
                "IMPORT_ENTITIES" => {
                    GoogleDatastoreAdminV1Beta1CommonMetadataOperationType::ImportEntities
                }
                "OPERATION_TYPE_UNSPECIFIED" => {
                    GoogleDatastoreAdminV1Beta1CommonMetadataOperationType::OperationTypeUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleDatastoreAdminV1Beta1CommonMetadataOperationType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleDatastoreAdminV1Beta1CommonMetadataOperationType
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleDatastoreAdminV1Beta1CommonMetadataState {
        #[doc = "Request has finished being cancelled after user called google.longrunning.Operations.CancelOperation."]
        Cancelled,
        #[doc = "Request is in the process of being cancelled after user called google.longrunning.Operations.CancelOperation on the operation."]
        Cancelling,
        #[doc = "Request has finished being processed, but encountered an error."]
        Failed,
        #[doc = "Request has been processed and is in its finalization stage."]
        Finalizing,
        #[doc = "Request is being prepared for processing."]
        Initializing,
        #[doc = "Request is actively being processed."]
        Processing,
        #[doc = "Unspecified."]
        StateUnspecified,
        #[doc = "Request has completed successfully."]
        Successful,
    }
    impl GoogleDatastoreAdminV1Beta1CommonMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleDatastoreAdminV1Beta1CommonMetadataState::Cancelled => "CANCELLED",
                GoogleDatastoreAdminV1Beta1CommonMetadataState::Cancelling => "CANCELLING",
                GoogleDatastoreAdminV1Beta1CommonMetadataState::Failed => "FAILED",
                GoogleDatastoreAdminV1Beta1CommonMetadataState::Finalizing => "FINALIZING",
                GoogleDatastoreAdminV1Beta1CommonMetadataState::Initializing => "INITIALIZING",
                GoogleDatastoreAdminV1Beta1CommonMetadataState::Processing => "PROCESSING",
                GoogleDatastoreAdminV1Beta1CommonMetadataState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
                GoogleDatastoreAdminV1Beta1CommonMetadataState::Successful => "SUCCESSFUL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleDatastoreAdminV1Beta1CommonMetadataState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleDatastoreAdminV1Beta1CommonMetadataState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleDatastoreAdminV1Beta1CommonMetadataState, ()> {
            Ok(match s {
                "CANCELLED" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Cancelled,
                "CANCELLING" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Cancelling,
                "FAILED" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Failed,
                "FINALIZING" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Finalizing,
                "INITIALIZING" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Initializing,
                "PROCESSING" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Processing,
                "STATE_UNSPECIFIED" => {
                    GoogleDatastoreAdminV1Beta1CommonMetadataState::StateUnspecified
                }
                "SUCCESSFUL" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Successful,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleDatastoreAdminV1Beta1CommonMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDatastoreAdminV1Beta1CommonMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleDatastoreAdminV1Beta1CommonMetadataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Cancelled,
                "CANCELLING" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Cancelling,
                "FAILED" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Failed,
                "FINALIZING" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Finalizing,
                "INITIALIZING" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Initializing,
                "PROCESSING" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Processing,
                "STATE_UNSPECIFIED" => {
                    GoogleDatastoreAdminV1Beta1CommonMetadataState::StateUnspecified
                }
                "SUCCESSFUL" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Successful,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1CommonMetadataState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1Beta1CommonMetadataState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleDatastoreAdminV1Beta1EntityFilter {
        #[doc = "If empty, then this represents all kinds."]
        #[serde(
            rename = "kinds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kinds: ::std::option::Option<Vec<String>>,
        #[doc = "An empty list represents all namespaces. This is the preferred usage for projects that don’t use namespaces. An empty string element represents the default namespace. This should be used if the project has data in non-default namespaces, but doesn’t want to include them. Each namespace in this list must be unique."]
        #[serde(
            rename = "namespaceIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub namespace_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1EntityFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1Beta1EntityFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleDatastoreAdminV1Beta1ExportEntitiesMetadata {
        #[doc = "Metadata common to all Datastore Admin operations."]
        #[serde(
            rename = "common",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Beta1CommonMetadata>,
        #[doc = "Description of which entities are being exported."]
        #[serde(
            rename = "entityFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_filter:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Beta1EntityFilter>,
        #[doc = "Location for the export metadata and data files. This will be the same value as the google.datastore.admin.v1beta1.ExportEntitiesRequest.output_url_prefix field. The final output location is provided in google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url."]
        #[serde(
            rename = "outputUrlPrefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_url_prefix: ::std::option::Option<String>,
        #[doc = "An estimate of the number of bytes processed."]
        #[serde(
            rename = "progressBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_bytes:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Beta1Progress>,
        #[doc = "An estimate of the number of entities processed."]
        #[serde(
            rename = "progressEntities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_entities:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Beta1Progress>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1ExportEntitiesMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1Beta1ExportEntitiesMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleDatastoreAdminV1Beta1ExportEntitiesRequest {
        #[doc = "Description of what data from the project is included in the export."]
        #[serde(
            rename = "entityFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_filter:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Beta1EntityFilter>,
        #[doc = "Client-assigned labels."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Location for the export metadata and data files. The full resource URL of the external storage location. Currently, only Google Cloud Storage is supported. So output_url_prefix should be of the form: `gs://BUCKET_NAME[/NAMESPACE_PATH]`, where `BUCKET_NAME` is the name of the Cloud Storage bucket and `NAMESPACE_PATH` is an optional Cloud Storage namespace path (this is not a Cloud Datastore namespace). For more information about Cloud Storage namespace paths, see [Object name considerations](https://cloud.google.com/storage/docs/naming#object-considerations). The resulting files will be nested deeper than the specified URL prefix. The final output URL will be provided in the google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url field. That value should be used for subsequent ImportEntities operations. By nesting the data files deeper, the same Cloud Storage bucket can be used in multiple ExportEntities operations without conflict."]
        #[serde(
            rename = "outputUrlPrefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_url_prefix: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1ExportEntitiesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1Beta1ExportEntitiesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleDatastoreAdminV1Beta1ExportEntitiesResponse {
        #[doc = "Location of the output metadata file. This can be used to begin an import into Cloud Datastore (this project or another project). See google.datastore.admin.v1beta1.ImportEntitiesRequest.input_url. Only present if the operation completed successfully."]
        #[serde(
            rename = "outputUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1ExportEntitiesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1Beta1ExportEntitiesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleDatastoreAdminV1Beta1ImportEntitiesMetadata {
        #[doc = "Metadata common to all Datastore Admin operations."]
        #[serde(
            rename = "common",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Beta1CommonMetadata>,
        #[doc = "Description of which entities are being imported."]
        #[serde(
            rename = "entityFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_filter:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Beta1EntityFilter>,
        #[doc = "The location of the import metadata file. This will be the same value as the google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url field."]
        #[serde(
            rename = "inputUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_url: ::std::option::Option<String>,
        #[doc = "An estimate of the number of bytes processed."]
        #[serde(
            rename = "progressBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_bytes:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Beta1Progress>,
        #[doc = "An estimate of the number of entities processed."]
        #[serde(
            rename = "progressEntities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_entities:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Beta1Progress>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1ImportEntitiesMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1Beta1ImportEntitiesMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleDatastoreAdminV1Beta1ImportEntitiesRequest {
        #[doc = "Optionally specify which kinds/namespaces are to be imported. If provided, the list must be a subset of the EntityFilter used in creating the export, otherwise a FAILED_PRECONDITION error will be returned. If no filter is specified then all entities from the export are imported."]
        #[serde(
            rename = "entityFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_filter:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Beta1EntityFilter>,
        #[doc = "The full resource URL of the external storage location. Currently, only Google Cloud Storage is supported. So input_url should be of the form: `gs://BUCKET_NAME[/NAMESPACE_PATH]/OVERALL_EXPORT_METADATA_FILE`, where `BUCKET_NAME` is the name of the Cloud Storage bucket, `NAMESPACE_PATH` is an optional Cloud Storage namespace path (this is not a Cloud Datastore namespace), and `OVERALL_EXPORT_METADATA_FILE` is the metadata file written by the ExportEntities operation. For more information about Cloud Storage namespace paths, see [Object name considerations](https://cloud.google.com/storage/docs/naming#object-considerations). For more information, see google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url."]
        #[serde(
            rename = "inputUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_url: ::std::option::Option<String>,
        #[doc = "Client-assigned labels."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1ImportEntitiesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1Beta1ImportEntitiesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleDatastoreAdminV1Beta1Progress {
        #[doc = "The amount of work that has been completed. Note that this may be greater than work_estimated."]
        #[serde(
            rename = "workCompleted",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub work_completed: ::std::option::Option<i64>,
        #[doc = "An estimate of how much work needs to be performed. May be zero if the work estimate is unavailable."]
        #[serde(
            rename = "workEstimated",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub work_estimated: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1Progress {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1Beta1Progress {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleDatastoreAdminV1CommonMetadata {
        #[doc = "The time the operation ended, either successfully or otherwise."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "The client-assigned labels which were provided when the operation was created. May also include additional labels."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The type of the operation. Can be used as a filter in ListOperationsRequest."]
        #[serde(
            rename = "operationType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation_type: ::std::option::Option<
            crate::schemas::GoogleDatastoreAdminV1CommonMetadataOperationType,
        >,
        #[doc = "The time that work began on the operation."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "The current state of the Operation."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1CommonMetadataState>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1CommonMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1CommonMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleDatastoreAdminV1CommonMetadataOperationType {
        #[doc = "CreateIndex."]
        CreateIndex,
        #[doc = "DeleteIndex."]
        DeleteIndex,
        #[doc = "ExportEntities."]
        ExportEntities,
        #[doc = "ImportEntities."]
        ImportEntities,
        #[doc = "Unspecified."]
        OperationTypeUnspecified,
    }
    impl GoogleDatastoreAdminV1CommonMetadataOperationType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleDatastoreAdminV1CommonMetadataOperationType::CreateIndex => "CREATE_INDEX",
                GoogleDatastoreAdminV1CommonMetadataOperationType::DeleteIndex => "DELETE_INDEX",
                GoogleDatastoreAdminV1CommonMetadataOperationType::ExportEntities => {
                    "EXPORT_ENTITIES"
                }
                GoogleDatastoreAdminV1CommonMetadataOperationType::ImportEntities => {
                    "IMPORT_ENTITIES"
                }
                GoogleDatastoreAdminV1CommonMetadataOperationType::OperationTypeUnspecified => {
                    "OPERATION_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleDatastoreAdminV1CommonMetadataOperationType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleDatastoreAdminV1CommonMetadataOperationType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleDatastoreAdminV1CommonMetadataOperationType, ()> {
            Ok(match s {
                "CREATE_INDEX" => GoogleDatastoreAdminV1CommonMetadataOperationType::CreateIndex,
                "DELETE_INDEX" => GoogleDatastoreAdminV1CommonMetadataOperationType::DeleteIndex,
                "EXPORT_ENTITIES" => {
                    GoogleDatastoreAdminV1CommonMetadataOperationType::ExportEntities
                }
                "IMPORT_ENTITIES" => {
                    GoogleDatastoreAdminV1CommonMetadataOperationType::ImportEntities
                }
                "OPERATION_TYPE_UNSPECIFIED" => {
                    GoogleDatastoreAdminV1CommonMetadataOperationType::OperationTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleDatastoreAdminV1CommonMetadataOperationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDatastoreAdminV1CommonMetadataOperationType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleDatastoreAdminV1CommonMetadataOperationType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATE_INDEX" => GoogleDatastoreAdminV1CommonMetadataOperationType::CreateIndex,
                "DELETE_INDEX" => GoogleDatastoreAdminV1CommonMetadataOperationType::DeleteIndex,
                "EXPORT_ENTITIES" => {
                    GoogleDatastoreAdminV1CommonMetadataOperationType::ExportEntities
                }
                "IMPORT_ENTITIES" => {
                    GoogleDatastoreAdminV1CommonMetadataOperationType::ImportEntities
                }
                "OPERATION_TYPE_UNSPECIFIED" => {
                    GoogleDatastoreAdminV1CommonMetadataOperationType::OperationTypeUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1CommonMetadataOperationType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1CommonMetadataOperationType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleDatastoreAdminV1CommonMetadataState {
        #[doc = "Request has finished being cancelled after user called google.longrunning.Operations.CancelOperation."]
        Cancelled,
        #[doc = "Request is in the process of being cancelled after user called google.longrunning.Operations.CancelOperation on the operation."]
        Cancelling,
        #[doc = "Request has finished being processed, but encountered an error."]
        Failed,
        #[doc = "Request has been processed and is in its finalization stage."]
        Finalizing,
        #[doc = "Request is being prepared for processing."]
        Initializing,
        #[doc = "Request is actively being processed."]
        Processing,
        #[doc = "Unspecified."]
        StateUnspecified,
        #[doc = "Request has completed successfully."]
        Successful,
    }
    impl GoogleDatastoreAdminV1CommonMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleDatastoreAdminV1CommonMetadataState::Cancelled => "CANCELLED",
                GoogleDatastoreAdminV1CommonMetadataState::Cancelling => "CANCELLING",
                GoogleDatastoreAdminV1CommonMetadataState::Failed => "FAILED",
                GoogleDatastoreAdminV1CommonMetadataState::Finalizing => "FINALIZING",
                GoogleDatastoreAdminV1CommonMetadataState::Initializing => "INITIALIZING",
                GoogleDatastoreAdminV1CommonMetadataState::Processing => "PROCESSING",
                GoogleDatastoreAdminV1CommonMetadataState::StateUnspecified => "STATE_UNSPECIFIED",
                GoogleDatastoreAdminV1CommonMetadataState::Successful => "SUCCESSFUL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleDatastoreAdminV1CommonMetadataState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleDatastoreAdminV1CommonMetadataState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleDatastoreAdminV1CommonMetadataState, ()> {
            Ok(match s {
                "CANCELLED" => GoogleDatastoreAdminV1CommonMetadataState::Cancelled,
                "CANCELLING" => GoogleDatastoreAdminV1CommonMetadataState::Cancelling,
                "FAILED" => GoogleDatastoreAdminV1CommonMetadataState::Failed,
                "FINALIZING" => GoogleDatastoreAdminV1CommonMetadataState::Finalizing,
                "INITIALIZING" => GoogleDatastoreAdminV1CommonMetadataState::Initializing,
                "PROCESSING" => GoogleDatastoreAdminV1CommonMetadataState::Processing,
                "STATE_UNSPECIFIED" => GoogleDatastoreAdminV1CommonMetadataState::StateUnspecified,
                "SUCCESSFUL" => GoogleDatastoreAdminV1CommonMetadataState::Successful,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleDatastoreAdminV1CommonMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDatastoreAdminV1CommonMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleDatastoreAdminV1CommonMetadataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => GoogleDatastoreAdminV1CommonMetadataState::Cancelled,
                "CANCELLING" => GoogleDatastoreAdminV1CommonMetadataState::Cancelling,
                "FAILED" => GoogleDatastoreAdminV1CommonMetadataState::Failed,
                "FINALIZING" => GoogleDatastoreAdminV1CommonMetadataState::Finalizing,
                "INITIALIZING" => GoogleDatastoreAdminV1CommonMetadataState::Initializing,
                "PROCESSING" => GoogleDatastoreAdminV1CommonMetadataState::Processing,
                "STATE_UNSPECIFIED" => GoogleDatastoreAdminV1CommonMetadataState::StateUnspecified,
                "SUCCESSFUL" => GoogleDatastoreAdminV1CommonMetadataState::Successful,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1CommonMetadataState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1CommonMetadataState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadata {
        #[doc = "The current state of migration from Cloud Datastore to Cloud Firestore in Datastore mode."]
        #[serde(
            rename = "migrationState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub migration_state: ::std::option::Option<
            crate::schemas::GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationState,
        >,
        #[doc = "The current step of migration from Cloud Datastore to Cloud Firestore in Datastore mode."]
        #[serde(
            rename = "migrationStep",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub migration_step: ::std::option::Option<
            crate::schemas::GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadata
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationState {
        #[doc = "The migration is complete."]
        Complete,
        #[doc = "Unspecified."]
        MigrationStateUnspecified,
        #[doc = "The migration is paused."]
        Paused,
        #[doc = "The migration is running."]
        Running,
    }
    impl GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationState {
        pub fn as_str(self) -> &'static str {
            match self { GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationState :: Complete => "COMPLETE" , GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationState :: MigrationStateUnspecified => "MIGRATION_STATE_UNSPECIFIED" , GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationState :: Paused => "PAUSED" , GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationState :: Running => "RUNNING" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationState
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationState
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationState,
            (),
        > {
            Ok (match s { "COMPLETE" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationState :: Complete , "MIGRATION_STATE_UNSPECIFIED" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationState :: MigrationStateUnspecified , "PAUSED" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationState :: Paused , "RUNNING" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationState :: Running , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationState
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationState
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationState
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "COMPLETE" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationState :: Complete , "MIGRATION_STATE_UNSPECIFIED" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationState :: MigrationStateUnspecified , "PAUSED" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationState :: Paused , "RUNNING" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationState :: Running , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationState
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep {
        #[doc = "Writes are applied synchronously to at least one replica."]
        ApplyWritesSynchronously,
        #[doc = "Data is copied to Cloud Firestore and then verified to match the data in Cloud Datastore."]
        CopyAndVerify,
        #[doc = "Unspecified."]
        MigrationStepUnspecified,
        #[doc = "Pre-migration: the database is prepared for migration."]
        Prepare,
        #[doc = "Eventually-consistent reads are redirected to Cloud Firestore."]
        RedirectEventuallyConsistentReads,
        #[doc = "Strongly-consistent reads are redirected to Cloud Firestore."]
        RedirectStronglyConsistentReads,
        #[doc = "Writes are redirected to Cloud Firestore."]
        RedirectWrites,
        #[doc = "Start of migration."]
        Start,
    }
    impl GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep {
        pub fn as_str(self) -> &'static str {
            match self { GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: ApplyWritesSynchronously => "APPLY_WRITES_SYNCHRONOUSLY" , GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: CopyAndVerify => "COPY_AND_VERIFY" , GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: MigrationStepUnspecified => "MIGRATION_STEP_UNSPECIFIED" , GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: Prepare => "PREPARE" , GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: RedirectEventuallyConsistentReads => "REDIRECT_EVENTUALLY_CONSISTENT_READS" , GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: RedirectStronglyConsistentReads => "REDIRECT_STRONGLY_CONSISTENT_READS" , GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: RedirectWrites => "REDIRECT_WRITES" , GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: Start => "START" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep,
            (),
        > {
            Ok (match s { "APPLY_WRITES_SYNCHRONOUSLY" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: ApplyWritesSynchronously , "COPY_AND_VERIFY" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: CopyAndVerify , "MIGRATION_STEP_UNSPECIFIED" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: MigrationStepUnspecified , "PREPARE" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: Prepare , "REDIRECT_EVENTUALLY_CONSISTENT_READS" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: RedirectEventuallyConsistentReads , "REDIRECT_STRONGLY_CONSISTENT_READS" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: RedirectStronglyConsistentReads , "REDIRECT_WRITES" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: RedirectWrites , "START" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: Start , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "APPLY_WRITES_SYNCHRONOUSLY" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: ApplyWritesSynchronously , "COPY_AND_VERIFY" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: CopyAndVerify , "MIGRATION_STEP_UNSPECIFIED" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: MigrationStepUnspecified , "PREPARE" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: Prepare , "REDIRECT_EVENTUALLY_CONSISTENT_READS" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: RedirectEventuallyConsistentReads , "REDIRECT_STRONGLY_CONSISTENT_READS" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: RedirectStronglyConsistentReads , "REDIRECT_WRITES" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: RedirectWrites , "START" => GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep :: Start , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleDatastoreAdminV1DatastoreFirestoreMigrationMetadataMigrationStep
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleDatastoreAdminV1EntityFilter {
        #[doc = "If empty, then this represents all kinds."]
        #[serde(
            rename = "kinds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kinds: ::std::option::Option<Vec<String>>,
        #[doc = "An empty list represents all namespaces. This is the preferred usage for projects that don’t use namespaces. An empty string element represents the default namespace. This should be used if the project has data in non-default namespaces, but doesn’t want to include them. Each namespace in this list must be unique."]
        #[serde(
            rename = "namespaceIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub namespace_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1EntityFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1EntityFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleDatastoreAdminV1ExportEntitiesMetadata {
        #[doc = "Metadata common to all Datastore Admin operations."]
        #[serde(
            rename = "common",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common: ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1CommonMetadata>,
        #[doc = "Description of which entities are being exported."]
        #[serde(
            rename = "entityFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_filter:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1EntityFilter>,
        #[doc = "Location for the export metadata and data files. This will be the same value as the google.datastore.admin.v1.ExportEntitiesRequest.output_url_prefix field. The final output location is provided in google.datastore.admin.v1.ExportEntitiesResponse.output_url."]
        #[serde(
            rename = "outputUrlPrefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_url_prefix: ::std::option::Option<String>,
        #[doc = "An estimate of the number of bytes processed."]
        #[serde(
            rename = "progressBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_bytes: ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Progress>,
        #[doc = "An estimate of the number of entities processed."]
        #[serde(
            rename = "progressEntities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_entities:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Progress>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1ExportEntitiesMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1ExportEntitiesMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleDatastoreAdminV1ExportEntitiesResponse {
        #[doc = "Location of the output metadata file. This can be used to begin an import into Cloud Datastore (this project or another project). See google.datastore.admin.v1.ImportEntitiesRequest.input_url. Only present if the operation completed successfully."]
        #[serde(
            rename = "outputUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1ExportEntitiesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1ExportEntitiesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleDatastoreAdminV1ImportEntitiesMetadata {
        #[doc = "Metadata common to all Datastore Admin operations."]
        #[serde(
            rename = "common",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common: ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1CommonMetadata>,
        #[doc = "Description of which entities are being imported."]
        #[serde(
            rename = "entityFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_filter:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1EntityFilter>,
        #[doc = "The location of the import metadata file. This will be the same value as the google.datastore.admin.v1.ExportEntitiesResponse.output_url field."]
        #[serde(
            rename = "inputUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_url: ::std::option::Option<String>,
        #[doc = "An estimate of the number of bytes processed."]
        #[serde(
            rename = "progressBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_bytes: ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Progress>,
        #[doc = "An estimate of the number of entities processed."]
        #[serde(
            rename = "progressEntities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_entities:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Progress>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1ImportEntitiesMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1ImportEntitiesMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleDatastoreAdminV1IndexOperationMetadata {
        #[doc = "Metadata common to all Datastore Admin operations."]
        #[serde(
            rename = "common",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common: ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1CommonMetadata>,
        #[doc = "The index resource ID that this operation is acting on."]
        #[serde(
            rename = "indexId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub index_id: ::std::option::Option<String>,
        #[doc = "An estimate of the number of entities processed."]
        #[serde(
            rename = "progressEntities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_entities:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Progress>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1IndexOperationMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1IndexOperationMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleDatastoreAdminV1MigrationProgressEvent {
        #[doc = "Details for the `PREPARE` step."]
        #[serde(
            rename = "prepareStepDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub prepare_step_details:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1PrepareStepDetails>,
        #[doc = "Details for the `REDIRECT_WRITES` step."]
        #[serde(
            rename = "redirectWritesStepDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub redirect_writes_step_details:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1RedirectWritesStepDetails>,
        #[doc = "The step that is starting. An event with step set to `START` indicates that the migration has been reverted back to the initial pre-migration state."]
        #[serde(
            rename = "step",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub step:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1MigrationProgressEventStep>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1MigrationProgressEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1MigrationProgressEvent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleDatastoreAdminV1MigrationProgressEventStep {
        #[doc = "Writes are applied synchronously to at least one replica."]
        ApplyWritesSynchronously,
        #[doc = "Data is copied to Cloud Firestore and then verified to match the data in Cloud Datastore."]
        CopyAndVerify,
        #[doc = "Unspecified."]
        MigrationStepUnspecified,
        #[doc = "Pre-migration: the database is prepared for migration."]
        Prepare,
        #[doc = "Eventually-consistent reads are redirected to Cloud Firestore."]
        RedirectEventuallyConsistentReads,
        #[doc = "Strongly-consistent reads are redirected to Cloud Firestore."]
        RedirectStronglyConsistentReads,
        #[doc = "Writes are redirected to Cloud Firestore."]
        RedirectWrites,
        #[doc = "Start of migration."]
        Start,
    }
    impl GoogleDatastoreAdminV1MigrationProgressEventStep {
        pub fn as_str(self) -> &'static str {
            match self { GoogleDatastoreAdminV1MigrationProgressEventStep :: ApplyWritesSynchronously => "APPLY_WRITES_SYNCHRONOUSLY" , GoogleDatastoreAdminV1MigrationProgressEventStep :: CopyAndVerify => "COPY_AND_VERIFY" , GoogleDatastoreAdminV1MigrationProgressEventStep :: MigrationStepUnspecified => "MIGRATION_STEP_UNSPECIFIED" , GoogleDatastoreAdminV1MigrationProgressEventStep :: Prepare => "PREPARE" , GoogleDatastoreAdminV1MigrationProgressEventStep :: RedirectEventuallyConsistentReads => "REDIRECT_EVENTUALLY_CONSISTENT_READS" , GoogleDatastoreAdminV1MigrationProgressEventStep :: RedirectStronglyConsistentReads => "REDIRECT_STRONGLY_CONSISTENT_READS" , GoogleDatastoreAdminV1MigrationProgressEventStep :: RedirectWrites => "REDIRECT_WRITES" , GoogleDatastoreAdminV1MigrationProgressEventStep :: Start => "START" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleDatastoreAdminV1MigrationProgressEventStep {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleDatastoreAdminV1MigrationProgressEventStep {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleDatastoreAdminV1MigrationProgressEventStep, ()> {
            Ok (match s { "APPLY_WRITES_SYNCHRONOUSLY" => GoogleDatastoreAdminV1MigrationProgressEventStep :: ApplyWritesSynchronously , "COPY_AND_VERIFY" => GoogleDatastoreAdminV1MigrationProgressEventStep :: CopyAndVerify , "MIGRATION_STEP_UNSPECIFIED" => GoogleDatastoreAdminV1MigrationProgressEventStep :: MigrationStepUnspecified , "PREPARE" => GoogleDatastoreAdminV1MigrationProgressEventStep :: Prepare , "REDIRECT_EVENTUALLY_CONSISTENT_READS" => GoogleDatastoreAdminV1MigrationProgressEventStep :: RedirectEventuallyConsistentReads , "REDIRECT_STRONGLY_CONSISTENT_READS" => GoogleDatastoreAdminV1MigrationProgressEventStep :: RedirectStronglyConsistentReads , "REDIRECT_WRITES" => GoogleDatastoreAdminV1MigrationProgressEventStep :: RedirectWrites , "START" => GoogleDatastoreAdminV1MigrationProgressEventStep :: Start , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleDatastoreAdminV1MigrationProgressEventStep {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDatastoreAdminV1MigrationProgressEventStep {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleDatastoreAdminV1MigrationProgressEventStep {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "APPLY_WRITES_SYNCHRONOUSLY" => GoogleDatastoreAdminV1MigrationProgressEventStep :: ApplyWritesSynchronously , "COPY_AND_VERIFY" => GoogleDatastoreAdminV1MigrationProgressEventStep :: CopyAndVerify , "MIGRATION_STEP_UNSPECIFIED" => GoogleDatastoreAdminV1MigrationProgressEventStep :: MigrationStepUnspecified , "PREPARE" => GoogleDatastoreAdminV1MigrationProgressEventStep :: Prepare , "REDIRECT_EVENTUALLY_CONSISTENT_READS" => GoogleDatastoreAdminV1MigrationProgressEventStep :: RedirectEventuallyConsistentReads , "REDIRECT_STRONGLY_CONSISTENT_READS" => GoogleDatastoreAdminV1MigrationProgressEventStep :: RedirectStronglyConsistentReads , "REDIRECT_WRITES" => GoogleDatastoreAdminV1MigrationProgressEventStep :: RedirectWrites , "START" => GoogleDatastoreAdminV1MigrationProgressEventStep :: Start , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1MigrationProgressEventStep {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1MigrationProgressEventStep {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleDatastoreAdminV1MigrationStateEvent {
        #[doc = "The new state of the migration."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1MigrationStateEventState>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1MigrationStateEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1MigrationStateEvent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleDatastoreAdminV1MigrationStateEventState {
        #[doc = "The migration is complete."]
        Complete,
        #[doc = "Unspecified."]
        MigrationStateUnspecified,
        #[doc = "The migration is paused."]
        Paused,
        #[doc = "The migration is running."]
        Running,
    }
    impl GoogleDatastoreAdminV1MigrationStateEventState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleDatastoreAdminV1MigrationStateEventState::Complete => "COMPLETE",
                GoogleDatastoreAdminV1MigrationStateEventState::MigrationStateUnspecified => {
                    "MIGRATION_STATE_UNSPECIFIED"
                }
                GoogleDatastoreAdminV1MigrationStateEventState::Paused => "PAUSED",
                GoogleDatastoreAdminV1MigrationStateEventState::Running => "RUNNING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleDatastoreAdminV1MigrationStateEventState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleDatastoreAdminV1MigrationStateEventState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleDatastoreAdminV1MigrationStateEventState, ()> {
            Ok(match s {
                "COMPLETE" => GoogleDatastoreAdminV1MigrationStateEventState::Complete,
                "MIGRATION_STATE_UNSPECIFIED" => {
                    GoogleDatastoreAdminV1MigrationStateEventState::MigrationStateUnspecified
                }
                "PAUSED" => GoogleDatastoreAdminV1MigrationStateEventState::Paused,
                "RUNNING" => GoogleDatastoreAdminV1MigrationStateEventState::Running,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleDatastoreAdminV1MigrationStateEventState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDatastoreAdminV1MigrationStateEventState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleDatastoreAdminV1MigrationStateEventState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMPLETE" => GoogleDatastoreAdminV1MigrationStateEventState::Complete,
                "MIGRATION_STATE_UNSPECIFIED" => {
                    GoogleDatastoreAdminV1MigrationStateEventState::MigrationStateUnspecified
                }
                "PAUSED" => GoogleDatastoreAdminV1MigrationStateEventState::Paused,
                "RUNNING" => GoogleDatastoreAdminV1MigrationStateEventState::Running,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1MigrationStateEventState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1MigrationStateEventState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleDatastoreAdminV1PrepareStepDetails {
        #[doc = "The concurrency mode this database will use when it reaches the `REDIRECT_WRITES` step."]
        #[serde(
            rename = "concurrencyMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub concurrency_mode: ::std::option::Option<
            crate::schemas::GoogleDatastoreAdminV1PrepareStepDetailsConcurrencyMode,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1PrepareStepDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1PrepareStepDetails {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleDatastoreAdminV1PrepareStepDetailsConcurrencyMode {
        #[doc = "Unspecified."]
        ConcurrencyModeUnspecified,
        #[doc = "Optimistic concurrency."]
        Optimistic,
        #[doc = "Optimistic concurrency with entity groups."]
        OptimisticWithEntityGroups,
        #[doc = "Pessimistic concurrency."]
        Pessimistic,
    }
    impl GoogleDatastoreAdminV1PrepareStepDetailsConcurrencyMode {
        pub fn as_str(self) -> &'static str {
            match self { GoogleDatastoreAdminV1PrepareStepDetailsConcurrencyMode :: ConcurrencyModeUnspecified => "CONCURRENCY_MODE_UNSPECIFIED" , GoogleDatastoreAdminV1PrepareStepDetailsConcurrencyMode :: Optimistic => "OPTIMISTIC" , GoogleDatastoreAdminV1PrepareStepDetailsConcurrencyMode :: OptimisticWithEntityGroups => "OPTIMISTIC_WITH_ENTITY_GROUPS" , GoogleDatastoreAdminV1PrepareStepDetailsConcurrencyMode :: Pessimistic => "PESSIMISTIC" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleDatastoreAdminV1PrepareStepDetailsConcurrencyMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleDatastoreAdminV1PrepareStepDetailsConcurrencyMode {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleDatastoreAdminV1PrepareStepDetailsConcurrencyMode, ()>
        {
            Ok (match s { "CONCURRENCY_MODE_UNSPECIFIED" => GoogleDatastoreAdminV1PrepareStepDetailsConcurrencyMode :: ConcurrencyModeUnspecified , "OPTIMISTIC" => GoogleDatastoreAdminV1PrepareStepDetailsConcurrencyMode :: Optimistic , "OPTIMISTIC_WITH_ENTITY_GROUPS" => GoogleDatastoreAdminV1PrepareStepDetailsConcurrencyMode :: OptimisticWithEntityGroups , "PESSIMISTIC" => GoogleDatastoreAdminV1PrepareStepDetailsConcurrencyMode :: Pessimistic , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleDatastoreAdminV1PrepareStepDetailsConcurrencyMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDatastoreAdminV1PrepareStepDetailsConcurrencyMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleDatastoreAdminV1PrepareStepDetailsConcurrencyMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CONCURRENCY_MODE_UNSPECIFIED" => GoogleDatastoreAdminV1PrepareStepDetailsConcurrencyMode :: ConcurrencyModeUnspecified , "OPTIMISTIC" => GoogleDatastoreAdminV1PrepareStepDetailsConcurrencyMode :: Optimistic , "OPTIMISTIC_WITH_ENTITY_GROUPS" => GoogleDatastoreAdminV1PrepareStepDetailsConcurrencyMode :: OptimisticWithEntityGroups , "PESSIMISTIC" => GoogleDatastoreAdminV1PrepareStepDetailsConcurrencyMode :: Pessimistic , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleDatastoreAdminV1PrepareStepDetailsConcurrencyMode
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleDatastoreAdminV1PrepareStepDetailsConcurrencyMode
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleDatastoreAdminV1Progress {
        #[doc = "The amount of work that has been completed. Note that this may be greater than work_estimated."]
        #[serde(
            rename = "workCompleted",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub work_completed: ::std::option::Option<i64>,
        #[doc = "An estimate of how much work needs to be performed. May be zero if the work estimate is unavailable."]
        #[serde(
            rename = "workEstimated",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub work_estimated: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1Progress {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1Progress {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleDatastoreAdminV1RedirectWritesStepDetails {
        #[doc = "Ths concurrency mode for this database."]
        #[serde(
            rename = "concurrencyMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub concurrency_mode: ::std::option::Option<
            crate::schemas::GoogleDatastoreAdminV1RedirectWritesStepDetailsConcurrencyMode,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1RedirectWritesStepDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1RedirectWritesStepDetails {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleDatastoreAdminV1RedirectWritesStepDetailsConcurrencyMode {
        #[doc = "Unspecified."]
        ConcurrencyModeUnspecified,
        #[doc = "Optimistic concurrency."]
        Optimistic,
        #[doc = "Optimistic concurrency with entity groups."]
        OptimisticWithEntityGroups,
        #[doc = "Pessimistic concurrency."]
        Pessimistic,
    }
    impl GoogleDatastoreAdminV1RedirectWritesStepDetailsConcurrencyMode {
        pub fn as_str(self) -> &'static str {
            match self { GoogleDatastoreAdminV1RedirectWritesStepDetailsConcurrencyMode :: ConcurrencyModeUnspecified => "CONCURRENCY_MODE_UNSPECIFIED" , GoogleDatastoreAdminV1RedirectWritesStepDetailsConcurrencyMode :: Optimistic => "OPTIMISTIC" , GoogleDatastoreAdminV1RedirectWritesStepDetailsConcurrencyMode :: OptimisticWithEntityGroups => "OPTIMISTIC_WITH_ENTITY_GROUPS" , GoogleDatastoreAdminV1RedirectWritesStepDetailsConcurrencyMode :: Pessimistic => "PESSIMISTIC" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleDatastoreAdminV1RedirectWritesStepDetailsConcurrencyMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleDatastoreAdminV1RedirectWritesStepDetailsConcurrencyMode {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleDatastoreAdminV1RedirectWritesStepDetailsConcurrencyMode, ()>
        {
            Ok (match s { "CONCURRENCY_MODE_UNSPECIFIED" => GoogleDatastoreAdminV1RedirectWritesStepDetailsConcurrencyMode :: ConcurrencyModeUnspecified , "OPTIMISTIC" => GoogleDatastoreAdminV1RedirectWritesStepDetailsConcurrencyMode :: Optimistic , "OPTIMISTIC_WITH_ENTITY_GROUPS" => GoogleDatastoreAdminV1RedirectWritesStepDetailsConcurrencyMode :: OptimisticWithEntityGroups , "PESSIMISTIC" => GoogleDatastoreAdminV1RedirectWritesStepDetailsConcurrencyMode :: Pessimistic , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleDatastoreAdminV1RedirectWritesStepDetailsConcurrencyMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDatastoreAdminV1RedirectWritesStepDetailsConcurrencyMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleDatastoreAdminV1RedirectWritesStepDetailsConcurrencyMode
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CONCURRENCY_MODE_UNSPECIFIED" => GoogleDatastoreAdminV1RedirectWritesStepDetailsConcurrencyMode :: ConcurrencyModeUnspecified , "OPTIMISTIC" => GoogleDatastoreAdminV1RedirectWritesStepDetailsConcurrencyMode :: Optimistic , "OPTIMISTIC_WITH_ENTITY_GROUPS" => GoogleDatastoreAdminV1RedirectWritesStepDetailsConcurrencyMode :: OptimisticWithEntityGroups , "PESSIMISTIC" => GoogleDatastoreAdminV1RedirectWritesStepDetailsConcurrencyMode :: Pessimistic , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleDatastoreAdminV1RedirectWritesStepDetailsConcurrencyMode
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleDatastoreAdminV1RedirectWritesStepDetailsConcurrencyMode
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleLongrunningOperation {
        #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
        #[serde(
            rename = "done",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub done: ::std::option::Option<bool>,
        #[doc = "The error result of the operation in case of failure or cancellation."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::Status>,
        #[doc = "Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`."]
        #[serde(
            rename = "response",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleLongrunningOperation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleLongrunningOperation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Status {
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<i32>,
        #[doc = "A list of messages that carry the error details. There is a common set of message types for APIs to use."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Status {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Status {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
}
pub mod params {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Alt {
        #[doc = "Responses with Content-Type of application/json"]
        Json,
        #[doc = "Media download with context-dependent Content-Type"]
        Media,
        #[doc = "Responses with Content-Type of application/x-protobuf"]
        Proto,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
                Alt::Media => "media",
                Alt::Proto => "proto",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Alt {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Alt {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Alt, ()> {
            Ok(match s {
                "json" => Alt::Json,
                "media" => Alt::Media,
                "proto" => Alt::Proto,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Alt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Alt {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Alt {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "json" => Alt::Json,
                "media" => Alt::Media,
                "proto" => Alt::Proto,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Alt {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Alt {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Xgafv {
        #[doc = "v1 error format"]
        _1,
        #[doc = "v2 error format"]
        _2,
    }
    impl Xgafv {
        pub fn as_str(self) -> &'static str {
            match self {
                Xgafv::_1 => "1",
                Xgafv::_2 => "2",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Xgafv {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Xgafv {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Xgafv, ()> {
            Ok(match s {
                "1" => Xgafv::_1,
                "2" => Xgafv::_2,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Xgafv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Xgafv {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Xgafv {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "1" => Xgafv::_1,
                "2" => Xgafv::_2,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Xgafv {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Xgafv {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
}
pub struct Client {
    reqwest: ::reqwest::Client,
    auth: Box<dyn ::google_api_auth::GetAccessToken>,
}
impl Client {
    pub fn new<A>(auth: A) -> Self
    where
        A: ::google_api_auth::GetAccessToken + 'static,
    {
        Client::with_reqwest_client(auth, ::reqwest::Client::builder().build().unwrap())
    }
    pub fn with_reqwest_client<A>(auth: A, reqwest: ::reqwest::Client) -> Self
    where
        A: ::google_api_auth::GetAccessToken + 'static,
    {
        Client {
            reqwest,
            auth: Box::new(auth),
        }
    }
    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
        self.auth.as_ref()
    }
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::resources::projects::ProjectsActions {
        crate::resources::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod projects {
        pub mod params {}
        pub struct ProjectsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ProjectsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Exports a copy of all or a subset of entities from Google Cloud Datastore to another storage system, such as Google Cloud Storage. Recent updates to entities may not be reflected in the export. The export occurs in the background and its progress can be monitored and managed via the Operation resource that is created. The output of an export may only be used once the associated operation is done. If an export operation is cancelled before completion it may leave partial data behind in Google Cloud Storage."]
            pub fn export(
                &self,
                request: crate::schemas::GoogleDatastoreAdminV1Beta1ExportEntitiesRequest,
                project_id: impl Into<String>,
            ) -> ExportRequestBuilder {
                ExportRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    project_id: project_id.into(),
                }
            }
            #[doc = "Imports entities into Google Cloud Datastore. Existing entities with the same key are overwritten. The import occurs in the background and its progress can be monitored and managed via the Operation resource that is created. If an ImportEntities operation is cancelled, it is possible that a subset of the data has already been imported to Cloud Datastore."]
            pub fn import(
                &self,
                request: crate::schemas::GoogleDatastoreAdminV1Beta1ImportEntitiesRequest,
                project_id: impl Into<String>,
            ) -> ImportRequestBuilder {
                ImportRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    project_id: project_id.into(),
                }
            }
        }
        #[doc = "Created via [ProjectsActions::export()](struct.ProjectsActions.html#method.export)"]
        #[derive(Debug, Clone)]
        pub struct ExportRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleDatastoreAdminV1Beta1ExportEntitiesRequest,
            project_id: String,
            access_token: ::std::option::Option<String>,
            alt: ::std::option::Option<crate::params::Alt>,
            callback: ::std::option::Option<String>,
            fields: ::std::option::Option<String>,
            key: ::std::option::Option<String>,
            oauth_token: ::std::option::Option<String>,
            pretty_print: ::std::option::Option<bool>,
            quota_user: ::std::option::Option<String>,
            upload_protocol: ::std::option::Option<String>,
            upload_type: ::std::option::Option<String>,
            xgafv: ::std::option::Option<crate::params::Xgafv>,
        }
        impl<'a> ExportRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. “raw”, “multipart”)."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. “media”, “multipart”)."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                let req = req.json(&self.request);
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://datastore.googleapis.com/".to_owned();
                output.push_str("v1beta1/projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":export");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [ProjectsActions::import()](struct.ProjectsActions.html#method.import)"]
        #[derive(Debug, Clone)]
        pub struct ImportRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleDatastoreAdminV1Beta1ImportEntitiesRequest,
            project_id: String,
            access_token: ::std::option::Option<String>,
            alt: ::std::option::Option<crate::params::Alt>,
            callback: ::std::option::Option<String>,
            fields: ::std::option::Option<String>,
            key: ::std::option::Option<String>,
            oauth_token: ::std::option::Option<String>,
            pretty_print: ::std::option::Option<bool>,
            quota_user: ::std::option::Option<String>,
            upload_protocol: ::std::option::Option<String>,
            upload_type: ::std::option::Option<String>,
            xgafv: ::std::option::Option<crate::params::Xgafv>,
        }
        impl<'a> ImportRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. “raw”, “multipart”)."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. “media”, “multipart”)."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                let req = req.json(&self.request);
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://datastore.googleapis.com/".to_owned();
                output.push_str("v1beta1/projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":import");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
    }
}
#[derive(Debug)]
pub enum Error {
    OAuth2(Box<dyn ::std::error::Error + Send + Sync>),
    JSON(::serde_json::Error),
    Reqwest {
        reqwest_err: ::reqwest::Error,
        body: Option<String>,
    },
    IO(std::io::Error),
    Other(Box<dyn ::std::error::Error + Send + Sync>),
}

impl Error {
    pub fn json_error(&self) -> Option<&::serde_json::Error> {
        match self {
            Error::OAuth2(_) => None,
            Error::JSON(err) => Some(err),
            Error::Reqwest { .. } => None,
            Error::IO(_) => None,
            Error::Other(_) => None,
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Error::OAuth2(err) => write!(f, "OAuth2 Error: {}", err),
            Error::JSON(err) => write!(f, "JSON Error: {}", err),
            Error::Reqwest { reqwest_err, body } => {
                write!(f, "Reqwest Error: {}", reqwest_err)?;
                if let Some(body) = body {
                    write!(f, ": {}", body)?;
                }
                Ok(())
            }
            Error::IO(err) => write!(f, "IO Error: {}", err),
            Error::Other(err) => write!(f, "Uknown Error: {}", err),
        }
    }
}

impl ::std::error::Error for Error {}

impl From<::serde_json::Error> for Error {
    fn from(err: ::serde_json::Error) -> Error {
        Error::JSON(err)
    }
}

impl From<::reqwest::Error> for Error {
    fn from(reqwest_err: ::reqwest::Error) -> Error {
        Error::Reqwest {
            reqwest_err,
            body: None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err)
    }
}
#[allow(dead_code)]
const SIMPLE: &::percent_encoding::AsciiSet = &::percent_encoding::NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

#[allow(dead_code)]
const RESERVED: &::percent_encoding::AsciiSet = &SIMPLE
    .remove(b'%')
    .remove(b':')
    .remove(b'/')
    .remove(b'?')
    .remove(b'#')
    .remove(b'[')
    .remove(b']')
    .remove(b'@')
    .remove(b'!')
    .remove(b'$')
    .remove(b'&')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')')
    .remove(b'*')
    .remove(b'+')
    .remove(b',')
    .remove(b';')
    .remove(b'=');
#[allow(dead_code)]
mod multipart {
    pub(crate) struct RelatedMultiPart {
        parts: Vec<Part>,
        boundary: String,
    }

    impl RelatedMultiPart {
        pub(crate) fn new() -> Self {
            RelatedMultiPart {
                parts: Vec::new(),
                boundary: ::textnonce::TextNonce::sized(68).unwrap().0,
            }
        }

        pub(crate) fn new_part(&mut self, part: Part) {
            self.parts.push(part);
        }

        pub(crate) fn boundary(&self) -> &str {
            &self.boundary
        }

        pub(crate) fn into_reader(self) -> RelatedMultiPartReader {
            let boundary_marker = boundary_marker(&self.boundary);
            RelatedMultiPartReader {
                state: RelatedMultiPartReaderState::WriteBoundary {
                    start: 0,
                    boundary: format!("{}\r\n", &boundary_marker),
                },
                boundary: boundary_marker,
                next_body: None,
                parts: self.parts.into_iter(),
            }
        }
    }

    pub(crate) struct Part {
        content_type: ::mime::Mime,
        body: Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>,
    }

    impl Part {
        pub(crate) fn new(
            content_type: ::mime::Mime,
            body: Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>,
        ) -> Part {
            Part { content_type, body }
        }
    }

    pub(crate) struct RelatedMultiPartReader {
        state: RelatedMultiPartReaderState,
        boundary: String,
        next_body: Option<Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>>,
        parts: std::vec::IntoIter<Part>,
    }

    enum RelatedMultiPartReaderState {
        WriteBoundary {
            start: usize,
            boundary: String,
        },
        WriteContentType {
            start: usize,
            content_type: Vec<u8>,
        },
        WriteBody {
            body: Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>,
        },
    }

    impl futures::io::AsyncRead for RelatedMultiPartReader {
        fn poll_read(
            mut self: std::pin::Pin<&mut Self>,
            ctx: &mut futures::task::Context,
            buf: &mut [u8],
        ) -> futures::task::Poll<Result<usize, futures::io::Error>> {
            use RelatedMultiPartReaderState::*;

            let mut bytes_written: usize = 0;
            loop {
                let rem_buf = &mut buf[bytes_written..];
                match &mut self.state {
                    WriteBoundary { start, boundary } => {
                        let bytes_to_copy = std::cmp::min(boundary.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&boundary.as_bytes()[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == boundary.len() {
                            let next_part = match self.parts.next() {
                                None => break,
                                Some(part) => part,
                            };
                            self.next_body = Some(next_part.body);
                            self.state = WriteContentType {
                                start: 0,
                                content_type: format!(
                                    "Content-Type: {}\r\n\r\n",
                                    next_part.content_type
                                )
                                .into_bytes(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteContentType {
                        start,
                        content_type,
                    } => {
                        let bytes_to_copy =
                            std::cmp::min(content_type.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&content_type[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == content_type.len() {
                            self.state = WriteBody {
                                body: self.next_body.take().unwrap(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteBody { body } => {
                        let body = std::pin::Pin::new(body);
                        let written = match futures::io::AsyncRead::poll_read(body, ctx, rem_buf) {
                            futures::task::Poll::Ready(Ok(n)) => n,
                            futures::task::Poll::Ready(Err(err)) => {
                                return futures::task::Poll::Ready(Err(err));
                            }
                            futures::task::Poll::Pending => return futures::task::Poll::Pending,
                        };
                        bytes_written += written;
                        if written == 0 {
                            self.state = WriteBoundary {
                                start: 0,
                                boundary: format!("\r\n{}\r\n", &self.boundary),
                            };
                        } else {
                            break;
                        }
                    }
                }
            }

            futures::task::Poll::Ready(Ok(bytes_written))
        }
    }

    fn boundary_marker(boundary: &str) -> String {
        let mut marker = String::with_capacity(boundary.len() + 2);
        marker.push_str("--");
        marker.push_str(boundary);
        marker
    }
}
// A serde helper module that can be used with the `with` attribute
// to deserialize any string to a FromStr type and serialize any
// Display type to a String. Google API's encode i64, u64 values as
// strings.
#[allow(dead_code)]
mod parsed_string {
    pub fn serialize<T, S>(
        value: &Option<T>,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error>
    where
        T: ::std::fmt::Display,
        S: ::serde::Serializer,
    {
        use ::serde::Serialize;
        value.as_ref().map(|x| x.to_string()).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> ::std::result::Result<Option<T>, D::Error>
    where
        T: ::std::str::FromStr,
        T::Err: ::std::fmt::Display,
        D: ::serde::de::Deserializer<'de>,
    {
        use ::serde::Deserialize;
        match Option::<String>::deserialize(deserializer)? {
            Some(x) => Ok(Some(x.parse().map_err(::serde::de::Error::custom)?)),
            None => Ok(None),
        }
    }
}
/// Represent the ability to extract the `nextPageToken` from a response.
pub trait GetNextPageToken {
    /// Get the `nextPageToken` from a response if present.
    fn next_page_token(&self) -> ::std::option::Option<String>;
}

impl GetNextPageToken for ::serde_json::Map<String, ::serde_json::Value> {
    fn next_page_token(&self) -> ::std::option::Option<String> {
        self.get("nextPageToken")
            .and_then(|t| t.as_str())
            .map(|s| s.to_owned())
    }
}
