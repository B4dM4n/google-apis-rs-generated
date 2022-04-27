#![doc = "# Resources and Methods\n    * [operations](resources/operations/struct.OperationsActions.html)\n      * [*delete*](resources/operations/struct.DeleteRequestBuilder.html)\n    * [projects](resources/projects/struct.ProjectsActions.html)\n      * [locations](resources/projects/locations/struct.LocationsActions.html)\n        * [*fetchProcessorTypes*](resources/projects/locations/struct.FetchProcessorTypesRequestBuilder.html), [*get*](resources/projects/locations/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/struct.ListRequestBuilder.html)\n        * [operations](resources/projects/locations/operations/struct.OperationsActions.html)\n          * [*cancel*](resources/projects/locations/operations/struct.CancelRequestBuilder.html), [*get*](resources/projects/locations/operations/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/operations/struct.ListRequestBuilder.html)\n        * [processors](resources/projects/locations/processors/struct.ProcessorsActions.html)\n          * [*batchProcess*](resources/projects/locations/processors/struct.BatchProcessRequestBuilder.html), [*create*](resources/projects/locations/processors/struct.CreateRequestBuilder.html), [*delete*](resources/projects/locations/processors/struct.DeleteRequestBuilder.html), [*disable*](resources/projects/locations/processors/struct.DisableRequestBuilder.html), [*enable*](resources/projects/locations/processors/struct.EnableRequestBuilder.html), [*get*](resources/projects/locations/processors/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/processors/struct.ListRequestBuilder.html), [*process*](resources/projects/locations/processors/struct.ProcessRequestBuilder.html), [*setDefaultProcessorVersion*](resources/projects/locations/processors/struct.SetDefaultProcessorVersionRequestBuilder.html)\n          * [human_review_config](resources/projects/locations/processors/human_review_config/struct.HumanReviewConfigActions.html)\n            * [*reviewDocument*](resources/projects/locations/processors/human_review_config/struct.ReviewDocumentRequestBuilder.html)\n          * [processor_versions](resources/projects/locations/processors/processor_versions/struct.ProcessorVersionsActions.html)\n            * [*batchProcess*](resources/projects/locations/processors/processor_versions/struct.BatchProcessRequestBuilder.html), [*delete*](resources/projects/locations/processors/processor_versions/struct.DeleteRequestBuilder.html), [*deploy*](resources/projects/locations/processors/processor_versions/struct.DeployRequestBuilder.html), [*get*](resources/projects/locations/processors/processor_versions/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/processors/processor_versions/struct.ListRequestBuilder.html), [*process*](resources/projects/locations/processors/processor_versions/struct.ProcessRequestBuilder.html), [*undeploy*](resources/projects/locations/processors/processor_versions/struct.UndeployRequestBuilder.html)\n      * [operations](resources/projects/operations/struct.OperationsActions.html)\n        * [*get*](resources/projects/operations/struct.GetRequestBuilder.html)\n    * [uiv_1beta_3](resources/uiv_1beta_3/struct.Uiv1Beta3Actions.html)\n      * [projects](resources/uiv_1beta_3/projects/struct.ProjectsActions.html)\n        * [locations](resources/uiv_1beta_3/projects/locations/struct.LocationsActions.html)\n          * [*get*](resources/uiv_1beta_3/projects/locations/struct.GetRequestBuilder.html), [*list*](resources/uiv_1beta_3/projects/locations/struct.ListRequestBuilder.html)\n          * [operations](resources/uiv_1beta_3/projects/locations/operations/struct.OperationsActions.html)\n            * [*cancel*](resources/uiv_1beta_3/projects/locations/operations/struct.CancelRequestBuilder.html), [*get*](resources/uiv_1beta_3/projects/locations/operations/struct.GetRequestBuilder.html), [*list*](resources/uiv_1beta_3/projects/locations/operations/struct.ListRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
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
    pub struct GoogleCloudDocumentaiUiv1Beta3BatchDeleteDocumentsMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3BatchDeleteDocumentsMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3BatchDeleteDocumentsMetadata
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiUiv1Beta3BatchDeleteDocumentsResponse {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3BatchDeleteDocumentsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3BatchDeleteDocumentsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadata { # [doc = "The basic metadata of the long running operation."] # [serde (rename = "commonMetadata" , default , skip_serializing_if = "std::option::Option::is_none")] pub common_metadata : :: std :: option :: Option < crate :: schemas :: GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadata > , # [doc = "The destination dataset split type."] # [serde (rename = "destDatasetType" , default , skip_serializing_if = "std::option::Option::is_none")] pub dest_dataset_type : :: std :: option :: Option < crate :: schemas :: GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestDatasetType > , # [doc = "The destination dataset split type."] # [serde (rename = "destSplitType" , default , skip_serializing_if = "std::option::Option::is_none")] pub dest_split_type : :: std :: option :: Option < crate :: schemas :: GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestSplitType > , # [doc = "The list of response details of each document."] # [serde (rename = "individualBatchMoveStatuses" , default , skip_serializing_if = "std::option::Option::is_none")] pub individual_batch_move_statuses : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataIndividualBatchMoveStatus > > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadata
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestDatasetType {
        #[doc = "Identifies the test documents."]
        DatasetSplitTest,
        #[doc = "Identifies the train documents."]
        DatasetSplitTrain,
        #[doc = "Default value if the enum is not set. go/protodosdonts#do-include-an-unspecified-value-in-an-enum"]
        DatasetSplitTypeUnspecified,
        #[doc = "Identifies the unassigned documents."]
        DatasetSplitUnassigned,
    }
    impl GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestDatasetType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestDatasetType :: DatasetSplitTest => "DATASET_SPLIT_TEST" , GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestDatasetType :: DatasetSplitTrain => "DATASET_SPLIT_TRAIN" , GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestDatasetType :: DatasetSplitTypeUnspecified => "DATASET_SPLIT_TYPE_UNSPECIFIED" , GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestDatasetType :: DatasetSplitUnassigned => "DATASET_SPLIT_UNASSIGNED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestDatasetType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestDatasetType
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestDatasetType,
            (),
        > {
            Ok (match s { "DATASET_SPLIT_TEST" => GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestDatasetType :: DatasetSplitTest , "DATASET_SPLIT_TRAIN" => GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestDatasetType :: DatasetSplitTrain , "DATASET_SPLIT_TYPE_UNSPECIFIED" => GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestDatasetType :: DatasetSplitTypeUnspecified , "DATASET_SPLIT_UNASSIGNED" => GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestDatasetType :: DatasetSplitUnassigned , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestDatasetType
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestDatasetType
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestDatasetType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "DATASET_SPLIT_TEST" => GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestDatasetType :: DatasetSplitTest , "DATASET_SPLIT_TRAIN" => GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestDatasetType :: DatasetSplitTrain , "DATASET_SPLIT_TYPE_UNSPECIFIED" => GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestDatasetType :: DatasetSplitTypeUnspecified , "DATASET_SPLIT_UNASSIGNED" => GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestDatasetType :: DatasetSplitUnassigned , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestDatasetType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestDatasetType
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestSplitType {
        #[doc = "Identifies the test documents."]
        DatasetSplitTest,
        #[doc = "Identifies the train documents."]
        DatasetSplitTrain,
        #[doc = "Default value if the enum is not set. go/protodosdonts#do-include-an-unspecified-value-in-an-enum"]
        DatasetSplitTypeUnspecified,
        #[doc = "Identifies the unassigned documents."]
        DatasetSplitUnassigned,
    }
    impl GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestSplitType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestSplitType :: DatasetSplitTest => "DATASET_SPLIT_TEST" , GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestSplitType :: DatasetSplitTrain => "DATASET_SPLIT_TRAIN" , GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestSplitType :: DatasetSplitTypeUnspecified => "DATASET_SPLIT_TYPE_UNSPECIFIED" , GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestSplitType :: DatasetSplitUnassigned => "DATASET_SPLIT_UNASSIGNED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestSplitType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestSplitType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestSplitType,
            (),
        > {
            Ok (match s { "DATASET_SPLIT_TEST" => GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestSplitType :: DatasetSplitTest , "DATASET_SPLIT_TRAIN" => GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestSplitType :: DatasetSplitTrain , "DATASET_SPLIT_TYPE_UNSPECIFIED" => GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestSplitType :: DatasetSplitTypeUnspecified , "DATASET_SPLIT_UNASSIGNED" => GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestSplitType :: DatasetSplitUnassigned , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestSplitType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestSplitType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestSplitType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "DATASET_SPLIT_TEST" => GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestSplitType :: DatasetSplitTest , "DATASET_SPLIT_TRAIN" => GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestSplitType :: DatasetSplitTrain , "DATASET_SPLIT_TYPE_UNSPECIFIED" => GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestSplitType :: DatasetSplitTypeUnspecified , "DATASET_SPLIT_UNASSIGNED" => GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestSplitType :: DatasetSplitUnassigned , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestSplitType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataDestSplitType
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataIndividualBatchMoveStatus {
        #[doc = "The document id of the document."]
        #[serde(
            rename = "documentId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub document_id:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiUiv1Beta3DocumentId>,
        #[doc = "The status of moving the document."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataIndividualBatchMoveStatus
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsMetadataIndividualBatchMoveStatus
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsResponse {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3BatchMoveDocumentsResponse
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
    pub struct GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadata {
        #[doc = "The creation time of the operation."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "A related resource to this operation."]
        #[serde(
            rename = "resource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource: ::std::option::Option<String>,
        #[doc = "The state of the operation."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState,
        >,
        #[doc = "A message providing more details about the current state of processing."]
        #[serde(
            rename = "stateMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state_message: ::std::option::Option<String>,
        #[doc = "The last update time of the operation."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadata
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState {
        #[doc = "Operation is cancelled."]
        Cancelled,
        #[doc = "Operation is being cancelled."]
        Cancelling,
        #[doc = "Operation failed."]
        Failed,
        #[doc = "Operation is still running."]
        Running,
        #[doc = "Unspecified state."]
        StateUnspecified,
        #[doc = "Operation succeeded."]
        Succeeded,
    }
    impl GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState::Cancelled => {
                    "CANCELLED"
                }
                GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState::Cancelling => {
                    "CANCELLING"
                }
                GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState::Failed => "FAILED",
                GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState::Running => "RUNNING",
                GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
                GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState::Succeeded => {
                    "SUCCEEDED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState, ()>
        {
            Ok(match s {
                "CANCELLED" => {
                    GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState::Cancelled
                }
                "CANCELLING" => {
                    GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState::Cancelling
                }
                "FAILED" => GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState::Failed,
                "RUNNING" => GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState::StateUnspecified
                }
                "SUCCEEDED" => {
                    GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState::Succeeded
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => {
                    GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState::Cancelled
                }
                "CANCELLING" => {
                    GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState::Cancelling
                }
                "FAILED" => GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState::Failed,
                "RUNNING" => GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState::StateUnspecified
                }
                "SUCCEEDED" => {
                    GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState::Succeeded
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
        for GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadataState
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
    pub struct GoogleCloudDocumentaiUiv1Beta3CreateLabelerPoolOperationMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3CreateLabelerPoolOperationMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3CreateLabelerPoolOperationMetadata
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
    pub struct GoogleCloudDocumentaiUiv1Beta3DeleteDataLabelingJobOperationMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3DeleteDataLabelingJobOperationMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3DeleteDataLabelingJobOperationMetadata
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
    pub struct GoogleCloudDocumentaiUiv1Beta3DeleteLabelerPoolOperationMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3DeleteLabelerPoolOperationMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3DeleteLabelerPoolOperationMetadata
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
    pub struct GoogleCloudDocumentaiUiv1Beta3DeleteProcessorMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3DeleteProcessorMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3DeleteProcessorMetadata
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
    pub struct GoogleCloudDocumentaiUiv1Beta3DeleteProcessorVersionMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3DeleteProcessorVersionMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3DeleteProcessorVersionMetadata
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
    pub struct GoogleCloudDocumentaiUiv1Beta3DeployProcessorVersionMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3DeployProcessorVersionMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3DeployProcessorVersionMetadata
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiUiv1Beta3DeployProcessorVersionResponse {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3DeployProcessorVersionResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3DeployProcessorVersionResponse
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
    pub struct GoogleCloudDocumentaiUiv1Beta3DisableProcessorMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3DisableProcessorMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3DisableProcessorMetadata
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiUiv1Beta3DisableProcessorResponse {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3DisableProcessorResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3DisableProcessorResponse
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
    pub struct GoogleCloudDocumentaiUiv1Beta3DocumentId {
        #[serde(
            rename = "gcsManagedDocId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_managed_doc_id: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiUiv1Beta3DocumentIdGCSManagedDocumentId,
        >,
        #[doc = "Points to a specific revision of the document if set."]
        #[serde(
            rename = "revisionReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_reference:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiUiv1Beta3RevisionReference>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiUiv1Beta3DocumentId {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiUiv1Beta3DocumentId {
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
    pub struct GoogleCloudDocumentaiUiv1Beta3DocumentIdGCSManagedDocumentId {
        #[doc = "Optional. Id of the document (indexed) managed by Content Warehouse."]
        #[serde(
            rename = "cwDocId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cw_doc_id: ::std::option::Option<String>,
        #[doc = "Required. The Cloud Storage uri where the actual document is stored."]
        #[serde(
            rename = "gcsUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3DocumentIdGCSManagedDocumentId
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3DocumentIdGCSManagedDocumentId
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
    pub struct GoogleCloudDocumentaiUiv1Beta3EnableProcessorMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3EnableProcessorMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3EnableProcessorMetadata
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiUiv1Beta3EnableProcessorResponse {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3EnableProcessorResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3EnableProcessorResponse
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
    pub struct GoogleCloudDocumentaiUiv1Beta3EvaluateProcessorVersionMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3EvaluateProcessorVersionMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3EvaluateProcessorVersionMetadata
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
    pub struct GoogleCloudDocumentaiUiv1Beta3EvaluateProcessorVersionResponse {
        #[doc = "The resource name of the created evaluation."]
        #[serde(
            rename = "evaluation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub evaluation: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3EvaluateProcessorVersionResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3EvaluateProcessorVersionResponse
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
    pub struct GoogleCloudDocumentaiUiv1Beta3ExportProcessorVersionMetadata {
        #[doc = "The common metadata about the operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3ExportProcessorVersionMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3ExportProcessorVersionMetadata
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
    pub struct GoogleCloudDocumentaiUiv1Beta3ExportProcessorVersionResponse {
        #[doc = "The Cloud Storage URI containing the output artifacts."]
        #[serde(
            rename = "gcsUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3ExportProcessorVersionResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3ExportProcessorVersionResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDocumentaiUiv1Beta3ImportDocumentsMetadata { # [doc = "The basic metadata of the long running operation."] # [serde (rename = "commonMetadata" , default , skip_serializing_if = "std::option::Option::is_none")] pub common_metadata : :: std :: option :: Option < crate :: schemas :: GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadata > , # [doc = "The list of response details of each document."] # [serde (rename = "individualImportStatuses" , default , skip_serializing_if = "std::option::Option::is_none")] pub individual_import_statuses : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudDocumentaiUiv1Beta3ImportDocumentsMetadataIndividualImportStatus > > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3ImportDocumentsMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3ImportDocumentsMetadata
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDocumentaiUiv1Beta3ImportDocumentsMetadataIndividualImportStatus {
        #[doc = "The source Cloud Storage URI of the document."]
        #[serde(
            rename = "inputGcsSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_gcs_source: ::std::option::Option<String>,
        #[doc = "The output_gcs_destination of the processed document if it was successful, otherwise empty."]
        #[serde(
            rename = "outputGcsDestination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_gcs_destination: ::std::option::Option<String>,
        #[doc = "The status of the importing of the document."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3ImportDocumentsMetadataIndividualImportStatus
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3ImportDocumentsMetadataIndividualImportStatus
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiUiv1Beta3ImportDocumentsResponse {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3ImportDocumentsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3ImportDocumentsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDocumentaiUiv1Beta3ResyncDatasetMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadata,
        >,
        #[doc = "Returns the newly added document Cloud Storage prefix if the documents are founded in Cloud Storage while not in Document Service storage."]
        #[serde(
            rename = "newlyAddedDocuments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub newly_added_documents: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiUiv1Beta3ResyncDatasetMetadataUpdatedDocument>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3ResyncDatasetMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiUiv1Beta3ResyncDatasetMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDocumentaiUiv1Beta3ResyncDatasetMetadataUpdatedDocument {
        #[doc = "The prefix of cloud storage, identifies the destination document which should be updated by resync pipeline."]
        #[serde(
            rename = "destinationPrefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_prefix: ::std::option::Option<String>,
        #[doc = "The prefix of cloud storage, identifies the original document which should be updated by resync pipeline."]
        #[serde(
            rename = "sourcePrefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_prefix: ::std::option::Option<String>,
        #[doc = "The final status of the documents which should be updated by resync pipeline."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3ResyncDatasetMetadataUpdatedDocument
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3ResyncDatasetMetadataUpdatedDocument
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiUiv1Beta3ResyncDatasetResponse {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3ResyncDatasetResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiUiv1Beta3ResyncDatasetResponse {
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
    pub struct GoogleCloudDocumentaiUiv1Beta3RevisionReference {
        #[doc = "Read the revision generated by the processor version, returns error if it does not exist."]
        #[serde(
            rename = "latestProcessorVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub latest_processor_version: ::std::option::Option<String>,
        #[doc = "Read the revision by the predefined case."]
        #[serde(
            rename = "revisionCase",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_case: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiUiv1Beta3RevisionReferenceRevisionCase,
        >,
        #[doc = "Read the revision given by the id, returns error if it does not exist."]
        #[serde(
            rename = "revisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiUiv1Beta3RevisionReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiUiv1Beta3RevisionReference {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiUiv1Beta3RevisionReferenceRevisionCase {
        #[doc = "The latest revision made by a human."]
        LatestHumanReview,
        #[doc = "The latest revision based on timestamp."]
        LatestTimestamp,
        #[doc = "Unspecified case, fallback to read the first (OCR) revision."]
        RevisionCaseUnspecified,
    }
    impl GoogleCloudDocumentaiUiv1Beta3RevisionReferenceRevisionCase {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDocumentaiUiv1Beta3RevisionReferenceRevisionCase :: LatestHumanReview => "LATEST_HUMAN_REVIEW" , GoogleCloudDocumentaiUiv1Beta3RevisionReferenceRevisionCase :: LatestTimestamp => "LATEST_TIMESTAMP" , GoogleCloudDocumentaiUiv1Beta3RevisionReferenceRevisionCase :: RevisionCaseUnspecified => "REVISION_CASE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiUiv1Beta3RevisionReferenceRevisionCase {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiUiv1Beta3RevisionReferenceRevisionCase {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiUiv1Beta3RevisionReferenceRevisionCase, ()>
        {
            Ok (match s { "LATEST_HUMAN_REVIEW" => GoogleCloudDocumentaiUiv1Beta3RevisionReferenceRevisionCase :: LatestHumanReview , "LATEST_TIMESTAMP" => GoogleCloudDocumentaiUiv1Beta3RevisionReferenceRevisionCase :: LatestTimestamp , "REVISION_CASE_UNSPECIFIED" => GoogleCloudDocumentaiUiv1Beta3RevisionReferenceRevisionCase :: RevisionCaseUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiUiv1Beta3RevisionReferenceRevisionCase {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiUiv1Beta3RevisionReferenceRevisionCase {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudDocumentaiUiv1Beta3RevisionReferenceRevisionCase
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "LATEST_HUMAN_REVIEW" => GoogleCloudDocumentaiUiv1Beta3RevisionReferenceRevisionCase :: LatestHumanReview , "LATEST_TIMESTAMP" => GoogleCloudDocumentaiUiv1Beta3RevisionReferenceRevisionCase :: LatestTimestamp , "REVISION_CASE_UNSPECIFIED" => GoogleCloudDocumentaiUiv1Beta3RevisionReferenceRevisionCase :: RevisionCaseUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3RevisionReferenceRevisionCase
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3RevisionReferenceRevisionCase
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
    pub struct GoogleCloudDocumentaiUiv1Beta3SetDefaultProcessorVersionMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3SetDefaultProcessorVersionMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3SetDefaultProcessorVersionMetadata
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiUiv1Beta3SetDefaultProcessorVersionResponse {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3SetDefaultProcessorVersionResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3SetDefaultProcessorVersionResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDocumentaiUiv1Beta3TrainProcessorVersionMetadata { # [doc = "The basic metadata of the long running operation."] # [serde (rename = "commonMetadata" , default , skip_serializing_if = "std::option::Option::is_none")] pub common_metadata : :: std :: option :: Option < crate :: schemas :: GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadata > , # [doc = "The test dataset validation information."] # [serde (rename = "testDatasetValidation" , default , skip_serializing_if = "std::option::Option::is_none")] pub test_dataset_validation : :: std :: option :: Option < crate :: schemas :: GoogleCloudDocumentaiUiv1Beta3TrainProcessorVersionMetadataDatasetValidation > , # [doc = "The training dataset validation information."] # [serde (rename = "trainingDatasetValidation" , default , skip_serializing_if = "std::option::Option::is_none")] pub training_dataset_validation : :: std :: option :: Option < crate :: schemas :: GoogleCloudDocumentaiUiv1Beta3TrainProcessorVersionMetadataDatasetValidation > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3TrainProcessorVersionMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3TrainProcessorVersionMetadata
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDocumentaiUiv1Beta3TrainProcessorVersionMetadataDatasetValidation {
        #[doc = "The total number of dataset errors."]
        #[serde(
            rename = "datasetErrorCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dataset_error_count: ::std::option::Option<i32>,
        #[doc = "Error information for the dataset as a whole. A maximum of 10 dataset errors will be returned. A single dataset error is terminal for training."]
        #[serde(
            rename = "datasetErrors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dataset_errors: ::std::option::Option<Vec<crate::schemas::GoogleRpcStatus>>,
        #[doc = "The total number of document errors."]
        #[serde(
            rename = "documentErrorCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub document_error_count: ::std::option::Option<i32>,
        #[doc = "Error information pertaining to specific documents. A maximum of 10 document errors will be returned. Any document with errors will not be used throughout training."]
        #[serde(
            rename = "documentErrors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub document_errors: ::std::option::Option<Vec<crate::schemas::GoogleRpcStatus>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3TrainProcessorVersionMetadataDatasetValidation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3TrainProcessorVersionMetadataDatasetValidation
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
    pub struct GoogleCloudDocumentaiUiv1Beta3TrainProcessorVersionResponse {
        #[doc = "The resource name of the processor version produced by training."]
        #[serde(
            rename = "processorVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub processor_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3TrainProcessorVersionResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3TrainProcessorVersionResponse
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
    pub struct GoogleCloudDocumentaiUiv1Beta3UndeployProcessorVersionMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3UndeployProcessorVersionMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3UndeployProcessorVersionMetadata
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiUiv1Beta3UndeployProcessorVersionResponse {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3UndeployProcessorVersionResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3UndeployProcessorVersionResponse
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
    pub struct GoogleCloudDocumentaiUiv1Beta3UpdateDatasetOperationMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3UpdateDatasetOperationMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3UpdateDatasetOperationMetadata
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
    pub struct GoogleCloudDocumentaiUiv1Beta3UpdateHumanReviewConfigMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3UpdateHumanReviewConfigMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3UpdateHumanReviewConfigMetadata
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
    pub struct GoogleCloudDocumentaiUiv1Beta3UpdateLabelerPoolOperationMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiUiv1Beta3CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiUiv1Beta3UpdateLabelerPoolOperationMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiUiv1Beta3UpdateLabelerPoolOperationMetadata
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
    pub struct GoogleCloudDocumentaiV1Alpha1AnalyzeHitlDataMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Alpha1CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Alpha1AnalyzeHitlDataMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Alpha1AnalyzeHitlDataMetadata {
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
    pub struct GoogleCloudDocumentaiV1Alpha1CommonOperationMetadata {
        #[doc = "The creation time of the operation."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "A related resource to this operation."]
        #[serde(
            rename = "resource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource: ::std::option::Option<String>,
        #[doc = "The state of the operation."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState,
        >,
        #[doc = "A message providing more details about the current state of processing."]
        #[serde(
            rename = "stateMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state_message: ::std::option::Option<String>,
        #[doc = "The last update time of the operation."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Alpha1CommonOperationMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Alpha1CommonOperationMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState {
        #[doc = "Operation is cancelled."]
        Cancelled,
        #[doc = "Operation is being cancelled."]
        Cancelling,
        #[doc = "Operation failed."]
        Failed,
        #[doc = "Operation is still running."]
        Running,
        #[doc = "Unspecified state."]
        StateUnspecified,
        #[doc = "Operation succeeded."]
        Succeeded,
    }
    impl GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState::Cancelled => "CANCELLED",
                GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState::Cancelling => {
                    "CANCELLING"
                }
                GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState::Failed => "FAILED",
                GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState::Running => "RUNNING",
                GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
                GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState::Succeeded => "SUCCEEDED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState, ()>
        {
            Ok(match s {
                "CANCELLED" => GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState::Cancelled,
                "CANCELLING" => {
                    GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState::Cancelling
                }
                "FAILED" => GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState::Failed,
                "RUNNING" => GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState::StateUnspecified
                }
                "SUCCEEDED" => GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState::Succeeded,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState::Cancelled,
                "CANCELLING" => {
                    GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState::Cancelling
                }
                "FAILED" => GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState::Failed,
                "RUNNING" => GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState::StateUnspecified
                }
                "SUCCEEDED" => GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState::Succeeded,
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
        for GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Alpha1CommonOperationMetadataState
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
    pub struct GoogleCloudDocumentaiV1BatchDocumentsInputConfig {
        #[doc = "The set of documents individually specified on Cloud Storage."]
        #[serde(
            rename = "gcsDocuments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_documents:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1GcsDocuments>,
        #[doc = "The set of documents that match the specified Cloud Storage [gcs_prefix]."]
        #[serde(
            rename = "gcsPrefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_prefix: ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1GcsPrefix>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1BatchDocumentsInputConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1BatchDocumentsInputConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDocumentaiV1BatchProcessMetadata {
        #[doc = "The creation time of the operation."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The list of response details of each document."]
        #[serde(
            rename = "individualProcessStatuses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub individual_process_statuses: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1BatchProcessMetadataIndividualProcessStatus>,
        >,
        #[doc = "The state of the current batch processing."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1BatchProcessMetadataState>,
        #[doc = "A message providing more details about the current state of processing. For example, the error message if the operation is failed."]
        #[serde(
            rename = "stateMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state_message: ::std::option::Option<String>,
        #[doc = "The last update time of the operation."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1BatchProcessMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1BatchProcessMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1BatchProcessMetadataState {
        #[doc = "The batch processing was cancelled."]
        Cancelled,
        #[doc = "The batch processing was being cancelled."]
        Cancelling,
        #[doc = "The batch processing has failed."]
        Failed,
        #[doc = "Request is being processed."]
        Running,
        #[doc = "The default value. This value is used if the state is omitted."]
        StateUnspecified,
        #[doc = "The batch processing completed successfully."]
        Succeeded,
        #[doc = "Request operation is waiting for scheduling."]
        Waiting,
    }
    impl GoogleCloudDocumentaiV1BatchProcessMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDocumentaiV1BatchProcessMetadataState::Cancelled => "CANCELLED",
                GoogleCloudDocumentaiV1BatchProcessMetadataState::Cancelling => "CANCELLING",
                GoogleCloudDocumentaiV1BatchProcessMetadataState::Failed => "FAILED",
                GoogleCloudDocumentaiV1BatchProcessMetadataState::Running => "RUNNING",
                GoogleCloudDocumentaiV1BatchProcessMetadataState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
                GoogleCloudDocumentaiV1BatchProcessMetadataState::Succeeded => "SUCCEEDED",
                GoogleCloudDocumentaiV1BatchProcessMetadataState::Waiting => "WAITING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiV1BatchProcessMetadataState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1BatchProcessMetadataState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiV1BatchProcessMetadataState, ()> {
            Ok(match s {
                "CANCELLED" => GoogleCloudDocumentaiV1BatchProcessMetadataState::Cancelled,
                "CANCELLING" => GoogleCloudDocumentaiV1BatchProcessMetadataState::Cancelling,
                "FAILED" => GoogleCloudDocumentaiV1BatchProcessMetadataState::Failed,
                "RUNNING" => GoogleCloudDocumentaiV1BatchProcessMetadataState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1BatchProcessMetadataState::StateUnspecified
                }
                "SUCCEEDED" => GoogleCloudDocumentaiV1BatchProcessMetadataState::Succeeded,
                "WAITING" => GoogleCloudDocumentaiV1BatchProcessMetadataState::Waiting,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1BatchProcessMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1BatchProcessMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDocumentaiV1BatchProcessMetadataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => GoogleCloudDocumentaiV1BatchProcessMetadataState::Cancelled,
                "CANCELLING" => GoogleCloudDocumentaiV1BatchProcessMetadataState::Cancelling,
                "FAILED" => GoogleCloudDocumentaiV1BatchProcessMetadataState::Failed,
                "RUNNING" => GoogleCloudDocumentaiV1BatchProcessMetadataState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1BatchProcessMetadataState::StateUnspecified
                }
                "SUCCEEDED" => GoogleCloudDocumentaiV1BatchProcessMetadataState::Succeeded,
                "WAITING" => GoogleCloudDocumentaiV1BatchProcessMetadataState::Waiting,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1BatchProcessMetadataState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1BatchProcessMetadataState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDocumentaiV1BatchProcessMetadataIndividualProcessStatus {
        #[doc = "The status of human review on the processed document."]
        #[serde(
            rename = "humanReviewStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub human_review_status:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1HumanReviewStatus>,
        #[doc = "The source of the document, same as the [input_gcs_source] field in the request when the batch process started. The batch process is started by take snapshot of that document, since a user can move or change that document during the process."]
        #[serde(
            rename = "inputGcsSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_gcs_source: ::std::option::Option<String>,
        #[doc = "The output_gcs_destination (in the request as `output_gcs_destination`) of the processed document if it was successful, otherwise empty."]
        #[serde(
            rename = "outputGcsDestination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_gcs_destination: ::std::option::Option<String>,
        #[doc = "The status of the processing of the document."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1BatchProcessMetadataIndividualProcessStatus
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1BatchProcessMetadataIndividualProcessStatus
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
    pub struct GoogleCloudDocumentaiV1BatchProcessRequest {
        #[doc = "The overall output config for batch process."]
        #[serde(
            rename = "documentOutputConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub document_output_config:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentOutputConfig>,
        #[doc = "The input documents for batch process."]
        #[serde(
            rename = "inputDocuments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_documents:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1BatchDocumentsInputConfig>,
        #[doc = "Whether Human Review feature should be skipped for this request. Default to false."]
        #[serde(
            rename = "skipHumanReview",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub skip_human_review: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1BatchProcessRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1BatchProcessRequest {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1BatchProcessResponse {}
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1BatchProcessResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1BatchProcessResponse {
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
    pub struct GoogleCloudDocumentaiV1Beta1BatchProcessDocumentsResponse {
        #[doc = "Responses for each individual document."]
        #[serde(
            rename = "responses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub responses: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1ProcessDocumentResponse>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta1BatchProcessDocumentsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta1BatchProcessDocumentsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta1BoundingPoly {
        #[doc = "The bounding polygon normalized vertices."]
        #[serde(
            rename = "normalizedVertices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_vertices: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1NormalizedVertex>,
        >,
        #[doc = "The bounding polygon vertices."]
        #[serde(
            rename = "vertices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vertices:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1Vertex>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1BoundingPoly {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1BoundingPoly {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDocumentaiV1Beta1Document {
        #[doc = "Optional. Inline document content, represented as a stream of bytes. Note: As with all `bytes` fields, protobuffers use a pure binary representation, whereas JSON representations use base64."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "A list of entities detected on Document.text. For document shards, entities in this list may cross shard boundaries."]
        #[serde(
            rename = "entities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entities:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentEntity>>,
        #[doc = "Placeholder. Relationship among Document.entities."]
        #[serde(
            rename = "entityRelations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_relations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentEntityRelation>,
        >,
        #[doc = "Any error that occurred while processing this document."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
        #[doc = "An IANA published MIME type (also referred to as media type). For more information, see https://www.iana.org/assignments/media-types/media-types.xhtml."]
        #[serde(
            rename = "mimeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime_type: ::std::option::Option<String>,
        #[doc = "Visual page layout for the Document."]
        #[serde(
            rename = "pages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pages:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPage>>,
        #[doc = "Placeholder. Revision history of this document."]
        #[serde(
            rename = "revisions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revisions: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentRevision>,
        >,
        #[doc = "Information about the sharding if this document is sharded part of a larger document. If the document is not sharded, this message is not specified."]
        #[serde(
            rename = "shardInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shard_info:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentShardInfo>,
        #[doc = "Optional. UTF-8 encoded text in reading order from the document."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
        #[doc = "Placeholder. A list of text corrections made to [Document.text]. This is usually used for annotating corrections to OCR mistakes. Text changes for a given revision may not overlap with each other."]
        #[serde(
            rename = "textChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_changes: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentTextChange>,
        >,
        #[doc = "Placeholder. Styles for the Document.text."]
        #[serde(
            rename = "textStyles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_styles:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentStyle>>,
        #[doc = "Optional. Currently supports Google Cloud Storage URI of the form `gs://bucket_name/object_name`. Object versioning is not supported. See [Google Cloud Storage Request URIs](https://cloud.google.com/storage/docs/reference-uris) for more info."]
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1Document {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1Document {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta1DocumentEntity {
        #[doc = "Optional. Confidence of detected Schema entity. Range [0, 1]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Optional. Canonical id. This will be a unique value in the entity list for this document."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Optional. Deprecated. Use `id` field instead."]
        #[serde(
            rename = "mentionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mention_id: ::std::option::Option<String>,
        #[doc = "Optional. Text value in the document e.g. `1600 Amphitheatre Pkwy`. If the entity is not present in the document, this field will be empty."]
        #[serde(
            rename = "mentionText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mention_text: ::std::option::Option<String>,
        #[doc = "Optional. This attribute indicates that the processing didn't actually identify this entity, but a confidence score was assigned that represent the potential that this could be a false negative. A non-present entity should have an empty mention_text and text_anchor."]
        #[serde(
            rename = "nonPresent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_present: ::std::option::Option<bool>,
        #[doc = "Optional. Normalized entity value. Absent if the extracted value could not be converted or the type (e.g. address) is not supported for certain parsers. This field is also only populated for certain supported document types."]
        #[serde(
            rename = "normalizedValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_value: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentEntityNormalizedValue,
        >,
        #[doc = "Optional. Represents the provenance of this entity wrt. the location on the page where it was found."]
        #[serde(
            rename = "pageAnchor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_anchor:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageAnchor>,
        #[doc = "Optional. Entities can be nested to form a hierarchical data structure representing the content in the document."]
        #[serde(
            rename = "properties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub properties:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentEntity>>,
        #[doc = "Optional. The history of this annotation."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentProvenance>,
        #[doc = "Required. Entity type from a schema e.g. `Address`."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "Optional. Whether the entity will be redacted for de-identification purposes."]
        #[serde(
            rename = "redacted",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub redacted: ::std::option::Option<bool>,
        #[doc = "Optional. Provenance of the entity. Text anchor indexing into the Document.text."]
        #[serde(
            rename = "textAnchor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_anchor:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentTextAnchor>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1DocumentEntity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentEntity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta1DocumentEntityNormalizedValue {
        #[doc = "Postal address. See also: https://github.com/googleapis/googleapis/blob/master/google/type/postal_address.proto"]
        #[serde(
            rename = "addressValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub address_value: ::std::option::Option<crate::schemas::GoogleTypePostalAddress>,
        #[doc = "Boolean value. Can be used for entities with binary values, or for checkboxes."]
        #[serde(
            rename = "booleanValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub boolean_value: ::std::option::Option<bool>,
        #[doc = "Date value. Includes year, month, day. See also: https://github.com/googleapis/googleapis/blob/master/google/type/date.proto"]
        #[serde(
            rename = "dateValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_value: ::std::option::Option<crate::schemas::GoogleTypeDate>,
        #[doc = "DateTime value. Includes date, time, and timezone. See also: https://github.com/googleapis/googleapis/blob/master/google/type/datetime.proto"]
        #[serde(
            rename = "datetimeValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub datetime_value: ::std::option::Option<crate::schemas::GoogleTypeDateTime>,
        #[doc = "Float value."]
        #[serde(
            rename = "floatValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub float_value: ::std::option::Option<f32>,
        #[doc = "Integer value."]
        #[serde(
            rename = "integerValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub integer_value: ::std::option::Option<i32>,
        #[doc = "Money value. See also: https://github.com/googleapis/googleapis/blob/master/google/type/money.proto"]
        #[serde(
            rename = "moneyValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub money_value: ::std::option::Option<crate::schemas::GoogleTypeMoney>,
        #[doc = "Optional. An optional field to store a normalized string. For some entity types, one of respective `structured_value` fields may also be populated. Also not all the types of `structured_value` will be normalized. For example, some processors may not generate float or int normalized text by default. Below are sample formats mapped to structured values. - Money/Currency type (`money_value`) is in the ISO 4217 text format. - Date type (`date_value`) is in the ISO 8601 text format. - Datetime type (`datetime_value`) is in the ISO 8601 text format."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta1DocumentEntityNormalizedValue
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta1DocumentEntityNormalizedValue
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
    pub struct GoogleCloudDocumentaiV1Beta1DocumentEntityRelation {
        #[doc = "Object entity id."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "Relationship description."]
        #[serde(
            rename = "relation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relation: ::std::option::Option<String>,
        #[doc = "Subject entity id."]
        #[serde(
            rename = "subjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subject_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1DocumentEntityRelation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentEntityRelation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta1DocumentPage {
        #[doc = "A list of visually detected text blocks on the page. A block has a set of lines (collected into paragraphs) that have a common line-spacing and orientation."]
        #[serde(
            rename = "blocks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub blocks: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageBlock>,
        >,
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageDetectedLanguage>,
        >,
        #[doc = "Physical dimension of the page."]
        #[serde(
            rename = "dimension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageDimension,
        >,
        #[doc = "A list of visually detected form fields on the page."]
        #[serde(
            rename = "formFields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub form_fields: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageFormField>,
        >,
        #[doc = "Rendered image for this page. This image is preprocessed to remove any skew, rotation, and distortions such that the annotation bounding boxes can be upright and axis-aligned."]
        #[serde(
            rename = "image",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageImage>,
        #[doc = "Layout for the page."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageLayout>,
        #[doc = "A list of visually detected text lines on the page. A collection of tokens that a human would perceive as a line."]
        #[serde(
            rename = "lines",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lines: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageLine>,
        >,
        #[doc = "1-based index for current Page in a parent Document. Useful when a page is taken out of a Document for individual processing."]
        #[serde(
            rename = "pageNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_number: ::std::option::Option<i32>,
        #[doc = "A list of visually detected text paragraphs on the page. A collection of lines that a human would perceive as a paragraph."]
        #[serde(
            rename = "paragraphs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub paragraphs: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageParagraph>,
        >,
        #[doc = "The history of this page."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentProvenance>,
        #[doc = "A list of visually detected symbols on the page."]
        #[serde(
            rename = "symbols",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub symbols: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageSymbol>,
        >,
        #[doc = "A list of visually detected tables on the page."]
        #[serde(
            rename = "tables",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tables: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageTable>,
        >,
        #[doc = "A list of visually detected tokens on the page."]
        #[serde(
            rename = "tokens",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tokens: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageToken>,
        >,
        #[doc = "Transformation matrices that were applied to the original document image to produce Page.image."]
        #[serde(
            rename = "transforms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transforms: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageMatrix>,
        >,
        #[doc = "A list of detected non-text visual elements e.g. checkbox, signature etc. on the page."]
        #[serde(
            rename = "visualElements",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub visual_elements: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageVisualElement>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1DocumentPage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentPage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta1DocumentPageAnchor {
        #[doc = "One or more references to visual page elements"]
        #[serde(
            rename = "pageRefs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_refs: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRef>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1DocumentPageAnchor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentPageAnchor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRef {
        #[doc = "Optional. Identifies the bounding polygon of a layout element on the page."]
        #[serde(
            rename = "boundingPoly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1BoundingPoly>,
        #[doc = "Optional. Confidence of detected page element, if applicable. Range [0, 1]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Optional. Deprecated. Use PageRef.bounding_poly instead."]
        #[serde(
            rename = "layoutId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout_id: ::std::option::Option<String>,
        #[doc = "Optional. The type of the layout element that is being referenced if any."]
        #[serde(
            rename = "layoutType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout_type: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType,
        >,
        #[doc = "Required. Index into the Document.pages element, for example using Document.pages to locate the related page element. This field is skipped when its value is the default 0. See https://developers.google.com/protocol-buffers/docs/proto3#json."]
        #[serde(
            rename = "page",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub page: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRef
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRef
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType {
        #[doc = "References a Page.blocks element."]
        Block,
        #[doc = "References a Page.form_fields element."]
        FormField,
        #[doc = "Layout Unspecified."]
        LayoutTypeUnspecified,
        #[doc = "References a Page.lines element."]
        Line,
        #[doc = "References a Page.paragraphs element."]
        Paragraph,
        #[doc = "Refrrences a Page.tables element."]
        Table,
        #[doc = "References a Page.tokens element."]
        Token,
        #[doc = "References a Page.visual_elements element."]
        VisualElement,
    }
    impl GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: Block => "BLOCK" , GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: FormField => "FORM_FIELD" , GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: LayoutTypeUnspecified => "LAYOUT_TYPE_UNSPECIFIED" , GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: Line => "LINE" , GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: Paragraph => "PARAGRAPH" , GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: Table => "TABLE" , GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: Token => "TOKEN" , GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: VisualElement => "VISUAL_ELEMENT" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType,
            (),
        > {
            Ok (match s { "BLOCK" => GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: Block , "FORM_FIELD" => GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: FormField , "LAYOUT_TYPE_UNSPECIFIED" => GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: LayoutTypeUnspecified , "LINE" => GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: Line , "PARAGRAPH" => GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: Paragraph , "TABLE" => GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: Table , "TOKEN" => GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: Token , "VISUAL_ELEMENT" => GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: VisualElement , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "BLOCK" => GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: Block , "FORM_FIELD" => GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: FormField , "LAYOUT_TYPE_UNSPECIFIED" => GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: LayoutTypeUnspecified , "LINE" => GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: Line , "PARAGRAPH" => GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: Paragraph , "TABLE" => GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: Table , "TOKEN" => GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: Token , "VISUAL_ELEMENT" => GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType :: VisualElement , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta1DocumentPageAnchorPageRefLayoutType
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta1DocumentPageBlock {
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageDetectedLanguage>,
        >,
        #[doc = "Layout for Block."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageLayout>,
        #[doc = "The history of this annotation."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentProvenance>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1DocumentPageBlock {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentPageBlock {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta1DocumentPageDetectedLanguage {
        #[doc = "Confidence of detected language. Range [0, 1]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see https://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta1DocumentPageDetectedLanguage
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta1DocumentPageDetectedLanguage
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta1DocumentPageDimension {
        #[doc = "Page height."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<f32>,
        #[doc = "Dimension unit."]
        #[serde(
            rename = "unit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit: ::std::option::Option<String>,
        #[doc = "Page width."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1DocumentPageDimension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentPageDimension {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta1DocumentPageFormField {
        #[doc = "Created for Labeling UI to export key text. If corrections were made to the text identified by the `field_name.text_anchor`, this field will contain the correction."]
        #[serde(
            rename = "correctedKeyText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub corrected_key_text: ::std::option::Option<String>,
        #[doc = "Created for Labeling UI to export value text. If corrections were made to the text identified by the `field_value.text_anchor`, this field will contain the correction."]
        #[serde(
            rename = "correctedValueText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub corrected_value_text: ::std::option::Option<String>,
        #[doc = "Layout for the FormField name. e.g. `Address`, `Email`, `Grand total`, `Phone number`, etc."]
        #[serde(
            rename = "fieldName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field_name:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageLayout>,
        #[doc = "Layout for the FormField value."]
        #[serde(
            rename = "fieldValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field_value:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageLayout>,
        #[doc = "A list of detected languages for name together with confidence."]
        #[serde(
            rename = "nameDetectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name_detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageDetectedLanguage>,
        >,
        #[doc = "The history of this annotation."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentProvenance>,
        #[doc = "A list of detected languages for value together with confidence."]
        #[serde(
            rename = "valueDetectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageDetectedLanguage>,
        >,
        #[doc = "If the value is non-textual, this field represents the type. Current valid values are: - blank (this indicates the field_value is normal text) - \"unfilled_checkbox\" - \"filled_checkbox\""]
        #[serde(
            rename = "valueType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1DocumentPageFormField {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentPageFormField {
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
    pub struct GoogleCloudDocumentaiV1Beta1DocumentPageImage {
        #[doc = "Raw byte content of the image."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Height of the image in pixels."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[doc = "Encoding mime type for the image."]
        #[serde(
            rename = "mimeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime_type: ::std::option::Option<String>,
        #[doc = "Width of the image in pixels."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1DocumentPageImage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentPageImage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta1DocumentPageLayout {
        #[doc = "The bounding polygon for the Layout."]
        #[serde(
            rename = "boundingPoly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1BoundingPoly>,
        #[doc = "Confidence of the current Layout within context of the object this layout is for. e.g. confidence can be for a single token, a table, a visual element, etc. depending on context. Range [0, 1]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Detected orientation for the Layout."]
        #[serde(
            rename = "orientation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub orientation: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation,
        >,
        #[doc = "Text anchor indexing into the Document.text."]
        #[serde(
            rename = "textAnchor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_anchor:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentTextAnchor>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1DocumentPageLayout {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentPageLayout {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation {
        #[doc = "Unspecified orientation."]
        OrientationUnspecified,
        #[doc = "Orientation is aligned with page down. Turn the head 180 degrees from upright to read."]
        PageDown,
        #[doc = "Orientation is aligned with page left. Turn the head 90 degrees counterclockwise from upright to read."]
        PageLeft,
        #[doc = "Orientation is aligned with page right. Turn the head 90 degrees clockwise from upright to read."]
        PageRight,
        #[doc = "Orientation is aligned with page up."]
        PageUp,
    }
    impl GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation :: OrientationUnspecified => "ORIENTATION_UNSPECIFIED" , GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation :: PageDown => "PAGE_DOWN" , GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation :: PageLeft => "PAGE_LEFT" , GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation :: PageRight => "PAGE_RIGHT" , GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation :: PageUp => "PAGE_UP" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation, ()>
        {
            Ok (match s { "ORIENTATION_UNSPECIFIED" => GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation :: OrientationUnspecified , "PAGE_DOWN" => GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation :: PageDown , "PAGE_LEFT" => GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation :: PageLeft , "PAGE_RIGHT" => GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation :: PageRight , "PAGE_UP" => GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation :: PageUp , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ORIENTATION_UNSPECIFIED" => GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation :: OrientationUnspecified , "PAGE_DOWN" => GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation :: PageDown , "PAGE_LEFT" => GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation :: PageLeft , "PAGE_RIGHT" => GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation :: PageRight , "PAGE_UP" => GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation :: PageUp , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta1DocumentPageLayoutOrientation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta1DocumentPageLine {
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageDetectedLanguage>,
        >,
        #[doc = "Layout for Line."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageLayout>,
        #[doc = "The history of this annotation."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentProvenance>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1DocumentPageLine {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentPageLine {
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
    pub struct GoogleCloudDocumentaiV1Beta1DocumentPageMatrix {
        #[doc = "Number of columns in the matrix."]
        #[serde(
            rename = "cols",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cols: ::std::option::Option<i32>,
        #[doc = "The matrix data."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "This encodes information about what data type the matrix uses. For example, 0 (CV_8U) is an unsigned 8-bit image. For the full list of OpenCV primitive data types, please refer to https://docs.opencv.org/4.3.0/d1/d1b/group__core__hal__interface.html"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<i32>,
        #[doc = "Number of rows in the matrix."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1DocumentPageMatrix {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentPageMatrix {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta1DocumentPageParagraph {
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageDetectedLanguage>,
        >,
        #[doc = "Layout for Paragraph."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageLayout>,
        #[doc = "The history of this annotation."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentProvenance>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1DocumentPageParagraph {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentPageParagraph {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta1DocumentPageSymbol {
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageDetectedLanguage>,
        >,
        #[doc = "Layout for Symbol."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageLayout>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1DocumentPageSymbol {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentPageSymbol {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta1DocumentPageTable {
        #[doc = "Body rows of the table."]
        #[serde(
            rename = "bodyRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub body_rows: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageTableTableRow>,
        >,
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageDetectedLanguage>,
        >,
        #[doc = "Header rows of the table."]
        #[serde(
            rename = "headerRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header_rows: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageTableTableRow>,
        >,
        #[doc = "Layout for Table."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageLayout>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1DocumentPageTable {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentPageTable {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta1DocumentPageTableTableCell {
        #[doc = "How many columns this cell spans."]
        #[serde(
            rename = "colSpan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub col_span: ::std::option::Option<i32>,
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageDetectedLanguage>,
        >,
        #[doc = "Layout for TableCell."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageLayout>,
        #[doc = "How many rows this cell spans."]
        #[serde(
            rename = "rowSpan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_span: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta1DocumentPageTableTableCell
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta1DocumentPageTableTableCell
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta1DocumentPageTableTableRow {
        #[doc = "Cells that make up this row."]
        #[serde(
            rename = "cells",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cells: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageTableTableCell>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta1DocumentPageTableTableRow
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta1DocumentPageTableTableRow
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta1DocumentPageToken {
        #[doc = "Detected break at the end of a Token."]
        #[serde(
            rename = "detectedBreak",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_break: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreak,
        >,
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageDetectedLanguage>,
        >,
        #[doc = "Layout for Token."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageLayout>,
        #[doc = "The history of this annotation."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentProvenance>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1DocumentPageToken {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentPageToken {
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
    pub struct GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreak {
        #[doc = "Detected break type."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreakType,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreak
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreak
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreakType {
        #[doc = "A hyphen that indicates that a token has been split across lines."]
        Hyphen,
        #[doc = "A single whitespace."]
        Space,
        #[doc = "Unspecified break type."]
        TypeUnspecified,
        #[doc = "A wider whitespace."]
        WideSpace,
    }
    impl GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreakType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreakType::Hyphen => "HYPHEN",
                GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreakType::Space => "SPACE",
                GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreakType::TypeUnspecified => {
                    "TYPE_UNSPECIFIED"
                }
                GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreakType::WideSpace => {
                    "WIDE_SPACE"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreakType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreakType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreakType, ()>
        {
            Ok(match s {
                "HYPHEN" => GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreakType::Hyphen,
                "SPACE" => GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreakType::Space,
                "TYPE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreakType::TypeUnspecified
                }
                "WIDE_SPACE" => {
                    GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreakType::WideSpace
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreakType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreakType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreakType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "HYPHEN" => GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreakType::Hyphen,
                "SPACE" => GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreakType::Space,
                "TYPE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreakType::TypeUnspecified
                }
                "WIDE_SPACE" => {
                    GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreakType::WideSpace
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
        for GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreakType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta1DocumentPageTokenDetectedBreakType
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta1DocumentPageVisualElement {
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageDetectedLanguage>,
        >,
        #[doc = "Layout for VisualElement."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentPageLayout>,
        #[doc = "Type of the VisualElement."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta1DocumentPageVisualElement
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta1DocumentPageVisualElement
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
    pub struct GoogleCloudDocumentaiV1Beta1DocumentProvenance {
        #[doc = "The Id of this operation. Needs to be unique within the scope of the revision."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<i32>,
        #[doc = "References to the original elements that are replaced."]
        #[serde(
            rename = "parents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parents: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentProvenanceParent>,
        >,
        #[doc = "The type of provenance operation."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentProvenanceType,
        >,
        #[doc = "The index of the revision that produced this element."]
        #[serde(
            rename = "revision",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1DocumentProvenance {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentProvenance {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1Beta1DocumentProvenanceType {
        #[doc = "Add an element."]
        Add,
        #[doc = "Element is reviewed and approved at human review, confidence will be set to 1.0."]
        EvalApproved,
        #[doc = "Request human review for the element identified by `parent`."]
        EvalRequested,
        #[doc = "Element is skipped in the validation process."]
        EvalSkipped,
        #[doc = "Operation type unspecified. If no operation is specified a provenance entry is simply used to match against a `parent`."]
        OperationTypeUnspecified,
        #[doc = "Remove an element identified by `parent`."]
        Remove,
        #[doc = "Replace an element identified by `parent`."]
        Replace,
    }
    impl GoogleCloudDocumentaiV1Beta1DocumentProvenanceType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDocumentaiV1Beta1DocumentProvenanceType::Add => "ADD",
                GoogleCloudDocumentaiV1Beta1DocumentProvenanceType::EvalApproved => "EVAL_APPROVED",
                GoogleCloudDocumentaiV1Beta1DocumentProvenanceType::EvalRequested => {
                    "EVAL_REQUESTED"
                }
                GoogleCloudDocumentaiV1Beta1DocumentProvenanceType::EvalSkipped => "EVAL_SKIPPED",
                GoogleCloudDocumentaiV1Beta1DocumentProvenanceType::OperationTypeUnspecified => {
                    "OPERATION_TYPE_UNSPECIFIED"
                }
                GoogleCloudDocumentaiV1Beta1DocumentProvenanceType::Remove => "REMOVE",
                GoogleCloudDocumentaiV1Beta1DocumentProvenanceType::Replace => "REPLACE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiV1Beta1DocumentProvenanceType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1Beta1DocumentProvenanceType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiV1Beta1DocumentProvenanceType, ()> {
            Ok(match s {
                "ADD" => GoogleCloudDocumentaiV1Beta1DocumentProvenanceType::Add,
                "EVAL_APPROVED" => GoogleCloudDocumentaiV1Beta1DocumentProvenanceType::EvalApproved,
                "EVAL_REQUESTED" => {
                    GoogleCloudDocumentaiV1Beta1DocumentProvenanceType::EvalRequested
                }
                "EVAL_SKIPPED" => GoogleCloudDocumentaiV1Beta1DocumentProvenanceType::EvalSkipped,
                "OPERATION_TYPE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1Beta1DocumentProvenanceType::OperationTypeUnspecified
                }
                "REMOVE" => GoogleCloudDocumentaiV1Beta1DocumentProvenanceType::Remove,
                "REPLACE" => GoogleCloudDocumentaiV1Beta1DocumentProvenanceType::Replace,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1Beta1DocumentProvenanceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1Beta1DocumentProvenanceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDocumentaiV1Beta1DocumentProvenanceType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADD" => GoogleCloudDocumentaiV1Beta1DocumentProvenanceType::Add,
                "EVAL_APPROVED" => GoogleCloudDocumentaiV1Beta1DocumentProvenanceType::EvalApproved,
                "EVAL_REQUESTED" => {
                    GoogleCloudDocumentaiV1Beta1DocumentProvenanceType::EvalRequested
                }
                "EVAL_SKIPPED" => GoogleCloudDocumentaiV1Beta1DocumentProvenanceType::EvalSkipped,
                "OPERATION_TYPE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1Beta1DocumentProvenanceType::OperationTypeUnspecified
                }
                "REMOVE" => GoogleCloudDocumentaiV1Beta1DocumentProvenanceType::Remove,
                "REPLACE" => GoogleCloudDocumentaiV1Beta1DocumentProvenanceType::Replace,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1DocumentProvenanceType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentProvenanceType {
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
    pub struct GoogleCloudDocumentaiV1Beta1DocumentProvenanceParent {
        #[doc = "The id of the parent provenance."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<i32>,
        #[doc = "The index of the parent item in the corresponding item list (eg. list of entities, properties within entities, etc.) in the parent revision."]
        #[serde(
            rename = "index",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub index: ::std::option::Option<i32>,
        #[doc = "The index of the index into current revision's parent_ids list."]
        #[serde(
            rename = "revision",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta1DocumentProvenanceParent
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentProvenanceParent {
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
    pub struct GoogleCloudDocumentaiV1Beta1DocumentRevision {
        #[doc = "If the change was made by a person specify the name or id of that person."]
        #[serde(
            rename = "agent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub agent: ::std::option::Option<String>,
        #[doc = "The time that the revision was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Human Review information of this revision."]
        #[serde(
            rename = "humanReview",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub human_review: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentRevisionHumanReview,
        >,
        #[doc = "Id of the revision. Unique within the context of the document."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The revisions that this revision is based on. This can include one or more parent (when documents are merged.) This field represents the index into the `revisions` field."]
        #[serde(
            rename = "parent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent: ::std::option::Option<Vec<i32>>,
        #[doc = "The revisions that this revision is based on. Must include all the ids that have anything to do with this revision - eg. there are `provenance.parent.revision` fields that index into this field."]
        #[serde(
            rename = "parentIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent_ids: ::std::option::Option<Vec<String>>,
        #[doc = "If the annotation was made by processor identify the processor by its resource name."]
        #[serde(
            rename = "processor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub processor: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1DocumentRevision {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentRevision {
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
    pub struct GoogleCloudDocumentaiV1Beta1DocumentRevisionHumanReview {
        #[doc = "Human review state. e.g. `requested`, `succeeded`, `rejected`."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<String>,
        #[doc = "A message providing more details about the current state of processing. For example, the rejection reason when the state is `rejected`."]
        #[serde(
            rename = "stateMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state_message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta1DocumentRevisionHumanReview
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta1DocumentRevisionHumanReview
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
    pub struct GoogleCloudDocumentaiV1Beta1DocumentShardInfo {
        #[doc = "Total number of shards."]
        #[serde(
            rename = "shardCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub shard_count: ::std::option::Option<i64>,
        #[doc = "The 0-based index of this shard."]
        #[serde(
            rename = "shardIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub shard_index: ::std::option::Option<i64>,
        #[doc = "The index of the first character in Document.text in the overall document global text."]
        #[serde(
            rename = "textOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub text_offset: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1DocumentShardInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentShardInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta1DocumentStyle {
        #[doc = "Text background color."]
        #[serde(
            rename = "backgroundColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub background_color: ::std::option::Option<crate::schemas::GoogleTypeColor>,
        #[doc = "Text color."]
        #[serde(
            rename = "color",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color: ::std::option::Option<crate::schemas::GoogleTypeColor>,
        #[doc = "Font size."]
        #[serde(
            rename = "fontSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub font_size: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentStyleFontSize,
        >,
        #[doc = "Font weight. Possible values are normal, bold, bolder, and lighter. https://www.w3schools.com/cssref/pr_font_weight.asp"]
        #[serde(
            rename = "fontWeight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub font_weight: ::std::option::Option<String>,
        #[doc = "Text anchor indexing into the Document.text."]
        #[serde(
            rename = "textAnchor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_anchor:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentTextAnchor>,
        #[doc = "Text decoration. Follows CSS standard. https://www.w3schools.com/cssref/pr_text_text-decoration.asp"]
        #[serde(
            rename = "textDecoration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_decoration: ::std::option::Option<String>,
        #[doc = "Text style. Possible values are normal, italic, and oblique. https://www.w3schools.com/cssref/pr_font_font-style.asp"]
        #[serde(
            rename = "textStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_style: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1DocumentStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta1DocumentStyleFontSize {
        #[doc = "Font size for the text."]
        #[serde(
            rename = "size",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub size: ::std::option::Option<f32>,
        #[doc = "Unit for the font size. Follows CSS naming (in, px, pt, etc.)."]
        #[serde(
            rename = "unit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1DocumentStyleFontSize {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentStyleFontSize {
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
    pub struct GoogleCloudDocumentaiV1Beta1DocumentTextAnchor {
        #[doc = "Contains the content of the text span so that users do not have to look it up in the text_segments. It is always populated for formFields."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<String>,
        #[doc = "The text segments from the Document.text."]
        #[serde(
            rename = "textSegments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_segments: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentTextAnchorTextSegment>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1DocumentTextAnchor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentTextAnchor {
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
    pub struct GoogleCloudDocumentaiV1Beta1DocumentTextAnchorTextSegment {
        #[doc = "TextSegment half open end UTF-8 char index in the Document.text."]
        #[serde(
            rename = "endIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub end_index: ::std::option::Option<i64>,
        #[doc = "TextSegment start UTF-8 char index in the Document.text."]
        #[serde(
            rename = "startIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub start_index: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta1DocumentTextAnchorTextSegment
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta1DocumentTextAnchorTextSegment
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
    pub struct GoogleCloudDocumentaiV1Beta1DocumentTextChange {
        #[doc = "The text that replaces the text identified in the `text_anchor`."]
        #[serde(
            rename = "changedText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub changed_text: ::std::option::Option<String>,
        #[doc = "The history of this annotation."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentProvenance>,
        >,
        #[doc = "Provenance of the correction. Text anchor indexing into the Document.text. There can only be a single `TextAnchor.text_segments` element. If the start and end index of the text segment are the same, the text change is inserted before that index."]
        #[serde(
            rename = "textAnchor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_anchor:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1DocumentTextAnchor>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1DocumentTextChange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1DocumentTextChange {
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
    pub struct GoogleCloudDocumentaiV1Beta1GcsDestination {
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1GcsDestination {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1GcsDestination {
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
    pub struct GoogleCloudDocumentaiV1Beta1GcsSource {
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1GcsSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1GcsSource {
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
    pub struct GoogleCloudDocumentaiV1Beta1InputConfig {
        #[doc = "The Google Cloud Storage location to read the input from. This must be a single file."]
        #[serde(
            rename = "gcsSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_source:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1GcsSource>,
        #[doc = "Required. Mimetype of the input. Current supported mimetypes are application/pdf, image/tiff, and image/gif. In addition, application/json type is supported for requests with ProcessDocumentRequest.automl_params field set. The JSON file needs to be in Document format."]
        #[serde(
            rename = "mimeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1InputConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1InputConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta1NormalizedVertex {
        #[doc = "X coordinate."]
        #[serde(
            rename = "x",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub x: ::std::option::Option<f32>,
        #[doc = "Y coordinate (starts from the top of the image)."]
        #[serde(
            rename = "y",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub y: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1NormalizedVertex {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1NormalizedVertex {
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
    pub struct GoogleCloudDocumentaiV1Beta1OperationMetadata {
        #[doc = "The creation time of the operation."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The state of the current batch processing."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta1OperationMetadataState,
        >,
        #[doc = "A message providing more details about the current state of processing."]
        #[serde(
            rename = "stateMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state_message: ::std::option::Option<String>,
        #[doc = "The last update time of the operation."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1OperationMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1OperationMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1Beta1OperationMetadataState {
        #[doc = "Request is received."]
        Accepted,
        #[doc = "The batch processing was cancelled."]
        Cancelled,
        #[doc = "The batch processing has failed."]
        Failed,
        #[doc = "Request is being processed."]
        Running,
        #[doc = "The default value. This value is used if the state is omitted."]
        StateUnspecified,
        #[doc = "The batch processing completed successfully."]
        Succeeded,
        #[doc = "Request operation is waiting for scheduling."]
        Waiting,
    }
    impl GoogleCloudDocumentaiV1Beta1OperationMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDocumentaiV1Beta1OperationMetadataState::Accepted => "ACCEPTED",
                GoogleCloudDocumentaiV1Beta1OperationMetadataState::Cancelled => "CANCELLED",
                GoogleCloudDocumentaiV1Beta1OperationMetadataState::Failed => "FAILED",
                GoogleCloudDocumentaiV1Beta1OperationMetadataState::Running => "RUNNING",
                GoogleCloudDocumentaiV1Beta1OperationMetadataState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
                GoogleCloudDocumentaiV1Beta1OperationMetadataState::Succeeded => "SUCCEEDED",
                GoogleCloudDocumentaiV1Beta1OperationMetadataState::Waiting => "WAITING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiV1Beta1OperationMetadataState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1Beta1OperationMetadataState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiV1Beta1OperationMetadataState, ()> {
            Ok(match s {
                "ACCEPTED" => GoogleCloudDocumentaiV1Beta1OperationMetadataState::Accepted,
                "CANCELLED" => GoogleCloudDocumentaiV1Beta1OperationMetadataState::Cancelled,
                "FAILED" => GoogleCloudDocumentaiV1Beta1OperationMetadataState::Failed,
                "RUNNING" => GoogleCloudDocumentaiV1Beta1OperationMetadataState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1Beta1OperationMetadataState::StateUnspecified
                }
                "SUCCEEDED" => GoogleCloudDocumentaiV1Beta1OperationMetadataState::Succeeded,
                "WAITING" => GoogleCloudDocumentaiV1Beta1OperationMetadataState::Waiting,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1Beta1OperationMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1Beta1OperationMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDocumentaiV1Beta1OperationMetadataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCEPTED" => GoogleCloudDocumentaiV1Beta1OperationMetadataState::Accepted,
                "CANCELLED" => GoogleCloudDocumentaiV1Beta1OperationMetadataState::Cancelled,
                "FAILED" => GoogleCloudDocumentaiV1Beta1OperationMetadataState::Failed,
                "RUNNING" => GoogleCloudDocumentaiV1Beta1OperationMetadataState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1Beta1OperationMetadataState::StateUnspecified
                }
                "SUCCEEDED" => GoogleCloudDocumentaiV1Beta1OperationMetadataState::Succeeded,
                "WAITING" => GoogleCloudDocumentaiV1Beta1OperationMetadataState::Waiting,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1OperationMetadataState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1OperationMetadataState {
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
    pub struct GoogleCloudDocumentaiV1Beta1OutputConfig {
        #[doc = "The Google Cloud Storage location to write the output to."]
        #[serde(
            rename = "gcsDestination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_destination:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1GcsDestination>,
        #[doc = "The max number of pages to include into each output Document shard JSON on Google Cloud Storage. The valid range is [1, 100]. If not specified, the default value is 20. For example, for one pdf file with 100 pages, 100 parsed pages will be produced. If `pages_per_shard` = 20, then 5 Document shard JSON files each containing 20 parsed pages will be written under the prefix OutputConfig.gcs_destination.uri and suffix pages-x-to-y.json where x and y are 1-indexed page numbers. Example GCS outputs with 157 pages and pages_per_shard = 50: pages-001-to-050.json pages-051-to-100.json pages-101-to-150.json pages-151-to-157.json"]
        #[serde(
            rename = "pagesPerShard",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pages_per_shard: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1OutputConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1OutputConfig {
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
    pub struct GoogleCloudDocumentaiV1Beta1ProcessDocumentResponse {
        #[doc = "Information about the input file. This is the same as the corresponding input config in the request."]
        #[serde(
            rename = "inputConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_config:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1InputConfig>,
        #[doc = "The output location of the parsed responses. The responses are written to this location as JSON-serialized `Document` objects."]
        #[serde(
            rename = "outputConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_config:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta1OutputConfig>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta1ProcessDocumentResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1ProcessDocumentResponse {
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
    pub struct GoogleCloudDocumentaiV1Beta1Vertex {
        #[doc = "X coordinate."]
        #[serde(
            rename = "x",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub x: ::std::option::Option<i32>,
        #[doc = "Y coordinate (starts from the top of the image)."]
        #[serde(
            rename = "y",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub y: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta1Vertex {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta1Vertex {
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
    pub struct GoogleCloudDocumentaiV1Beta2BatchProcessDocumentsResponse {
        #[doc = "Responses for each individual document."]
        #[serde(
            rename = "responses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub responses: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2ProcessDocumentResponse>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta2BatchProcessDocumentsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta2BatchProcessDocumentsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta2BoundingPoly {
        #[doc = "The bounding polygon normalized vertices."]
        #[serde(
            rename = "normalizedVertices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_vertices: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2NormalizedVertex>,
        >,
        #[doc = "The bounding polygon vertices."]
        #[serde(
            rename = "vertices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vertices:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2Vertex>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2BoundingPoly {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2BoundingPoly {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDocumentaiV1Beta2Document {
        #[doc = "Optional. Inline document content, represented as a stream of bytes. Note: As with all `bytes` fields, protobuffers use a pure binary representation, whereas JSON representations use base64."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "A list of entities detected on Document.text. For document shards, entities in this list may cross shard boundaries."]
        #[serde(
            rename = "entities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entities:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentEntity>>,
        #[doc = "Placeholder. Relationship among Document.entities."]
        #[serde(
            rename = "entityRelations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_relations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentEntityRelation>,
        >,
        #[doc = "Any error that occurred while processing this document."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
        #[doc = "Labels for this document."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentLabel>>,
        #[doc = "An IANA published MIME type (also referred to as media type). For more information, see https://www.iana.org/assignments/media-types/media-types.xhtml."]
        #[serde(
            rename = "mimeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime_type: ::std::option::Option<String>,
        #[doc = "Visual page layout for the Document."]
        #[serde(
            rename = "pages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pages:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPage>>,
        #[doc = "Placeholder. Revision history of this document."]
        #[serde(
            rename = "revisions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revisions: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentRevision>,
        >,
        #[doc = "Information about the sharding if this document is sharded part of a larger document. If the document is not sharded, this message is not specified."]
        #[serde(
            rename = "shardInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shard_info:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentShardInfo>,
        #[doc = "Optional. UTF-8 encoded text in reading order from the document."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
        #[doc = "Placeholder. A list of text corrections made to [Document.text]. This is usually used for annotating corrections to OCR mistakes. Text changes for a given revision may not overlap with each other."]
        #[serde(
            rename = "textChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_changes: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentTextChange>,
        >,
        #[doc = "Placeholder. Styles for the Document.text."]
        #[serde(
            rename = "textStyles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_styles:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentStyle>>,
        #[doc = "Optional. Currently supports Google Cloud Storage URI of the form `gs://bucket_name/object_name`. Object versioning is not supported. See [Google Cloud Storage Request URIs](https://cloud.google.com/storage/docs/reference-uris) for more info."]
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2Document {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2Document {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta2DocumentEntity {
        #[doc = "Optional. Confidence of detected Schema entity. Range [0, 1]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Optional. Canonical id. This will be a unique value in the entity list for this document."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Optional. Deprecated. Use `id` field instead."]
        #[serde(
            rename = "mentionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mention_id: ::std::option::Option<String>,
        #[doc = "Optional. Text value in the document e.g. `1600 Amphitheatre Pkwy`. If the entity is not present in the document, this field will be empty."]
        #[serde(
            rename = "mentionText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mention_text: ::std::option::Option<String>,
        #[doc = "Optional. This attribute indicates that the processing didn't actually identify this entity, but a confidence score was assigned that represent the potential that this could be a false negative. A non-present entity should have an empty mention_text and text_anchor."]
        #[serde(
            rename = "nonPresent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_present: ::std::option::Option<bool>,
        #[doc = "Optional. Normalized entity value. Absent if the extracted value could not be converted or the type (e.g. address) is not supported for certain parsers. This field is also only populated for certain supported document types."]
        #[serde(
            rename = "normalizedValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_value: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentEntityNormalizedValue,
        >,
        #[doc = "Optional. Represents the provenance of this entity wrt. the location on the page where it was found."]
        #[serde(
            rename = "pageAnchor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_anchor:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageAnchor>,
        #[doc = "Optional. Entities can be nested to form a hierarchical data structure representing the content in the document."]
        #[serde(
            rename = "properties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub properties:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentEntity>>,
        #[doc = "Optional. The history of this annotation."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentProvenance>,
        #[doc = "Required. Entity type from a schema e.g. `Address`."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "Optional. Whether the entity will be redacted for de-identification purposes."]
        #[serde(
            rename = "redacted",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub redacted: ::std::option::Option<bool>,
        #[doc = "Optional. Provenance of the entity. Text anchor indexing into the Document.text."]
        #[serde(
            rename = "textAnchor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_anchor:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentTextAnchor>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentEntity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentEntity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta2DocumentEntityNormalizedValue {
        #[doc = "Postal address. See also: https://github.com/googleapis/googleapis/blob/master/google/type/postal_address.proto"]
        #[serde(
            rename = "addressValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub address_value: ::std::option::Option<crate::schemas::GoogleTypePostalAddress>,
        #[doc = "Boolean value. Can be used for entities with binary values, or for checkboxes."]
        #[serde(
            rename = "booleanValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub boolean_value: ::std::option::Option<bool>,
        #[doc = "Date value. Includes year, month, day. See also: https://github.com/googleapis/googleapis/blob/master/google/type/date.proto"]
        #[serde(
            rename = "dateValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_value: ::std::option::Option<crate::schemas::GoogleTypeDate>,
        #[doc = "DateTime value. Includes date, time, and timezone. See also: https://github.com/googleapis/googleapis/blob/master/google/type/datetime.proto"]
        #[serde(
            rename = "datetimeValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub datetime_value: ::std::option::Option<crate::schemas::GoogleTypeDateTime>,
        #[doc = "Float value."]
        #[serde(
            rename = "floatValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub float_value: ::std::option::Option<f32>,
        #[doc = "Integer value."]
        #[serde(
            rename = "integerValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub integer_value: ::std::option::Option<i32>,
        #[doc = "Money value. See also: https://github.com/googleapis/googleapis/blob/master/google/type/money.proto"]
        #[serde(
            rename = "moneyValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub money_value: ::std::option::Option<crate::schemas::GoogleTypeMoney>,
        #[doc = "Optional. An optional field to store a normalized string. For some entity types, one of respective `structured_value` fields may also be populated. Also not all the types of `structured_value` will be normalized. For example, some processors may not generate float or int normalized text by default. Below are sample formats mapped to structured values. - Money/Currency type (`money_value`) is in the ISO 4217 text format. - Date type (`date_value`) is in the ISO 8601 text format. - Datetime type (`datetime_value`) is in the ISO 8601 text format."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta2DocumentEntityNormalizedValue
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta2DocumentEntityNormalizedValue
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
    pub struct GoogleCloudDocumentaiV1Beta2DocumentEntityRelation {
        #[doc = "Object entity id."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "Relationship description."]
        #[serde(
            rename = "relation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relation: ::std::option::Option<String>,
        #[doc = "Subject entity id."]
        #[serde(
            rename = "subjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subject_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentEntityRelation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentEntityRelation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta2DocumentLabel {
        #[doc = "Label is generated AutoML model. This field stores the full resource name of the AutoML model. Format: `projects/{project-id}/locations/{location-id}/models/{model-id}`"]
        #[serde(
            rename = "automlModel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub automl_model: ::std::option::Option<String>,
        #[doc = "Confidence score between 0 and 1 for label assignment."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Name of the label. When the label is generated from AutoML Text Classification model, this field represents the name of the category."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentLabel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentLabel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta2DocumentPage {
        #[doc = "A list of visually detected text blocks on the page. A block has a set of lines (collected into paragraphs) that have a common line-spacing and orientation."]
        #[serde(
            rename = "blocks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub blocks: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageBlock>,
        >,
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageDetectedLanguage>,
        >,
        #[doc = "Physical dimension of the page."]
        #[serde(
            rename = "dimension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageDimension,
        >,
        #[doc = "A list of visually detected form fields on the page."]
        #[serde(
            rename = "formFields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub form_fields: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageFormField>,
        >,
        #[doc = "Rendered image for this page. This image is preprocessed to remove any skew, rotation, and distortions such that the annotation bounding boxes can be upright and axis-aligned."]
        #[serde(
            rename = "image",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageImage>,
        #[doc = "Layout for the page."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageLayout>,
        #[doc = "A list of visually detected text lines on the page. A collection of tokens that a human would perceive as a line."]
        #[serde(
            rename = "lines",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lines: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageLine>,
        >,
        #[doc = "1-based index for current Page in a parent Document. Useful when a page is taken out of a Document for individual processing."]
        #[serde(
            rename = "pageNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_number: ::std::option::Option<i32>,
        #[doc = "A list of visually detected text paragraphs on the page. A collection of lines that a human would perceive as a paragraph."]
        #[serde(
            rename = "paragraphs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub paragraphs: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageParagraph>,
        >,
        #[doc = "The history of this page."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentProvenance>,
        #[doc = "A list of visually detected symbols on the page."]
        #[serde(
            rename = "symbols",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub symbols: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageSymbol>,
        >,
        #[doc = "A list of visually detected tables on the page."]
        #[serde(
            rename = "tables",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tables: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageTable>,
        >,
        #[doc = "A list of visually detected tokens on the page."]
        #[serde(
            rename = "tokens",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tokens: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageToken>,
        >,
        #[doc = "Transformation matrices that were applied to the original document image to produce Page.image."]
        #[serde(
            rename = "transforms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transforms: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageMatrix>,
        >,
        #[doc = "A list of detected non-text visual elements e.g. checkbox, signature etc. on the page."]
        #[serde(
            rename = "visualElements",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub visual_elements: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageVisualElement>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentPage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentPage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta2DocumentPageAnchor {
        #[doc = "One or more references to visual page elements"]
        #[serde(
            rename = "pageRefs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_refs: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRef>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentPageAnchor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentPageAnchor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRef {
        #[doc = "Optional. Identifies the bounding polygon of a layout element on the page."]
        #[serde(
            rename = "boundingPoly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2BoundingPoly>,
        #[doc = "Optional. Confidence of detected page element, if applicable. Range [0, 1]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Optional. Deprecated. Use PageRef.bounding_poly instead."]
        #[serde(
            rename = "layoutId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout_id: ::std::option::Option<String>,
        #[doc = "Optional. The type of the layout element that is being referenced if any."]
        #[serde(
            rename = "layoutType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout_type: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType,
        >,
        #[doc = "Required. Index into the Document.pages element, for example using Document.pages to locate the related page element. This field is skipped when its value is the default 0. See https://developers.google.com/protocol-buffers/docs/proto3#json."]
        #[serde(
            rename = "page",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub page: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRef
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRef
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType {
        #[doc = "References a Page.blocks element."]
        Block,
        #[doc = "References a Page.form_fields element."]
        FormField,
        #[doc = "Layout Unspecified."]
        LayoutTypeUnspecified,
        #[doc = "References a Page.lines element."]
        Line,
        #[doc = "References a Page.paragraphs element."]
        Paragraph,
        #[doc = "Refrrences a Page.tables element."]
        Table,
        #[doc = "References a Page.tokens element."]
        Token,
        #[doc = "References a Page.visual_elements element."]
        VisualElement,
    }
    impl GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: Block => "BLOCK" , GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: FormField => "FORM_FIELD" , GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: LayoutTypeUnspecified => "LAYOUT_TYPE_UNSPECIFIED" , GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: Line => "LINE" , GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: Paragraph => "PARAGRAPH" , GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: Table => "TABLE" , GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: Token => "TOKEN" , GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: VisualElement => "VISUAL_ELEMENT" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType,
            (),
        > {
            Ok (match s { "BLOCK" => GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: Block , "FORM_FIELD" => GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: FormField , "LAYOUT_TYPE_UNSPECIFIED" => GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: LayoutTypeUnspecified , "LINE" => GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: Line , "PARAGRAPH" => GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: Paragraph , "TABLE" => GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: Table , "TOKEN" => GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: Token , "VISUAL_ELEMENT" => GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: VisualElement , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "BLOCK" => GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: Block , "FORM_FIELD" => GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: FormField , "LAYOUT_TYPE_UNSPECIFIED" => GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: LayoutTypeUnspecified , "LINE" => GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: Line , "PARAGRAPH" => GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: Paragraph , "TABLE" => GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: Table , "TOKEN" => GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: Token , "VISUAL_ELEMENT" => GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType :: VisualElement , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta2DocumentPageAnchorPageRefLayoutType
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta2DocumentPageBlock {
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageDetectedLanguage>,
        >,
        #[doc = "Layout for Block."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageLayout>,
        #[doc = "The history of this annotation."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentProvenance>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentPageBlock {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentPageBlock {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta2DocumentPageDetectedLanguage {
        #[doc = "Confidence of detected language. Range [0, 1]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see https://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta2DocumentPageDetectedLanguage
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta2DocumentPageDetectedLanguage
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta2DocumentPageDimension {
        #[doc = "Page height."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<f32>,
        #[doc = "Dimension unit."]
        #[serde(
            rename = "unit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit: ::std::option::Option<String>,
        #[doc = "Page width."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentPageDimension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentPageDimension {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta2DocumentPageFormField {
        #[doc = "Created for Labeling UI to export key text. If corrections were made to the text identified by the `field_name.text_anchor`, this field will contain the correction."]
        #[serde(
            rename = "correctedKeyText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub corrected_key_text: ::std::option::Option<String>,
        #[doc = "Created for Labeling UI to export value text. If corrections were made to the text identified by the `field_value.text_anchor`, this field will contain the correction."]
        #[serde(
            rename = "correctedValueText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub corrected_value_text: ::std::option::Option<String>,
        #[doc = "Layout for the FormField name. e.g. `Address`, `Email`, `Grand total`, `Phone number`, etc."]
        #[serde(
            rename = "fieldName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field_name:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageLayout>,
        #[doc = "Layout for the FormField value."]
        #[serde(
            rename = "fieldValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field_value:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageLayout>,
        #[doc = "A list of detected languages for name together with confidence."]
        #[serde(
            rename = "nameDetectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name_detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageDetectedLanguage>,
        >,
        #[doc = "The history of this annotation."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentProvenance>,
        #[doc = "A list of detected languages for value together with confidence."]
        #[serde(
            rename = "valueDetectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageDetectedLanguage>,
        >,
        #[doc = "If the value is non-textual, this field represents the type. Current valid values are: - blank (this indicates the field_value is normal text) - \"unfilled_checkbox\" - \"filled_checkbox\""]
        #[serde(
            rename = "valueType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentPageFormField {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentPageFormField {
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
    pub struct GoogleCloudDocumentaiV1Beta2DocumentPageImage {
        #[doc = "Raw byte content of the image."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Height of the image in pixels."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[doc = "Encoding mime type for the image."]
        #[serde(
            rename = "mimeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime_type: ::std::option::Option<String>,
        #[doc = "Width of the image in pixels."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentPageImage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentPageImage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta2DocumentPageLayout {
        #[doc = "The bounding polygon for the Layout."]
        #[serde(
            rename = "boundingPoly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2BoundingPoly>,
        #[doc = "Confidence of the current Layout within context of the object this layout is for. e.g. confidence can be for a single token, a table, a visual element, etc. depending on context. Range [0, 1]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Detected orientation for the Layout."]
        #[serde(
            rename = "orientation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub orientation: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation,
        >,
        #[doc = "Text anchor indexing into the Document.text."]
        #[serde(
            rename = "textAnchor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_anchor:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentTextAnchor>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentPageLayout {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentPageLayout {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation {
        #[doc = "Unspecified orientation."]
        OrientationUnspecified,
        #[doc = "Orientation is aligned with page down. Turn the head 180 degrees from upright to read."]
        PageDown,
        #[doc = "Orientation is aligned with page left. Turn the head 90 degrees counterclockwise from upright to read."]
        PageLeft,
        #[doc = "Orientation is aligned with page right. Turn the head 90 degrees clockwise from upright to read."]
        PageRight,
        #[doc = "Orientation is aligned with page up."]
        PageUp,
    }
    impl GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation :: OrientationUnspecified => "ORIENTATION_UNSPECIFIED" , GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation :: PageDown => "PAGE_DOWN" , GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation :: PageLeft => "PAGE_LEFT" , GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation :: PageRight => "PAGE_RIGHT" , GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation :: PageUp => "PAGE_UP" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation, ()>
        {
            Ok (match s { "ORIENTATION_UNSPECIFIED" => GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation :: OrientationUnspecified , "PAGE_DOWN" => GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation :: PageDown , "PAGE_LEFT" => GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation :: PageLeft , "PAGE_RIGHT" => GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation :: PageRight , "PAGE_UP" => GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation :: PageUp , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ORIENTATION_UNSPECIFIED" => GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation :: OrientationUnspecified , "PAGE_DOWN" => GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation :: PageDown , "PAGE_LEFT" => GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation :: PageLeft , "PAGE_RIGHT" => GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation :: PageRight , "PAGE_UP" => GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation :: PageUp , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta2DocumentPageLayoutOrientation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta2DocumentPageLine {
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageDetectedLanguage>,
        >,
        #[doc = "Layout for Line."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageLayout>,
        #[doc = "The history of this annotation."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentProvenance>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentPageLine {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentPageLine {
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
    pub struct GoogleCloudDocumentaiV1Beta2DocumentPageMatrix {
        #[doc = "Number of columns in the matrix."]
        #[serde(
            rename = "cols",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cols: ::std::option::Option<i32>,
        #[doc = "The matrix data."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "This encodes information about what data type the matrix uses. For example, 0 (CV_8U) is an unsigned 8-bit image. For the full list of OpenCV primitive data types, please refer to https://docs.opencv.org/4.3.0/d1/d1b/group__core__hal__interface.html"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<i32>,
        #[doc = "Number of rows in the matrix."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentPageMatrix {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentPageMatrix {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta2DocumentPageParagraph {
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageDetectedLanguage>,
        >,
        #[doc = "Layout for Paragraph."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageLayout>,
        #[doc = "The history of this annotation."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentProvenance>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentPageParagraph {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentPageParagraph {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta2DocumentPageSymbol {
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageDetectedLanguage>,
        >,
        #[doc = "Layout for Symbol."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageLayout>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentPageSymbol {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentPageSymbol {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta2DocumentPageTable {
        #[doc = "Body rows of the table."]
        #[serde(
            rename = "bodyRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub body_rows: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageTableTableRow>,
        >,
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageDetectedLanguage>,
        >,
        #[doc = "Header rows of the table."]
        #[serde(
            rename = "headerRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header_rows: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageTableTableRow>,
        >,
        #[doc = "Layout for Table."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageLayout>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentPageTable {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentPageTable {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta2DocumentPageTableTableCell {
        #[doc = "How many columns this cell spans."]
        #[serde(
            rename = "colSpan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub col_span: ::std::option::Option<i32>,
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageDetectedLanguage>,
        >,
        #[doc = "Layout for TableCell."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageLayout>,
        #[doc = "How many rows this cell spans."]
        #[serde(
            rename = "rowSpan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_span: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta2DocumentPageTableTableCell
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta2DocumentPageTableTableCell
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta2DocumentPageTableTableRow {
        #[doc = "Cells that make up this row."]
        #[serde(
            rename = "cells",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cells: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageTableTableCell>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta2DocumentPageTableTableRow
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta2DocumentPageTableTableRow
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta2DocumentPageToken {
        #[doc = "Detected break at the end of a Token."]
        #[serde(
            rename = "detectedBreak",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_break: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreak,
        >,
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageDetectedLanguage>,
        >,
        #[doc = "Layout for Token."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageLayout>,
        #[doc = "The history of this annotation."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentProvenance>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentPageToken {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentPageToken {
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
    pub struct GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreak {
        #[doc = "Detected break type."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreakType,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreak
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreak
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreakType {
        #[doc = "A hyphen that indicates that a token has been split across lines."]
        Hyphen,
        #[doc = "A single whitespace."]
        Space,
        #[doc = "Unspecified break type."]
        TypeUnspecified,
        #[doc = "A wider whitespace."]
        WideSpace,
    }
    impl GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreakType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreakType::Hyphen => "HYPHEN",
                GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreakType::Space => "SPACE",
                GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreakType::TypeUnspecified => {
                    "TYPE_UNSPECIFIED"
                }
                GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreakType::WideSpace => {
                    "WIDE_SPACE"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreakType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreakType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreakType, ()>
        {
            Ok(match s {
                "HYPHEN" => GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreakType::Hyphen,
                "SPACE" => GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreakType::Space,
                "TYPE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreakType::TypeUnspecified
                }
                "WIDE_SPACE" => {
                    GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreakType::WideSpace
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreakType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreakType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreakType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "HYPHEN" => GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreakType::Hyphen,
                "SPACE" => GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreakType::Space,
                "TYPE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreakType::TypeUnspecified
                }
                "WIDE_SPACE" => {
                    GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreakType::WideSpace
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
        for GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreakType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta2DocumentPageTokenDetectedBreakType
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta2DocumentPageVisualElement {
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageDetectedLanguage>,
        >,
        #[doc = "Layout for VisualElement."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentPageLayout>,
        #[doc = "Type of the VisualElement."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta2DocumentPageVisualElement
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta2DocumentPageVisualElement
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
    pub struct GoogleCloudDocumentaiV1Beta2DocumentProvenance {
        #[doc = "The Id of this operation. Needs to be unique within the scope of the revision."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<i32>,
        #[doc = "References to the original elements that are replaced."]
        #[serde(
            rename = "parents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parents: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentProvenanceParent>,
        >,
        #[doc = "The type of provenance operation."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentProvenanceType,
        >,
        #[doc = "The index of the revision that produced this element."]
        #[serde(
            rename = "revision",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentProvenance {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentProvenance {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1Beta2DocumentProvenanceType {
        #[doc = "Add an element."]
        Add,
        #[doc = "Element is reviewed and approved at human review, confidence will be set to 1.0."]
        EvalApproved,
        #[doc = "Request human review for the element identified by `parent`."]
        EvalRequested,
        #[doc = "Element is skipped in the validation process."]
        EvalSkipped,
        #[doc = "Operation type unspecified. If no operation is specified a provenance entry is simply used to match against a `parent`."]
        OperationTypeUnspecified,
        #[doc = "Remove an element identified by `parent`."]
        Remove,
        #[doc = "Replace an element identified by `parent`."]
        Replace,
    }
    impl GoogleCloudDocumentaiV1Beta2DocumentProvenanceType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDocumentaiV1Beta2DocumentProvenanceType::Add => "ADD",
                GoogleCloudDocumentaiV1Beta2DocumentProvenanceType::EvalApproved => "EVAL_APPROVED",
                GoogleCloudDocumentaiV1Beta2DocumentProvenanceType::EvalRequested => {
                    "EVAL_REQUESTED"
                }
                GoogleCloudDocumentaiV1Beta2DocumentProvenanceType::EvalSkipped => "EVAL_SKIPPED",
                GoogleCloudDocumentaiV1Beta2DocumentProvenanceType::OperationTypeUnspecified => {
                    "OPERATION_TYPE_UNSPECIFIED"
                }
                GoogleCloudDocumentaiV1Beta2DocumentProvenanceType::Remove => "REMOVE",
                GoogleCloudDocumentaiV1Beta2DocumentProvenanceType::Replace => "REPLACE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiV1Beta2DocumentProvenanceType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1Beta2DocumentProvenanceType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiV1Beta2DocumentProvenanceType, ()> {
            Ok(match s {
                "ADD" => GoogleCloudDocumentaiV1Beta2DocumentProvenanceType::Add,
                "EVAL_APPROVED" => GoogleCloudDocumentaiV1Beta2DocumentProvenanceType::EvalApproved,
                "EVAL_REQUESTED" => {
                    GoogleCloudDocumentaiV1Beta2DocumentProvenanceType::EvalRequested
                }
                "EVAL_SKIPPED" => GoogleCloudDocumentaiV1Beta2DocumentProvenanceType::EvalSkipped,
                "OPERATION_TYPE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1Beta2DocumentProvenanceType::OperationTypeUnspecified
                }
                "REMOVE" => GoogleCloudDocumentaiV1Beta2DocumentProvenanceType::Remove,
                "REPLACE" => GoogleCloudDocumentaiV1Beta2DocumentProvenanceType::Replace,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1Beta2DocumentProvenanceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1Beta2DocumentProvenanceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDocumentaiV1Beta2DocumentProvenanceType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADD" => GoogleCloudDocumentaiV1Beta2DocumentProvenanceType::Add,
                "EVAL_APPROVED" => GoogleCloudDocumentaiV1Beta2DocumentProvenanceType::EvalApproved,
                "EVAL_REQUESTED" => {
                    GoogleCloudDocumentaiV1Beta2DocumentProvenanceType::EvalRequested
                }
                "EVAL_SKIPPED" => GoogleCloudDocumentaiV1Beta2DocumentProvenanceType::EvalSkipped,
                "OPERATION_TYPE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1Beta2DocumentProvenanceType::OperationTypeUnspecified
                }
                "REMOVE" => GoogleCloudDocumentaiV1Beta2DocumentProvenanceType::Remove,
                "REPLACE" => GoogleCloudDocumentaiV1Beta2DocumentProvenanceType::Replace,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentProvenanceType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentProvenanceType {
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
    pub struct GoogleCloudDocumentaiV1Beta2DocumentProvenanceParent {
        #[doc = "The id of the parent provenance."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<i32>,
        #[doc = "The index of the parent item in the corresponding item list (eg. list of entities, properties within entities, etc.) in the parent revision."]
        #[serde(
            rename = "index",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub index: ::std::option::Option<i32>,
        #[doc = "The index of the index into current revision's parent_ids list."]
        #[serde(
            rename = "revision",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta2DocumentProvenanceParent
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentProvenanceParent {
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
    pub struct GoogleCloudDocumentaiV1Beta2DocumentRevision {
        #[doc = "If the change was made by a person specify the name or id of that person."]
        #[serde(
            rename = "agent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub agent: ::std::option::Option<String>,
        #[doc = "The time that the revision was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Human Review information of this revision."]
        #[serde(
            rename = "humanReview",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub human_review: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentRevisionHumanReview,
        >,
        #[doc = "Id of the revision. Unique within the context of the document."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The revisions that this revision is based on. This can include one or more parent (when documents are merged.) This field represents the index into the `revisions` field."]
        #[serde(
            rename = "parent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent: ::std::option::Option<Vec<i32>>,
        #[doc = "The revisions that this revision is based on. Must include all the ids that have anything to do with this revision - eg. there are `provenance.parent.revision` fields that index into this field."]
        #[serde(
            rename = "parentIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent_ids: ::std::option::Option<Vec<String>>,
        #[doc = "If the annotation was made by processor identify the processor by its resource name."]
        #[serde(
            rename = "processor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub processor: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentRevision {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentRevision {
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
    pub struct GoogleCloudDocumentaiV1Beta2DocumentRevisionHumanReview {
        #[doc = "Human review state. e.g. `requested`, `succeeded`, `rejected`."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<String>,
        #[doc = "A message providing more details about the current state of processing. For example, the rejection reason when the state is `rejected`."]
        #[serde(
            rename = "stateMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state_message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta2DocumentRevisionHumanReview
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta2DocumentRevisionHumanReview
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
    pub struct GoogleCloudDocumentaiV1Beta2DocumentShardInfo {
        #[doc = "Total number of shards."]
        #[serde(
            rename = "shardCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub shard_count: ::std::option::Option<i64>,
        #[doc = "The 0-based index of this shard."]
        #[serde(
            rename = "shardIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub shard_index: ::std::option::Option<i64>,
        #[doc = "The index of the first character in Document.text in the overall document global text."]
        #[serde(
            rename = "textOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub text_offset: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentShardInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentShardInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta2DocumentStyle {
        #[doc = "Text background color."]
        #[serde(
            rename = "backgroundColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub background_color: ::std::option::Option<crate::schemas::GoogleTypeColor>,
        #[doc = "Text color."]
        #[serde(
            rename = "color",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color: ::std::option::Option<crate::schemas::GoogleTypeColor>,
        #[doc = "Font size."]
        #[serde(
            rename = "fontSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub font_size: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentStyleFontSize,
        >,
        #[doc = "Font weight. Possible values are normal, bold, bolder, and lighter. https://www.w3schools.com/cssref/pr_font_weight.asp"]
        #[serde(
            rename = "fontWeight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub font_weight: ::std::option::Option<String>,
        #[doc = "Text anchor indexing into the Document.text."]
        #[serde(
            rename = "textAnchor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_anchor:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentTextAnchor>,
        #[doc = "Text decoration. Follows CSS standard. https://www.w3schools.com/cssref/pr_text_text-decoration.asp"]
        #[serde(
            rename = "textDecoration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_decoration: ::std::option::Option<String>,
        #[doc = "Text style. Possible values are normal, italic, and oblique. https://www.w3schools.com/cssref/pr_font_font-style.asp"]
        #[serde(
            rename = "textStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_style: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta2DocumentStyleFontSize {
        #[doc = "Font size for the text."]
        #[serde(
            rename = "size",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub size: ::std::option::Option<f32>,
        #[doc = "Unit for the font size. Follows CSS naming (in, px, pt, etc.)."]
        #[serde(
            rename = "unit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentStyleFontSize {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentStyleFontSize {
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
    pub struct GoogleCloudDocumentaiV1Beta2DocumentTextAnchor {
        #[doc = "Contains the content of the text span so that users do not have to look it up in the text_segments. It is always populated for formFields."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<String>,
        #[doc = "The text segments from the Document.text."]
        #[serde(
            rename = "textSegments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_segments: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentTextAnchorTextSegment>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentTextAnchor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentTextAnchor {
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
    pub struct GoogleCloudDocumentaiV1Beta2DocumentTextAnchorTextSegment {
        #[doc = "TextSegment half open end UTF-8 char index in the Document.text."]
        #[serde(
            rename = "endIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub end_index: ::std::option::Option<i64>,
        #[doc = "TextSegment start UTF-8 char index in the Document.text."]
        #[serde(
            rename = "startIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub start_index: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta2DocumentTextAnchorTextSegment
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta2DocumentTextAnchorTextSegment
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
    pub struct GoogleCloudDocumentaiV1Beta2DocumentTextChange {
        #[doc = "The text that replaces the text identified in the `text_anchor`."]
        #[serde(
            rename = "changedText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub changed_text: ::std::option::Option<String>,
        #[doc = "The history of this annotation."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentProvenance>,
        >,
        #[doc = "Provenance of the correction. Text anchor indexing into the Document.text. There can only be a single `TextAnchor.text_segments` element. If the start and end index of the text segment are the same, the text change is inserted before that index."]
        #[serde(
            rename = "textAnchor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_anchor:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2DocumentTextAnchor>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2DocumentTextChange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2DocumentTextChange {
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
    pub struct GoogleCloudDocumentaiV1Beta2GcsDestination {
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2GcsDestination {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2GcsDestination {
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
    pub struct GoogleCloudDocumentaiV1Beta2GcsSource {
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2GcsSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2GcsSource {
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
    pub struct GoogleCloudDocumentaiV1Beta2InputConfig {
        #[doc = "Content in bytes, represented as a stream of bytes. Note: As with all `bytes` fields, proto buffer messages use a pure binary representation, whereas JSON representations use base64. This field only works for synchronous ProcessDocument method."]
        #[serde(
            rename = "contents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contents: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "The Google Cloud Storage location to read the input from. This must be a single file."]
        #[serde(
            rename = "gcsSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_source:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2GcsSource>,
        #[doc = "Required. Mimetype of the input. Current supported mimetypes are application/pdf, image/tiff, and image/gif. In addition, application/json type is supported for requests with ProcessDocumentRequest.automl_params field set. The JSON file needs to be in Document format."]
        #[serde(
            rename = "mimeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2InputConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2InputConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta2NormalizedVertex {
        #[doc = "X coordinate."]
        #[serde(
            rename = "x",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub x: ::std::option::Option<f32>,
        #[doc = "Y coordinate (starts from the top of the image)."]
        #[serde(
            rename = "y",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub y: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2NormalizedVertex {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2NormalizedVertex {
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
    pub struct GoogleCloudDocumentaiV1Beta2OperationMetadata {
        #[doc = "The creation time of the operation."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The state of the current batch processing."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta2OperationMetadataState,
        >,
        #[doc = "A message providing more details about the current state of processing."]
        #[serde(
            rename = "stateMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state_message: ::std::option::Option<String>,
        #[doc = "The last update time of the operation."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2OperationMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2OperationMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1Beta2OperationMetadataState {
        #[doc = "Request is received."]
        Accepted,
        #[doc = "The batch processing was cancelled."]
        Cancelled,
        #[doc = "The batch processing has failed."]
        Failed,
        #[doc = "Request is being processed."]
        Running,
        #[doc = "The default value. This value is used if the state is omitted."]
        StateUnspecified,
        #[doc = "The batch processing completed successfully."]
        Succeeded,
        #[doc = "Request operation is waiting for scheduling."]
        Waiting,
    }
    impl GoogleCloudDocumentaiV1Beta2OperationMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDocumentaiV1Beta2OperationMetadataState::Accepted => "ACCEPTED",
                GoogleCloudDocumentaiV1Beta2OperationMetadataState::Cancelled => "CANCELLED",
                GoogleCloudDocumentaiV1Beta2OperationMetadataState::Failed => "FAILED",
                GoogleCloudDocumentaiV1Beta2OperationMetadataState::Running => "RUNNING",
                GoogleCloudDocumentaiV1Beta2OperationMetadataState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
                GoogleCloudDocumentaiV1Beta2OperationMetadataState::Succeeded => "SUCCEEDED",
                GoogleCloudDocumentaiV1Beta2OperationMetadataState::Waiting => "WAITING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiV1Beta2OperationMetadataState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1Beta2OperationMetadataState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiV1Beta2OperationMetadataState, ()> {
            Ok(match s {
                "ACCEPTED" => GoogleCloudDocumentaiV1Beta2OperationMetadataState::Accepted,
                "CANCELLED" => GoogleCloudDocumentaiV1Beta2OperationMetadataState::Cancelled,
                "FAILED" => GoogleCloudDocumentaiV1Beta2OperationMetadataState::Failed,
                "RUNNING" => GoogleCloudDocumentaiV1Beta2OperationMetadataState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1Beta2OperationMetadataState::StateUnspecified
                }
                "SUCCEEDED" => GoogleCloudDocumentaiV1Beta2OperationMetadataState::Succeeded,
                "WAITING" => GoogleCloudDocumentaiV1Beta2OperationMetadataState::Waiting,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1Beta2OperationMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1Beta2OperationMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDocumentaiV1Beta2OperationMetadataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCEPTED" => GoogleCloudDocumentaiV1Beta2OperationMetadataState::Accepted,
                "CANCELLED" => GoogleCloudDocumentaiV1Beta2OperationMetadataState::Cancelled,
                "FAILED" => GoogleCloudDocumentaiV1Beta2OperationMetadataState::Failed,
                "RUNNING" => GoogleCloudDocumentaiV1Beta2OperationMetadataState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1Beta2OperationMetadataState::StateUnspecified
                }
                "SUCCEEDED" => GoogleCloudDocumentaiV1Beta2OperationMetadataState::Succeeded,
                "WAITING" => GoogleCloudDocumentaiV1Beta2OperationMetadataState::Waiting,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2OperationMetadataState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2OperationMetadataState {
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
    pub struct GoogleCloudDocumentaiV1Beta2OutputConfig {
        #[doc = "The Google Cloud Storage location to write the output to."]
        #[serde(
            rename = "gcsDestination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_destination:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2GcsDestination>,
        #[doc = "The max number of pages to include into each output Document shard JSON on Google Cloud Storage. The valid range is [1, 100]. If not specified, the default value is 20. For example, for one pdf file with 100 pages, 100 parsed pages will be produced. If `pages_per_shard` = 20, then 5 Document shard JSON files each containing 20 parsed pages will be written under the prefix OutputConfig.gcs_destination.uri and suffix pages-x-to-y.json where x and y are 1-indexed page numbers. Example GCS outputs with 157 pages and pages_per_shard = 50: pages-001-to-050.json pages-051-to-100.json pages-101-to-150.json pages-151-to-157.json"]
        #[serde(
            rename = "pagesPerShard",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pages_per_shard: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2OutputConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2OutputConfig {
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
    pub struct GoogleCloudDocumentaiV1Beta2ProcessDocumentResponse {
        #[doc = "Information about the input file. This is the same as the corresponding input config in the request."]
        #[serde(
            rename = "inputConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_config:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2InputConfig>,
        #[doc = "The output location of the parsed responses. The responses are written to this location as JSON-serialized `Document` objects."]
        #[serde(
            rename = "outputConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_config:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta2OutputConfig>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta2ProcessDocumentResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2ProcessDocumentResponse {
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
    pub struct GoogleCloudDocumentaiV1Beta2Vertex {
        #[doc = "X coordinate."]
        #[serde(
            rename = "x",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub x: ::std::option::Option<i32>,
        #[doc = "Y coordinate (starts from the top of the image)."]
        #[serde(
            rename = "y",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub y: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta2Vertex {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta2Vertex {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDocumentaiV1Beta3BatchProcessMetadata { # [doc = "The creation time of the operation."] # [serde (rename = "createTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub create_time : :: std :: option :: Option < String > , # [doc = "The list of response details of each document."] # [serde (rename = "individualProcessStatuses" , default , skip_serializing_if = "std::option::Option::is_none")] pub individual_process_statuses : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudDocumentaiV1Beta3BatchProcessMetadataIndividualProcessStatus > > , # [doc = "The state of the current batch processing."] # [serde (rename = "state" , default , skip_serializing_if = "std::option::Option::is_none")] pub state : :: std :: option :: Option < crate :: schemas :: GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState > , # [doc = "A message providing more details about the current state of processing. For example, the error message if the operation is failed."] # [serde (rename = "stateMessage" , default , skip_serializing_if = "std::option::Option::is_none")] pub state_message : :: std :: option :: Option < String > , # [doc = "The last update time of the operation."] # [serde (rename = "updateTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub update_time : :: std :: option :: Option < String > , }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta3BatchProcessMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta3BatchProcessMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState {
        #[doc = "The batch processing was cancelled."]
        Cancelled,
        #[doc = "The batch processing was being cancelled."]
        Cancelling,
        #[doc = "The batch processing has failed."]
        Failed,
        #[doc = "Request is being processed."]
        Running,
        #[doc = "The default value. This value is used if the state is omitted."]
        StateUnspecified,
        #[doc = "The batch processing completed successfully."]
        Succeeded,
        #[doc = "Request operation is waiting for scheduling."]
        Waiting,
    }
    impl GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState::Cancelled => "CANCELLED",
                GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState::Cancelling => "CANCELLING",
                GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState::Failed => "FAILED",
                GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState::Running => "RUNNING",
                GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
                GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState::Succeeded => "SUCCEEDED",
                GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState::Waiting => "WAITING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState, ()>
        {
            Ok(match s {
                "CANCELLED" => GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState::Cancelled,
                "CANCELLING" => GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState::Cancelling,
                "FAILED" => GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState::Failed,
                "RUNNING" => GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState::StateUnspecified
                }
                "SUCCEEDED" => GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState::Succeeded,
                "WAITING" => GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState::Waiting,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState::Cancelled,
                "CANCELLING" => GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState::Cancelling,
                "FAILED" => GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState::Failed,
                "RUNNING" => GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState::StateUnspecified
                }
                "SUCCEEDED" => GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState::Succeeded,
                "WAITING" => GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState::Waiting,
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
        for GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta3BatchProcessMetadataState
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDocumentaiV1Beta3BatchProcessMetadataIndividualProcessStatus {
        #[doc = "The name of the operation triggered by the processed document. If the human review process is not triggered, this field will be empty. It has the same response type and metadata as the long running operation returned by ReviewDocument method."]
        #[serde(
            rename = "humanReviewOperation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub human_review_operation: ::std::option::Option<String>,
        #[doc = "The status of human review on the processed document."]
        #[serde(
            rename = "humanReviewStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub human_review_status:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Beta3HumanReviewStatus>,
        #[doc = "The source of the document, same as the [input_gcs_source] field in the request when the batch process started. The batch process is started by take snapshot of that document, since a user can move or change that document during the process."]
        #[serde(
            rename = "inputGcsSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_gcs_source: ::std::option::Option<String>,
        #[doc = "The output_gcs_destination (in the request as `output_gcs_destination`) of the processed document if it was successful, otherwise empty."]
        #[serde(
            rename = "outputGcsDestination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_gcs_destination: ::std::option::Option<String>,
        #[doc = "The status of the processing of the document."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta3BatchProcessMetadataIndividualProcessStatus
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta3BatchProcessMetadataIndividualProcessStatus
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta3BatchProcessResponse {}
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta3BatchProcessResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta3BatchProcessResponse {
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
    pub struct GoogleCloudDocumentaiV1Beta3CommonOperationMetadata {
        #[doc = "The creation time of the operation."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "A related resource to this operation."]
        #[serde(
            rename = "resource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource: ::std::option::Option<String>,
        #[doc = "The state of the operation."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState,
        >,
        #[doc = "A message providing more details about the current state of processing."]
        #[serde(
            rename = "stateMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state_message: ::std::option::Option<String>,
        #[doc = "The last update time of the operation."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta3CommonOperationMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta3CommonOperationMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState {
        #[doc = "Operation is cancelled."]
        Cancelled,
        #[doc = "Operation is being cancelled."]
        Cancelling,
        #[doc = "Operation failed."]
        Failed,
        #[doc = "Operation is still running."]
        Running,
        #[doc = "Unspecified state."]
        StateUnspecified,
        #[doc = "Operation succeeded."]
        Succeeded,
    }
    impl GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState::Cancelled => "CANCELLED",
                GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState::Cancelling => {
                    "CANCELLING"
                }
                GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState::Failed => "FAILED",
                GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState::Running => "RUNNING",
                GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
                GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState::Succeeded => "SUCCEEDED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState, ()>
        {
            Ok(match s {
                "CANCELLED" => GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState::Cancelled,
                "CANCELLING" => {
                    GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState::Cancelling
                }
                "FAILED" => GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState::Failed,
                "RUNNING" => GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState::StateUnspecified
                }
                "SUCCEEDED" => GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState::Succeeded,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState::Cancelled,
                "CANCELLING" => {
                    GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState::Cancelling
                }
                "FAILED" => GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState::Failed,
                "RUNNING" => GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState::StateUnspecified
                }
                "SUCCEEDED" => GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState::Succeeded,
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
        for GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta3CommonOperationMetadataState
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
    pub struct GoogleCloudDocumentaiV1Beta3DeleteProcessorMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta3CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta3DeleteProcessorMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta3DeleteProcessorMetadata {
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
    pub struct GoogleCloudDocumentaiV1Beta3DeleteProcessorVersionMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta3CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta3DeleteProcessorVersionMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta3DeleteProcessorVersionMetadata
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
    pub struct GoogleCloudDocumentaiV1Beta3DeployProcessorVersionMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta3CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta3DeployProcessorVersionMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta3DeployProcessorVersionMetadata
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta3DeployProcessorVersionResponse {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta3DeployProcessorVersionResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta3DeployProcessorVersionResponse
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
    pub struct GoogleCloudDocumentaiV1Beta3DisableProcessorMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta3CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta3DisableProcessorMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta3DisableProcessorMetadata {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta3DisableProcessorResponse {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta3DisableProcessorResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta3DisableProcessorResponse {
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
    pub struct GoogleCloudDocumentaiV1Beta3EnableProcessorMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta3CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta3EnableProcessorMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta3EnableProcessorMetadata {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta3EnableProcessorResponse {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta3EnableProcessorResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta3EnableProcessorResponse {
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
    pub struct GoogleCloudDocumentaiV1Beta3HumanReviewStatus {
        #[doc = "The name of the operation triggered by the processed document. This field is populated only when the [state] is [HUMAN_REVIEW_IN_PROGRESS]. It has the same response type and metadata as the long running operation returned by [ReviewDocument] method."]
        #[serde(
            rename = "humanReviewOperation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub human_review_operation: ::std::option::Option<String>,
        #[doc = "The state of human review on the processing request."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta3HumanReviewStatusState,
        >,
        #[doc = "A message providing more details about the human review state."]
        #[serde(
            rename = "stateMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state_message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta3HumanReviewStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta3HumanReviewStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1Beta3HumanReviewStatusState {
        #[doc = "Some error happened during triggering human review, see the [state_message] for details."]
        Error,
        #[doc = "Human review validation is triggered and the document is under review."]
        InProgress,
        #[doc = "Human review is skipped for the document. This can happen because human review is not enabled on the processor or the processing request has been set to skip this document."]
        Skipped,
        #[doc = "Human review state is unspecified. Most likely due to an internal error."]
        StateUnspecified,
        #[doc = "Human review validation is triggered and passed, so no review is needed."]
        ValidationPassed,
    }
    impl GoogleCloudDocumentaiV1Beta3HumanReviewStatusState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDocumentaiV1Beta3HumanReviewStatusState::Error => "ERROR",
                GoogleCloudDocumentaiV1Beta3HumanReviewStatusState::InProgress => "IN_PROGRESS",
                GoogleCloudDocumentaiV1Beta3HumanReviewStatusState::Skipped => "SKIPPED",
                GoogleCloudDocumentaiV1Beta3HumanReviewStatusState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
                GoogleCloudDocumentaiV1Beta3HumanReviewStatusState::ValidationPassed => {
                    "VALIDATION_PASSED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiV1Beta3HumanReviewStatusState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1Beta3HumanReviewStatusState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiV1Beta3HumanReviewStatusState, ()> {
            Ok(match s {
                "ERROR" => GoogleCloudDocumentaiV1Beta3HumanReviewStatusState::Error,
                "IN_PROGRESS" => GoogleCloudDocumentaiV1Beta3HumanReviewStatusState::InProgress,
                "SKIPPED" => GoogleCloudDocumentaiV1Beta3HumanReviewStatusState::Skipped,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1Beta3HumanReviewStatusState::StateUnspecified
                }
                "VALIDATION_PASSED" => {
                    GoogleCloudDocumentaiV1Beta3HumanReviewStatusState::ValidationPassed
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1Beta3HumanReviewStatusState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1Beta3HumanReviewStatusState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDocumentaiV1Beta3HumanReviewStatusState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ERROR" => GoogleCloudDocumentaiV1Beta3HumanReviewStatusState::Error,
                "IN_PROGRESS" => GoogleCloudDocumentaiV1Beta3HumanReviewStatusState::InProgress,
                "SKIPPED" => GoogleCloudDocumentaiV1Beta3HumanReviewStatusState::Skipped,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1Beta3HumanReviewStatusState::StateUnspecified
                }
                "VALIDATION_PASSED" => {
                    GoogleCloudDocumentaiV1Beta3HumanReviewStatusState::ValidationPassed
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
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta3HumanReviewStatusState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta3HumanReviewStatusState {
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
    pub struct GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta3CommonOperationMetadata,
        >,
        #[doc = "The creation time of the operation."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The Crowd Compute question ID."]
        #[serde(
            rename = "questionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub question_id: ::std::option::Option<String>,
        #[doc = "Used only when Operation.done is false."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState,
        >,
        #[doc = "A message providing more details about the current state of processing. For example, the error message if the operation is failed."]
        #[serde(
            rename = "stateMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state_message: ::std::option::Option<String>,
        #[doc = "The last update time of the operation."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadata
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState {
        #[doc = "Operation is cancelled."]
        Cancelled,
        #[doc = "Operation is being cancelled."]
        Cancelling,
        #[doc = "Operation failed."]
        Failed,
        #[doc = "Operation is still running."]
        Running,
        #[doc = "Unspecified state."]
        StateUnspecified,
        #[doc = "Operation succeeded."]
        Succeeded,
    }
    impl GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState :: Cancelled => "CANCELLED" , GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState :: Cancelling => "CANCELLING" , GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState :: Failed => "FAILED" , GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState :: Running => "RUNNING" , GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState :: StateUnspecified => "STATE_UNSPECIFIED" , GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState :: Succeeded => "SUCCEEDED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState,
            (),
        > {
            Ok (match s { "CANCELLED" => GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState :: Cancelled , "CANCELLING" => GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState :: Cancelling , "FAILED" => GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState :: Failed , "RUNNING" => GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState :: Running , "STATE_UNSPECIFIED" => GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState :: StateUnspecified , "SUCCEEDED" => GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState :: Succeeded , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CANCELLED" => GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState :: Cancelled , "CANCELLING" => GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState :: Cancelling , "FAILED" => GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState :: Failed , "RUNNING" => GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState :: Running , "STATE_UNSPECIFIED" => GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState :: StateUnspecified , "SUCCEEDED" => GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState :: Succeeded , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta3ReviewDocumentOperationMetadataState
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
    pub struct GoogleCloudDocumentaiV1Beta3ReviewDocumentResponse {
        #[doc = "The Cloud Storage uri for the human reviewed document."]
        #[serde(
            rename = "gcsDestination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_destination: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Beta3ReviewDocumentResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Beta3ReviewDocumentResponse {
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
    pub struct GoogleCloudDocumentaiV1Beta3SetDefaultProcessorVersionMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta3CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta3SetDefaultProcessorVersionMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta3SetDefaultProcessorVersionMetadata
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta3SetDefaultProcessorVersionResponse {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta3SetDefaultProcessorVersionResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta3SetDefaultProcessorVersionResponse
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
    pub struct GoogleCloudDocumentaiV1Beta3UndeployProcessorVersionMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1Beta3CommonOperationMetadata,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta3UndeployProcessorVersionMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta3UndeployProcessorVersionMetadata
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1Beta3UndeployProcessorVersionResponse {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1Beta3UndeployProcessorVersionResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1Beta3UndeployProcessorVersionResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1BoundingPoly {
        #[doc = "The bounding polygon normalized vertices."]
        #[serde(
            rename = "normalizedVertices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_vertices:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1NormalizedVertex>>,
        #[doc = "The bounding polygon vertices."]
        #[serde(
            rename = "vertices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vertices: ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1Vertex>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1BoundingPoly {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1BoundingPoly {
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
    pub struct GoogleCloudDocumentaiV1CommonOperationMetadata {
        #[doc = "The creation time of the operation."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "A related resource to this operation."]
        #[serde(
            rename = "resource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource: ::std::option::Option<String>,
        #[doc = "The state of the operation."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1CommonOperationMetadataState,
        >,
        #[doc = "A message providing more details about the current state of processing."]
        #[serde(
            rename = "stateMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state_message: ::std::option::Option<String>,
        #[doc = "The last update time of the operation."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1CommonOperationMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1CommonOperationMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1CommonOperationMetadataState {
        #[doc = "Operation is cancelled."]
        Cancelled,
        #[doc = "Operation is being cancelled."]
        Cancelling,
        #[doc = "Operation failed."]
        Failed,
        #[doc = "Operation is still running."]
        Running,
        #[doc = "Unspecified state."]
        StateUnspecified,
        #[doc = "Operation succeeded."]
        Succeeded,
    }
    impl GoogleCloudDocumentaiV1CommonOperationMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDocumentaiV1CommonOperationMetadataState::Cancelled => "CANCELLED",
                GoogleCloudDocumentaiV1CommonOperationMetadataState::Cancelling => "CANCELLING",
                GoogleCloudDocumentaiV1CommonOperationMetadataState::Failed => "FAILED",
                GoogleCloudDocumentaiV1CommonOperationMetadataState::Running => "RUNNING",
                GoogleCloudDocumentaiV1CommonOperationMetadataState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
                GoogleCloudDocumentaiV1CommonOperationMetadataState::Succeeded => "SUCCEEDED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiV1CommonOperationMetadataState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1CommonOperationMetadataState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiV1CommonOperationMetadataState, ()>
        {
            Ok(match s {
                "CANCELLED" => GoogleCloudDocumentaiV1CommonOperationMetadataState::Cancelled,
                "CANCELLING" => GoogleCloudDocumentaiV1CommonOperationMetadataState::Cancelling,
                "FAILED" => GoogleCloudDocumentaiV1CommonOperationMetadataState::Failed,
                "RUNNING" => GoogleCloudDocumentaiV1CommonOperationMetadataState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1CommonOperationMetadataState::StateUnspecified
                }
                "SUCCEEDED" => GoogleCloudDocumentaiV1CommonOperationMetadataState::Succeeded,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1CommonOperationMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1CommonOperationMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDocumentaiV1CommonOperationMetadataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => GoogleCloudDocumentaiV1CommonOperationMetadataState::Cancelled,
                "CANCELLING" => GoogleCloudDocumentaiV1CommonOperationMetadataState::Cancelling,
                "FAILED" => GoogleCloudDocumentaiV1CommonOperationMetadataState::Failed,
                "RUNNING" => GoogleCloudDocumentaiV1CommonOperationMetadataState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1CommonOperationMetadataState::StateUnspecified
                }
                "SUCCEEDED" => GoogleCloudDocumentaiV1CommonOperationMetadataState::Succeeded,
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
        for GoogleCloudDocumentaiV1CommonOperationMetadataState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1CommonOperationMetadataState {
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
    pub struct GoogleCloudDocumentaiV1DeleteProcessorMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1CommonOperationMetadata>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DeleteProcessorMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DeleteProcessorMetadata {
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
    pub struct GoogleCloudDocumentaiV1DeleteProcessorVersionMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1CommonOperationMetadata>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1DeleteProcessorVersionMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1DeleteProcessorVersionMetadata
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
    pub struct GoogleCloudDocumentaiV1DeployProcessorVersionMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1CommonOperationMetadata>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1DeployProcessorVersionMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1DeployProcessorVersionMetadata
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DeployProcessorVersionRequest {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1DeployProcessorVersionRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DeployProcessorVersionRequest {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DeployProcessorVersionResponse {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1DeployProcessorVersionResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1DeployProcessorVersionResponse
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
    pub struct GoogleCloudDocumentaiV1DisableProcessorMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1CommonOperationMetadata>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DisableProcessorMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DisableProcessorMetadata {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DisableProcessorRequest {}
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DisableProcessorRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DisableProcessorRequest {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DisableProcessorResponse {}
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DisableProcessorResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DisableProcessorResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDocumentaiV1Document {
        #[doc = "Optional. Inline document content, represented as a stream of bytes. Note: As with all `bytes` fields, protobuffers use a pure binary representation, whereas JSON representations use base64."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "A list of entities detected on Document.text. For document shards, entities in this list may cross shard boundaries."]
        #[serde(
            rename = "entities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entities:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentEntity>>,
        #[doc = "Placeholder. Relationship among Document.entities."]
        #[serde(
            rename = "entityRelations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_relations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentEntityRelation>,
        >,
        #[doc = "Any error that occurred while processing this document."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
        #[doc = "An IANA published MIME type (also referred to as media type). For more information, see https://www.iana.org/assignments/media-types/media-types.xhtml."]
        #[serde(
            rename = "mimeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime_type: ::std::option::Option<String>,
        #[doc = "Visual page layout for the Document."]
        #[serde(
            rename = "pages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pages: ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPage>>,
        #[doc = "Placeholder. Revision history of this document."]
        #[serde(
            rename = "revisions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revisions:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentRevision>>,
        #[doc = "Information about the sharding if this document is sharded part of a larger document. If the document is not sharded, this message is not specified."]
        #[serde(
            rename = "shardInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shard_info:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentShardInfo>,
        #[doc = "Optional. UTF-8 encoded text in reading order from the document."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
        #[doc = "Placeholder. A list of text corrections made to [Document.text]. This is usually used for annotating corrections to OCR mistakes. Text changes for a given revision may not overlap with each other."]
        #[serde(
            rename = "textChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_changes:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentTextChange>>,
        #[doc = "Placeholder. Styles for the Document.text."]
        #[serde(
            rename = "textStyles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_styles:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentStyle>>,
        #[doc = "Optional. Currently supports Google Cloud Storage URI of the form `gs://bucket_name/object_name`. Object versioning is not supported. See [Google Cloud Storage Request URIs](https://cloud.google.com/storage/docs/reference-uris) for more info."]
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Document {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Document {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DocumentEntity {
        #[doc = "Optional. Confidence of detected Schema entity. Range [0, 1]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Optional. Canonical id. This will be a unique value in the entity list for this document."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Optional. Deprecated. Use `id` field instead."]
        #[serde(
            rename = "mentionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mention_id: ::std::option::Option<String>,
        #[doc = "Optional. Text value in the document e.g. `1600 Amphitheatre Pkwy`. If the entity is not present in the document, this field will be empty."]
        #[serde(
            rename = "mentionText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mention_text: ::std::option::Option<String>,
        #[doc = "Optional. This attribute indicates that the processing didn't actually identify this entity, but a confidence score was assigned that represent the potential that this could be a false negative. A non-present entity should have an empty mention_text and text_anchor."]
        #[serde(
            rename = "nonPresent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_present: ::std::option::Option<bool>,
        #[doc = "Optional. Normalized entity value. Absent if the extracted value could not be converted or the type (e.g. address) is not supported for certain parsers. This field is also only populated for certain supported document types."]
        #[serde(
            rename = "normalizedValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_value: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1DocumentEntityNormalizedValue,
        >,
        #[doc = "Optional. Represents the provenance of this entity wrt. the location on the page where it was found."]
        #[serde(
            rename = "pageAnchor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_anchor:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentPageAnchor>,
        #[doc = "Optional. Entities can be nested to form a hierarchical data structure representing the content in the document."]
        #[serde(
            rename = "properties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub properties:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentEntity>>,
        #[doc = "Optional. The history of this annotation."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentProvenance>,
        #[doc = "Required. Entity type from a schema e.g. `Address`."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "Optional. Whether the entity will be redacted for de-identification purposes."]
        #[serde(
            rename = "redacted",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub redacted: ::std::option::Option<bool>,
        #[doc = "Optional. Provenance of the entity. Text anchor indexing into the Document.text."]
        #[serde(
            rename = "textAnchor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_anchor:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentTextAnchor>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentEntity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentEntity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DocumentEntityNormalizedValue {
        #[doc = "Postal address. See also: https://github.com/googleapis/googleapis/blob/master/google/type/postal_address.proto"]
        #[serde(
            rename = "addressValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub address_value: ::std::option::Option<crate::schemas::GoogleTypePostalAddress>,
        #[doc = "Boolean value. Can be used for entities with binary values, or for checkboxes."]
        #[serde(
            rename = "booleanValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub boolean_value: ::std::option::Option<bool>,
        #[doc = "Date value. Includes year, month, day. See also: https://github.com/googleapis/googleapis/blob/master/google/type/date.proto"]
        #[serde(
            rename = "dateValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_value: ::std::option::Option<crate::schemas::GoogleTypeDate>,
        #[doc = "DateTime value. Includes date, time, and timezone. See also: https://github.com/googleapis/googleapis/blob/master/google/type/datetime.proto"]
        #[serde(
            rename = "datetimeValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub datetime_value: ::std::option::Option<crate::schemas::GoogleTypeDateTime>,
        #[doc = "Float value."]
        #[serde(
            rename = "floatValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub float_value: ::std::option::Option<f32>,
        #[doc = "Integer value."]
        #[serde(
            rename = "integerValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub integer_value: ::std::option::Option<i32>,
        #[doc = "Money value. See also: https://github.com/googleapis/googleapis/blob/master/google/type/money.proto"]
        #[serde(
            rename = "moneyValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub money_value: ::std::option::Option<crate::schemas::GoogleTypeMoney>,
        #[doc = "Optional. An optional field to store a normalized string. For some entity types, one of respective `structured_value` fields may also be populated. Also not all the types of `structured_value` will be normalized. For example, some processors may not generate float or int normalized text by default. Below are sample formats mapped to structured values. - Money/Currency type (`money_value`) is in the ISO 4217 text format. - Date type (`date_value`) is in the ISO 8601 text format. - Datetime type (`datetime_value`) is in the ISO 8601 text format."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1DocumentEntityNormalizedValue
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentEntityNormalizedValue {
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
    pub struct GoogleCloudDocumentaiV1DocumentEntityRelation {
        #[doc = "Object entity id."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "Relationship description."]
        #[serde(
            rename = "relation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relation: ::std::option::Option<String>,
        #[doc = "Subject entity id."]
        #[serde(
            rename = "subjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subject_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentEntityRelation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentEntityRelation {
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
    pub struct GoogleCloudDocumentaiV1DocumentOutputConfig {
        #[doc = "Output config to write the results to Cloud Storage."]
        #[serde(
            rename = "gcsOutputConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_output_config: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1DocumentOutputConfigGcsOutputConfig,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentOutputConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentOutputConfig {
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
    pub struct GoogleCloudDocumentaiV1DocumentOutputConfigGcsOutputConfig {
        #[doc = "The Cloud Storage uri (a directory) of the output."]
        #[serde(
            rename = "gcsUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1DocumentOutputConfigGcsOutputConfig
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1DocumentOutputConfigGcsOutputConfig
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DocumentPage {
        #[doc = "A list of visually detected text blocks on the page. A block has a set of lines (collected into paragraphs) that have a common line-spacing and orientation."]
        #[serde(
            rename = "blocks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub blocks:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageBlock>>,
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageDetectedLanguage>,
        >,
        #[doc = "Physical dimension of the page."]
        #[serde(
            rename = "dimension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentPageDimension>,
        #[doc = "A list of visually detected form fields on the page."]
        #[serde(
            rename = "formFields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub form_fields: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageFormField>,
        >,
        #[doc = "Rendered image for this page. This image is preprocessed to remove any skew, rotation, and distortions such that the annotation bounding boxes can be upright and axis-aligned."]
        #[serde(
            rename = "image",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image: ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentPageImage>,
        #[doc = "Layout for the page."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentPageLayout>,
        #[doc = "A list of visually detected text lines on the page. A collection of tokens that a human would perceive as a line."]
        #[serde(
            rename = "lines",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lines:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageLine>>,
        #[doc = "1-based index for current Page in a parent Document. Useful when a page is taken out of a Document for individual processing."]
        #[serde(
            rename = "pageNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_number: ::std::option::Option<i32>,
        #[doc = "A list of visually detected text paragraphs on the page. A collection of lines that a human would perceive as a paragraph."]
        #[serde(
            rename = "paragraphs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub paragraphs: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageParagraph>,
        >,
        #[doc = "The history of this page."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentProvenance>,
        #[doc = "A list of visually detected symbols on the page."]
        #[serde(
            rename = "symbols",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub symbols:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageSymbol>>,
        #[doc = "A list of visually detected tables on the page."]
        #[serde(
            rename = "tables",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tables:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageTable>>,
        #[doc = "A list of visually detected tokens on the page."]
        #[serde(
            rename = "tokens",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tokens:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageToken>>,
        #[doc = "Transformation matrices that were applied to the original document image to produce Page.image."]
        #[serde(
            rename = "transforms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transforms:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageMatrix>>,
        #[doc = "A list of detected non-text visual elements e.g. checkbox, signature etc. on the page."]
        #[serde(
            rename = "visualElements",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub visual_elements: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageVisualElement>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentPage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentPage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DocumentPageAnchor {
        #[doc = "One or more references to visual page elements"]
        #[serde(
            rename = "pageRefs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_refs: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageAnchorPageRef>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentPageAnchor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentPageAnchor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DocumentPageAnchorPageRef {
        #[doc = "Optional. Identifies the bounding polygon of a layout element on the page."]
        #[serde(
            rename = "boundingPoly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1BoundingPoly>,
        #[doc = "Optional. Confidence of detected page element, if applicable. Range [0, 1]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Optional. Deprecated. Use PageRef.bounding_poly instead."]
        #[serde(
            rename = "layoutId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout_id: ::std::option::Option<String>,
        #[doc = "Optional. The type of the layout element that is being referenced if any."]
        #[serde(
            rename = "layoutType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout_type: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType,
        >,
        #[doc = "Required. Index into the Document.pages element, for example using Document.pages to locate the related page element. This field is skipped when its value is the default 0. See https://developers.google.com/protocol-buffers/docs/proto3#json."]
        #[serde(
            rename = "page",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub page: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentPageAnchorPageRef {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentPageAnchorPageRef {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType {
        #[doc = "References a Page.blocks element."]
        Block,
        #[doc = "References a Page.form_fields element."]
        FormField,
        #[doc = "Layout Unspecified."]
        LayoutTypeUnspecified,
        #[doc = "References a Page.lines element."]
        Line,
        #[doc = "References a Page.paragraphs element."]
        Paragraph,
        #[doc = "Refrrences a Page.tables element."]
        Table,
        #[doc = "References a Page.tokens element."]
        Token,
        #[doc = "References a Page.visual_elements element."]
        VisualElement,
    }
    impl GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: Block => "BLOCK" , GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: FormField => "FORM_FIELD" , GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: LayoutTypeUnspecified => "LAYOUT_TYPE_UNSPECIFIED" , GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: Line => "LINE" , GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: Paragraph => "PARAGRAPH" , GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: Table => "TABLE" , GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: Token => "TOKEN" , GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: VisualElement => "VISUAL_ELEMENT" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType, ()>
        {
            Ok (match s { "BLOCK" => GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: Block , "FORM_FIELD" => GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: FormField , "LAYOUT_TYPE_UNSPECIFIED" => GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: LayoutTypeUnspecified , "LINE" => GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: Line , "PARAGRAPH" => GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: Paragraph , "TABLE" => GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: Table , "TOKEN" => GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: Token , "VISUAL_ELEMENT" => GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: VisualElement , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "BLOCK" => GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: Block , "FORM_FIELD" => GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: FormField , "LAYOUT_TYPE_UNSPECIFIED" => GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: LayoutTypeUnspecified , "LINE" => GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: Line , "PARAGRAPH" => GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: Paragraph , "TABLE" => GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: Table , "TOKEN" => GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: Token , "VISUAL_ELEMENT" => GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType :: VisualElement , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1DocumentPageAnchorPageRefLayoutType
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DocumentPageBlock {
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageDetectedLanguage>,
        >,
        #[doc = "Layout for Block."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentPageLayout>,
        #[doc = "The history of this annotation."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentProvenance>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentPageBlock {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentPageBlock {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DocumentPageDetectedLanguage {
        #[doc = "Confidence of detected language. Range [0, 1]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see https://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1DocumentPageDetectedLanguage
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentPageDetectedLanguage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DocumentPageDimension {
        #[doc = "Page height."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<f32>,
        #[doc = "Dimension unit."]
        #[serde(
            rename = "unit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit: ::std::option::Option<String>,
        #[doc = "Page width."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentPageDimension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentPageDimension {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DocumentPageFormField {
        #[doc = "Created for Labeling UI to export key text. If corrections were made to the text identified by the `field_name.text_anchor`, this field will contain the correction."]
        #[serde(
            rename = "correctedKeyText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub corrected_key_text: ::std::option::Option<String>,
        #[doc = "Created for Labeling UI to export value text. If corrections were made to the text identified by the `field_value.text_anchor`, this field will contain the correction."]
        #[serde(
            rename = "correctedValueText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub corrected_value_text: ::std::option::Option<String>,
        #[doc = "Layout for the FormField name. e.g. `Address`, `Email`, `Grand total`, `Phone number`, etc."]
        #[serde(
            rename = "fieldName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field_name:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentPageLayout>,
        #[doc = "Layout for the FormField value."]
        #[serde(
            rename = "fieldValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field_value:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentPageLayout>,
        #[doc = "A list of detected languages for name together with confidence."]
        #[serde(
            rename = "nameDetectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name_detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageDetectedLanguage>,
        >,
        #[doc = "The history of this annotation."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentProvenance>,
        #[doc = "A list of detected languages for value together with confidence."]
        #[serde(
            rename = "valueDetectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageDetectedLanguage>,
        >,
        #[doc = "If the value is non-textual, this field represents the type. Current valid values are: - blank (this indicates the field_value is normal text) - \"unfilled_checkbox\" - \"filled_checkbox\""]
        #[serde(
            rename = "valueType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentPageFormField {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentPageFormField {
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
    pub struct GoogleCloudDocumentaiV1DocumentPageImage {
        #[doc = "Raw byte content of the image."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Height of the image in pixels."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[doc = "Encoding mime type for the image."]
        #[serde(
            rename = "mimeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime_type: ::std::option::Option<String>,
        #[doc = "Width of the image in pixels."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentPageImage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentPageImage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DocumentPageLayout {
        #[doc = "The bounding polygon for the Layout."]
        #[serde(
            rename = "boundingPoly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1BoundingPoly>,
        #[doc = "Confidence of the current Layout within context of the object this layout is for. e.g. confidence can be for a single token, a table, a visual element, etc. depending on context. Range [0, 1]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Detected orientation for the Layout."]
        #[serde(
            rename = "orientation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub orientation: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1DocumentPageLayoutOrientation,
        >,
        #[doc = "Text anchor indexing into the Document.text."]
        #[serde(
            rename = "textAnchor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_anchor:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentTextAnchor>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentPageLayout {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentPageLayout {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1DocumentPageLayoutOrientation {
        #[doc = "Unspecified orientation."]
        OrientationUnspecified,
        #[doc = "Orientation is aligned with page down. Turn the head 180 degrees from upright to read."]
        PageDown,
        #[doc = "Orientation is aligned with page left. Turn the head 90 degrees counterclockwise from upright to read."]
        PageLeft,
        #[doc = "Orientation is aligned with page right. Turn the head 90 degrees clockwise from upright to read."]
        PageRight,
        #[doc = "Orientation is aligned with page up."]
        PageUp,
    }
    impl GoogleCloudDocumentaiV1DocumentPageLayoutOrientation {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDocumentaiV1DocumentPageLayoutOrientation::OrientationUnspecified => {
                    "ORIENTATION_UNSPECIFIED"
                }
                GoogleCloudDocumentaiV1DocumentPageLayoutOrientation::PageDown => "PAGE_DOWN",
                GoogleCloudDocumentaiV1DocumentPageLayoutOrientation::PageLeft => "PAGE_LEFT",
                GoogleCloudDocumentaiV1DocumentPageLayoutOrientation::PageRight => "PAGE_RIGHT",
                GoogleCloudDocumentaiV1DocumentPageLayoutOrientation::PageUp => "PAGE_UP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiV1DocumentPageLayoutOrientation {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1DocumentPageLayoutOrientation {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiV1DocumentPageLayoutOrientation, ()>
        {
            Ok(match s {
                "ORIENTATION_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1DocumentPageLayoutOrientation::OrientationUnspecified
                }
                "PAGE_DOWN" => GoogleCloudDocumentaiV1DocumentPageLayoutOrientation::PageDown,
                "PAGE_LEFT" => GoogleCloudDocumentaiV1DocumentPageLayoutOrientation::PageLeft,
                "PAGE_RIGHT" => GoogleCloudDocumentaiV1DocumentPageLayoutOrientation::PageRight,
                "PAGE_UP" => GoogleCloudDocumentaiV1DocumentPageLayoutOrientation::PageUp,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1DocumentPageLayoutOrientation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1DocumentPageLayoutOrientation {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDocumentaiV1DocumentPageLayoutOrientation {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ORIENTATION_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1DocumentPageLayoutOrientation::OrientationUnspecified
                }
                "PAGE_DOWN" => GoogleCloudDocumentaiV1DocumentPageLayoutOrientation::PageDown,
                "PAGE_LEFT" => GoogleCloudDocumentaiV1DocumentPageLayoutOrientation::PageLeft,
                "PAGE_RIGHT" => GoogleCloudDocumentaiV1DocumentPageLayoutOrientation::PageRight,
                "PAGE_UP" => GoogleCloudDocumentaiV1DocumentPageLayoutOrientation::PageUp,
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
        for GoogleCloudDocumentaiV1DocumentPageLayoutOrientation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentPageLayoutOrientation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DocumentPageLine {
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageDetectedLanguage>,
        >,
        #[doc = "Layout for Line."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentPageLayout>,
        #[doc = "The history of this annotation."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentProvenance>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentPageLine {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentPageLine {
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
    pub struct GoogleCloudDocumentaiV1DocumentPageMatrix {
        #[doc = "Number of columns in the matrix."]
        #[serde(
            rename = "cols",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cols: ::std::option::Option<i32>,
        #[doc = "The matrix data."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "This encodes information about what data type the matrix uses. For example, 0 (CV_8U) is an unsigned 8-bit image. For the full list of OpenCV primitive data types, please refer to https://docs.opencv.org/4.3.0/d1/d1b/group__core__hal__interface.html"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<i32>,
        #[doc = "Number of rows in the matrix."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentPageMatrix {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentPageMatrix {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DocumentPageParagraph {
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageDetectedLanguage>,
        >,
        #[doc = "Layout for Paragraph."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentPageLayout>,
        #[doc = "The history of this annotation."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentProvenance>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentPageParagraph {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentPageParagraph {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DocumentPageSymbol {
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageDetectedLanguage>,
        >,
        #[doc = "Layout for Symbol."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentPageLayout>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentPageSymbol {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentPageSymbol {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DocumentPageTable {
        #[doc = "Body rows of the table."]
        #[serde(
            rename = "bodyRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub body_rows: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageTableTableRow>,
        >,
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageDetectedLanguage>,
        >,
        #[doc = "Header rows of the table."]
        #[serde(
            rename = "headerRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header_rows: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageTableTableRow>,
        >,
        #[doc = "Layout for Table."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentPageLayout>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentPageTable {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentPageTable {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DocumentPageTableTableCell {
        #[doc = "How many columns this cell spans."]
        #[serde(
            rename = "colSpan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub col_span: ::std::option::Option<i32>,
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageDetectedLanguage>,
        >,
        #[doc = "Layout for TableCell."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentPageLayout>,
        #[doc = "How many rows this cell spans."]
        #[serde(
            rename = "rowSpan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_span: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentPageTableTableCell {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentPageTableTableCell {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DocumentPageTableTableRow {
        #[doc = "Cells that make up this row."]
        #[serde(
            rename = "cells",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cells: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageTableTableCell>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentPageTableTableRow {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentPageTableTableRow {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DocumentPageToken {
        #[doc = "Detected break at the end of a Token."]
        #[serde(
            rename = "detectedBreak",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_break: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreak,
        >,
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageDetectedLanguage>,
        >,
        #[doc = "Layout for Token."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentPageLayout>,
        #[doc = "The history of this annotation."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentProvenance>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentPageToken {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentPageToken {
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
    pub struct GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreak {
        #[doc = "Detected break type."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakType,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreak
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreak
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakType {
        #[doc = "A hyphen that indicates that a token has been split across lines."]
        Hyphen,
        #[doc = "A single whitespace."]
        Space,
        #[doc = "Unspecified break type."]
        TypeUnspecified,
        #[doc = "A wider whitespace."]
        WideSpace,
    }
    impl GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakType::Hyphen => "HYPHEN",
                GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakType::Space => "SPACE",
                GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakType::TypeUnspecified => {
                    "TYPE_UNSPECIFIED"
                }
                GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakType::WideSpace => {
                    "WIDE_SPACE"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakType, ()>
        {
            Ok(match s {
                "HYPHEN" => GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakType::Hyphen,
                "SPACE" => GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakType::Space,
                "TYPE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakType::TypeUnspecified
                }
                "WIDE_SPACE" => {
                    GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakType::WideSpace
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "HYPHEN" => GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakType::Hyphen,
                "SPACE" => GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakType::Space,
                "TYPE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakType::TypeUnspecified
                }
                "WIDE_SPACE" => {
                    GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakType::WideSpace
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
        for GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1DocumentPageTokenDetectedBreakType
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DocumentPageVisualElement {
        #[doc = "A list of detected languages together with confidence."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentPageDetectedLanguage>,
        >,
        #[doc = "Layout for VisualElement."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentPageLayout>,
        #[doc = "Type of the VisualElement."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentPageVisualElement {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentPageVisualElement {
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
    pub struct GoogleCloudDocumentaiV1DocumentProvenance {
        #[doc = "The Id of this operation. Needs to be unique within the scope of the revision."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<i32>,
        #[doc = "References to the original elements that are replaced."]
        #[serde(
            rename = "parents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parents: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentProvenanceParent>,
        >,
        #[doc = "The type of provenance operation."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentProvenanceType>,
        #[doc = "The index of the revision that produced this element."]
        #[serde(
            rename = "revision",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentProvenance {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentProvenance {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1DocumentProvenanceType {
        #[doc = "Add an element."]
        Add,
        #[doc = "Element is reviewed and approved at human review, confidence will be set to 1.0."]
        EvalApproved,
        #[doc = "Request human review for the element identified by `parent`."]
        EvalRequested,
        #[doc = "Element is skipped in the validation process."]
        EvalSkipped,
        #[doc = "Operation type unspecified. If no operation is specified a provenance entry is simply used to match against a `parent`."]
        OperationTypeUnspecified,
        #[doc = "Remove an element identified by `parent`."]
        Remove,
        #[doc = "Replace an element identified by `parent`."]
        Replace,
    }
    impl GoogleCloudDocumentaiV1DocumentProvenanceType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDocumentaiV1DocumentProvenanceType::Add => "ADD",
                GoogleCloudDocumentaiV1DocumentProvenanceType::EvalApproved => "EVAL_APPROVED",
                GoogleCloudDocumentaiV1DocumentProvenanceType::EvalRequested => "EVAL_REQUESTED",
                GoogleCloudDocumentaiV1DocumentProvenanceType::EvalSkipped => "EVAL_SKIPPED",
                GoogleCloudDocumentaiV1DocumentProvenanceType::OperationTypeUnspecified => {
                    "OPERATION_TYPE_UNSPECIFIED"
                }
                GoogleCloudDocumentaiV1DocumentProvenanceType::Remove => "REMOVE",
                GoogleCloudDocumentaiV1DocumentProvenanceType::Replace => "REPLACE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiV1DocumentProvenanceType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1DocumentProvenanceType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiV1DocumentProvenanceType, ()> {
            Ok(match s {
                "ADD" => GoogleCloudDocumentaiV1DocumentProvenanceType::Add,
                "EVAL_APPROVED" => GoogleCloudDocumentaiV1DocumentProvenanceType::EvalApproved,
                "EVAL_REQUESTED" => GoogleCloudDocumentaiV1DocumentProvenanceType::EvalRequested,
                "EVAL_SKIPPED" => GoogleCloudDocumentaiV1DocumentProvenanceType::EvalSkipped,
                "OPERATION_TYPE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1DocumentProvenanceType::OperationTypeUnspecified
                }
                "REMOVE" => GoogleCloudDocumentaiV1DocumentProvenanceType::Remove,
                "REPLACE" => GoogleCloudDocumentaiV1DocumentProvenanceType::Replace,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1DocumentProvenanceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1DocumentProvenanceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDocumentaiV1DocumentProvenanceType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADD" => GoogleCloudDocumentaiV1DocumentProvenanceType::Add,
                "EVAL_APPROVED" => GoogleCloudDocumentaiV1DocumentProvenanceType::EvalApproved,
                "EVAL_REQUESTED" => GoogleCloudDocumentaiV1DocumentProvenanceType::EvalRequested,
                "EVAL_SKIPPED" => GoogleCloudDocumentaiV1DocumentProvenanceType::EvalSkipped,
                "OPERATION_TYPE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1DocumentProvenanceType::OperationTypeUnspecified
                }
                "REMOVE" => GoogleCloudDocumentaiV1DocumentProvenanceType::Remove,
                "REPLACE" => GoogleCloudDocumentaiV1DocumentProvenanceType::Replace,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentProvenanceType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentProvenanceType {
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
    pub struct GoogleCloudDocumentaiV1DocumentProvenanceParent {
        #[doc = "The id of the parent provenance."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<i32>,
        #[doc = "The index of the parent item in the corresponding item list (eg. list of entities, properties within entities, etc.) in the parent revision."]
        #[serde(
            rename = "index",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub index: ::std::option::Option<i32>,
        #[doc = "The index of the index into current revision's parent_ids list."]
        #[serde(
            rename = "revision",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentProvenanceParent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentProvenanceParent {
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
    pub struct GoogleCloudDocumentaiV1DocumentRevision {
        #[doc = "If the change was made by a person specify the name or id of that person."]
        #[serde(
            rename = "agent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub agent: ::std::option::Option<String>,
        #[doc = "The time that the revision was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Human Review information of this revision."]
        #[serde(
            rename = "humanReview",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub human_review: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1DocumentRevisionHumanReview,
        >,
        #[doc = "Id of the revision. Unique within the context of the document."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The revisions that this revision is based on. This can include one or more parent (when documents are merged.) This field represents the index into the `revisions` field."]
        #[serde(
            rename = "parent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent: ::std::option::Option<Vec<i32>>,
        #[doc = "The revisions that this revision is based on. Must include all the ids that have anything to do with this revision - eg. there are `provenance.parent.revision` fields that index into this field."]
        #[serde(
            rename = "parentIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent_ids: ::std::option::Option<Vec<String>>,
        #[doc = "If the annotation was made by processor identify the processor by its resource name."]
        #[serde(
            rename = "processor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub processor: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentRevision {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentRevision {
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
    pub struct GoogleCloudDocumentaiV1DocumentRevisionHumanReview {
        #[doc = "Human review state. e.g. `requested`, `succeeded`, `rejected`."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<String>,
        #[doc = "A message providing more details about the current state of processing. For example, the rejection reason when the state is `rejected`."]
        #[serde(
            rename = "stateMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state_message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentRevisionHumanReview {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentRevisionHumanReview {
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
    pub struct GoogleCloudDocumentaiV1DocumentShardInfo {
        #[doc = "Total number of shards."]
        #[serde(
            rename = "shardCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub shard_count: ::std::option::Option<i64>,
        #[doc = "The 0-based index of this shard."]
        #[serde(
            rename = "shardIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub shard_index: ::std::option::Option<i64>,
        #[doc = "The index of the first character in Document.text in the overall document global text."]
        #[serde(
            rename = "textOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub text_offset: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentShardInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentShardInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DocumentStyle {
        #[doc = "Text background color."]
        #[serde(
            rename = "backgroundColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub background_color: ::std::option::Option<crate::schemas::GoogleTypeColor>,
        #[doc = "Text color."]
        #[serde(
            rename = "color",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color: ::std::option::Option<crate::schemas::GoogleTypeColor>,
        #[doc = "Font size."]
        #[serde(
            rename = "fontSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub font_size:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentStyleFontSize>,
        #[doc = "Font weight. Possible values are normal, bold, bolder, and lighter. https://www.w3schools.com/cssref/pr_font_weight.asp"]
        #[serde(
            rename = "fontWeight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub font_weight: ::std::option::Option<String>,
        #[doc = "Text anchor indexing into the Document.text."]
        #[serde(
            rename = "textAnchor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_anchor:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentTextAnchor>,
        #[doc = "Text decoration. Follows CSS standard. https://www.w3schools.com/cssref/pr_text_text-decoration.asp"]
        #[serde(
            rename = "textDecoration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_decoration: ::std::option::Option<String>,
        #[doc = "Text style. Possible values are normal, italic, and oblique. https://www.w3schools.com/cssref/pr_font_font-style.asp"]
        #[serde(
            rename = "textStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_style: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1DocumentStyleFontSize {
        #[doc = "Font size for the text."]
        #[serde(
            rename = "size",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub size: ::std::option::Option<f32>,
        #[doc = "Unit for the font size. Follows CSS naming (in, px, pt, etc.)."]
        #[serde(
            rename = "unit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentStyleFontSize {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentStyleFontSize {
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
    pub struct GoogleCloudDocumentaiV1DocumentTextAnchor {
        #[doc = "Contains the content of the text span so that users do not have to look it up in the text_segments. It is always populated for formFields."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<String>,
        #[doc = "The text segments from the Document.text."]
        #[serde(
            rename = "textSegments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_segments: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentTextAnchorTextSegment>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentTextAnchor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentTextAnchor {
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
    pub struct GoogleCloudDocumentaiV1DocumentTextAnchorTextSegment {
        #[doc = "TextSegment half open end UTF-8 char index in the Document.text."]
        #[serde(
            rename = "endIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub end_index: ::std::option::Option<i64>,
        #[doc = "TextSegment start UTF-8 char index in the Document.text."]
        #[serde(
            rename = "startIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub start_index: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1DocumentTextAnchorTextSegment
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentTextAnchorTextSegment {
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
    pub struct GoogleCloudDocumentaiV1DocumentTextChange {
        #[doc = "The text that replaces the text identified in the `text_anchor`."]
        #[serde(
            rename = "changedText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub changed_text: ::std::option::Option<String>,
        #[doc = "The history of this annotation."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1DocumentProvenance>>,
        #[doc = "Provenance of the correction. Text anchor indexing into the Document.text. There can only be a single `TextAnchor.text_segments` element. If the start and end index of the text segment are the same, the text change is inserted before that index."]
        #[serde(
            rename = "textAnchor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_anchor:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1DocumentTextAnchor>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1DocumentTextChange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1DocumentTextChange {
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
    pub struct GoogleCloudDocumentaiV1EnableProcessorMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1CommonOperationMetadata>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1EnableProcessorMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1EnableProcessorMetadata {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1EnableProcessorRequest {}
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1EnableProcessorRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1EnableProcessorRequest {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1EnableProcessorResponse {}
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1EnableProcessorResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1EnableProcessorResponse {
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
    pub struct GoogleCloudDocumentaiV1FetchProcessorTypesResponse {
        #[doc = "The list of processor types."]
        #[serde(
            rename = "processorTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub processor_types:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1ProcessorType>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1FetchProcessorTypesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1FetchProcessorTypesResponse {
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
    pub struct GoogleCloudDocumentaiV1GcsDocument {
        #[doc = "The Cloud Storage object uri."]
        #[serde(
            rename = "gcsUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_uri: ::std::option::Option<String>,
        #[doc = "An IANA MIME type (RFC6838) of the content."]
        #[serde(
            rename = "mimeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1GcsDocument {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1GcsDocument {
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
    pub struct GoogleCloudDocumentaiV1GcsDocuments {
        #[doc = "The list of documents."]
        #[serde(
            rename = "documents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub documents:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1GcsDocument>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1GcsDocuments {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1GcsDocuments {
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
    pub struct GoogleCloudDocumentaiV1GcsPrefix {
        #[doc = "The URI prefix."]
        #[serde(
            rename = "gcsUriPrefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_uri_prefix: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1GcsPrefix {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1GcsPrefix {
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
    pub struct GoogleCloudDocumentaiV1HumanReviewStatus {
        #[doc = "The name of the operation triggered by the processed document. This field is populated only when the [state] is [HUMAN_REVIEW_IN_PROGRESS]. It has the same response type and metadata as the long running operation returned by [ReviewDocument] method."]
        #[serde(
            rename = "humanReviewOperation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub human_review_operation: ::std::option::Option<String>,
        #[doc = "The state of human review on the processing request."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1HumanReviewStatusState>,
        #[doc = "A message providing more details about the human review state."]
        #[serde(
            rename = "stateMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state_message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1HumanReviewStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1HumanReviewStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1HumanReviewStatusState {
        #[doc = "Some error happened during triggering human review, see the [state_message] for details."]
        Error,
        #[doc = "Human review validation is triggered and the document is under review."]
        InProgress,
        #[doc = "Human review is skipped for the document. This can happen because human review is not enabled on the processor or the processing request has been set to skip this document."]
        Skipped,
        #[doc = "Human review state is unspecified. Most likely due to an internal error."]
        StateUnspecified,
        #[doc = "Human review validation is triggered and passed, so no review is needed."]
        ValidationPassed,
    }
    impl GoogleCloudDocumentaiV1HumanReviewStatusState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDocumentaiV1HumanReviewStatusState::Error => "ERROR",
                GoogleCloudDocumentaiV1HumanReviewStatusState::InProgress => "IN_PROGRESS",
                GoogleCloudDocumentaiV1HumanReviewStatusState::Skipped => "SKIPPED",
                GoogleCloudDocumentaiV1HumanReviewStatusState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
                GoogleCloudDocumentaiV1HumanReviewStatusState::ValidationPassed => {
                    "VALIDATION_PASSED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiV1HumanReviewStatusState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1HumanReviewStatusState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiV1HumanReviewStatusState, ()> {
            Ok(match s {
                "ERROR" => GoogleCloudDocumentaiV1HumanReviewStatusState::Error,
                "IN_PROGRESS" => GoogleCloudDocumentaiV1HumanReviewStatusState::InProgress,
                "SKIPPED" => GoogleCloudDocumentaiV1HumanReviewStatusState::Skipped,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1HumanReviewStatusState::StateUnspecified
                }
                "VALIDATION_PASSED" => {
                    GoogleCloudDocumentaiV1HumanReviewStatusState::ValidationPassed
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1HumanReviewStatusState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1HumanReviewStatusState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDocumentaiV1HumanReviewStatusState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ERROR" => GoogleCloudDocumentaiV1HumanReviewStatusState::Error,
                "IN_PROGRESS" => GoogleCloudDocumentaiV1HumanReviewStatusState::InProgress,
                "SKIPPED" => GoogleCloudDocumentaiV1HumanReviewStatusState::Skipped,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1HumanReviewStatusState::StateUnspecified
                }
                "VALIDATION_PASSED" => {
                    GoogleCloudDocumentaiV1HumanReviewStatusState::ValidationPassed
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
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1HumanReviewStatusState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1HumanReviewStatusState {
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
    pub struct GoogleCloudDocumentaiV1ListProcessorVersionsResponse {
        #[doc = "Points to the next processor, otherwise empty."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The list of processors."]
        #[serde(
            rename = "processorVersions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub processor_versions:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1ProcessorVersion>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1ListProcessorVersionsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1ListProcessorVersionsResponse {
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
    pub struct GoogleCloudDocumentaiV1ListProcessorsResponse {
        #[doc = "Points to the next processor, otherwise empty."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The list of processors."]
        #[serde(
            rename = "processors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub processors:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDocumentaiV1Processor>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1ListProcessorsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1ListProcessorsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1NormalizedVertex {
        #[doc = "X coordinate."]
        #[serde(
            rename = "x",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub x: ::std::option::Option<f32>,
        #[doc = "Y coordinate (starts from the top of the image)."]
        #[serde(
            rename = "y",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub y: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1NormalizedVertex {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1NormalizedVertex {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDocumentaiV1ProcessRequest {
        #[doc = "An inline document proto."]
        #[serde(
            rename = "inlineDocument",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inline_document: ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Document>,
        #[doc = "A raw document content (bytes)."]
        #[serde(
            rename = "rawDocument",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub raw_document: ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1RawDocument>,
        #[doc = "Whether Human Review feature should be skipped for this request. Default to false."]
        #[serde(
            rename = "skipHumanReview",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub skip_human_review: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1ProcessRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1ProcessRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDocumentaiV1ProcessResponse {
        #[doc = "The document payload, will populate fields based on the processor's behavior."]
        #[serde(
            rename = "document",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub document: ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Document>,
        #[doc = "The status of human review on the processed document."]
        #[serde(
            rename = "humanReviewStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub human_review_status:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1HumanReviewStatus>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1ProcessResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1ProcessResponse {
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
    pub struct GoogleCloudDocumentaiV1Processor {
        #[doc = "The time the processor was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The default processor version."]
        #[serde(
            rename = "defaultProcessorVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_processor_version: ::std::option::Option<String>,
        #[doc = "The display name of the processor."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The KMS key used for encryption/decryption in CMEK scenarios. See https://cloud.google.com/security-key-management."]
        #[serde(
            rename = "kmsKeyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kms_key_name: ::std::option::Option<String>,
        #[doc = "Output only. Immutable. The resource name of the processor. Format: `projects/{project}/locations/{location}/processors/{processor}`"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. Immutable. The http endpoint that can be called to invoke processing."]
        #[serde(
            rename = "processEndpoint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub process_endpoint: ::std::option::Option<String>,
        #[doc = "The processor type, e.g., OCR_PROCESSOR, INVOICE_PROCESSOR, etc. To get a list of processors types, see FetchProcessorTypes."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "Output only. The state of the processor."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1ProcessorState>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Processor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Processor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1ProcessorState {
        #[doc = "The processor is being created, will become either ENABLED (for successful creation) or FAILED (for failed ones). Once a processor is in this state, it can then be used for document processing, but the feature dependencies of the processor might not be fully created yet."]
        Creating,
        #[doc = "The processor is being deleted, will be removed if successful."]
        Deleting,
        #[doc = "The processor is disabled."]
        Disabled,
        #[doc = "The processor is being disabled, will become DISABLED if successful."]
        Disabling,
        #[doc = "The processor is enabled, i.e., has an enabled version which can currently serve processing requests and all the feature dependencies have been successfully initialized."]
        Enabled,
        #[doc = "The processor is being enabled, will become ENABLED if successful."]
        Enabling,
        #[doc = "The processor failed during creation or initialization of feature dependencies. The user should delete the processor and recreate one as all the functionalities of the processor are disabled."]
        Failed,
        #[doc = "The processor is in an unspecified state."]
        StateUnspecified,
    }
    impl GoogleCloudDocumentaiV1ProcessorState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDocumentaiV1ProcessorState::Creating => "CREATING",
                GoogleCloudDocumentaiV1ProcessorState::Deleting => "DELETING",
                GoogleCloudDocumentaiV1ProcessorState::Disabled => "DISABLED",
                GoogleCloudDocumentaiV1ProcessorState::Disabling => "DISABLING",
                GoogleCloudDocumentaiV1ProcessorState::Enabled => "ENABLED",
                GoogleCloudDocumentaiV1ProcessorState::Enabling => "ENABLING",
                GoogleCloudDocumentaiV1ProcessorState::Failed => "FAILED",
                GoogleCloudDocumentaiV1ProcessorState::StateUnspecified => "STATE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiV1ProcessorState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1ProcessorState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleCloudDocumentaiV1ProcessorState, ()> {
            Ok(match s {
                "CREATING" => GoogleCloudDocumentaiV1ProcessorState::Creating,
                "DELETING" => GoogleCloudDocumentaiV1ProcessorState::Deleting,
                "DISABLED" => GoogleCloudDocumentaiV1ProcessorState::Disabled,
                "DISABLING" => GoogleCloudDocumentaiV1ProcessorState::Disabling,
                "ENABLED" => GoogleCloudDocumentaiV1ProcessorState::Enabled,
                "ENABLING" => GoogleCloudDocumentaiV1ProcessorState::Enabling,
                "FAILED" => GoogleCloudDocumentaiV1ProcessorState::Failed,
                "STATE_UNSPECIFIED" => GoogleCloudDocumentaiV1ProcessorState::StateUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1ProcessorState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1ProcessorState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDocumentaiV1ProcessorState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATING" => GoogleCloudDocumentaiV1ProcessorState::Creating,
                "DELETING" => GoogleCloudDocumentaiV1ProcessorState::Deleting,
                "DISABLED" => GoogleCloudDocumentaiV1ProcessorState::Disabled,
                "DISABLING" => GoogleCloudDocumentaiV1ProcessorState::Disabling,
                "ENABLED" => GoogleCloudDocumentaiV1ProcessorState::Enabled,
                "ENABLING" => GoogleCloudDocumentaiV1ProcessorState::Enabling,
                "FAILED" => GoogleCloudDocumentaiV1ProcessorState::Failed,
                "STATE_UNSPECIFIED" => GoogleCloudDocumentaiV1ProcessorState::StateUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1ProcessorState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1ProcessorState {
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
    pub struct GoogleCloudDocumentaiV1ProcessorType {
        #[doc = "Whether the processor type allows creation. If true, users can create a processor of this processor type. Otherwise, users need to request access."]
        #[serde(
            rename = "allowCreation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_creation: ::std::option::Option<bool>,
        #[doc = "The locations in which this processor is available."]
        #[serde(
            rename = "availableLocations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub available_locations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDocumentaiV1ProcessorTypeLocationInfo>,
        >,
        #[doc = "The processor category, used by UI to group processor types."]
        #[serde(
            rename = "category",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category: ::std::option::Option<String>,
        #[doc = "Launch stage of the processor type"]
        #[serde(
            rename = "launchStage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub launch_stage:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1ProcessorTypeLaunchStage>,
        #[doc = "The resource name of the processor type. Format: projects/{project}/processorTypes/{processor_type}"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The type of the processor, e.g., \"invoice_parsing\"."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1ProcessorType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1ProcessorType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1ProcessorTypeLaunchStage {
        #[doc = "Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don't have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases."]
        Alpha,
        #[doc = "Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases."]
        Beta,
        #[doc = "Deprecated features are scheduled to be shut down and removed. For more information, see the \"Deprecation Policy\" section of our [Terms of Service](https://cloud.google.com/terms/) and the [Google Cloud Platform Subject to the Deprecation Policy](https://cloud.google.com/terms/deprecation) documentation."]
        Deprecated,
        #[doc = "Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released."]
        EarlyAccess,
        #[doc = "GA features are open to all developers and are considered stable and fully qualified for production use."]
        Ga,
        #[doc = "Do not use this default value."]
        LaunchStageUnspecified,
        #[doc = "Prelaunch features are hidden from users and are only visible internally."]
        Prelaunch,
        #[doc = "The feature is not yet implemented. Users can not use it."]
        Unimplemented,
    }
    impl GoogleCloudDocumentaiV1ProcessorTypeLaunchStage {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::Alpha => "ALPHA",
                GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::Beta => "BETA",
                GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::Deprecated => "DEPRECATED",
                GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::EarlyAccess => "EARLY_ACCESS",
                GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::Ga => "GA",
                GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::LaunchStageUnspecified => {
                    "LAUNCH_STAGE_UNSPECIFIED"
                }
                GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::Prelaunch => "PRELAUNCH",
                GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::Unimplemented => "UNIMPLEMENTED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiV1ProcessorTypeLaunchStage {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1ProcessorTypeLaunchStage {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiV1ProcessorTypeLaunchStage, ()> {
            Ok(match s {
                "ALPHA" => GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::Alpha,
                "BETA" => GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::Beta,
                "DEPRECATED" => GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::Deprecated,
                "EARLY_ACCESS" => GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::EarlyAccess,
                "GA" => GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::Ga,
                "LAUNCH_STAGE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::LaunchStageUnspecified
                }
                "PRELAUNCH" => GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::Prelaunch,
                "UNIMPLEMENTED" => GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::Unimplemented,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1ProcessorTypeLaunchStage {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1ProcessorTypeLaunchStage {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDocumentaiV1ProcessorTypeLaunchStage {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALPHA" => GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::Alpha,
                "BETA" => GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::Beta,
                "DEPRECATED" => GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::Deprecated,
                "EARLY_ACCESS" => GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::EarlyAccess,
                "GA" => GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::Ga,
                "LAUNCH_STAGE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::LaunchStageUnspecified
                }
                "PRELAUNCH" => GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::Prelaunch,
                "UNIMPLEMENTED" => GoogleCloudDocumentaiV1ProcessorTypeLaunchStage::Unimplemented,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1ProcessorTypeLaunchStage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1ProcessorTypeLaunchStage {
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
    pub struct GoogleCloudDocumentaiV1ProcessorTypeLocationInfo {
        #[doc = "The location id, currently must be one of [us, eu]."]
        #[serde(
            rename = "locationId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1ProcessorTypeLocationInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1ProcessorTypeLocationInfo {
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
    pub struct GoogleCloudDocumentaiV1ProcessorVersion {
        #[doc = "The time the processor version was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "If set, information about the eventual deprecation of this version."]
        #[serde(
            rename = "deprecationInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deprecation_info: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1ProcessorVersionDeprecationInfo,
        >,
        #[doc = "The display name of the processor version."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Denotes that this ProcessorVersion is managed by google."]
        #[serde(
            rename = "googleManaged",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_managed: ::std::option::Option<bool>,
        #[doc = "The KMS key name used for encryption."]
        #[serde(
            rename = "kmsKeyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kms_key_name: ::std::option::Option<String>,
        #[doc = "The KMS key version with which data is encrypted."]
        #[serde(
            rename = "kmsKeyVersionName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kms_key_version_name: ::std::option::Option<String>,
        #[doc = "The resource name of the processor version. Format: `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processor_version}`"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The state of the processor version."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1ProcessorVersionState>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1ProcessorVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1ProcessorVersion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1ProcessorVersionState {
        #[doc = "The processor version is being created."]
        Creating,
        #[doc = "The processor version is being deleted."]
        Deleting,
        #[doc = "The processor version is deployed and can be used for processing."]
        Deployed,
        #[doc = "The processor version is being deployed."]
        Deploying,
        #[doc = "The processor version failed and is in an indeterminate state."]
        Failed,
        #[doc = "The processor version is in an unspecified state."]
        StateUnspecified,
        #[doc = "The processor version is not deployed and cannot be used for processing."]
        Undeployed,
        #[doc = "The processor version is being undeployed."]
        Undeploying,
    }
    impl GoogleCloudDocumentaiV1ProcessorVersionState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDocumentaiV1ProcessorVersionState::Creating => "CREATING",
                GoogleCloudDocumentaiV1ProcessorVersionState::Deleting => "DELETING",
                GoogleCloudDocumentaiV1ProcessorVersionState::Deployed => "DEPLOYED",
                GoogleCloudDocumentaiV1ProcessorVersionState::Deploying => "DEPLOYING",
                GoogleCloudDocumentaiV1ProcessorVersionState::Failed => "FAILED",
                GoogleCloudDocumentaiV1ProcessorVersionState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
                GoogleCloudDocumentaiV1ProcessorVersionState::Undeployed => "UNDEPLOYED",
                GoogleCloudDocumentaiV1ProcessorVersionState::Undeploying => "UNDEPLOYING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiV1ProcessorVersionState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1ProcessorVersionState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiV1ProcessorVersionState, ()> {
            Ok(match s {
                "CREATING" => GoogleCloudDocumentaiV1ProcessorVersionState::Creating,
                "DELETING" => GoogleCloudDocumentaiV1ProcessorVersionState::Deleting,
                "DEPLOYED" => GoogleCloudDocumentaiV1ProcessorVersionState::Deployed,
                "DEPLOYING" => GoogleCloudDocumentaiV1ProcessorVersionState::Deploying,
                "FAILED" => GoogleCloudDocumentaiV1ProcessorVersionState::Failed,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1ProcessorVersionState::StateUnspecified
                }
                "UNDEPLOYED" => GoogleCloudDocumentaiV1ProcessorVersionState::Undeployed,
                "UNDEPLOYING" => GoogleCloudDocumentaiV1ProcessorVersionState::Undeploying,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1ProcessorVersionState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1ProcessorVersionState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDocumentaiV1ProcessorVersionState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATING" => GoogleCloudDocumentaiV1ProcessorVersionState::Creating,
                "DELETING" => GoogleCloudDocumentaiV1ProcessorVersionState::Deleting,
                "DEPLOYED" => GoogleCloudDocumentaiV1ProcessorVersionState::Deployed,
                "DEPLOYING" => GoogleCloudDocumentaiV1ProcessorVersionState::Deploying,
                "FAILED" => GoogleCloudDocumentaiV1ProcessorVersionState::Failed,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDocumentaiV1ProcessorVersionState::StateUnspecified
                }
                "UNDEPLOYED" => GoogleCloudDocumentaiV1ProcessorVersionState::Undeployed,
                "UNDEPLOYING" => GoogleCloudDocumentaiV1ProcessorVersionState::Undeploying,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1ProcessorVersionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1ProcessorVersionState {
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
    pub struct GoogleCloudDocumentaiV1ProcessorVersionDeprecationInfo {
        #[doc = "The time at which this processor version will be deprecated."]
        #[serde(
            rename = "deprecationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deprecation_time: ::std::option::Option<String>,
        #[doc = "If set, the processor version that will be used as a replacement."]
        #[serde(
            rename = "replacementProcessorVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replacement_processor_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1ProcessorVersionDeprecationInfo
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1ProcessorVersionDeprecationInfo
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
    pub struct GoogleCloudDocumentaiV1RawDocument {
        #[doc = "Inline document content."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "An IANA MIME type (RFC6838) indicating the nature and format of the [content]."]
        #[serde(
            rename = "mimeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1RawDocument {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1RawDocument {
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
    pub struct GoogleCloudDocumentaiV1ReviewDocumentOperationMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1CommonOperationMetadata>,
        #[doc = "The Crowd Compute question ID."]
        #[serde(
            rename = "questionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub question_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1ReviewDocumentOperationMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1ReviewDocumentOperationMetadata
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDocumentaiV1ReviewDocumentRequest {
        #[doc = "Whether the validation should be performed on the ad-hoc review request."]
        #[serde(
            rename = "enableSchemaValidation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_schema_validation: ::std::option::Option<bool>,
        #[doc = "An inline document proto."]
        #[serde(
            rename = "inlineDocument",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inline_document: ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1Document>,
        #[doc = "The priority of the human review task."]
        #[serde(
            rename = "priority",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub priority: ::std::option::Option<
            crate::schemas::GoogleCloudDocumentaiV1ReviewDocumentRequestPriority,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1ReviewDocumentRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1ReviewDocumentRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDocumentaiV1ReviewDocumentRequestPriority {
        #[doc = "The default priority level."]
        Default,
        #[doc = "The urgent priority level. The labeling manager should allocate labeler resource to the urgent task queue to respect this priority level."]
        Urgent,
    }
    impl GoogleCloudDocumentaiV1ReviewDocumentRequestPriority {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDocumentaiV1ReviewDocumentRequestPriority::Default => "DEFAULT",
                GoogleCloudDocumentaiV1ReviewDocumentRequestPriority::Urgent => "URGENT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudDocumentaiV1ReviewDocumentRequestPriority {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudDocumentaiV1ReviewDocumentRequestPriority {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudDocumentaiV1ReviewDocumentRequestPriority, ()>
        {
            Ok(match s {
                "DEFAULT" => GoogleCloudDocumentaiV1ReviewDocumentRequestPriority::Default,
                "URGENT" => GoogleCloudDocumentaiV1ReviewDocumentRequestPriority::Urgent,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudDocumentaiV1ReviewDocumentRequestPriority {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDocumentaiV1ReviewDocumentRequestPriority {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDocumentaiV1ReviewDocumentRequestPriority {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEFAULT" => GoogleCloudDocumentaiV1ReviewDocumentRequestPriority::Default,
                "URGENT" => GoogleCloudDocumentaiV1ReviewDocumentRequestPriority::Urgent,
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
        for GoogleCloudDocumentaiV1ReviewDocumentRequestPriority
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1ReviewDocumentRequestPriority {
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
    pub struct GoogleCloudDocumentaiV1ReviewDocumentResponse {
        #[doc = "The Cloud Storage uri for the human reviewed document."]
        #[serde(
            rename = "gcsDestination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_destination: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1ReviewDocumentResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1ReviewDocumentResponse {
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
    pub struct GoogleCloudDocumentaiV1SetDefaultProcessorVersionMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1CommonOperationMetadata>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1SetDefaultProcessorVersionMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1SetDefaultProcessorVersionMetadata
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
    pub struct GoogleCloudDocumentaiV1SetDefaultProcessorVersionRequest {
        #[doc = "Required. The resource name of child ProcessorVersion to use as default."]
        #[serde(
            rename = "defaultProcessorVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_processor_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1SetDefaultProcessorVersionRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1SetDefaultProcessorVersionRequest
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1SetDefaultProcessorVersionResponse {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1SetDefaultProcessorVersionResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1SetDefaultProcessorVersionResponse
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
    pub struct GoogleCloudDocumentaiV1UndeployProcessorVersionMetadata {
        #[doc = "The basic metadata of the long running operation."]
        #[serde(
            rename = "commonMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_metadata:
            ::std::option::Option<crate::schemas::GoogleCloudDocumentaiV1CommonOperationMetadata>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1UndeployProcessorVersionMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1UndeployProcessorVersionMetadata
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1UndeployProcessorVersionRequest {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1UndeployProcessorVersionRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1UndeployProcessorVersionRequest
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDocumentaiV1UndeployProcessorVersionResponse {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDocumentaiV1UndeployProcessorVersionResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDocumentaiV1UndeployProcessorVersionResponse
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
    pub struct GoogleCloudDocumentaiV1Vertex {
        #[doc = "X coordinate."]
        #[serde(
            rename = "x",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub x: ::std::option::Option<i32>,
        #[doc = "Y coordinate (starts from the top of the image)."]
        #[serde(
            rename = "y",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub y: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDocumentaiV1Vertex {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDocumentaiV1Vertex {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudLocationListLocationsResponse {
        #[doc = "A list of locations that matches the specified filter in the request."]
        #[serde(
            rename = "locations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locations: ::std::option::Option<Vec<crate::schemas::GoogleCloudLocationLocation>>,
        #[doc = "The standard List next-page token."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudLocationListLocationsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudLocationListLocationsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudLocationLocation {
        #[doc = "The friendly name for this location, typically a nearby city name. For example, \"Tokyo\"."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Cross-service attributes for the location. For example {\"cloud.googleapis.com/region\": \"us-east1\"}"]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The canonical id for this location. For example: `\"us-east1\"`."]
        #[serde(
            rename = "locationId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location_id: ::std::option::Option<String>,
        #[doc = "Service-specific metadata. For example the available capacity at the given location."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Resource name for the location, which may vary between implementations. For example: `\"projects/example-project/locations/us-east1\"`"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudLocationLocation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudLocationLocation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleLongrunningListOperationsResponse {
        #[doc = "The standard List next-page token."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "A list of operations that matches the specified filter in the request."]
        #[serde(
            rename = "operations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operations: ::std::option::Option<Vec<crate::schemas::GoogleLongrunningOperation>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleLongrunningListOperationsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleLongrunningListOperationsResponse {
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
        pub error: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleProtobufEmpty {}
    impl ::google_field_selector::FieldSelector for GoogleProtobufEmpty {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleProtobufEmpty {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleRpcStatus {
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
    impl ::google_field_selector::FieldSelector for GoogleRpcStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleRpcStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleTypeColor {
        #[doc = "The fraction of this color that should be applied to the pixel. That is, the final pixel color is defined by the equation: `pixel color = alpha * (this color) + (1.0 - alpha) * (background color)` This means that a value of 1.0 corresponds to a solid color, whereas a value of 0.0 corresponds to a completely transparent color. This uses a wrapper message rather than a simple float scalar so that it is possible to distinguish between a default value and the value being unset. If omitted, this color object is rendered as a solid color (as if the alpha value had been explicitly given a value of 1.0)."]
        #[serde(
            rename = "alpha",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alpha: ::std::option::Option<f32>,
        #[doc = "The amount of blue in the color as a value in the interval [0, 1]."]
        #[serde(
            rename = "blue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub blue: ::std::option::Option<f32>,
        #[doc = "The amount of green in the color as a value in the interval [0, 1]."]
        #[serde(
            rename = "green",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub green: ::std::option::Option<f32>,
        #[doc = "The amount of red in the color as a value in the interval [0, 1]."]
        #[serde(
            rename = "red",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub red: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleTypeColor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleTypeColor {
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
    pub struct GoogleTypeDate {
        #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
        #[serde(
            rename = "day",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub day: ::std::option::Option<i32>,
        #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
        #[serde(
            rename = "month",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub month: ::std::option::Option<i32>,
        #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
        #[serde(
            rename = "year",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub year: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleTypeDate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleTypeDate {
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
    pub struct GoogleTypeDateTime {
        #[doc = "Required. Day of month. Must be from 1 to 31 and valid for the year and month."]
        #[serde(
            rename = "day",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub day: ::std::option::Option<i32>,
        #[doc = "Required. Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
        #[serde(
            rename = "hours",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hours: ::std::option::Option<i32>,
        #[doc = "Required. Minutes of hour of day. Must be from 0 to 59."]
        #[serde(
            rename = "minutes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub minutes: ::std::option::Option<i32>,
        #[doc = "Required. Month of year. Must be from 1 to 12."]
        #[serde(
            rename = "month",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub month: ::std::option::Option<i32>,
        #[doc = "Required. Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
        #[serde(
            rename = "nanos",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub nanos: ::std::option::Option<i32>,
        #[doc = "Required. Seconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds."]
        #[serde(
            rename = "seconds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub seconds: ::std::option::Option<i32>,
        #[doc = "Time zone."]
        #[serde(
            rename = "timeZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_zone: ::std::option::Option<crate::schemas::GoogleTypeTimeZone>,
        #[doc = "UTC offset. Must be whole seconds, between -18 hours and +18 hours. For example, a UTC offset of -4:00 would be represented as { seconds: -14400 }."]
        #[serde(
            rename = "utcOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub utc_offset: ::std::option::Option<String>,
        #[doc = "Optional. Year of date. Must be from 1 to 9999, or 0 if specifying a datetime without a year."]
        #[serde(
            rename = "year",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub year: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleTypeDateTime {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleTypeDateTime {
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
    pub struct GoogleTypeMoney {
        #[doc = "The three-letter currency code defined in ISO 4217."]
        #[serde(
            rename = "currencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000."]
        #[serde(
            rename = "nanos",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub nanos: ::std::option::Option<i32>,
        #[doc = "The whole units of the amount. For example if `currencyCode` is `\"USD\"`, then 1 unit is one US dollar."]
        #[serde(
            rename = "units",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub units: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleTypeMoney {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleTypeMoney {
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
    pub struct GoogleTypePostalAddress {
        #[doc = "Unstructured address lines describing the lower levels of an address. Because values in address_lines do not have type information and may sometimes contain multiple values in a single field (e.g. \"Austin, TX\"), it is important that the line order is clear. The order of address lines should be \"envelope order\" for the country/region of the address. In places where this can vary (e.g. Japan), address_language is used to make it explicit (e.g. \"ja\" for large-to-small ordering and \"ja-Latn\" or \"en\" for small-to-large). This way, the most specific line of an address can be selected based on the language. The minimum permitted structural representation of an address consists of a region_code with all remaining information placed in the address_lines. It would be possible to format such an address very approximately without geocoding, but no semantic reasoning could be made about any of the address components until it was at least partially resolved. Creating an address only containing a region_code and address_lines, and then geocoding is the recommended way to handle completely unstructured addresses (as opposed to guessing which parts of the address should be localities or administrative areas)."]
        #[serde(
            rename = "addressLines",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub address_lines: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. Highest administrative subdivision which is used for postal addresses of a country or region. For example, this can be a state, a province, an oblast, or a prefecture. Specifically, for Spain this is the province and not the autonomous community (e.g. \"Barcelona\" and not \"Catalonia\"). Many countries don't use an administrative area in postal addresses. E.g. in Switzerland this should be left unpopulated."]
        #[serde(
            rename = "administrativeArea",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub administrative_area: ::std::option::Option<String>,
        #[doc = "Optional. BCP-47 language code of the contents of this address (if known). This is often the UI language of the input form or is expected to match one of the languages used in the address' country/region, or their transliterated equivalents. This can affect formatting in certain countries, but is not critical to the correctness of the data and will never affect any validation or other non-formatting related operations. If this value is not known, it should be omitted (rather than specifying a possibly incorrect default). Examples: \"zh-Hant\", \"ja\", \"ja-Latn\", \"en\"."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Optional. Generally refers to the city/town portion of the address. Examples: US city, IT comune, UK post town. In regions of the world where localities are not well defined or do not fit into this structure well, leave locality empty and use address_lines."]
        #[serde(
            rename = "locality",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locality: ::std::option::Option<String>,
        #[doc = "Optional. The name of the organization at the address."]
        #[serde(
            rename = "organization",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub organization: ::std::option::Option<String>,
        #[doc = "Optional. Postal code of the address. Not all countries use or require postal codes to be present, but where they are used, they may trigger additional validation with other parts of the address (e.g. state/zip validation in the U.S.A.)."]
        #[serde(
            rename = "postalCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub postal_code: ::std::option::Option<String>,
        #[doc = "Optional. The recipient at the address. This field may, under certain circumstances, contain multiline information. For example, it might contain \"care of\" information."]
        #[serde(
            rename = "recipients",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recipients: ::std::option::Option<Vec<String>>,
        #[doc = "Required. CLDR region code of the country/region of the address. This is never inferred and it is up to the user to ensure the value is correct. See https://cldr.unicode.org/ and https://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: \"CH\" for Switzerland."]
        #[serde(
            rename = "regionCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region_code: ::std::option::Option<String>,
        #[doc = "The schema revision of the `PostalAddress`. This must be set to 0, which is the latest revision. All new revisions **must** be backward compatible with old revisions."]
        #[serde(
            rename = "revision",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision: ::std::option::Option<i32>,
        #[doc = "Optional. Additional, country-specific, sorting code. This is not used in most regions. Where it is used, the value is either a string like \"CEDEX\", optionally followed by a number (e.g. \"CEDEX 7\"), or just a number alone, representing the \"sector code\" (Jamaica), \"delivery area indicator\" (Malawi) or \"post office indicator\" (e.g. Côte d'Ivoire)."]
        #[serde(
            rename = "sortingCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sorting_code: ::std::option::Option<String>,
        #[doc = "Optional. Sublocality of the address. For example, this can be neighborhoods, boroughs, districts."]
        #[serde(
            rename = "sublocality",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sublocality: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleTypePostalAddress {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleTypePostalAddress {
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
    pub struct GoogleTypeTimeZone {
        #[doc = "IANA Time Zone Database time zone, e.g. \"America/New_York\"."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Optional. IANA Time Zone Database version number, e.g. \"2019a\"."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleTypeTimeZone {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleTypeTimeZone {
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
    #[doc = "Actions that can be performed on the operations resource"]
    pub fn operations(&self) -> crate::resources::operations::OperationsActions {
        crate::resources::operations::OperationsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::resources::projects::ProjectsActions {
        crate::resources::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the uiv_1beta_3 resource"]
    pub fn uiv_1beta_3(&self) -> crate::resources::uiv_1beta_3::Uiv1Beta3Actions {
        crate::resources::uiv_1beta_3::Uiv1Beta3Actions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod operations {
        pub mod params {}
        pub struct OperationsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> OperationsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`."]
            pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder {
                DeleteRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
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
                    name: name.into(),
                }
            }
        }
        #[doc = "Created via [OperationsActions::delete()](struct.OperationsActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            name: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> DeleteRequestBuilder<'a> {
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
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                let fields: Option<String> = if fields.is_empty() {
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
            ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
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
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://documentai.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
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
            #[doc = "Actions that can be performed on the locations resource"]
            pub fn locations(&self) -> crate::resources::projects::locations::LocationsActions {
                crate::resources::projects::locations::LocationsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the operations resource"]
            pub fn operations(&self) -> crate::resources::projects::operations::OperationsActions {
                crate::resources::projects::operations::OperationsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod locations {
            pub mod params {}
            pub struct LocationsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> LocationsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Fetches processor types. Note that we do not use ListProcessorTypes here because it is not paginated."]
                pub fn fetch_processor_types(
                    &self,
                    parent: impl Into<String>,
                ) -> FetchProcessorTypesRequestBuilder {
                    FetchProcessorTypesRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
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
                        parent: parent.into(),
                    }
                }
                #[doc = "Gets information about a location."]
                pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
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
                        name: name.into(),
                    }
                }
                #[doc = "Lists information about the supported locations for this service."]
                pub fn list(&self, name: impl Into<String>) -> ListRequestBuilder {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
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
                        name: name.into(),
                        filter: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Actions that can be performed on the operations resource"]
                pub fn operations(
                    &self,
                ) -> crate::resources::projects::locations::operations::OperationsActions
                {
                    crate::resources::projects::locations::operations::OperationsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
                #[doc = "Actions that can be performed on the processors resource"]
                pub fn processors(
                    &self,
                ) -> crate::resources::projects::locations::processors::ProcessorsActions
                {
                    crate::resources::projects::locations::processors::ProcessorsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            #[doc = "Created via [LocationsActions::fetch_processor_types()](struct.LocationsActions.html#method.fetch_processor_types)"]
            #[derive(Debug, Clone)]
            pub struct FetchProcessorTypesRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> FetchProcessorTypesRequestBuilder<'a> {
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                    let fields: Option<String> = if fields.is_empty() {
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
                ) -> Result<
                    crate::schemas::GoogleCloudDocumentaiV1FetchProcessorTypesResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudDocumentaiV1FetchProcessorTypesResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(Some("*")).await
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub async fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
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
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://documentai.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":fetchProcessorTypes");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
            #[doc = "Created via [LocationsActions::get()](struct.LocationsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> GetRequestBuilder<'a> {
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                    let fields: Option<String> = if fields.is_empty() {
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
                ) -> Result<crate::schemas::GoogleCloudLocationLocation, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudLocationLocation, crate::Error>
                {
                    self.execute_with_fields(Some("*")).await
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub async fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
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
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://documentai.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
            #[doc = "Created via [LocationsActions::list()](struct.LocationsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                filter: Option<String>,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "A filter to narrow down results to a preferred subset. The filtering language accepts strings like `\"displayName=tokyo\"`, and is documented in more detail in [AIP-160](https://google.aip.dev/160)."]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "The maximum number of results to return. If not set, the service selects a default."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A page token received from the `next_page_token` field in the response. Send that page token to receive the subsequent page."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                    let fields: Option<String> = if fields.is_empty() {
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
                ) -> Result<crate::schemas::GoogleCloudLocationListLocationsResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudLocationListLocationsResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*")).await
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub async fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
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
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://documentai.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/locations");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("filter", &self.filter)]);
                    req = req.query(&[("pageSize", &self.page_size)]);
                    req = req.query(&[("pageToken", &self.page_token)]);
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
            pub mod operations {
                pub mod params {}
                pub struct OperationsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> OperationsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."]
                    pub fn cancel(&self, name: impl Into<String>) -> CancelRequestBuilder {
                        CancelRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
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
                            name: name.into(),
                        }
                    }
                    #[doc = "Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."]
                    pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                        GetRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
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
                            name: name.into(),
                        }
                    }
                    #[doc = "Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id."]
                    pub fn list(&self, name: impl Into<String>) -> ListRequestBuilder {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
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
                            name: name.into(),
                            filter: None,
                            page_size: None,
                            page_token: None,
                        }
                    }
                }
                #[doc = "Created via [OperationsActions::cancel()](struct.OperationsActions.html#method.cancel)"]
                #[derive(Debug, Clone)]
                pub struct CancelRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> CancelRequestBuilder<'a> {
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
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                        let fields: Option<String> = if fields.is_empty() {
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
                    ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                    {
                        self.execute_with_fields(Some("*")).await
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub async fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
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
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://documentai.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":cancel");
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
                #[doc = "Created via [OperationsActions::get()](struct.OperationsActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> GetRequestBuilder<'a> {
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
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                        let fields: Option<String> = if fields.is_empty() {
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
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(Some("*")).await
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub async fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
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
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://documentai.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
                #[doc = "Created via [OperationsActions::list()](struct.OperationsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    filter: Option<String>,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> ListRequestBuilder<'a> {
                    #[doc = "The standard list filter."]
                    pub fn filter(mut self, value: impl Into<String>) -> Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "The standard list page size."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "The standard list page token."]
                    pub fn page_token(mut self, value: impl Into<String>) -> Self {
                        self.page_token = Some(value.into());
                        self
                    }
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
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                        let fields: Option<String> = if fields.is_empty() {
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
                    ) -> Result<crate::schemas::GoogleLongrunningListOperationsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningListOperationsResponse, crate::Error>
                    {
                        self.execute_with_fields(Some("*")).await
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub async fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
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
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://documentai.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[("filter", &self.filter)]);
                        req = req.query(&[("pageSize", &self.page_size)]);
                        req = req.query(&[("pageToken", &self.page_token)]);
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
            pub mod processors {
                pub mod params {}
                pub struct ProcessorsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> ProcessorsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "LRO endpoint to batch process many documents. The output is written to Cloud Storage as JSON in the [Document] format."]
                    pub fn batch_process(
                        &self,
                        request: crate::schemas::GoogleCloudDocumentaiV1BatchProcessRequest,
                        name: impl Into<String>,
                    ) -> BatchProcessRequestBuilder {
                        BatchProcessRequestBuilder {
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
                            name: name.into(),
                        }
                    }
                    #[doc = "Creates a processor from the type processor that the user chose. The processor will be at \"ENABLED\" state by default after its creation."]
                    pub fn create(
                        &self,
                        request: crate::schemas::GoogleCloudDocumentaiV1Processor,
                        parent: impl Into<String>,
                    ) -> CreateRequestBuilder {
                        CreateRequestBuilder {
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
                            parent: parent.into(),
                        }
                    }
                    #[doc = "Deletes the processor, unloads all deployed model artifacts if it was enabled and then deletes all artifacts associated with this processor."]
                    pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder {
                        DeleteRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
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
                            name: name.into(),
                        }
                    }
                    #[doc = "Disables a processor"]
                    pub fn disable(
                        &self,
                        request: crate::schemas::GoogleCloudDocumentaiV1DisableProcessorRequest,
                        name: impl Into<String>,
                    ) -> DisableRequestBuilder {
                        DisableRequestBuilder {
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
                            name: name.into(),
                        }
                    }
                    #[doc = "Enables a processor"]
                    pub fn enable(
                        &self,
                        request: crate::schemas::GoogleCloudDocumentaiV1EnableProcessorRequest,
                        name: impl Into<String>,
                    ) -> EnableRequestBuilder {
                        EnableRequestBuilder {
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
                            name: name.into(),
                        }
                    }
                    #[doc = "Gets a processor detail."]
                    pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                        GetRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
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
                            name: name.into(),
                        }
                    }
                    #[doc = "Lists all processors which belong to this project."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
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
                            parent: parent.into(),
                            page_size: None,
                            page_token: None,
                        }
                    }
                    #[doc = "Processes a single document."]
                    pub fn process(
                        &self,
                        request: crate::schemas::GoogleCloudDocumentaiV1ProcessRequest,
                        name: impl Into<String>,
                    ) -> ProcessRequestBuilder {
                        ProcessRequestBuilder {
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
                            name: name.into(),
                        }
                    }
                    #[doc = "Set the default (active) version of a Processor that will be used in ProcessDocument and BatchProcessDocuments."]
                    pub fn set_default_processor_version(
                        &self,
                        request : crate :: schemas :: GoogleCloudDocumentaiV1SetDefaultProcessorVersionRequest,
                        processor: impl Into<String>,
                    ) -> SetDefaultProcessorVersionRequestBuilder {
                        SetDefaultProcessorVersionRequestBuilder {
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
                            processor: processor.into(),
                        }
                    }
                    #[doc = "Actions that can be performed on the human_review_config resource"]                    pub fn human_review_config (& self) -> crate :: resources :: projects :: locations :: processors :: human_review_config :: HumanReviewConfigActions{
                        crate :: resources :: projects :: locations :: processors :: human_review_config :: HumanReviewConfigActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                    #[doc = "Actions that can be performed on the processor_versions resource"]                    pub fn processor_versions (& self) -> crate :: resources :: projects :: locations :: processors :: processor_versions :: ProcessorVersionsActions{
                        crate :: resources :: projects :: locations :: processors :: processor_versions :: ProcessorVersionsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                }
                #[doc = "Created via [ProcessorsActions::batch_process()](struct.ProcessorsActions.html#method.batch_process)"]
                #[derive(Debug, Clone)]
                pub struct BatchProcessRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudDocumentaiV1BatchProcessRequest,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> BatchProcessRequestBuilder<'a> {
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
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                        let fields: Option<String> = if fields.is_empty() {
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
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(Some("*")).await
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub async fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
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
                        let mut output = "https://documentai.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":batchProcess");
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
                #[doc = "Created via [ProcessorsActions::create()](struct.ProcessorsActions.html#method.create)"]
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudDocumentaiV1Processor,
                    parent: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> CreateRequestBuilder<'a> {
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
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                        let fields: Option<String> = if fields.is_empty() {
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
                    ) -> Result<crate::schemas::GoogleCloudDocumentaiV1Processor, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudDocumentaiV1Processor, crate::Error>
                    {
                        self.execute_with_fields(Some("*")).await
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub async fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
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
                        let mut output = "https://documentai.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/processors");
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
                #[doc = "Created via [ProcessorsActions::delete()](struct.ProcessorsActions.html#method.delete)"]
                #[derive(Debug, Clone)]
                pub struct DeleteRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> DeleteRequestBuilder<'a> {
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
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                        let fields: Option<String> = if fields.is_empty() {
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
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(Some("*")).await
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub async fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
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
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://documentai.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
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
                #[doc = "Created via [ProcessorsActions::disable()](struct.ProcessorsActions.html#method.disable)"]
                #[derive(Debug, Clone)]
                pub struct DisableRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudDocumentaiV1DisableProcessorRequest,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> DisableRequestBuilder<'a> {
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
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                        let fields: Option<String> = if fields.is_empty() {
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
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(Some("*")).await
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub async fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
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
                        let mut output = "https://documentai.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":disable");
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
                #[doc = "Created via [ProcessorsActions::enable()](struct.ProcessorsActions.html#method.enable)"]
                #[derive(Debug, Clone)]
                pub struct EnableRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudDocumentaiV1EnableProcessorRequest,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> EnableRequestBuilder<'a> {
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
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                        let fields: Option<String> = if fields.is_empty() {
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
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(Some("*")).await
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub async fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
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
                        let mut output = "https://documentai.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":enable");
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
                #[doc = "Created via [ProcessorsActions::get()](struct.ProcessorsActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> GetRequestBuilder<'a> {
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
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                        let fields: Option<String> = if fields.is_empty() {
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
                    ) -> Result<crate::schemas::GoogleCloudDocumentaiV1Processor, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudDocumentaiV1Processor, crate::Error>
                    {
                        self.execute_with_fields(Some("*")).await
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub async fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
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
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://documentai.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
                #[doc = "Created via [ProcessorsActions::list()](struct.ProcessorsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> ListRequestBuilder<'a> {
                    #[doc = "The maximum number of processors to return. If unspecified, at most 50 processors will be returned. The maximum value is 100; values above 100 will be coerced to 100."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "We will return the processors sorted by creation time. The page token will point to the next processor."]
                    pub fn page_token(mut self, value: impl Into<String>) -> Self {
                        self.page_token = Some(value.into());
                        self
                    }
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
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                        let fields: Option<String> = if fields.is_empty() {
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
                    ) -> Result<
                        crate::schemas::GoogleCloudDocumentaiV1ListProcessorsResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudDocumentaiV1ListProcessorsResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(Some("*")).await
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub async fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
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
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://documentai.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/processors");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[("pageSize", &self.page_size)]);
                        req = req.query(&[("pageToken", &self.page_token)]);
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
                #[doc = "Created via [ProcessorsActions::process()](struct.ProcessorsActions.html#method.process)"]
                #[derive(Debug, Clone)]
                pub struct ProcessRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudDocumentaiV1ProcessRequest,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> ProcessRequestBuilder<'a> {
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
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                        let fields: Option<String> = if fields.is_empty() {
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
                    ) -> Result<crate::schemas::GoogleCloudDocumentaiV1ProcessResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudDocumentaiV1ProcessResponse, crate::Error>
                    {
                        self.execute_with_fields(Some("*")).await
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub async fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
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
                        let mut output = "https://documentai.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":process");
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
                #[doc = "Created via [ProcessorsActions::set_default_processor_version()](struct.ProcessorsActions.html#method.set_default_processor_version)"]
                #[derive(Debug, Clone)]
                pub struct SetDefaultProcessorVersionRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request:
                        crate::schemas::GoogleCloudDocumentaiV1SetDefaultProcessorVersionRequest,
                    processor: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> SetDefaultProcessorVersionRequestBuilder<'a> {
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
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                        let fields: Option<String> = if fields.is_empty() {
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
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(Some("*")).await
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub async fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
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
                        let mut output = "https://documentai.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.processor;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":setDefaultProcessorVersion");
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
                pub mod human_review_config {
                    pub mod params {}
                    pub struct HumanReviewConfigActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> HumanReviewConfigActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Send a document for Human Review. The input document should be processed by the specified processor."]
                        pub fn review_document(
                            &self,
                            request: crate::schemas::GoogleCloudDocumentaiV1ReviewDocumentRequest,
                            human_review_config: impl Into<String>,
                        ) -> ReviewDocumentRequestBuilder {
                            ReviewDocumentRequestBuilder {
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
                                human_review_config: human_review_config.into(),
                            }
                        }
                    }
                    #[doc = "Created via [HumanReviewConfigActions::review_document()](struct.HumanReviewConfigActions.html#method.review_document)"]
                    #[derive(Debug, Clone)]
                    pub struct ReviewDocumentRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::GoogleCloudDocumentaiV1ReviewDocumentRequest,
                        human_review_config: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> ReviewDocumentRequestBuilder<'a> {
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
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
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
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(Some("*")).await
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub async fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
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
                            let mut output = "https://documentai.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.human_review_config;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str(":reviewDocument");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
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
                pub mod processor_versions {
                    pub mod params {}
                    pub struct ProcessorVersionsActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> ProcessorVersionsActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "LRO endpoint to batch process many documents. The output is written to Cloud Storage as JSON in the [Document] format."]
                        pub fn batch_process(
                            &self,
                            request: crate::schemas::GoogleCloudDocumentaiV1BatchProcessRequest,
                            name: impl Into<String>,
                        ) -> BatchProcessRequestBuilder {
                            BatchProcessRequestBuilder {
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
                                name: name.into(),
                            }
                        }
                        #[doc = "Deletes the processor version, all artifacts under the processor version will be deleted."]
                        pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder {
                            DeleteRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
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
                                name: name.into(),
                            }
                        }
                        #[doc = "Deploys the processor version."]
                        pub fn deploy(
                            &self,
                            request : crate :: schemas :: GoogleCloudDocumentaiV1DeployProcessorVersionRequest,
                            name: impl Into<String>,
                        ) -> DeployRequestBuilder {
                            DeployRequestBuilder {
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
                                name: name.into(),
                            }
                        }
                        #[doc = "Gets a processor version detail."]
                        pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                            GetRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
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
                                name: name.into(),
                            }
                        }
                        #[doc = "Lists all versions of a processor."]
                        pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
                            ListRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
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
                                parent: parent.into(),
                                page_size: None,
                                page_token: None,
                            }
                        }
                        #[doc = "Processes a single document."]
                        pub fn process(
                            &self,
                            request: crate::schemas::GoogleCloudDocumentaiV1ProcessRequest,
                            name: impl Into<String>,
                        ) -> ProcessRequestBuilder {
                            ProcessRequestBuilder {
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
                                name: name.into(),
                            }
                        }
                        #[doc = "Undeploys the processor version."]
                        pub fn undeploy(
                            &self,
                            request : crate :: schemas :: GoogleCloudDocumentaiV1UndeployProcessorVersionRequest,
                            name: impl Into<String>,
                        ) -> UndeployRequestBuilder {
                            UndeployRequestBuilder {
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
                                name: name.into(),
                            }
                        }
                    }
                    #[doc = "Created via [ProcessorVersionsActions::batch_process()](struct.ProcessorVersionsActions.html#method.batch_process)"]
                    #[derive(Debug, Clone)]
                    pub struct BatchProcessRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::GoogleCloudDocumentaiV1BatchProcessRequest,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> BatchProcessRequestBuilder<'a> {
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
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
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
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(Some("*")).await
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub async fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
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
                            let mut output = "https://documentai.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str(":batchProcess");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
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
                    #[doc = "Created via [ProcessorVersionsActions::delete()](struct.ProcessorVersionsActions.html#method.delete)"]
                    #[derive(Debug, Clone)]
                    pub struct DeleteRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> DeleteRequestBuilder<'a> {
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
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
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
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(Some("*")).await
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub async fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
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
                            Ok(req.send().await?.error_for_status()?.json().await?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://documentai.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
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
                    #[doc = "Created via [ProcessorVersionsActions::deploy()](struct.ProcessorVersionsActions.html#method.deploy)"]
                    #[derive(Debug, Clone)]
                    pub struct DeployRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request:
                            crate::schemas::GoogleCloudDocumentaiV1DeployProcessorVersionRequest,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> DeployRequestBuilder<'a> {
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
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
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
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(Some("*")).await
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub async fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
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
                            let mut output = "https://documentai.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str(":deploy");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
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
                    #[doc = "Created via [ProcessorVersionsActions::get()](struct.ProcessorVersionsActions.html#method.get)"]
                    #[derive(Debug, Clone)]
                    pub struct GetRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> GetRequestBuilder<'a> {
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
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
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
                        ) -> Result<
                            crate::schemas::GoogleCloudDocumentaiV1ProcessorVersion,
                            crate::Error,
                        > {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<
                            crate::schemas::GoogleCloudDocumentaiV1ProcessorVersion,
                            crate::Error,
                        > {
                            self.execute_with_fields(Some("*")).await
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub async fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
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
                            Ok(req.send().await?.error_for_status()?.json().await?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://documentai.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
                    #[doc = "Created via [ProcessorVersionsActions::list()](struct.ProcessorVersionsActions.html#method.list)"]
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        parent: String,
                        page_size: Option<i32>,
                        page_token: Option<String>,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> ListRequestBuilder<'a> {
                        #[doc = "The maximum number of processor versions to return. If unspecified, at most 10 processor versions will be returned. The maximum value is 20; values above 20 will be coerced to 20."]
                        pub fn page_size(mut self, value: i32) -> Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "We will return the processor versions sorted by creation time. The page token will point to the next processor version."]
                        pub fn page_token(mut self, value: impl Into<String>) -> Self {
                            self.page_token = Some(value.into());
                            self
                        }
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
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
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
                        ) -> Result<
                            crate::schemas::GoogleCloudDocumentaiV1ListProcessorVersionsResponse,
                            crate::Error,
                        > {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<
                            crate::schemas::GoogleCloudDocumentaiV1ListProcessorVersionsResponse,
                            crate::Error,
                        > {
                            self.execute_with_fields(Some("*")).await
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub async fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
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
                            Ok(req.send().await?.error_for_status()?.json().await?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://documentai.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/processorVersions");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                            req = req.query(&[("pageSize", &self.page_size)]);
                            req = req.query(&[("pageToken", &self.page_token)]);
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
                    #[doc = "Created via [ProcessorVersionsActions::process()](struct.ProcessorVersionsActions.html#method.process)"]
                    #[derive(Debug, Clone)]
                    pub struct ProcessRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::GoogleCloudDocumentaiV1ProcessRequest,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> ProcessRequestBuilder<'a> {
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
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
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
                        ) -> Result<
                            crate::schemas::GoogleCloudDocumentaiV1ProcessResponse,
                            crate::Error,
                        > {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<
                            crate::schemas::GoogleCloudDocumentaiV1ProcessResponse,
                            crate::Error,
                        > {
                            self.execute_with_fields(Some("*")).await
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub async fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
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
                            let mut output = "https://documentai.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str(":process");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
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
                    #[doc = "Created via [ProcessorVersionsActions::undeploy()](struct.ProcessorVersionsActions.html#method.undeploy)"]
                    #[derive(Debug, Clone)]
                    pub struct UndeployRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request:
                            crate::schemas::GoogleCloudDocumentaiV1UndeployProcessorVersionRequest,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> UndeployRequestBuilder<'a> {
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
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
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
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(Some("*")).await
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub async fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
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
                            let mut output = "https://documentai.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str(":undeploy");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
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
        }
        pub mod operations {
            pub mod params {}
            pub struct OperationsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> OperationsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."]
                pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
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
                        name: name.into(),
                    }
                }
            }
            #[doc = "Created via [OperationsActions::get()](struct.OperationsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> GetRequestBuilder<'a> {
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                    let fields: Option<String> = if fields.is_empty() {
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
                ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                {
                    self.execute_with_fields(Some("*")).await
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub async fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
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
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://documentai.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
    pub mod uiv_1beta_3 {
        pub mod params {}
        pub struct Uiv1Beta3Actions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> Uiv1Beta3Actions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the projects resource"]
            pub fn projects(&self) -> crate::resources::uiv_1beta_3::projects::ProjectsActions {
                crate::resources::uiv_1beta_3::projects::ProjectsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
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
                #[doc = "Actions that can be performed on the locations resource"]
                pub fn locations(
                    &self,
                ) -> crate::resources::uiv_1beta_3::projects::locations::LocationsActions
                {
                    crate::resources::uiv_1beta_3::projects::locations::LocationsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            pub mod locations {
                pub mod params {}
                pub struct LocationsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> LocationsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Gets information about a location."]
                    pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                        GetRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
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
                            name: name.into(),
                        }
                    }
                    #[doc = "Lists information about the supported locations for this service."]
                    pub fn list(&self, name: impl Into<String>) -> ListRequestBuilder {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
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
                            name: name.into(),
                            filter: None,
                            page_size: None,
                            page_token: None,
                        }
                    }
                    #[doc = "Actions that can be performed on the operations resource"]                    pub fn operations (& self) -> crate :: resources :: uiv_1beta_3 :: projects :: locations :: operations :: OperationsActions{
                        crate :: resources :: uiv_1beta_3 :: projects :: locations :: operations :: OperationsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                }
                #[doc = "Created via [LocationsActions::get()](struct.LocationsActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> GetRequestBuilder<'a> {
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
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                        let fields: Option<String> = if fields.is_empty() {
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
                    ) -> Result<crate::schemas::GoogleCloudLocationLocation, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudLocationLocation, crate::Error>
                    {
                        self.execute_with_fields(Some("*")).await
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub async fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
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
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://documentai.googleapis.com/".to_owned();
                        output.push_str("uiv1beta3/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
                #[doc = "Created via [LocationsActions::list()](struct.LocationsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    filter: Option<String>,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> ListRequestBuilder<'a> {
                    #[doc = "A filter to narrow down results to a preferred subset. The filtering language accepts strings like `\"displayName=tokyo\"`, and is documented in more detail in [AIP-160](https://google.aip.dev/160)."]
                    pub fn filter(mut self, value: impl Into<String>) -> Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "The maximum number of results to return. If not set, the service selects a default."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A page token received from the `next_page_token` field in the response. Send that page token to receive the subsequent page."]
                    pub fn page_token(mut self, value: impl Into<String>) -> Self {
                        self.page_token = Some(value.into());
                        self
                    }
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
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                        let fields: Option<String> = if fields.is_empty() {
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
                    ) -> Result<
                        crate::schemas::GoogleCloudLocationListLocationsResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudLocationListLocationsResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(Some("*")).await
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub async fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
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
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://documentai.googleapis.com/".to_owned();
                        output.push_str("uiv1beta3/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/locations");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[("filter", &self.filter)]);
                        req = req.query(&[("pageSize", &self.page_size)]);
                        req = req.query(&[("pageToken", &self.page_token)]);
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
                pub mod operations {
                    pub mod params {}
                    pub struct OperationsActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> OperationsActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."]
                        pub fn cancel(&self, name: impl Into<String>) -> CancelRequestBuilder {
                            CancelRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
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
                                name: name.into(),
                            }
                        }
                        #[doc = "Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."]
                        pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                            GetRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
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
                                name: name.into(),
                            }
                        }
                        #[doc = "Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id."]
                        pub fn list(&self, name: impl Into<String>) -> ListRequestBuilder {
                            ListRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
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
                                name: name.into(),
                                filter: None,
                                page_size: None,
                                page_token: None,
                            }
                        }
                    }
                    #[doc = "Created via [OperationsActions::cancel()](struct.OperationsActions.html#method.cancel)"]
                    #[derive(Debug, Clone)]
                    pub struct CancelRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> CancelRequestBuilder<'a> {
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
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
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
                        ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                        {
                            self.execute_with_fields(Some("*")).await
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub async fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
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
                            Ok(req.send().await?.error_for_status()?.json().await?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://documentai.googleapis.com/".to_owned();
                            output.push_str("uiv1beta3/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str(":cancel");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
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
                    #[doc = "Created via [OperationsActions::get()](struct.OperationsActions.html#method.get)"]
                    #[derive(Debug, Clone)]
                    pub struct GetRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> GetRequestBuilder<'a> {
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
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
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
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(Some("*")).await
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub async fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
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
                            Ok(req.send().await?.error_for_status()?.json().await?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://documentai.googleapis.com/".to_owned();
                            output.push_str("uiv1beta3/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
                    #[doc = "Created via [OperationsActions::list()](struct.OperationsActions.html#method.list)"]
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        name: String,
                        filter: Option<String>,
                        page_size: Option<i32>,
                        page_token: Option<String>,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> ListRequestBuilder<'a> {
                        #[doc = "The standard list filter."]
                        pub fn filter(mut self, value: impl Into<String>) -> Self {
                            self.filter = Some(value.into());
                            self
                        }
                        #[doc = "The standard list page size."]
                        pub fn page_size(mut self, value: i32) -> Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "The standard list page token."]
                        pub fn page_token(mut self, value: impl Into<String>) -> Self {
                            self.page_token = Some(value.into());
                            self
                        }
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
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
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
                        ) -> Result<
                            crate::schemas::GoogleLongrunningListOperationsResponse,
                            crate::Error,
                        > {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<
                            crate::schemas::GoogleLongrunningListOperationsResponse,
                            crate::Error,
                        > {
                            self.execute_with_fields(Some("*")).await
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub async fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
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
                            Ok(req.send().await?.error_for_status()?.json().await?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://documentai.googleapis.com/".to_owned();
                            output.push_str("uiv1beta3/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                            req = req.query(&[("filter", &self.filter)]);
                            req = req.query(&[("pageSize", &self.page_size)]);
                            req = req.query(&[("pageToken", &self.page_token)]);
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
