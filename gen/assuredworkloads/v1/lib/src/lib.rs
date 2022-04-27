#![doc = "# Resources and Methods\n    * [organizations](resources/organizations/struct.OrganizationsActions.html)\n      * [locations](resources/organizations/locations/struct.LocationsActions.html)\n        * [operations](resources/organizations/locations/operations/struct.OperationsActions.html)\n          * [*get*](resources/organizations/locations/operations/struct.GetRequestBuilder.html), [*list*](resources/organizations/locations/operations/struct.ListRequestBuilder.html)\n        * [workloads](resources/organizations/locations/workloads/struct.WorkloadsActions.html)\n          * [*create*](resources/organizations/locations/workloads/struct.CreateRequestBuilder.html), [*delete*](resources/organizations/locations/workloads/struct.DeleteRequestBuilder.html), [*get*](resources/organizations/locations/workloads/struct.GetRequestBuilder.html), [*list*](resources/organizations/locations/workloads/struct.ListRequestBuilder.html), [*patch*](resources/organizations/locations/workloads/struct.PatchRequestBuilder.html)\n"]
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
    pub struct GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadata { # [doc = "Optional. Compliance controls that should be applied to the resources managed by the workload."] # [serde (rename = "complianceRegime" , default , skip_serializing_if = "std::option::Option::is_none")] pub compliance_regime : :: std :: option :: Option < crate :: schemas :: GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime > , # [doc = "Optional. Time when the operation was created."] # [serde (rename = "createTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub create_time : :: std :: option :: Option < String > , # [doc = "Optional. The display name of the workload."] # [serde (rename = "displayName" , default , skip_serializing_if = "std::option::Option::is_none")] pub display_name : :: std :: option :: Option < String > , # [doc = "Optional. The parent of the workload."] # [serde (rename = "parent" , default , skip_serializing_if = "std::option::Option::is_none")] pub parent : :: std :: option :: Option < String > , # [doc = "Optional. Resource properties in the input that are used for creating/customizing workload resources."] # [serde (rename = "resourceSettings" , default , skip_serializing_if = "std::option::Option::is_none")] pub resource_settings : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettings > > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadata
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime {
        #[doc = "Assured Workloads For Canada Regions and Support controls"]
        CaRegionsAndSupport,
        #[doc = "Criminal Justice Information Services (CJIS) Security policies."]
        Cjis,
        #[doc = "Unknown compliance regime."]
        ComplianceRegimeUnspecified,
        #[doc = "Assured Workloads For EU Regions and Support controls"]
        EuRegionsAndSupport,
        #[doc = "FedRAMP High data protection controls"]
        FedrampHigh,
        #[doc = "FedRAMP Moderate data protection controls"]
        FedrampModerate,
        #[doc = "Health Insurance Portability and Accountability Act controls"]
        Hipaa,
        #[doc = "Health Information Trust Alliance controls"]
        Hitrust,
        #[doc = "Information protection as per DoD IL4 requirements."]
        Il4,
        #[doc = "Assured Workloads For US Regions data protection controls"]
        UsRegionalAccess,
    }
    impl GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: CaRegionsAndSupport => "CA_REGIONS_AND_SUPPORT" , GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: Cjis => "CJIS" , GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: ComplianceRegimeUnspecified => "COMPLIANCE_REGIME_UNSPECIFIED" , GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: EuRegionsAndSupport => "EU_REGIONS_AND_SUPPORT" , GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: FedrampHigh => "FEDRAMP_HIGH" , GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: FedrampModerate => "FEDRAMP_MODERATE" , GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: Hipaa => "HIPAA" , GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: Hitrust => "HITRUST" , GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: Il4 => "IL4" , GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: UsRegionalAccess => "US_REGIONAL_ACCESS" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime,
            (),
        > {
            Ok (match s { "CA_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: CaRegionsAndSupport , "CJIS" => GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: Cjis , "COMPLIANCE_REGIME_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: ComplianceRegimeUnspecified , "EU_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: EuRegionsAndSupport , "FEDRAMP_HIGH" => GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: FedrampHigh , "FEDRAMP_MODERATE" => GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: FedrampModerate , "HIPAA" => GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: Hipaa , "HITRUST" => GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: Hitrust , "IL4" => GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: Il4 , "US_REGIONAL_ACCESS" => GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: UsRegionalAccess , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CA_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: CaRegionsAndSupport , "CJIS" => GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: Cjis , "COMPLIANCE_REGIME_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: ComplianceRegimeUnspecified , "EU_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: EuRegionsAndSupport , "FEDRAMP_HIGH" => GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: FedrampHigh , "FEDRAMP_MODERATE" => GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: FedrampModerate , "HIPAA" => GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: Hipaa , "HITRUST" => GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: Hitrust , "IL4" => GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: Il4 , "US_REGIONAL_ACCESS" => GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime :: UsRegionalAccess , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1CreateWorkloadOperationMetadataComplianceRegime
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
    pub struct GoogleCloudAssuredworkloadsV1Beta1Workload {
        #[doc = "Input only. The billing account used for the resources which are direct children of workload. This billing account is initially associated with the resources created as part of Workload creation. After the initial creation of these resources, the customer can change the assigned billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF`."]
        #[serde(
            rename = "billingAccount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub billing_account: ::std::option::Option<String>,
        #[doc = "Input only. Immutable. Settings specific to resources needed for CJIS."]
        #[serde(
            rename = "cjisSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cjis_settings: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1WorkloadCJISSettings,
        >,
        #[doc = "Required. Immutable. Compliance Regime associated with this workload."]
        #[serde(
            rename = "complianceRegime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compliance_regime: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime,
        >,
        #[doc = "Output only. Immutable. The Workload creation timestamp."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Required. The user-assigned display name of the Workload. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, and spaces. Example: My Workload"]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Optional. Indicates the sovereignty status of the given workload. Currently meant to be used by Europe/Canada customers."]
        #[serde(
            rename = "enableSovereignControls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_sovereign_controls: ::std::option::Option<bool>,
        #[doc = "Optional. ETag of the workload, it is calculated on the basis of the Workload contents. It will be used in Update & Delete operations."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "Input only. Immutable. Settings specific to resources needed for FedRAMP High."]
        #[serde(
            rename = "fedrampHighSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fedramp_high_settings: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1WorkloadFedrampHighSettings,
        >,
        #[doc = "Input only. Immutable. Settings specific to resources needed for FedRAMP Moderate."]
        #[serde(
            rename = "fedrampModerateSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fedramp_moderate_settings: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1WorkloadFedrampModerateSettings,
        >,
        #[doc = "Input only. Immutable. Settings specific to resources needed for IL4."]
        #[serde(
            rename = "il4Settings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub il_4_settings: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1WorkloadIL4Settings,
        >,
        #[doc = "Output only. Represents the KAJ enrollment state of the given workload."]
        #[serde(
            rename = "kajEnrollmentState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kaj_enrollment_state: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1WorkloadKajEnrollmentState,
        >,
        #[doc = "Input only. Settings used to create a CMEK crypto key. When set, a project with a KMS CMEK key is provisioned. This field is deprecated as of Feb 28, 2022. In order to create a Keyring, callers should specify, ENCRYPTION_KEYS_PROJECT or KEYRING in ResourceSettings.resource_type field."]
        #[serde(
            rename = "kmsSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kms_settings: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1WorkloadKMSSettings,
        >,
        #[doc = "Optional. Labels applied to the workload."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Optional. The resource name of the workload. Format: organizations/{organization}/locations/{location}/workloads/{workload} Read-only."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Input only. The parent resource for the resources managed by this Assured Workload. May be either empty or a folder resource which is a child of the Workload parent. If not specified all resources are created under the parent organization. Format: folders/{folder_id}"]
        #[serde(
            rename = "provisionedResourcesParent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provisioned_resources_parent: ::std::option::Option<String>,
        #[doc = "Input only. Resource properties that are used to customize workload resources. These properties (such as custom project id) will be used to create workload resources if possible. This field is optional."]
        #[serde(
            rename = "resourceSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_settings: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettings>,
        >,
        #[doc = "Output only. The resources associated with this workload. These resources will be created when creating the workload. If any of the projects already exist, the workload creation will fail. Always read only."]
        #[serde(
            rename = "resources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resources: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfo>,
        >,
        #[doc = "Output only. Represents the SAA enrollment response of the given workload. SAA enrollment response is queried during GetWorkload call. In failure cases, user friendly error message is shown in SAA details page."]
        #[serde(
            rename = "saaEnrollmentResponse",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub saa_enrollment_response: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponse,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssuredworkloadsV1Beta1Workload {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssuredworkloadsV1Beta1Workload {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime {
        #[doc = "Assured Workloads For Canada Regions and Support controls"]
        CaRegionsAndSupport,
        #[doc = "Criminal Justice Information Services (CJIS) Security policies."]
        Cjis,
        #[doc = "Unknown compliance regime."]
        ComplianceRegimeUnspecified,
        #[doc = "Assured Workloads For EU Regions and Support controls"]
        EuRegionsAndSupport,
        #[doc = "FedRAMP High data protection controls"]
        FedrampHigh,
        #[doc = "FedRAMP Moderate data protection controls"]
        FedrampModerate,
        #[doc = "Health Insurance Portability and Accountability Act controls"]
        Hipaa,
        #[doc = "Health Information Trust Alliance controls"]
        Hitrust,
        #[doc = "Information protection as per DoD IL4 requirements."]
        Il4,
        #[doc = "Assured Workloads For US Regions data protection controls"]
        UsRegionalAccess,
    }
    impl GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: CaRegionsAndSupport => "CA_REGIONS_AND_SUPPORT" , GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Cjis => "CJIS" , GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: ComplianceRegimeUnspecified => "COMPLIANCE_REGIME_UNSPECIFIED" , GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: EuRegionsAndSupport => "EU_REGIONS_AND_SUPPORT" , GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: FedrampHigh => "FEDRAMP_HIGH" , GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: FedrampModerate => "FEDRAMP_MODERATE" , GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Hipaa => "HIPAA" , GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Hitrust => "HITRUST" , GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Il4 => "IL4" , GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: UsRegionalAccess => "US_REGIONAL_ACCESS" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime, ()>
        {
            Ok (match s { "CA_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: CaRegionsAndSupport , "CJIS" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Cjis , "COMPLIANCE_REGIME_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: ComplianceRegimeUnspecified , "EU_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: EuRegionsAndSupport , "FEDRAMP_HIGH" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: FedrampHigh , "FEDRAMP_MODERATE" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: FedrampModerate , "HIPAA" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Hipaa , "HITRUST" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Hitrust , "IL4" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Il4 , "US_REGIONAL_ACCESS" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: UsRegionalAccess , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CA_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: CaRegionsAndSupport , "CJIS" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Cjis , "COMPLIANCE_REGIME_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: ComplianceRegimeUnspecified , "EU_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: EuRegionsAndSupport , "FEDRAMP_HIGH" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: FedrampHigh , "FEDRAMP_MODERATE" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: FedrampModerate , "HIPAA" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Hipaa , "HITRUST" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Hitrust , "IL4" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Il4 , "US_REGIONAL_ACCESS" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: UsRegionalAccess , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsV1Beta1WorkloadKajEnrollmentState {
        #[doc = "Complete State for KAJ Enrollment."]
        KajEnrollmentStateComplete,
        #[doc = "Pending State for KAJ Enrollment."]
        KajEnrollmentStatePending,
        #[doc = "Default State for KAJ Enrollment."]
        KajEnrollmentStateUnspecified,
    }
    impl GoogleCloudAssuredworkloadsV1Beta1WorkloadKajEnrollmentState {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsV1Beta1WorkloadKajEnrollmentState :: KajEnrollmentStateComplete => "KAJ_ENROLLMENT_STATE_COMPLETE" , GoogleCloudAssuredworkloadsV1Beta1WorkloadKajEnrollmentState :: KajEnrollmentStatePending => "KAJ_ENROLLMENT_STATE_PENDING" , GoogleCloudAssuredworkloadsV1Beta1WorkloadKajEnrollmentState :: KajEnrollmentStateUnspecified => "KAJ_ENROLLMENT_STATE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudAssuredworkloadsV1Beta1WorkloadKajEnrollmentState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudAssuredworkloadsV1Beta1WorkloadKajEnrollmentState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudAssuredworkloadsV1Beta1WorkloadKajEnrollmentState, ()>
        {
            Ok (match s { "KAJ_ENROLLMENT_STATE_COMPLETE" => GoogleCloudAssuredworkloadsV1Beta1WorkloadKajEnrollmentState :: KajEnrollmentStateComplete , "KAJ_ENROLLMENT_STATE_PENDING" => GoogleCloudAssuredworkloadsV1Beta1WorkloadKajEnrollmentState :: KajEnrollmentStatePending , "KAJ_ENROLLMENT_STATE_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1Beta1WorkloadKajEnrollmentState :: KajEnrollmentStateUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudAssuredworkloadsV1Beta1WorkloadKajEnrollmentState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudAssuredworkloadsV1Beta1WorkloadKajEnrollmentState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadKajEnrollmentState
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "KAJ_ENROLLMENT_STATE_COMPLETE" => GoogleCloudAssuredworkloadsV1Beta1WorkloadKajEnrollmentState :: KajEnrollmentStateComplete , "KAJ_ENROLLMENT_STATE_PENDING" => GoogleCloudAssuredworkloadsV1Beta1WorkloadKajEnrollmentState :: KajEnrollmentStatePending , "KAJ_ENROLLMENT_STATE_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1Beta1WorkloadKajEnrollmentState :: KajEnrollmentStateUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadKajEnrollmentState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadKajEnrollmentState
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
    pub struct GoogleCloudAssuredworkloadsV1Beta1WorkloadCJISSettings {
        #[doc = "Input only. Immutable. Settings used to create a CMEK crypto key."]
        #[serde(
            rename = "kmsSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kms_settings: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1WorkloadKMSSettings,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadCJISSettings
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadCJISSettings
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
    pub struct GoogleCloudAssuredworkloadsV1Beta1WorkloadFedrampHighSettings {
        #[doc = "Input only. Immutable. Settings used to create a CMEK crypto key."]
        #[serde(
            rename = "kmsSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kms_settings: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1WorkloadKMSSettings,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadFedrampHighSettings
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadFedrampHighSettings
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
    pub struct GoogleCloudAssuredworkloadsV1Beta1WorkloadFedrampModerateSettings {
        #[doc = "Input only. Immutable. Settings used to create a CMEK crypto key."]
        #[serde(
            rename = "kmsSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kms_settings: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1WorkloadKMSSettings,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadFedrampModerateSettings
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadFedrampModerateSettings
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
    pub struct GoogleCloudAssuredworkloadsV1Beta1WorkloadIL4Settings {
        #[doc = "Input only. Immutable. Settings used to create a CMEK crypto key."]
        #[serde(
            rename = "kmsSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kms_settings: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1WorkloadKMSSettings,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadIL4Settings
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadIL4Settings
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
    pub struct GoogleCloudAssuredworkloadsV1Beta1WorkloadKMSSettings {
        #[doc = "Required. Input only. Immutable. The time at which the Key Management Service will automatically create a new version of the crypto key and mark it as the primary."]
        #[serde(
            rename = "nextRotationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_rotation_time: ::std::option::Option<String>,
        #[doc = "Required. Input only. Immutable. [next_rotation_time] will be advanced by this period when the Key Management Service automatically rotates a key. Must be at least 24 hours and at most 876,000 hours."]
        #[serde(
            rename = "rotationPeriod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rotation_period: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadKMSSettings
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadKMSSettings
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
    pub struct GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfo {
        #[doc = "Resource identifier. For a project this represents project_number."]
        #[serde(
            rename = "resourceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub resource_id: ::std::option::Option<i64>,
        #[doc = "Indicates the type of resource."]
        #[serde(
            rename = "resourceType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_type: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfo
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfo
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType {
        #[doc = "Consumer Folder."]
        ConsumerFolder,
        #[doc = "Deprecated. Existing workloads will continue to support this, but new CreateWorkloadRequests should not specify this as an input value."]
        ConsumerProject,
        #[doc = "Consumer project containing encryption keys."]
        EncryptionKeysProject,
        #[doc = "Keyring resource that hosts encryption keys."]
        Keyring,
        #[doc = "Unknown resource type."]
        ResourceTypeUnspecified,
    }
    impl GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType :: ConsumerFolder => "CONSUMER_FOLDER" , GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType :: ConsumerProject => "CONSUMER_PROJECT" , GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType :: EncryptionKeysProject => "ENCRYPTION_KEYS_PROJECT" , GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType :: Keyring => "KEYRING" , GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType :: ResourceTypeUnspecified => "RESOURCE_TYPE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType,
            (),
        > {
            Ok (match s { "CONSUMER_FOLDER" => GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType :: ConsumerFolder , "CONSUMER_PROJECT" => GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType :: ConsumerProject , "ENCRYPTION_KEYS_PROJECT" => GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType :: EncryptionKeysProject , "KEYRING" => GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType :: Keyring , "RESOURCE_TYPE_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType :: ResourceTypeUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CONSUMER_FOLDER" => GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType :: ConsumerFolder , "CONSUMER_PROJECT" => GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType :: ConsumerProject , "ENCRYPTION_KEYS_PROJECT" => GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType :: EncryptionKeysProject , "KEYRING" => GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType :: Keyring , "RESOURCE_TYPE_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType :: ResourceTypeUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceInfoResourceType
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
    pub struct GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettings {
        #[doc = "User-assigned resource display name. If not empty it will be used to create a resource with the specified name."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Resource identifier. For a project this represents project_id. If the project is already taken, the workload creation will fail. For KeyRing, this represents the keyring_id. For a folder, don't set this value as folder_id is assigned by Google."]
        #[serde(
            rename = "resourceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_id: ::std::option::Option<String>,
        #[doc = "Indicates the type of resource. This field should be specified to correspond the id to the right project type (CONSUMER_PROJECT or ENCRYPTION_KEYS_PROJECT)"]
        #[serde(
            rename = "resourceType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_type: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettings
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettings
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType {
        #[doc = "Consumer Folder."]
        ConsumerFolder,
        #[doc = "Deprecated. Existing workloads will continue to support this, but new CreateWorkloadRequests should not specify this as an input value."]
        ConsumerProject,
        #[doc = "Consumer project containing encryption keys."]
        EncryptionKeysProject,
        #[doc = "Keyring resource that hosts encryption keys."]
        Keyring,
        #[doc = "Unknown resource type."]
        ResourceTypeUnspecified,
    }
    impl GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType :: ConsumerFolder => "CONSUMER_FOLDER" , GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType :: ConsumerProject => "CONSUMER_PROJECT" , GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType :: EncryptionKeysProject => "ENCRYPTION_KEYS_PROJECT" , GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType :: Keyring => "KEYRING" , GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType :: ResourceTypeUnspecified => "RESOURCE_TYPE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType,
            (),
        > {
            Ok (match s { "CONSUMER_FOLDER" => GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType :: ConsumerFolder , "CONSUMER_PROJECT" => GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType :: ConsumerProject , "ENCRYPTION_KEYS_PROJECT" => GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType :: EncryptionKeysProject , "KEYRING" => GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType :: Keyring , "RESOURCE_TYPE_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType :: ResourceTypeUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CONSUMER_FOLDER" => GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType :: ConsumerFolder , "CONSUMER_PROJECT" => GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType :: ConsumerProject , "ENCRYPTION_KEYS_PROJECT" => GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType :: EncryptionKeysProject , "KEYRING" => GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType :: Keyring , "RESOURCE_TYPE_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType :: ResourceTypeUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadResourceSettingsResourceType
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
    pub struct GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponse { # [doc = "Indicates SAA enrollment setup error if any."] # [serde (rename = "setupErrors" , default , skip_serializing_if = "std::option::Option::is_none")] pub setup_errors : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems > > , # [doc = "Indicates SAA enrollment status of a given workload."] # [serde (rename = "setupStatus" , default , skip_serializing_if = "std::option::Option::is_none")] pub setup_status : :: std :: option :: Option < crate :: schemas :: GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupStatus > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems {
        #[doc = "Invalid states for all customers, to be redirected to AA UI for additional details."]
        ErrorInvalidBaseSetup,
        #[doc = "Returned when there is not an EKM key configured."]
        ErrorMissingExternalSigningKey,
        #[doc = "Returned when there are no enrolled services or the customer is enrolled in CAA only for a subset of services."]
        ErrorNotAllServicesEnrolled,
        #[doc = "Returned when exception was encountered during evaluation of other criteria."]
        ErrorSetupCheckFailed,
        #[doc = "Unspecified."]
        SetupErrorUnspecified,
    }
    impl GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorInvalidBaseSetup => "ERROR_INVALID_BASE_SETUP" , GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorMissingExternalSigningKey => "ERROR_MISSING_EXTERNAL_SIGNING_KEY" , GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorNotAllServicesEnrolled => "ERROR_NOT_ALL_SERVICES_ENROLLED" , GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorSetupCheckFailed => "ERROR_SETUP_CHECK_FAILED" , GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems :: SetupErrorUnspecified => "SETUP_ERROR_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems,
            (),
        > {
            Ok (match s { "ERROR_INVALID_BASE_SETUP" => GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorInvalidBaseSetup , "ERROR_MISSING_EXTERNAL_SIGNING_KEY" => GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorMissingExternalSigningKey , "ERROR_NOT_ALL_SERVICES_ENROLLED" => GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorNotAllServicesEnrolled , "ERROR_SETUP_CHECK_FAILED" => GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorSetupCheckFailed , "SETUP_ERROR_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems :: SetupErrorUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ERROR_INVALID_BASE_SETUP" => GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorInvalidBaseSetup , "ERROR_MISSING_EXTERNAL_SIGNING_KEY" => GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorMissingExternalSigningKey , "ERROR_NOT_ALL_SERVICES_ENROLLED" => GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorNotAllServicesEnrolled , "ERROR_SETUP_CHECK_FAILED" => GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorSetupCheckFailed , "SETUP_ERROR_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems :: SetupErrorUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupErrorsItems
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupStatus {
        #[doc = "Unspecified."]
        SetupStateUnspecified,
        #[doc = "SAA enrollment comopleted."]
        StatusComplete,
        #[doc = "SAA enrollment pending."]
        StatusPending,
    }
    impl GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupStatus {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupStatus :: SetupStateUnspecified => "SETUP_STATE_UNSPECIFIED" , GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupStatus :: StatusComplete => "STATUS_COMPLETE" , GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupStatus :: StatusPending => "STATUS_PENDING" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupStatus
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupStatus
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupStatus,
            (),
        > {
            Ok (match s { "SETUP_STATE_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupStatus :: SetupStateUnspecified , "STATUS_COMPLETE" => GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupStatus :: StatusComplete , "STATUS_PENDING" => GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupStatus :: StatusPending , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupStatus
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupStatus
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupStatus
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "SETUP_STATE_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupStatus :: SetupStateUnspecified , "STATUS_COMPLETE" => GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupStatus :: StatusComplete , "STATUS_PENDING" => GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupStatus :: StatusPending , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupStatus
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadSaaEnrollmentResponseSetupStatus
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
    pub struct GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadata { # [doc = "Optional. Compliance controls that should be applied to the resources managed by the workload."] # [serde (rename = "complianceRegime" , default , skip_serializing_if = "std::option::Option::is_none")] pub compliance_regime : :: std :: option :: Option < crate :: schemas :: GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime > , # [doc = "Optional. Time when the operation was created."] # [serde (rename = "createTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub create_time : :: std :: option :: Option < String > , # [doc = "Optional. The display name of the workload."] # [serde (rename = "displayName" , default , skip_serializing_if = "std::option::Option::is_none")] pub display_name : :: std :: option :: Option < String > , # [doc = "Optional. The parent of the workload."] # [serde (rename = "parent" , default , skip_serializing_if = "std::option::Option::is_none")] pub parent : :: std :: option :: Option < String > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadata
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime {
        #[doc = "Assured Workloads For Canada Regions and Support controls"]
        CaRegionsAndSupport,
        #[doc = "Criminal Justice Information Services (CJIS) Security policies."]
        Cjis,
        #[doc = "Unknown compliance regime."]
        ComplianceRegimeUnspecified,
        #[doc = "Assured Workloads For EU Regions and Support controls"]
        EuRegionsAndSupport,
        #[doc = "FedRAMP High data protection controls"]
        FedrampHigh,
        #[doc = "FedRAMP Moderate data protection controls"]
        FedrampModerate,
        #[doc = "Health Insurance Portability and Accountability Act controls"]
        Hipaa,
        #[doc = "Health Information Trust Alliance controls"]
        Hitrust,
        #[doc = "Information protection as per DoD IL4 requirements."]
        Il4,
        #[doc = "Assured Workloads For US Regions data protection controls"]
        UsRegionalAccess,
    }
    impl GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: CaRegionsAndSupport => "CA_REGIONS_AND_SUPPORT" , GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: Cjis => "CJIS" , GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: ComplianceRegimeUnspecified => "COMPLIANCE_REGIME_UNSPECIFIED" , GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: EuRegionsAndSupport => "EU_REGIONS_AND_SUPPORT" , GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: FedrampHigh => "FEDRAMP_HIGH" , GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: FedrampModerate => "FEDRAMP_MODERATE" , GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: Hipaa => "HIPAA" , GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: Hitrust => "HITRUST" , GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: Il4 => "IL4" , GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: UsRegionalAccess => "US_REGIONAL_ACCESS" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime,
            (),
        > {
            Ok (match s { "CA_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: CaRegionsAndSupport , "CJIS" => GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: Cjis , "COMPLIANCE_REGIME_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: ComplianceRegimeUnspecified , "EU_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: EuRegionsAndSupport , "FEDRAMP_HIGH" => GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: FedrampHigh , "FEDRAMP_MODERATE" => GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: FedrampModerate , "HIPAA" => GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: Hipaa , "HITRUST" => GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: Hitrust , "IL4" => GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: Il4 , "US_REGIONAL_ACCESS" => GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: UsRegionalAccess , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CA_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: CaRegionsAndSupport , "CJIS" => GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: Cjis , "COMPLIANCE_REGIME_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: ComplianceRegimeUnspecified , "EU_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: EuRegionsAndSupport , "FEDRAMP_HIGH" => GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: FedrampHigh , "FEDRAMP_MODERATE" => GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: FedrampModerate , "HIPAA" => GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: Hipaa , "HITRUST" => GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: Hitrust , "IL4" => GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: Il4 , "US_REGIONAL_ACCESS" => GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime :: UsRegionalAccess , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1CreateWorkloadOperationMetadataComplianceRegime
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
    pub struct GoogleCloudAssuredworkloadsV1ListWorkloadsResponse {
        #[doc = "The next page token. Return empty if reached the last page."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "List of Workloads under a given parent."]
        #[serde(
            rename = "workloads",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub workloads:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudAssuredworkloadsV1Workload>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssuredworkloadsV1ListWorkloadsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssuredworkloadsV1ListWorkloadsResponse {
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
    pub struct GoogleCloudAssuredworkloadsV1Workload {
        #[doc = "Optional. The billing account used for the resources which are direct children of workload. This billing account is initially associated with the resources created as part of Workload creation. After the initial creation of these resources, the customer can change the assigned billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF`."]
        #[serde(
            rename = "billingAccount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub billing_account: ::std::option::Option<String>,
        #[doc = "Required. Immutable. Compliance Regime associated with this workload."]
        #[serde(
            rename = "complianceRegime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compliance_regime: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime,
        >,
        #[doc = "Output only. Immutable. The Workload creation timestamp."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Required. The user-assigned display name of the Workload. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, and spaces. Example: My Workload"]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Optional. Indicates the sovereignty status of the given workload. Currently meant to be used by Europe/Canada customers."]
        #[serde(
            rename = "enableSovereignControls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_sovereign_controls: ::std::option::Option<bool>,
        #[doc = "Optional. ETag of the workload, it is calculated on the basis of the Workload contents. It will be used in Update & Delete operations."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "Output only. Represents the KAJ enrollment state of the given workload."]
        #[serde(
            rename = "kajEnrollmentState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kaj_enrollment_state: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentState,
        >,
        #[doc = "Input only. Settings used to create a CMEK crypto key. When set, a project with a KMS CMEK key is provisioned. This field is deprecated as of Feb 28, 2022. In order to create a Keyring, callers should specify, ENCRYPTION_KEYS_PROJECT or KEYRING in ResourceSettings.resource_type field."]
        #[serde(
            rename = "kmsSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kms_settings:
            ::std::option::Option<crate::schemas::GoogleCloudAssuredworkloadsV1WorkloadKMSSettings>,
        #[doc = "Optional. Labels applied to the workload."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Optional. The resource name of the workload. Format: organizations/{organization}/locations/{location}/workloads/{workload} Read-only."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Input only. The parent resource for the resources managed by this Assured Workload. May be either empty or a folder resource which is a child of the Workload parent. If not specified all resources are created under the parent organization. Format: folders/{folder_id}"]
        #[serde(
            rename = "provisionedResourcesParent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provisioned_resources_parent: ::std::option::Option<String>,
        #[doc = "Input only. Resource properties that are used to customize workload resources. These properties (such as custom project id) will be used to create workload resources if possible. This field is optional."]
        #[serde(
            rename = "resourceSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_settings: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudAssuredworkloadsV1WorkloadResourceSettings>,
        >,
        #[doc = "Output only. The resources associated with this workload. These resources will be created when creating the workload. If any of the projects already exist, the workload creation will fail. Always read only."]
        #[serde(
            rename = "resources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resources: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudAssuredworkloadsV1WorkloadResourceInfo>,
        >,
        #[doc = "Output only. Represents the SAA enrollment response of the given workload. SAA enrollment response is queried during GetWorkload call. In failure cases, user friendly error message is shown in SAA details page."]
        #[serde(
            rename = "saaEnrollmentResponse",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub saa_enrollment_response: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponse,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssuredworkloadsV1Workload {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssuredworkloadsV1Workload {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime {
        #[doc = "Assured Workloads For Canada Regions and Support controls"]
        CaRegionsAndSupport,
        #[doc = "Criminal Justice Information Services (CJIS) Security policies."]
        Cjis,
        #[doc = "Unknown compliance regime."]
        ComplianceRegimeUnspecified,
        #[doc = "Assured Workloads For EU Regions and Support controls"]
        EuRegionsAndSupport,
        #[doc = "FedRAMP High data protection controls"]
        FedrampHigh,
        #[doc = "FedRAMP Moderate data protection controls"]
        FedrampModerate,
        #[doc = "Health Insurance Portability and Accountability Act controls"]
        Hipaa,
        #[doc = "Health Information Trust Alliance controls"]
        Hitrust,
        #[doc = "Information protection as per DoD IL4 requirements."]
        Il4,
        #[doc = "Assured Workloads For US Regions data protection controls"]
        UsRegionalAccess,
    }
    impl GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: CaRegionsAndSupport => "CA_REGIONS_AND_SUPPORT" , GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: Cjis => "CJIS" , GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: ComplianceRegimeUnspecified => "COMPLIANCE_REGIME_UNSPECIFIED" , GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: EuRegionsAndSupport => "EU_REGIONS_AND_SUPPORT" , GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: FedrampHigh => "FEDRAMP_HIGH" , GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: FedrampModerate => "FEDRAMP_MODERATE" , GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: Hipaa => "HIPAA" , GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: Hitrust => "HITRUST" , GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: Il4 => "IL4" , GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: UsRegionalAccess => "US_REGIONAL_ACCESS" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime, ()>
        {
            Ok (match s { "CA_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: CaRegionsAndSupport , "CJIS" => GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: Cjis , "COMPLIANCE_REGIME_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: ComplianceRegimeUnspecified , "EU_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: EuRegionsAndSupport , "FEDRAMP_HIGH" => GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: FedrampHigh , "FEDRAMP_MODERATE" => GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: FedrampModerate , "HIPAA" => GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: Hipaa , "HITRUST" => GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: Hitrust , "IL4" => GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: Il4 , "US_REGIONAL_ACCESS" => GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: UsRegionalAccess , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CA_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: CaRegionsAndSupport , "CJIS" => GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: Cjis , "COMPLIANCE_REGIME_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: ComplianceRegimeUnspecified , "EU_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: EuRegionsAndSupport , "FEDRAMP_HIGH" => GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: FedrampHigh , "FEDRAMP_MODERATE" => GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: FedrampModerate , "HIPAA" => GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: Hipaa , "HITRUST" => GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: Hitrust , "IL4" => GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: Il4 , "US_REGIONAL_ACCESS" => GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime :: UsRegionalAccess , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1WorkloadComplianceRegime
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentState {
        #[doc = "Complete State for KAJ Enrollment."]
        KajEnrollmentStateComplete,
        #[doc = "Pending State for KAJ Enrollment."]
        KajEnrollmentStatePending,
        #[doc = "Default State for KAJ Enrollment."]
        KajEnrollmentStateUnspecified,
    }
    impl GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentState {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentState :: KajEnrollmentStateComplete => "KAJ_ENROLLMENT_STATE_COMPLETE" , GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentState :: KajEnrollmentStatePending => "KAJ_ENROLLMENT_STATE_PENDING" , GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentState :: KajEnrollmentStateUnspecified => "KAJ_ENROLLMENT_STATE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentState, ()>
        {
            Ok (match s { "KAJ_ENROLLMENT_STATE_COMPLETE" => GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentState :: KajEnrollmentStateComplete , "KAJ_ENROLLMENT_STATE_PENDING" => GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentState :: KajEnrollmentStatePending , "KAJ_ENROLLMENT_STATE_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentState :: KajEnrollmentStateUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "KAJ_ENROLLMENT_STATE_COMPLETE" => GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentState :: KajEnrollmentStateComplete , "KAJ_ENROLLMENT_STATE_PENDING" => GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentState :: KajEnrollmentStatePending , "KAJ_ENROLLMENT_STATE_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentState :: KajEnrollmentStateUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1WorkloadKajEnrollmentState
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
    pub struct GoogleCloudAssuredworkloadsV1WorkloadKMSSettings {
        #[doc = "Required. Input only. Immutable. The time at which the Key Management Service will automatically create a new version of the crypto key and mark it as the primary."]
        #[serde(
            rename = "nextRotationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_rotation_time: ::std::option::Option<String>,
        #[doc = "Required. Input only. Immutable. [next_rotation_time] will be advanced by this period when the Key Management Service automatically rotates a key. Must be at least 24 hours and at most 876,000 hours."]
        #[serde(
            rename = "rotationPeriod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rotation_period: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssuredworkloadsV1WorkloadKMSSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssuredworkloadsV1WorkloadKMSSettings {
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
    pub struct GoogleCloudAssuredworkloadsV1WorkloadResourceInfo {
        #[doc = "Resource identifier. For a project this represents project_number."]
        #[serde(
            rename = "resourceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub resource_id: ::std::option::Option<i64>,
        #[doc = "Indicates the type of resource."]
        #[serde(
            rename = "resourceType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_type: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssuredworkloadsV1WorkloadResourceInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssuredworkloadsV1WorkloadResourceInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType {
        #[doc = "Consumer Folder."]
        ConsumerFolder,
        #[doc = "Consumer project. AssuredWorkloads Projects are no longer supported. This field will be ignored only in CreateWorkload requests. ListWorkloads and GetWorkload will continue to provide projects information. Use CONSUMER_FOLDER instead."]
        ConsumerProject,
        #[doc = "Consumer project containing encryption keys."]
        EncryptionKeysProject,
        #[doc = "Keyring resource that hosts encryption keys."]
        Keyring,
        #[doc = "Unknown resource type."]
        ResourceTypeUnspecified,
    }
    impl GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType :: ConsumerFolder => "CONSUMER_FOLDER" , GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType :: ConsumerProject => "CONSUMER_PROJECT" , GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType :: EncryptionKeysProject => "ENCRYPTION_KEYS_PROJECT" , GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType :: Keyring => "KEYRING" , GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType :: ResourceTypeUnspecified => "RESOURCE_TYPE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType, ()>
        {
            Ok (match s { "CONSUMER_FOLDER" => GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType :: ConsumerFolder , "CONSUMER_PROJECT" => GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType :: ConsumerProject , "ENCRYPTION_KEYS_PROJECT" => GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType :: EncryptionKeysProject , "KEYRING" => GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType :: Keyring , "RESOURCE_TYPE_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType :: ResourceTypeUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CONSUMER_FOLDER" => GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType :: ConsumerFolder , "CONSUMER_PROJECT" => GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType :: ConsumerProject , "ENCRYPTION_KEYS_PROJECT" => GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType :: EncryptionKeysProject , "KEYRING" => GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType :: Keyring , "RESOURCE_TYPE_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType :: ResourceTypeUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1WorkloadResourceInfoResourceType
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
    pub struct GoogleCloudAssuredworkloadsV1WorkloadResourceSettings {
        #[doc = "User-assigned resource display name. If not empty it will be used to create a resource with the specified name."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Resource identifier. For a project this represents project_id. If the project is already taken, the workload creation will fail. For KeyRing, this represents the keyring_id. For a folder, don't set this value as folder_id is assigned by Google."]
        #[serde(
            rename = "resourceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_id: ::std::option::Option<String>,
        #[doc = "Indicates the type of resource. This field should be specified to correspond the id to the right project type (CONSUMER_PROJECT or ENCRYPTION_KEYS_PROJECT)"]
        #[serde(
            rename = "resourceType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_type: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1WorkloadResourceSettings
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1WorkloadResourceSettings
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType {
        #[doc = "Consumer Folder."]
        ConsumerFolder,
        #[doc = "Consumer project. AssuredWorkloads Projects are no longer supported. This field will be ignored only in CreateWorkload requests. ListWorkloads and GetWorkload will continue to provide projects information. Use CONSUMER_FOLDER instead."]
        ConsumerProject,
        #[doc = "Consumer project containing encryption keys."]
        EncryptionKeysProject,
        #[doc = "Keyring resource that hosts encryption keys."]
        Keyring,
        #[doc = "Unknown resource type."]
        ResourceTypeUnspecified,
    }
    impl GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType :: ConsumerFolder => "CONSUMER_FOLDER" , GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType :: ConsumerProject => "CONSUMER_PROJECT" , GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType :: EncryptionKeysProject => "ENCRYPTION_KEYS_PROJECT" , GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType :: Keyring => "KEYRING" , GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType :: ResourceTypeUnspecified => "RESOURCE_TYPE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType,
            (),
        > {
            Ok (match s { "CONSUMER_FOLDER" => GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType :: ConsumerFolder , "CONSUMER_PROJECT" => GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType :: ConsumerProject , "ENCRYPTION_KEYS_PROJECT" => GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType :: EncryptionKeysProject , "KEYRING" => GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType :: Keyring , "RESOURCE_TYPE_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType :: ResourceTypeUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CONSUMER_FOLDER" => GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType :: ConsumerFolder , "CONSUMER_PROJECT" => GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType :: ConsumerProject , "ENCRYPTION_KEYS_PROJECT" => GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType :: EncryptionKeysProject , "KEYRING" => GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType :: Keyring , "RESOURCE_TYPE_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType :: ResourceTypeUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1WorkloadResourceSettingsResourceType
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
    pub struct GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponse { # [doc = "Indicates SAA enrollment setup error if any."] # [serde (rename = "setupErrors" , default , skip_serializing_if = "std::option::Option::is_none")] pub setup_errors : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems > > , # [doc = "Indicates SAA enrollment status of a given workload."] # [serde (rename = "setupStatus" , default , skip_serializing_if = "std::option::Option::is_none")] pub setup_status : :: std :: option :: Option < crate :: schemas :: GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatus > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems {
        #[doc = "Invalid states for all customers, to be redirected to AA UI for additional details."]
        ErrorInvalidBaseSetup,
        #[doc = "Returned when there is not an EKM key configured."]
        ErrorMissingExternalSigningKey,
        #[doc = "Returned when there are no enrolled services or the customer is enrolled in CAA only for a subset of services."]
        ErrorNotAllServicesEnrolled,
        #[doc = "Returned when exception was encountered during evaluation of other criteria."]
        ErrorSetupCheckFailed,
        #[doc = "Unspecified."]
        SetupErrorUnspecified,
    }
    impl GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorInvalidBaseSetup => "ERROR_INVALID_BASE_SETUP" , GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorMissingExternalSigningKey => "ERROR_MISSING_EXTERNAL_SIGNING_KEY" , GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorNotAllServicesEnrolled => "ERROR_NOT_ALL_SERVICES_ENROLLED" , GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorSetupCheckFailed => "ERROR_SETUP_CHECK_FAILED" , GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems :: SetupErrorUnspecified => "SETUP_ERROR_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems,
            (),
        > {
            Ok (match s { "ERROR_INVALID_BASE_SETUP" => GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorInvalidBaseSetup , "ERROR_MISSING_EXTERNAL_SIGNING_KEY" => GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorMissingExternalSigningKey , "ERROR_NOT_ALL_SERVICES_ENROLLED" => GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorNotAllServicesEnrolled , "ERROR_SETUP_CHECK_FAILED" => GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorSetupCheckFailed , "SETUP_ERROR_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems :: SetupErrorUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ERROR_INVALID_BASE_SETUP" => GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorInvalidBaseSetup , "ERROR_MISSING_EXTERNAL_SIGNING_KEY" => GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorMissingExternalSigningKey , "ERROR_NOT_ALL_SERVICES_ENROLLED" => GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorNotAllServicesEnrolled , "ERROR_SETUP_CHECK_FAILED" => GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorSetupCheckFailed , "SETUP_ERROR_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems :: SetupErrorUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupErrorsItems
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatus {
        #[doc = "Unspecified."]
        SetupStateUnspecified,
        #[doc = "SAA enrollment comopleted."]
        StatusComplete,
        #[doc = "SAA enrollment pending."]
        StatusPending,
    }
    impl GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatus {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatus :: SetupStateUnspecified => "SETUP_STATE_UNSPECIFIED" , GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatus :: StatusComplete => "STATUS_COMPLETE" , GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatus :: StatusPending => "STATUS_PENDING" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatus
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatus {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatus,
            (),
        > {
            Ok (match s { "SETUP_STATE_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatus :: SetupStateUnspecified , "STATUS_COMPLETE" => GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatus :: StatusComplete , "STATUS_PENDING" => GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatus :: StatusPending , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatus
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "SETUP_STATE_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatus :: SetupStateUnspecified , "STATUS_COMPLETE" => GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatus :: StatusComplete , "STATUS_PENDING" => GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatus :: StatusPending , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatus
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1WorkloadSaaEnrollmentResponseSetupStatus
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
    pub struct GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadata { # [doc = "Optional. Compliance controls that should be applied to the resources managed by the workload."] # [serde (rename = "complianceRegime" , default , skip_serializing_if = "std::option::Option::is_none")] pub compliance_regime : :: std :: option :: Option < crate :: schemas :: GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime > , # [doc = "Optional. Time when the operation was created."] # [serde (rename = "createTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub create_time : :: std :: option :: Option < String > , # [doc = "Optional. The display name of the workload."] # [serde (rename = "displayName" , default , skip_serializing_if = "std::option::Option::is_none")] pub display_name : :: std :: option :: Option < String > , # [doc = "Optional. The parent of the workload."] # [serde (rename = "parent" , default , skip_serializing_if = "std::option::Option::is_none")] pub parent : :: std :: option :: Option < String > , # [doc = "Optional. Resource properties in the input that are used for creating/customizing workload resources."] # [serde (rename = "resourceSettings" , default , skip_serializing_if = "std::option::Option::is_none")] pub resource_settings : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettings > > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadata
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime
    {
        #[doc = "Assured Workloads For Canada Regions and Support controls"]
        CaRegionsAndSupport,
        #[doc = "Criminal Justice Information Services (CJIS) Security policies."]
        Cjis,
        #[doc = "Unknown compliance regime."]
        ComplianceRegimeUnspecified,
        #[doc = "Assured Workloads For EU Regions and Support controls"]
        EuRegionsAndSupport,
        #[doc = "FedRAMP High data protection controls"]
        FedrampHigh,
        #[doc = "FedRAMP Moderate data protection controls"]
        FedrampModerate,
        #[doc = "Health Insurance Portability and Accountability Act controls"]
        Hipaa,
        #[doc = "Health Information Trust Alliance controls"]
        Hitrust,
        #[doc = "Information protection as per DoD IL4 requirements."]
        Il4,
        #[doc = "Assured Workloads For US Regions data protection controls"]
        UsRegionalAccess,
    }
    impl GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: CaRegionsAndSupport => "CA_REGIONS_AND_SUPPORT" , GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: Cjis => "CJIS" , GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: ComplianceRegimeUnspecified => "COMPLIANCE_REGIME_UNSPECIFIED" , GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: EuRegionsAndSupport => "EU_REGIONS_AND_SUPPORT" , GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: FedrampHigh => "FEDRAMP_HIGH" , GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: FedrampModerate => "FEDRAMP_MODERATE" , GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: Hipaa => "HIPAA" , GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: Hitrust => "HITRUST" , GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: Il4 => "IL4" , GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: UsRegionalAccess => "US_REGIONAL_ACCESS" , }
        }
    }
    impl :: std :: convert :: AsRef < str > for GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime { fn as_ref (& self) -> & str { self . as_str () } }
    impl :: std :: str :: FromStr for GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime { type Err = () ; fn from_str (s : & str) -> :: std :: result :: Result < GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime , () > { Ok (match s { "CA_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: CaRegionsAndSupport , "CJIS" => GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: Cjis , "COMPLIANCE_REGIME_UNSPECIFIED" => GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: ComplianceRegimeUnspecified , "EU_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: EuRegionsAndSupport , "FEDRAMP_HIGH" => GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: FedrampHigh , "FEDRAMP_MODERATE" => GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: FedrampModerate , "HIPAA" => GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: Hipaa , "HITRUST" => GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: Hitrust , "IL4" => GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: Il4 , "US_REGIONAL_ACCESS" => GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: UsRegionalAccess , _ => return Err (()) , }) } }
    impl :: std :: fmt :: Display for GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> :: std :: fmt :: Result { f . write_str (self . as_str ()) } }
    impl :: serde :: Serialize for GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime { fn serialize < S > (& self , serializer : S) -> :: std :: result :: Result < S :: Ok , S :: Error > where S : :: serde :: ser :: Serializer { serializer . serialize_str (self . as_str ()) } }
    impl < 'de > :: serde :: Deserialize < 'de > for GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime { fn deserialize < D > (deserializer : D) -> :: std :: result :: Result < Self , D :: Error > where D : :: serde :: de :: Deserializer < 'de > , { let value : & 'de str = < & str > :: deserialize (deserializer) ? ; Ok (match value { "CA_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: CaRegionsAndSupport , "CJIS" => GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: Cjis , "COMPLIANCE_REGIME_UNSPECIFIED" => GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: ComplianceRegimeUnspecified , "EU_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: EuRegionsAndSupport , "FEDRAMP_HIGH" => GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: FedrampHigh , "FEDRAMP_MODERATE" => GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: FedrampModerate , "HIPAA" => GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: Hipaa , "HITRUST" => GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: Hitrust , "IL4" => GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: Il4 , "US_REGIONAL_ACCESS" => GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime :: UsRegionalAccess , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , }) } }
    impl :: google_field_selector :: FieldSelector for GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime { fn fields () -> Vec < :: google_field_selector :: Field > { Vec :: new () } }
    impl :: google_field_selector :: ToFieldType for GoogleCloudAssuredworkloadsVersioningV1MainCreateWorkloadOperationMetadataComplianceRegime { fn field_type () -> :: google_field_selector :: FieldType { :: google_field_selector :: FieldType :: Leaf } }
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
    pub struct GoogleCloudAssuredworkloadsVersioningV1MainWorkload { # [doc = "Input only. The billing account used for the resources which are direct children of workload. This billing account is initially associated with the resources created as part of Workload creation. After the initial creation of these resources, the customer can change the assigned billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF`."] # [serde (rename = "billingAccount" , default , skip_serializing_if = "std::option::Option::is_none")] pub billing_account : :: std :: option :: Option < String > , # [doc = "Required. Input only. Immutable. Settings specific to resources needed for CJIS."] # [serde (rename = "cjisSettings" , default , skip_serializing_if = "std::option::Option::is_none")] pub cjis_settings : :: std :: option :: Option < crate :: schemas :: GoogleCloudAssuredworkloadsVersioningV1MainWorkloadCJISSettings > , # [doc = "Required. Immutable. Compliance Regime associated with this workload."] # [serde (rename = "complianceRegime" , default , skip_serializing_if = "std::option::Option::is_none")] pub compliance_regime : :: std :: option :: Option < crate :: schemas :: GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime > , # [doc = "Output only. Immutable. The Workload creation timestamp."] # [serde (rename = "createTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub create_time : :: std :: option :: Option < String > , # [doc = "Required. The user-assigned display name of the Workload. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, and spaces. Example: My Workload"] # [serde (rename = "displayName" , default , skip_serializing_if = "std::option::Option::is_none")] pub display_name : :: std :: option :: Option < String > , # [doc = "Optional. Indicates the sovereignty status of the given workload. Currently meant to be used by Europe/Canada customers."] # [serde (rename = "enableSovereignControls" , default , skip_serializing_if = "std::option::Option::is_none")] pub enable_sovereign_controls : :: std :: option :: Option < bool > , # [doc = "Optional. ETag of the workload, it is calculated on the basis of the Workload contents. It will be used in Update & Delete operations."] # [serde (rename = "etag" , default , skip_serializing_if = "std::option::Option::is_none")] pub etag : :: std :: option :: Option < String > , # [doc = "Required. Input only. Immutable. Settings specific to resources needed for FedRAMP High."] # [serde (rename = "fedrampHighSettings" , default , skip_serializing_if = "std::option::Option::is_none")] pub fedramp_high_settings : :: std :: option :: Option < crate :: schemas :: GoogleCloudAssuredworkloadsVersioningV1MainWorkloadFedrampHighSettings > , # [doc = "Required. Input only. Immutable. Settings specific to resources needed for FedRAMP Moderate."] # [serde (rename = "fedrampModerateSettings" , default , skip_serializing_if = "std::option::Option::is_none")] pub fedramp_moderate_settings : :: std :: option :: Option < crate :: schemas :: GoogleCloudAssuredworkloadsVersioningV1MainWorkloadFedrampModerateSettings > , # [doc = "Required. Input only. Immutable. Settings specific to resources needed for IL4."] # [serde (rename = "il4Settings" , default , skip_serializing_if = "std::option::Option::is_none")] pub il_4_settings : :: std :: option :: Option < crate :: schemas :: GoogleCloudAssuredworkloadsVersioningV1MainWorkloadIL4Settings > , # [doc = "Output only. Represents the KAJ enrollment state of the given workload."] # [serde (rename = "kajEnrollmentState" , default , skip_serializing_if = "std::option::Option::is_none")] pub kaj_enrollment_state : :: std :: option :: Option < crate :: schemas :: GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKajEnrollmentState > , # [doc = "Input only. Settings used to create a CMEK crypto key. When set, a project with a KMS CMEK key is provisioned. This field is deprecated as of Feb 28, 2022. In order to create a Keyring, callers should specify, ENCRYPTION_KEYS_PROJECT or KEYRING in ResourceSettings.resource_type field."] # [serde (rename = "kmsSettings" , default , skip_serializing_if = "std::option::Option::is_none")] pub kms_settings : :: std :: option :: Option < crate :: schemas :: GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKMSSettings > , # [doc = "Optional. Labels applied to the workload."] # [serde (rename = "labels" , default , skip_serializing_if = "std::option::Option::is_none")] pub labels : :: std :: option :: Option < :: std :: collections :: BTreeMap < String , String > > , # [doc = "Optional. The resource name of the workload. Format: organizations/{organization}/locations/{location}/workloads/{workload} Read-only."] # [serde (rename = "name" , default , skip_serializing_if = "std::option::Option::is_none")] pub name : :: std :: option :: Option < String > , # [doc = "Input only. The parent resource for the resources managed by this Assured Workload. May be either empty or a folder resource which is a child of the Workload parent. If not specified all resources are created under the parent organization. Format: folders/{folder_id}"] # [serde (rename = "provisionedResourcesParent" , default , skip_serializing_if = "std::option::Option::is_none")] pub provisioned_resources_parent : :: std :: option :: Option < String > , # [doc = "Input only. Resource properties that are used to customize workload resources. These properties (such as custom project id) will be used to create workload resources if possible. This field is optional."] # [serde (rename = "resourceSettings" , default , skip_serializing_if = "std::option::Option::is_none")] pub resource_settings : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettings > > , # [doc = "Output only. The resources associated with this workload. These resources will be created when creating the workload. If any of the projects already exist, the workload creation will fail. Always read only."] # [serde (rename = "resources" , default , skip_serializing_if = "std::option::Option::is_none")] pub resources : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfo > > , # [doc = "Output only. Represents the SAA enrollment response of the given workload. SAA enrollment response is queried during GetWorkload call. In failure cases, user friendly error message is shown in SAA details page."] # [serde (rename = "saaEnrollmentResponse" , default , skip_serializing_if = "std::option::Option::is_none")] pub saa_enrollment_response : :: std :: option :: Option < crate :: schemas :: GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponse > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkload
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssuredworkloadsVersioningV1MainWorkload {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime {
        #[doc = "Assured Workloads For Canada Regions and Support controls"]
        CaRegionsAndSupport,
        #[doc = "Criminal Justice Information Services (CJIS) Security policies."]
        Cjis,
        #[doc = "Unknown compliance regime."]
        ComplianceRegimeUnspecified,
        #[doc = "Assured Workloads For EU Regions and Support controls"]
        EuRegionsAndSupport,
        #[doc = "FedRAMP High data protection controls"]
        FedrampHigh,
        #[doc = "FedRAMP Moderate data protection controls"]
        FedrampModerate,
        #[doc = "Health Insurance Portability and Accountability Act controls"]
        Hipaa,
        #[doc = "Health Information Trust Alliance controls"]
        Hitrust,
        #[doc = "Information protection as per DoD IL4 requirements."]
        Il4,
        #[doc = "Assured Workloads For US Regions data protection controls"]
        UsRegionalAccess,
    }
    impl GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: CaRegionsAndSupport => "CA_REGIONS_AND_SUPPORT" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: Cjis => "CJIS" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: ComplianceRegimeUnspecified => "COMPLIANCE_REGIME_UNSPECIFIED" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: EuRegionsAndSupport => "EU_REGIONS_AND_SUPPORT" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: FedrampHigh => "FEDRAMP_HIGH" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: FedrampModerate => "FEDRAMP_MODERATE" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: Hipaa => "HIPAA" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: Hitrust => "HITRUST" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: Il4 => "IL4" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: UsRegionalAccess => "US_REGIONAL_ACCESS" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime,
            (),
        > {
            Ok (match s { "CA_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: CaRegionsAndSupport , "CJIS" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: Cjis , "COMPLIANCE_REGIME_UNSPECIFIED" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: ComplianceRegimeUnspecified , "EU_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: EuRegionsAndSupport , "FEDRAMP_HIGH" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: FedrampHigh , "FEDRAMP_MODERATE" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: FedrampModerate , "HIPAA" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: Hipaa , "HITRUST" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: Hitrust , "IL4" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: Il4 , "US_REGIONAL_ACCESS" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: UsRegionalAccess , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CA_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: CaRegionsAndSupport , "CJIS" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: Cjis , "COMPLIANCE_REGIME_UNSPECIFIED" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: ComplianceRegimeUnspecified , "EU_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: EuRegionsAndSupport , "FEDRAMP_HIGH" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: FedrampHigh , "FEDRAMP_MODERATE" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: FedrampModerate , "HIPAA" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: Hipaa , "HITRUST" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: Hitrust , "IL4" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: Il4 , "US_REGIONAL_ACCESS" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime :: UsRegionalAccess , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadComplianceRegime
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKajEnrollmentState {
        #[doc = "Complete State for KAJ Enrollment."]
        KajEnrollmentStateComplete,
        #[doc = "Pending State for KAJ Enrollment."]
        KajEnrollmentStatePending,
        #[doc = "Default State for KAJ Enrollment."]
        KajEnrollmentStateUnspecified,
    }
    impl GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKajEnrollmentState {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKajEnrollmentState :: KajEnrollmentStateComplete => "KAJ_ENROLLMENT_STATE_COMPLETE" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKajEnrollmentState :: KajEnrollmentStatePending => "KAJ_ENROLLMENT_STATE_PENDING" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKajEnrollmentState :: KajEnrollmentStateUnspecified => "KAJ_ENROLLMENT_STATE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKajEnrollmentState
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKajEnrollmentState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKajEnrollmentState,
            (),
        > {
            Ok (match s { "KAJ_ENROLLMENT_STATE_COMPLETE" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKajEnrollmentState :: KajEnrollmentStateComplete , "KAJ_ENROLLMENT_STATE_PENDING" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKajEnrollmentState :: KajEnrollmentStatePending , "KAJ_ENROLLMENT_STATE_UNSPECIFIED" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKajEnrollmentState :: KajEnrollmentStateUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKajEnrollmentState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKajEnrollmentState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKajEnrollmentState
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "KAJ_ENROLLMENT_STATE_COMPLETE" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKajEnrollmentState :: KajEnrollmentStateComplete , "KAJ_ENROLLMENT_STATE_PENDING" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKajEnrollmentState :: KajEnrollmentStatePending , "KAJ_ENROLLMENT_STATE_UNSPECIFIED" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKajEnrollmentState :: KajEnrollmentStateUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKajEnrollmentState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKajEnrollmentState
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
    pub struct GoogleCloudAssuredworkloadsVersioningV1MainWorkloadCJISSettings {
        #[doc = "Input only. Immutable. Settings used to create a CMEK crypto key."]
        #[serde(
            rename = "kmsSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kms_settings: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKMSSettings,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadCJISSettings
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadCJISSettings
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
    pub struct GoogleCloudAssuredworkloadsVersioningV1MainWorkloadFedrampHighSettings {
        #[doc = "Input only. Immutable. Settings used to create a CMEK crypto key."]
        #[serde(
            rename = "kmsSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kms_settings: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKMSSettings,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadFedrampHighSettings
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadFedrampHighSettings
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
    pub struct GoogleCloudAssuredworkloadsVersioningV1MainWorkloadFedrampModerateSettings {
        #[doc = "Input only. Immutable. Settings used to create a CMEK crypto key."]
        #[serde(
            rename = "kmsSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kms_settings: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKMSSettings,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadFedrampModerateSettings
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadFedrampModerateSettings
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
    pub struct GoogleCloudAssuredworkloadsVersioningV1MainWorkloadIL4Settings {
        #[doc = "Input only. Immutable. Settings used to create a CMEK crypto key."]
        #[serde(
            rename = "kmsSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kms_settings: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKMSSettings,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadIL4Settings
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadIL4Settings
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
    pub struct GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKMSSettings {
        #[doc = "Required. Input only. Immutable. The time at which the Key Management Service will automatically create a new version of the crypto key and mark it as the primary."]
        #[serde(
            rename = "nextRotationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_rotation_time: ::std::option::Option<String>,
        #[doc = "Required. Input only. Immutable. [next_rotation_time] will be advanced by this period when the Key Management Service automatically rotates a key. Must be at least 24 hours and at most 876,000 hours."]
        #[serde(
            rename = "rotationPeriod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rotation_period: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKMSSettings
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadKMSSettings
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
    pub struct GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfo { # [doc = "Resource identifier. For a project this represents project_number."] # [serde (rename = "resourceId" , default , skip_serializing_if = "std::option::Option::is_none")] # [serde (with = "crate::parsed_string")] pub resource_id : :: std :: option :: Option < i64 > , # [doc = "Indicates the type of resource."] # [serde (rename = "resourceType" , default , skip_serializing_if = "std::option::Option::is_none")] pub resource_type : :: std :: option :: Option < crate :: schemas :: GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfo
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfo
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType {
        #[doc = "Consumer Folder."]
        ConsumerFolder,
        #[doc = "Deprecated. Existing workloads will continue to support this, but new CreateWorkloadRequests should not specify this as an input value."]
        ConsumerProject,
        #[doc = "Consumer project containing encryption keys."]
        EncryptionKeysProject,
        #[doc = "Keyring resource that hosts encryption keys."]
        Keyring,
        #[doc = "Unknown resource type."]
        ResourceTypeUnspecified,
    }
    impl GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType :: ConsumerFolder => "CONSUMER_FOLDER" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType :: ConsumerProject => "CONSUMER_PROJECT" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType :: EncryptionKeysProject => "ENCRYPTION_KEYS_PROJECT" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType :: Keyring => "KEYRING" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType :: ResourceTypeUnspecified => "RESOURCE_TYPE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType,
            (),
        > {
            Ok (match s { "CONSUMER_FOLDER" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType :: ConsumerFolder , "CONSUMER_PROJECT" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType :: ConsumerProject , "ENCRYPTION_KEYS_PROJECT" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType :: EncryptionKeysProject , "KEYRING" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType :: Keyring , "RESOURCE_TYPE_UNSPECIFIED" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType :: ResourceTypeUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CONSUMER_FOLDER" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType :: ConsumerFolder , "CONSUMER_PROJECT" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType :: ConsumerProject , "ENCRYPTION_KEYS_PROJECT" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType :: EncryptionKeysProject , "KEYRING" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType :: Keyring , "RESOURCE_TYPE_UNSPECIFIED" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType :: ResourceTypeUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceInfoResourceType
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
    pub struct GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettings { # [doc = "User-assigned resource display name. If not empty it will be used to create a resource with the specified name."] # [serde (rename = "displayName" , default , skip_serializing_if = "std::option::Option::is_none")] pub display_name : :: std :: option :: Option < String > , # [doc = "Resource identifier. For a project this represents project_id. If the project is already taken, the workload creation will fail. For KeyRing, this represents the keyring_id. For a folder, don't set this value as folder_id is assigned by Google."] # [serde (rename = "resourceId" , default , skip_serializing_if = "std::option::Option::is_none")] pub resource_id : :: std :: option :: Option < String > , # [doc = "Indicates the type of resource. This field should be specified to correspond the id to the right project type (CONSUMER_PROJECT or ENCRYPTION_KEYS_PROJECT)"] # [serde (rename = "resourceType" , default , skip_serializing_if = "std::option::Option::is_none")] pub resource_type : :: std :: option :: Option < crate :: schemas :: GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettings
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettings
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType {
        #[doc = "Consumer Folder."]
        ConsumerFolder,
        #[doc = "Deprecated. Existing workloads will continue to support this, but new CreateWorkloadRequests should not specify this as an input value."]
        ConsumerProject,
        #[doc = "Consumer project containing encryption keys."]
        EncryptionKeysProject,
        #[doc = "Keyring resource that hosts encryption keys."]
        Keyring,
        #[doc = "Unknown resource type."]
        ResourceTypeUnspecified,
    }
    impl GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType :: ConsumerFolder => "CONSUMER_FOLDER" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType :: ConsumerProject => "CONSUMER_PROJECT" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType :: EncryptionKeysProject => "ENCRYPTION_KEYS_PROJECT" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType :: Keyring => "KEYRING" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType :: ResourceTypeUnspecified => "RESOURCE_TYPE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType,
            (),
        > {
            Ok (match s { "CONSUMER_FOLDER" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType :: ConsumerFolder , "CONSUMER_PROJECT" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType :: ConsumerProject , "ENCRYPTION_KEYS_PROJECT" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType :: EncryptionKeysProject , "KEYRING" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType :: Keyring , "RESOURCE_TYPE_UNSPECIFIED" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType :: ResourceTypeUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CONSUMER_FOLDER" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType :: ConsumerFolder , "CONSUMER_PROJECT" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType :: ConsumerProject , "ENCRYPTION_KEYS_PROJECT" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType :: EncryptionKeysProject , "KEYRING" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType :: Keyring , "RESOURCE_TYPE_UNSPECIFIED" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType :: ResourceTypeUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadResourceSettingsResourceType
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
    pub struct GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponse { # [doc = "Indicates SAA enrollment setup error if any."] # [serde (rename = "setupErrors" , default , skip_serializing_if = "std::option::Option::is_none")] pub setup_errors : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems > > , # [doc = "Indicates SAA enrollment status of a given workload."] # [serde (rename = "setupStatus" , default , skip_serializing_if = "std::option::Option::is_none")] pub setup_status : :: std :: option :: Option < crate :: schemas :: GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupStatus > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems {
        #[doc = "Invalid states for all customers, to be redirected to AA UI for additional details."]
        ErrorInvalidBaseSetup,
        #[doc = "Returned when there is not an EKM key configured."]
        ErrorMissingExternalSigningKey,
        #[doc = "Returned when there are no enrolled services or the customer is enrolled in CAA only for a subset of services."]
        ErrorNotAllServicesEnrolled,
        #[doc = "Returned when exception was encountered during evaluation of other criteria."]
        ErrorSetupCheckFailed,
        #[doc = "Unspecified."]
        SetupErrorUnspecified,
    }
    impl GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorInvalidBaseSetup => "ERROR_INVALID_BASE_SETUP" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorMissingExternalSigningKey => "ERROR_MISSING_EXTERNAL_SIGNING_KEY" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorNotAllServicesEnrolled => "ERROR_NOT_ALL_SERVICES_ENROLLED" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorSetupCheckFailed => "ERROR_SETUP_CHECK_FAILED" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems :: SetupErrorUnspecified => "SETUP_ERROR_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems
    {
        type Err = ();        fn from_str (s : & str) -> :: std :: result :: Result < GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems , () >{
            Ok (match s { "ERROR_INVALID_BASE_SETUP" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorInvalidBaseSetup , "ERROR_MISSING_EXTERNAL_SIGNING_KEY" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorMissingExternalSigningKey , "ERROR_NOT_ALL_SERVICES_ENROLLED" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorNotAllServicesEnrolled , "ERROR_SETUP_CHECK_FAILED" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorSetupCheckFailed , "SETUP_ERROR_UNSPECIFIED" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems :: SetupErrorUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ERROR_INVALID_BASE_SETUP" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorInvalidBaseSetup , "ERROR_MISSING_EXTERNAL_SIGNING_KEY" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorMissingExternalSigningKey , "ERROR_NOT_ALL_SERVICES_ENROLLED" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorNotAllServicesEnrolled , "ERROR_SETUP_CHECK_FAILED" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems :: ErrorSetupCheckFailed , "SETUP_ERROR_UNSPECIFIED" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems :: SetupErrorUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupErrorsItems
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupStatus {
        #[doc = "Unspecified."]
        SetupStateUnspecified,
        #[doc = "SAA enrollment comopleted."]
        StatusComplete,
        #[doc = "SAA enrollment pending."]
        StatusPending,
    }
    impl GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupStatus {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupStatus :: SetupStateUnspecified => "SETUP_STATE_UNSPECIFIED" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupStatus :: StatusComplete => "STATUS_COMPLETE" , GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupStatus :: StatusPending => "STATUS_PENDING" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupStatus
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupStatus
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupStatus,
            (),
        > {
            Ok (match s { "SETUP_STATE_UNSPECIFIED" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupStatus :: SetupStateUnspecified , "STATUS_COMPLETE" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupStatus :: StatusComplete , "STATUS_PENDING" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupStatus :: StatusPending , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupStatus
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupStatus
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupStatus
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "SETUP_STATE_UNSPECIFIED" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupStatus :: SetupStateUnspecified , "STATUS_COMPLETE" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupStatus :: StatusComplete , "STATUS_PENDING" => GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupStatus :: StatusPending , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupStatus
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsVersioningV1MainWorkloadSaaEnrollmentResponseSetupStatus
    {
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
    reqwest: ::reqwest::blocking::Client,
    auth: Box<dyn ::google_api_auth::GetAccessToken>,
}
impl Client {
    pub fn new<A>(auth: A) -> Self
    where
        A: ::google_api_auth::GetAccessToken + 'static,
    {
        Client::with_reqwest_client(
            auth,
            ::reqwest::blocking::Client::builder()
                .timeout(None)
                .build()
                .unwrap(),
        )
    }
    pub fn with_reqwest_client<A>(auth: A, reqwest: ::reqwest::blocking::Client) -> Self
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
    #[doc = "Actions that can be performed on the organizations resource"]
    pub fn organizations(&self) -> crate::resources::organizations::OrganizationsActions {
        crate::resources::organizations::OrganizationsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod organizations {
        pub mod params {}
        pub struct OrganizationsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> OrganizationsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the locations resource"]
            pub fn locations(
                &self,
            ) -> crate::resources::organizations::locations::LocationsActions {
                crate::resources::organizations::locations::LocationsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod locations {
            pub mod params {}
            pub struct LocationsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> LocationsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Actions that can be performed on the operations resource"]
                pub fn operations(
                    &self,
                ) -> crate::resources::organizations::locations::operations::OperationsActions
                {
                    crate::resources::organizations::locations::operations::OperationsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
                #[doc = "Actions that can be performed on the workloads resource"]
                pub fn workloads(
                    &self,
                ) -> crate::resources::organizations::locations::workloads::WorkloadsActions
                {
                    crate::resources::organizations::locations::workloads::WorkloadsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            pub mod operations {
                pub mod params {}
                pub struct OperationsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::blocking::Client,
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
                #[doc = "Created via [OperationsActions::get()](struct.OperationsActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://assuredworkloads.googleapis.com/".to_owned();
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
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
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
                        req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[doc = "Created via [OperationsActions::list()](struct.OperationsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                    #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                    #[doc = r" populated fields in the yielded items will be determined by the"]
                    #[doc = r" `FieldSelector` implementation."]
                    pub fn iter_operations<T>(self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_operations_with_fields(fields)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_operations_with_default_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleLongrunningOperation>
                    {
                        self.iter_operations_with_fields(None::<String>)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_operations_with_all_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleLongrunningOperation>
                    {
                        self.iter_operations_with_fields(Some("*"))
                    }
                    pub fn iter_operations_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        self.fields = Some({
                            let mut selector = concat!("nextPageToken,", "operations").to_owned();
                            let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                            if !items_fields.is_empty() {
                                selector.push_str("(");
                                selector.push_str(items_fields);
                                selector.push_str(")");
                            }
                            selector
                        });
                        crate::iter::PageItemIter::new(self, "operations")
                    }
                    pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_with_fields(fields)
                    }
                    pub fn iter_with_default_fields(
                        self,
                    ) -> crate::iter::PageIter<
                        Self,
                        crate::schemas::GoogleLongrunningListOperationsResponse,
                    > {
                        self.iter_with_fields(None::<&str>)
                    }
                    pub fn iter_with_all_fields(
                        self,
                    ) -> crate::iter::PageIter<
                        Self,
                        crate::schemas::GoogleLongrunningListOperationsResponse,
                    > {
                        self.iter_with_fields(Some("*"))
                    }
                    pub fn iter_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        let mut fields =
                            fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                        if !fields.is_empty() {
                            match fields.chars().rev().nth(0) {
                                Some(',') | None => {}
                                _ => fields.push_str(","),
                            }
                            fields.push_str("nextPageToken");
                            self.fields = Some(fields);
                        }
                        crate::iter::PageIter::new(self)
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningListOperationsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningListOperationsResponse, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://assuredworkloads.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/operations");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
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
                        req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        self._execute()
                    }
                }
            }
            pub mod workloads {
                pub mod params {}
                pub struct WorkloadsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> WorkloadsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Creates Assured Workload."]
                    pub fn create(
                        &self,
                        request: crate::schemas::GoogleCloudAssuredworkloadsV1Workload,
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
                            external_id: None,
                        }
                    }
                    #[doc = "Deletes the workload. Make sure that workload's direct children are already in a deleted state, otherwise the request will fail with a FAILED_PRECONDITION error."]
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
                            etag: None,
                        }
                    }
                    #[doc = "Gets Assured Workload associated with a CRM Node"]
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
                    #[doc = "Lists Assured Workloads under a CRM Node."]
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
                            filter: None,
                            page_size: None,
                            page_token: None,
                        }
                    }
                    #[doc = "Updates an existing workload. Currently allows updating of workload display_name and labels. For force updates don't set etag field in the Workload. Only one update operation per workload can be in progress."]
                    pub fn patch(
                        &self,
                        request: crate::schemas::GoogleCloudAssuredworkloadsV1Workload,
                        name: impl Into<String>,
                    ) -> PatchRequestBuilder {
                        PatchRequestBuilder {
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
                            update_mask: None,
                        }
                    }
                }
                #[doc = "Created via [WorkloadsActions::create()](struct.WorkloadsActions.html#method.create)"]
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudAssuredworkloadsV1Workload,
                    parent: String,
                    external_id: Option<String>,
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
                    #[doc = "Optional. A identifier associated with the workload and underlying projects which allows for the break down of billing costs for a workload. The value provided for the identifier will add a label to the workload and contained projects with the identifier as the value."]
                    pub fn external_id(mut self, value: impl Into<String>) -> Self {
                        self.external_id = Some(value.into());
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
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        let req = req.json(&self.request);
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://assuredworkloads.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/workloads");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                        req = req.query(&[("externalId", &self.external_id)]);
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
                        req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[doc = "Created via [WorkloadsActions::delete()](struct.WorkloadsActions.html#method.delete)"]
                #[derive(Debug, Clone)]
                pub struct DeleteRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    etag: Option<String>,
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
                    #[doc = "Optional. The etag of the workload. If this is provided, it must match the server's etag."]
                    pub fn etag(mut self, value: impl Into<String>) -> Self {
                        self.etag = Some(value.into());
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
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://assuredworkloads.googleapis.com/".to_owned();
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
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
                        req = req.query(&[("etag", &self.etag)]);
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
                        req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[doc = "Created via [WorkloadsActions::get()](struct.WorkloadsActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudAssuredworkloadsV1Workload, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudAssuredworkloadsV1Workload, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://assuredworkloads.googleapis.com/".to_owned();
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
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
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
                        req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[doc = "Created via [WorkloadsActions::list()](struct.WorkloadsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
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
                    #[doc = "A custom filter for filtering by properties of a workload. At this time, only filtering by labels is supported."]
                    pub fn filter(mut self, value: impl Into<String>) -> Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "Page size."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "Page token returned from previous request. Page token contains context from previous request. Page token needs to be passed in the second and following requests."]
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
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                    #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                    #[doc = r" populated fields in the yielded items will be determined by the"]
                    #[doc = r" `FieldSelector` implementation."]
                    pub fn iter_workloads<T>(self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_workloads_with_fields(fields)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_workloads_with_default_fields(
                        self,
                    ) -> crate::iter::PageItemIter<
                        Self,
                        crate::schemas::GoogleCloudAssuredworkloadsV1Workload,
                    > {
                        self.iter_workloads_with_fields(None::<String>)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_workloads_with_all_fields(
                        self,
                    ) -> crate::iter::PageItemIter<
                        Self,
                        crate::schemas::GoogleCloudAssuredworkloadsV1Workload,
                    > {
                        self.iter_workloads_with_fields(Some("*"))
                    }
                    pub fn iter_workloads_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        self.fields = Some({
                            let mut selector = concat!("nextPageToken,", "workloads").to_owned();
                            let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                            if !items_fields.is_empty() {
                                selector.push_str("(");
                                selector.push_str(items_fields);
                                selector.push_str(")");
                            }
                            selector
                        });
                        crate::iter::PageItemIter::new(self, "workloads")
                    }
                    pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_with_fields(fields)
                    }
                    pub fn iter_with_default_fields(
                        self,
                    ) -> crate::iter::PageIter<
                        Self,
                        crate::schemas::GoogleCloudAssuredworkloadsV1ListWorkloadsResponse,
                    > {
                        self.iter_with_fields(None::<&str>)
                    }
                    pub fn iter_with_all_fields(
                        self,
                    ) -> crate::iter::PageIter<
                        Self,
                        crate::schemas::GoogleCloudAssuredworkloadsV1ListWorkloadsResponse,
                    > {
                        self.iter_with_fields(Some("*"))
                    }
                    pub fn iter_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        let mut fields =
                            fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                        if !fields.is_empty() {
                            match fields.chars().rev().nth(0) {
                                Some(',') | None => {}
                                _ => fields.push_str(","),
                            }
                            fields.push_str("nextPageToken");
                            self.fields = Some(fields);
                        }
                        crate::iter::PageIter::new(self)
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudAssuredworkloadsV1ListWorkloadsResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudAssuredworkloadsV1ListWorkloadsResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://assuredworkloads.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/workloads");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
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
                        req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        self._execute()
                    }
                }
                #[doc = "Created via [WorkloadsActions::patch()](struct.WorkloadsActions.html#method.patch)"]
                #[derive(Debug, Clone)]
                pub struct PatchRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudAssuredworkloadsV1Workload,
                    name: String,
                    update_mask: Option<String>,
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
                impl<'a> PatchRequestBuilder<'a> {
                    #[doc = "Required. The list of fields to be updated."]
                    pub fn update_mask(mut self, value: impl Into<String>) -> Self {
                        self.update_mask = Some(value.into());
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
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudAssuredworkloadsV1Workload, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudAssuredworkloadsV1Workload, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        let req = req.json(&self.request);
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://assuredworkloads.googleapis.com/".to_owned();
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
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let mut req = self.reqwest.request(::reqwest::Method::PATCH, path);
                        req = req.query(&[("updateMask", &self.update_mask)]);
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
                        req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
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
    Other(Box<dyn ::std::error::Error + Send + Sync>),
}

impl Error {
    pub fn json_error(&self) -> Option<&::serde_json::Error> {
        match self {
            Error::OAuth2(_) => None,
            Error::JSON(err) => Some(err),
            Error::Reqwest { .. } => None,
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

/// Check the response to see if the status code represents an error. If so
/// convert it into the Reqwest variant of Error.
fn error_from_response(
    response: ::reqwest::blocking::Response,
) -> Result<::reqwest::blocking::Response, Error> {
    match response.error_for_status_ref() {
        Err(reqwest_err) => {
            let body = response.text().ok();
            Err(Error::Reqwest { reqwest_err, body })
        }
        Ok(_) => Ok(response),
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
        body: Box<dyn ::std::io::Read + Send>,
    }

    impl Part {
        pub(crate) fn new(
            content_type: ::mime::Mime,
            body: Box<dyn ::std::io::Read + Send>,
        ) -> Part {
            Part { content_type, body }
        }
    }

    pub(crate) struct RelatedMultiPartReader {
        state: RelatedMultiPartReaderState,
        boundary: String,
        next_body: Option<Box<dyn ::std::io::Read + Send>>,
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
            body: Box<dyn ::std::io::Read + Send>,
        },
    }

    impl ::std::io::Read for RelatedMultiPartReader {
        fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
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
                        let written = body.read(rem_buf)?;
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
            Ok(bytes_written)
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
pub mod iter {
    pub trait IterableMethod {
        fn set_page_token(&mut self, value: String);
        fn execute<T>(&mut self) -> Result<T, crate::Error>
        where
            T: ::serde::de::DeserializeOwned;
    }

    pub struct PageIter<M, T> {
        pub method: M,
        pub finished: bool,
        pub _phantom: ::std::marker::PhantomData<T>,
    }

    impl<M, T> PageIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        pub(crate) fn new(method: M) -> Self {
            PageIter {
                method,
                finished: false,
                _phantom: ::std::marker::PhantomData,
            }
        }
    }

    impl<M, T> Iterator for PageIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        type Item = Result<T, crate::Error>;

        fn next(&mut self) -> Option<Result<T, crate::Error>> {
            if self.finished {
                return None;
            }
            let paginated_result: ::serde_json::Map<String, ::serde_json::Value> =
                match self.method.execute() {
                    Ok(r) => r,
                    Err(err) => return Some(Err(err)),
                };
            if let Some(next_page_token) = paginated_result
                .get("nextPageToken")
                .and_then(|t| t.as_str())
            {
                self.method.set_page_token(next_page_token.to_owned());
            } else {
                self.finished = true;
            }

            Some(
                match ::serde_json::from_value(::serde_json::Value::Object(paginated_result)) {
                    Ok(resp) => Ok(resp),
                    Err(err) => Err(err.into()),
                },
            )
        }
    }

    pub struct PageItemIter<M, T> {
        items_field: &'static str,
        page_iter: PageIter<M, ::serde_json::Map<String, ::serde_json::Value>>,
        items: ::std::vec::IntoIter<T>,
    }

    impl<M, T> PageItemIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        pub(crate) fn new(method: M, items_field: &'static str) -> Self {
            PageItemIter {
                items_field,
                page_iter: PageIter::new(method),
                items: Vec::new().into_iter(),
            }
        }
    }

    impl<M, T> Iterator for PageItemIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        type Item = Result<T, crate::Error>;

        fn next(&mut self) -> Option<Result<T, crate::Error>> {
            loop {
                if let Some(v) = self.items.next() {
                    return Some(Ok(v));
                }

                let next_page = self.page_iter.next();
                match next_page {
                    None => return None,
                    Some(Err(err)) => return Some(Err(err)),
                    Some(Ok(next_page)) => {
                        let mut next_page: ::serde_json::Map<String, ::serde_json::Value> =
                            next_page;
                        let items_array = match next_page.remove(self.items_field) {
                            Some(items) => items,
                            None => {
                                return Some(Err(crate::Error::Other(
                                    format!("no {} field found in iter response", self.items_field)
                                        .into(),
                                )))
                            }
                        };
                        let items_vec: Result<Vec<T>, _> = ::serde_json::from_value(items_array);
                        match items_vec {
                            Ok(items) => self.items = items.into_iter(),
                            Err(err) => return Some(Err(err.into())),
                        }
                    }
                }
            }
        }
    }
}
