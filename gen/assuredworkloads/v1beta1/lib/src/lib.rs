#![allow(rustdoc::bare_urls)]
#![doc = "# Resources and Methods\n* [organizations](resources/organizations/struct.OrganizationsActions.html)\n  * [locations](resources/organizations/locations/struct.LocationsActions.html)\n    * [operations](resources/organizations/locations/operations/struct.OperationsActions.html)\n      * [*get*](resources/organizations/locations/operations/struct.GetRequestBuilder.html), [*list*](resources/organizations/locations/operations/struct.ListRequestBuilder.html)\n    * [workloads](resources/organizations/locations/workloads/struct.WorkloadsActions.html)\n      * [*create*](resources/organizations/locations/workloads/struct.CreateRequestBuilder.html), [*delete*](resources/organizations/locations/workloads/struct.DeleteRequestBuilder.html), [*get*](resources/organizations/locations/workloads/struct.GetRequestBuilder.html), [*list*](resources/organizations/locations/workloads/struct.ListRequestBuilder.html), [*patch*](resources/organizations/locations/workloads/struct.PatchRequestBuilder.html), [*restrictAllowedResources*](resources/organizations/locations/workloads/struct.RestrictAllowedResourcesRequestBuilder.html)\n      * [organizations](resources/organizations/locations/workloads/organizations/struct.OrganizationsActions.html)\n        * [locations](resources/organizations/locations/workloads/organizations/locations/struct.LocationsActions.html)\n          * [workloads](resources/organizations/locations/workloads/organizations/locations/workloads/struct.WorkloadsActions.html)\n            * [*analyzeWorkloadMove*](resources/organizations/locations/workloads/organizations/locations/workloads/struct.AnalyzeWorkloadMoveRequestBuilder.html)\n      * [violations](resources/organizations/locations/workloads/violations/struct.ViolationsActions.html)\n        * [*acknowledge*](resources/organizations/locations/workloads/violations/struct.AcknowledgeRequestBuilder.html), [*get*](resources/organizations/locations/workloads/violations/struct.GetRequestBuilder.html), [*list*](resources/organizations/locations/workloads/violations/struct.ListRequestBuilder.html)\n* [projects](resources/projects/struct.ProjectsActions.html)\n  * [organizations](resources/projects/organizations/struct.OrganizationsActions.html)\n    * [locations](resources/projects/organizations/locations/struct.LocationsActions.html)\n      * [workloads](resources/projects/organizations/locations/workloads/struct.WorkloadsActions.html)\n        * [*analyzeWorkloadMove*](resources/projects/organizations/locations/workloads/struct.AnalyzeWorkloadMoveRequestBuilder.html)\n"]
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
    pub struct GoogleCloudAssuredworkloadsV1Beta1AcknowledgeViolationRequest {
        #[doc = "Required. Business justification explaining the need for violation acknowledgement"]
        #[serde(
            rename = "comment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub comment: ::std::option::Option<String>,
        #[doc = "Optional. This field is deprecated and will be removed in future version of the API. Name of the OrgPolicy which was modified with non-compliant change and resulted in this violation. Format: projects/{project_number}/policies/{constraint_name} folders/{folder_id}/policies/{constraint_name} organizations/{organization_id}/policies/{constraint_name}"]
        #[serde(
            rename = "nonCompliantOrgPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_compliant_org_policy: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1AcknowledgeViolationRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1AcknowledgeViolationRequest
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
    pub struct GoogleCloudAssuredworkloadsV1Beta1AcknowledgeViolationResponse {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1AcknowledgeViolationResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1AcknowledgeViolationResponse
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
    pub struct GoogleCloudAssuredworkloadsV1Beta1AnalyzeWorkloadMoveResponse {
        #[doc = "A list of blockers that should be addressed before moving the source project or project-based workload to the destination folder-based workload."]
        #[serde(
            rename = "blockers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub blockers: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1AnalyzeWorkloadMoveResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1AnalyzeWorkloadMoveResponse
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
    pub struct GoogleCloudAssuredworkloadsV1Beta1ListViolationsResponse {
        #[doc = "The next page token. Returns empty if reached the last page."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "List of Violations under a Workload."]
        #[serde(
            rename = "violations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub violations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudAssuredworkloadsV1Beta1Violation>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1ListViolationsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1ListViolationsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for GoogleCloudAssuredworkloadsV1Beta1ListViolationsResponse {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
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
    pub struct GoogleCloudAssuredworkloadsV1Beta1ListWorkloadsResponse {
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
            ::std::option::Option<Vec<crate::schemas::GoogleCloudAssuredworkloadsV1Beta1Workload>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1ListWorkloadsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1ListWorkloadsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for GoogleCloudAssuredworkloadsV1Beta1ListWorkloadsResponse {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
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
    pub struct GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequest { # [doc = "Required. The type of restriction for using gcp products in the Workload environment."] # [serde (rename = "restrictionType" , default , skip_serializing_if = "std::option::Option::is_none")] pub restriction_type : :: std :: option :: Option < crate :: schemas :: GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequestRestrictionType > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequest
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequestRestrictionType {
        #[doc = "Allow the use all of all gcp products, irrespective of the compliance posture. This effectively removes gcp.restrictServiceUsage OrgPolicy on the AssuredWorkloads Folder."]
        AllowAllGcpResources,
        #[doc = "Based on Workload’s compliance regime, allowed list changes. See - https://cloud.google.com/assured-workloads/docs/supported-products for the list of supported resources."]
        AllowCompliantResources,
        #[doc = "Similar to ALLOW_COMPLIANT_RESOURCES but adds the list of compliant resources to the existing list of compliant resources. Effective org-policy of the Folder is considered to ensure there is no disruption to the existing customer workflows."]
        AppendCompliantResources,
        #[doc = "Unknown restriction type."]
        RestrictionTypeUnspecified,
    }
    impl GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequestRestrictionType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequestRestrictionType :: AllowAllGcpResources => "ALLOW_ALL_GCP_RESOURCES" , GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequestRestrictionType :: AllowCompliantResources => "ALLOW_COMPLIANT_RESOURCES" , GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequestRestrictionType :: AppendCompliantResources => "APPEND_COMPLIANT_RESOURCES" , GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequestRestrictionType :: RestrictionTypeUnspecified => "RESTRICTION_TYPE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequestRestrictionType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequestRestrictionType
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequestRestrictionType,
            (),
        > {
            Ok (match s { "ALLOW_ALL_GCP_RESOURCES" => GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequestRestrictionType :: AllowAllGcpResources , "ALLOW_COMPLIANT_RESOURCES" => GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequestRestrictionType :: AllowCompliantResources , "APPEND_COMPLIANT_RESOURCES" => GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequestRestrictionType :: AppendCompliantResources , "RESTRICTION_TYPE_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequestRestrictionType :: RestrictionTypeUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequestRestrictionType
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequestRestrictionType
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequestRestrictionType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ALLOW_ALL_GCP_RESOURCES" => GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequestRestrictionType :: AllowAllGcpResources , "ALLOW_COMPLIANT_RESOURCES" => GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequestRestrictionType :: AllowCompliantResources , "APPEND_COMPLIANT_RESOURCES" => GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequestRestrictionType :: AppendCompliantResources , "RESTRICTION_TYPE_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequestRestrictionType :: RestrictionTypeUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequestRestrictionType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequestRestrictionType
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
    pub struct GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesResponse {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesResponse
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
    pub struct GoogleCloudAssuredworkloadsV1Beta1Violation {
        #[doc = "A boolean that indicates if the violation is acknowledged"]
        #[serde(
            rename = "acknowledged",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub acknowledged: ::std::option::Option<bool>,
        #[doc = "Optional. Timestamp when this violation was acknowledged last. This will be absent when acknowledged field is marked as false."]
        #[serde(
            rename = "acknowledgementTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub acknowledgement_time: ::std::option::Option<String>,
        #[doc = "Output only. Immutable. Audit Log Link for violated resource Format: https://console.cloud.google.com/logs/query;query={logName}{protoPayload.resourceName}{timeRange}{folder}"]
        #[serde(
            rename = "auditLogLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audit_log_link: ::std::option::Option<String>,
        #[doc = "Output only. Time of the event which triggered the Violation."]
        #[serde(
            rename = "beginTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub begin_time: ::std::option::Option<String>,
        #[doc = "Output only. Category under which this violation is mapped. e.g. Location, Service Usage, Access, Encryption, etc."]
        #[serde(
            rename = "category",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category: ::std::option::Option<String>,
        #[doc = "Output only. Description for the Violation. e.g. OrgPolicy gcp.resourceLocations has non compliant value."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Output only. Immutable. Audit Log link to find business justification provided for violation exception. Format: https://console.cloud.google.com/logs/query;query={logName}{protoPayload.resourceName}{protoPayload.methodName}{timeRange}{organization}"]
        #[serde(
            rename = "exceptionAuditLogLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exception_audit_log_link: ::std::option::Option<String>,
        #[doc = "Output only. Immutable. Name of the Violation. Format: organizations/{organization}/locations/{location}/workloads/{workload_id}/violations/{violations_id}"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. Immutable. Name of the OrgPolicy which was modified with non-compliant change and resulted this violation. Format: projects/{project_number}/policies/{constraint_name} folders/{folder_id}/policies/{constraint_name} organizations/{organization_id}/policies/{constraint_name}"]
        #[serde(
            rename = "nonCompliantOrgPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_compliant_org_policy: ::std::option::Option<String>,
        #[doc = "Output only. Immutable. The org-policy-constraint that was incorrectly changed, which resulted in this violation."]
        #[serde(
            rename = "orgPolicyConstraint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub org_policy_constraint: ::std::option::Option<String>,
        #[doc = "Output only. Compliance violation remediation"]
        #[serde(
            rename = "remediation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub remediation: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1ViolationRemediation,
        >,
        #[doc = "Output only. Time of the event which fixed the Violation. If the violation is ACTIVE this will be empty."]
        #[serde(
            rename = "resolveTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resolve_time: ::std::option::Option<String>,
        #[doc = "Output only. State of the violation"]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state:
            ::std::option::Option<crate::schemas::GoogleCloudAssuredworkloadsV1Beta1ViolationState>,
        #[doc = "Output only. The last time when the Violation record was updated."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssuredworkloadsV1Beta1Violation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssuredworkloadsV1Beta1Violation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsV1Beta1ViolationState {
        #[doc = "Violation is Exception"]
        Exception,
        #[doc = "Violation is resolved."]
        Resolved,
        #[doc = "Unspecified state."]
        StateUnspecified,
        #[doc = "Violation is Unresolved"]
        Unresolved,
    }
    impl GoogleCloudAssuredworkloadsV1Beta1ViolationState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudAssuredworkloadsV1Beta1ViolationState::Exception => "EXCEPTION",
                GoogleCloudAssuredworkloadsV1Beta1ViolationState::Resolved => "RESOLVED",
                GoogleCloudAssuredworkloadsV1Beta1ViolationState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
                GoogleCloudAssuredworkloadsV1Beta1ViolationState::Unresolved => "UNRESOLVED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudAssuredworkloadsV1Beta1ViolationState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudAssuredworkloadsV1Beta1ViolationState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudAssuredworkloadsV1Beta1ViolationState, ()> {
            Ok(match s {
                "EXCEPTION" => GoogleCloudAssuredworkloadsV1Beta1ViolationState::Exception,
                "RESOLVED" => GoogleCloudAssuredworkloadsV1Beta1ViolationState::Resolved,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudAssuredworkloadsV1Beta1ViolationState::StateUnspecified
                }
                "UNRESOLVED" => GoogleCloudAssuredworkloadsV1Beta1ViolationState::Unresolved,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudAssuredworkloadsV1Beta1ViolationState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudAssuredworkloadsV1Beta1ViolationState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudAssuredworkloadsV1Beta1ViolationState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXCEPTION" => GoogleCloudAssuredworkloadsV1Beta1ViolationState::Exception,
                "RESOLVED" => GoogleCloudAssuredworkloadsV1Beta1ViolationState::Resolved,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudAssuredworkloadsV1Beta1ViolationState::StateUnspecified
                }
                "UNRESOLVED" => GoogleCloudAssuredworkloadsV1Beta1ViolationState::Unresolved,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssuredworkloadsV1Beta1ViolationState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssuredworkloadsV1Beta1ViolationState {
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
    pub struct GoogleCloudAssuredworkloadsV1Beta1ViolationRemediation {
        #[doc = "Values that can resolve the violation For example: for list org policy violations, this will either be the list of allowed or denied values"]
        #[serde(
            rename = "compliantValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compliant_values: ::std::option::Option<Vec<String>>,
        #[doc = "Required. Remediation instructions to resolve violations"]
        #[serde(
            rename = "instructions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instructions: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationInstructions,
        >,
        #[doc = "Output only. Reemediation type based on the type of org policy values violated"]
        #[serde(
            rename = "remediationType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub remediation_type: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1ViolationRemediation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1ViolationRemediation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType {
        #[doc = "Remediation type for boolean org policy"]
        RemediationBooleanOrgPolicyViolation,
        #[doc = "Remediation type for list org policy which have allowed values in the monitoring rule"]
        RemediationListAllowedValuesOrgPolicyViolation,
        #[doc = "Remediation type for list org policy which have denied values in the monitoring rule"]
        RemediationListDeniedValuesOrgPolicyViolation,
        #[doc = "Remediation type for gcp.restrictCmekCryptoKeyProjects"]
        RemediationRestrictCmekCryptoKeyProjectsOrgPolicyViolation,
        #[doc = "Unspecified remediation type"]
        RemediationTypeUnspecified,
    }
    impl GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType :: RemediationBooleanOrgPolicyViolation => "REMEDIATION_BOOLEAN_ORG_POLICY_VIOLATION" , GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType :: RemediationListAllowedValuesOrgPolicyViolation => "REMEDIATION_LIST_ALLOWED_VALUES_ORG_POLICY_VIOLATION" , GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType :: RemediationListDeniedValuesOrgPolicyViolation => "REMEDIATION_LIST_DENIED_VALUES_ORG_POLICY_VIOLATION" , GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType :: RemediationRestrictCmekCryptoKeyProjectsOrgPolicyViolation => "REMEDIATION_RESTRICT_CMEK_CRYPTO_KEY_PROJECTS_ORG_POLICY_VIOLATION" , GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType :: RemediationTypeUnspecified => "REMEDIATION_TYPE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType,
            (),
        > {
            Ok (match s { "REMEDIATION_BOOLEAN_ORG_POLICY_VIOLATION" => GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType :: RemediationBooleanOrgPolicyViolation , "REMEDIATION_LIST_ALLOWED_VALUES_ORG_POLICY_VIOLATION" => GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType :: RemediationListAllowedValuesOrgPolicyViolation , "REMEDIATION_LIST_DENIED_VALUES_ORG_POLICY_VIOLATION" => GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType :: RemediationListDeniedValuesOrgPolicyViolation , "REMEDIATION_RESTRICT_CMEK_CRYPTO_KEY_PROJECTS_ORG_POLICY_VIOLATION" => GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType :: RemediationRestrictCmekCryptoKeyProjectsOrgPolicyViolation , "REMEDIATION_TYPE_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType :: RemediationTypeUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "REMEDIATION_BOOLEAN_ORG_POLICY_VIOLATION" => GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType :: RemediationBooleanOrgPolicyViolation , "REMEDIATION_LIST_ALLOWED_VALUES_ORG_POLICY_VIOLATION" => GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType :: RemediationListAllowedValuesOrgPolicyViolation , "REMEDIATION_LIST_DENIED_VALUES_ORG_POLICY_VIOLATION" => GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType :: RemediationListDeniedValuesOrgPolicyViolation , "REMEDIATION_RESTRICT_CMEK_CRYPTO_KEY_PROJECTS_ORG_POLICY_VIOLATION" => GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType :: RemediationRestrictCmekCryptoKeyProjectsOrgPolicyViolation , "REMEDIATION_TYPE_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType :: RemediationTypeUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationRemediationType
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
    pub struct GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationInstructions { # [doc = "Remediation instructions to resolve violation via cloud console"] # [serde (rename = "consoleInstructions" , default , skip_serializing_if = "std::option::Option::is_none")] pub console_instructions : :: std :: option :: Option < crate :: schemas :: GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationInstructionsConsole > , # [doc = "Remediation instructions to resolve violation via gcloud cli"] # [serde (rename = "gcloudInstructions" , default , skip_serializing_if = "std::option::Option::is_none")] pub gcloud_instructions : :: std :: option :: Option < crate :: schemas :: GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationInstructionsGcloud > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationInstructions
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationInstructions
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
    pub struct GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationInstructionsConsole {
        #[doc = "Additional urls for more information about steps"]
        #[serde(
            rename = "additionalLinks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub additional_links: ::std::option::Option<Vec<String>>,
        #[doc = "Link to console page where violations can be resolved"]
        #[serde(
            rename = "consoleUris",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub console_uris: ::std::option::Option<Vec<String>>,
        #[doc = "Steps to resolve violation via cloud console"]
        #[serde(
            rename = "steps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub steps: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationInstructionsConsole
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationInstructionsConsole
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
    pub struct GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationInstructionsGcloud {
        #[doc = "Additional urls for more information about steps"]
        #[serde(
            rename = "additionalLinks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub additional_links: ::std::option::Option<Vec<String>>,
        #[doc = "Gcloud command to resolve violation"]
        #[serde(
            rename = "gcloudCommands",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcloud_commands: ::std::option::Option<Vec<String>>,
        #[doc = "Steps to resolve violation via gcloud cli"]
        #[serde(
            rename = "steps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub steps: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationInstructionsGcloud
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1ViolationRemediationInstructionsGcloud
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
        #[doc = "Optional. The billing account used for the resources which are direct children of workload. This billing account is initially associated with the resources created as part of Workload creation. After the initial creation of these resources, the customer can change the assigned billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF`."]
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
        #[doc = "Output only. Count of active Violations in the Workload."]
        #[serde(
            rename = "complianceStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compliance_status: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceStatus,
        >,
        #[doc = "Output only. Urls for services which are compliant for this Assured Workload, but which are currently disallowed by the ResourceUsageRestriction org policy. Invoke RestrictAllowedResources endpoint to allow your project developers to use these services in their environment.“"]
        #[serde(
            rename = "compliantButDisallowedServices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compliant_but_disallowed_services: ::std::option::Option<Vec<String>>,
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
        #[doc = "Optional. Partner regime associated with this workload."]
        #[serde(
            rename = "partner",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub partner: ::std::option::Option<
            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1WorkloadPartner,
        >,
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
        #[doc = "Assured Workloads for Partners;"]
        AssuredWorkloadsForPartners,
        #[doc = "Assured Workloads for Australia Regions and Support controls Available for public preview consumption. Don’t create production workloads."]
        AuRegionsAndUsSupport,
        #[doc = "Assured Workloads for Canada Protected B regime"]
        CaProtectedB,
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
        #[doc = "Assured Workloads for Israel"]
        IsrRegions,
        #[doc = "International Traffic in Arms Regulations"]
        Itar,
        #[doc = "Assured Workloads For US Regions data protection controls"]
        UsRegionalAccess,
    }
    impl GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: AssuredWorkloadsForPartners => "ASSURED_WORKLOADS_FOR_PARTNERS" , GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: AuRegionsAndUsSupport => "AU_REGIONS_AND_US_SUPPORT" , GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: CaProtectedB => "CA_PROTECTED_B" , GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: CaRegionsAndSupport => "CA_REGIONS_AND_SUPPORT" , GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Cjis => "CJIS" , GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: ComplianceRegimeUnspecified => "COMPLIANCE_REGIME_UNSPECIFIED" , GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: EuRegionsAndSupport => "EU_REGIONS_AND_SUPPORT" , GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: FedrampHigh => "FEDRAMP_HIGH" , GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: FedrampModerate => "FEDRAMP_MODERATE" , GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Hipaa => "HIPAA" , GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Hitrust => "HITRUST" , GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Il4 => "IL4" , GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: IsrRegions => "ISR_REGIONS" , GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Itar => "ITAR" , GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: UsRegionalAccess => "US_REGIONAL_ACCESS" , }
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
            Ok (match s { "ASSURED_WORKLOADS_FOR_PARTNERS" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: AssuredWorkloadsForPartners , "AU_REGIONS_AND_US_SUPPORT" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: AuRegionsAndUsSupport , "CA_PROTECTED_B" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: CaProtectedB , "CA_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: CaRegionsAndSupport , "CJIS" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Cjis , "COMPLIANCE_REGIME_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: ComplianceRegimeUnspecified , "EU_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: EuRegionsAndSupport , "FEDRAMP_HIGH" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: FedrampHigh , "FEDRAMP_MODERATE" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: FedrampModerate , "HIPAA" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Hipaa , "HITRUST" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Hitrust , "IL4" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Il4 , "ISR_REGIONS" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: IsrRegions , "ITAR" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Itar , "US_REGIONAL_ACCESS" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: UsRegionalAccess , _ => return Err (()) , })
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
            Ok (match value { "ASSURED_WORKLOADS_FOR_PARTNERS" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: AssuredWorkloadsForPartners , "AU_REGIONS_AND_US_SUPPORT" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: AuRegionsAndUsSupport , "CA_PROTECTED_B" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: CaProtectedB , "CA_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: CaRegionsAndSupport , "CJIS" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Cjis , "COMPLIANCE_REGIME_UNSPECIFIED" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: ComplianceRegimeUnspecified , "EU_REGIONS_AND_SUPPORT" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: EuRegionsAndSupport , "FEDRAMP_HIGH" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: FedrampHigh , "FEDRAMP_MODERATE" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: FedrampModerate , "HIPAA" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Hipaa , "HITRUST" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Hitrust , "IL4" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Il4 , "ISR_REGIONS" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: IsrRegions , "ITAR" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: Itar , "US_REGIONAL_ACCESS" => GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceRegime :: UsRegionalAccess , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
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
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssuredworkloadsV1Beta1WorkloadPartner {
        #[doc = "Enum representing S3NS partner."]
        LocalControlsByS3Ns,
        PartnerUnspecified,
        #[doc = "Enum representing T_SYSTEM partner."]
        SovereignControlsByTSystems,
    }
    impl GoogleCloudAssuredworkloadsV1Beta1WorkloadPartner {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudAssuredworkloadsV1Beta1WorkloadPartner::LocalControlsByS3Ns => {
                    "LOCAL_CONTROLS_BY_S3NS"
                }
                GoogleCloudAssuredworkloadsV1Beta1WorkloadPartner::PartnerUnspecified => {
                    "PARTNER_UNSPECIFIED"
                }
                GoogleCloudAssuredworkloadsV1Beta1WorkloadPartner::SovereignControlsByTSystems => {
                    "SOVEREIGN_CONTROLS_BY_T_SYSTEMS"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudAssuredworkloadsV1Beta1WorkloadPartner {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudAssuredworkloadsV1Beta1WorkloadPartner {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudAssuredworkloadsV1Beta1WorkloadPartner, ()> {
            Ok(match s {
                "LOCAL_CONTROLS_BY_S3NS" => {
                    GoogleCloudAssuredworkloadsV1Beta1WorkloadPartner::LocalControlsByS3Ns
                }
                "PARTNER_UNSPECIFIED" => {
                    GoogleCloudAssuredworkloadsV1Beta1WorkloadPartner::PartnerUnspecified
                }
                "SOVEREIGN_CONTROLS_BY_T_SYSTEMS" => {
                    GoogleCloudAssuredworkloadsV1Beta1WorkloadPartner::SovereignControlsByTSystems
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudAssuredworkloadsV1Beta1WorkloadPartner {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudAssuredworkloadsV1Beta1WorkloadPartner {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudAssuredworkloadsV1Beta1WorkloadPartner {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LOCAL_CONTROLS_BY_S3NS" => {
                    GoogleCloudAssuredworkloadsV1Beta1WorkloadPartner::LocalControlsByS3Ns
                }
                "PARTNER_UNSPECIFIED" => {
                    GoogleCloudAssuredworkloadsV1Beta1WorkloadPartner::PartnerUnspecified
                }
                "SOVEREIGN_CONTROLS_BY_T_SYSTEMS" => {
                    GoogleCloudAssuredworkloadsV1Beta1WorkloadPartner::SovereignControlsByTSystems
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
    impl ::google_field_selector::FieldSelector for GoogleCloudAssuredworkloadsV1Beta1WorkloadPartner {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssuredworkloadsV1Beta1WorkloadPartner {
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
    pub struct GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceStatus {
        #[doc = "Count of active Violations which are acknowledged in the Workload."]
        #[serde(
            rename = "acknowledgedViolationCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub acknowledged_violation_count: ::std::option::Option<i32>,
        #[doc = "Count of active Violations which haven’t been acknowledged."]
        #[serde(
            rename = "activeViolationCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub active_violation_count: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceStatus
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssuredworkloadsV1Beta1WorkloadComplianceStatus
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
        #[doc = "Required. Input only. Immutable. \\[next_rotation_time\\] will be advanced by this period when the Key Management Service automatically rotates a key. Must be at least 24 hours and at most 876,000 hours."]
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
        #[doc = "Resource identifier. For a project this represents project_id. If the project is already taken, the workload creation will fail. For KeyRing, this represents the keyring_id. For a folder, don’t set this value as folder_id is assigned by Google."]
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
    impl crate::GetNextPageToken<String> for GoogleLongrunningListOperationsResponse {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
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
    #[doc = "Actions that can be performed on the organizations resource"]
    pub fn organizations(&self) -> crate::resources::organizations::OrganizationsActions {
        crate::resources::organizations::OrganizationsActions {
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
}
pub mod resources {
    pub mod organizations {
        pub mod params {}
        pub struct OrganizationsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
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
                pub(crate) reqwest: &'a reqwest::Client,
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
                    #[doc = "Lists operations that match the specified filter in the request. If the server doesn’t support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id."]
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
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
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
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://assuredworkloads.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
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
                    filter: ::std::option::Option<String>,
                    page_size: ::std::option::Option<i32>,
                    page_token: ::std::option::Option<String>,
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
                    #[doc = "\nExecute the request and yield each item in the `operations` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                    pub fn stream_operations<T>(
                        self,
                    ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned
                            + ::google_field_selector::FieldSelector
                            + 'a,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: ::std::option::Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.stream_operations_with_fields(fields)
                    }
                    #[doc = "\nExecute the request and yield each item in the `operations` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                    pub fn stream_operations_with_default_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<crate::schemas::GoogleLongrunningOperation, crate::Error>,
                    > + 'a {
                        self.stream_operations_with_fields(None::<String>)
                    }
                    #[doc = "\nExecute the request and yield each item in the `operations` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                    pub fn stream_operations_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<crate::schemas::GoogleLongrunningOperation, crate::Error>,
                    > + 'a {
                        self.stream_operations_with_fields(Some("*"))
                    }
                    #[doc = "\nExecute the request and yield each item in the `operations` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                    pub fn stream_operations_with_fields<T, F>(
                        mut self,
                        fields: ::std::option::Option<F>,
                    ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + 'a,
                        F: AsRef<str>,
                    {
                        #[derive(:: serde :: Deserialize, :: serde :: Serialize)]
                        struct Page<T> {
                            #[serde(rename = "nextPageToken")]
                            pub next_page_token: ::std::option::Option<String>,
                            #[serde(rename = "operations")]
                            pub items: Vec<T>,
                        }
                        impl<T> crate::GetNextPageToken<String> for Page<T> {
                            fn next_page_token(&self) -> ::std::option::Option<String> {
                                self.next_page_token.to_owned()
                            }
                        }
                        impl<T> crate::stream::IntoPageItems for Page<T> {
                            type Items = Vec<T>;
                            fn into_page_items(self) -> Self::Items {
                                self.items
                            }
                        }
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
                        crate::stream::page_item_stream::<_, Page<T>>(self)
                    }
                    #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                    #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                    #[doc = r" token is returned."]
                    #[doc = r""]
                    #[doc = r" Requests the field given by the [`FieldSelector`] implementation from the server."]
                    #[doc = r""]
                    #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                    #[doc = r" [`FieldSelector`]: ::google_field_selector::FieldSelector"]
                    pub fn stream<T>(
                        self,
                    ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                    where
                        T: crate::GetNextPageToken<String>
                            + ::serde::de::DeserializeOwned
                            + ::google_field_selector::FieldSelector
                            + 'a,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: ::std::option::Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.stream_with_fields(fields)
                    }
                    #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                    #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                    #[doc = r" repeated until no page token is returned."]
                    #[doc = r""]
                    #[doc = r" Requests the default set of fields from the server."]
                    pub fn stream_with_default_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<
                            crate::schemas::GoogleLongrunningListOperationsResponse,
                            crate::Error,
                        >,
                    > + 'a {
                        self.stream_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                    #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                    #[doc = r" repeated until no page token is returned."]
                    #[doc = r""]
                    #[doc = r" Requests all fields from the server."]
                    pub fn stream_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<
                            crate::schemas::GoogleLongrunningListOperationsResponse,
                            crate::Error,
                        >,
                    > + 'a {
                        self.stream_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                    #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                    #[doc = r" token is returned."]
                    #[doc = r""]
                    #[doc = r" Only the given `fields` are requested from the server. If the list of fields is not"]
                    #[doc = r" empty, the `nextPageToken` field will be added to the list."]
                    #[doc = r""]
                    #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                    pub fn stream_with_fields<T, F>(
                        mut self,
                        fields: ::std::option::Option<F>,
                    ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                    where
                        T: crate::GetNextPageToken<String> + ::serde::de::DeserializeOwned + 'a,
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
                        crate::stream::page_stream(self)
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
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://assuredworkloads.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
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
                #[async_trait::async_trait]
                impl<'a> crate::stream::StreamableMethod for ListRequestBuilder<'a> {
                    type PageToken = String;
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    async fn execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: crate::GetNextPageToken<String> + ::serde::de::DeserializeOwned,
                    {
                        self._execute().await
                    }
                }
            }
            pub mod workloads {
                pub mod params {}
                pub struct WorkloadsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> WorkloadsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Creates Assured Workload."]
                    pub fn create(
                        &self,
                        request: crate::schemas::GoogleCloudAssuredworkloadsV1Beta1Workload,
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
                    #[doc = "Deletes the workload. Make sure that workload’s direct children are already in a deleted state, otherwise the request will fail with a FAILED_PRECONDITION error. In addition to assuredworkloads.workload.delete permission, the user should also have orgpolicy.policy.set permission on the deleted folder to remove Assured Workloads OrgPolicies."]
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
                    #[doc = "Updates an existing workload. Currently allows updating of workload display_name and labels. For force updates don’t set etag field in the Workload. Only one update operation per workload can be in progress."]
                    pub fn patch(
                        &self,
                        request: crate::schemas::GoogleCloudAssuredworkloadsV1Beta1Workload,
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
                    #[doc = "Restrict the list of resources allowed in the Workload environment. The current list of allowed products can be found at https://cloud.google.com/assured-workloads/docs/supported-products In addition to assuredworkloads.workload.update permission, the user should also have orgpolicy.policy.set permission on the folder resource to use this functionality."]
                    pub fn restrict_allowed_resources(
                        &self,
                        request : crate :: schemas :: GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequest,
                        name: impl Into<String>,
                    ) -> RestrictAllowedResourcesRequestBuilder {
                        RestrictAllowedResourcesRequestBuilder {
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
                    #[doc = "Actions that can be performed on the organizations resource"]                    pub fn organizations (& self) -> crate :: resources :: organizations :: locations :: workloads :: organizations :: OrganizationsActions{
                        crate :: resources :: organizations :: locations :: workloads :: organizations :: OrganizationsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                    #[doc = "Actions that can be performed on the violations resource"]                    pub fn violations (& self) -> crate :: resources :: organizations :: locations :: workloads :: violations :: ViolationsActions{
                        crate :: resources :: organizations :: locations :: workloads :: violations :: ViolationsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                }
                #[doc = "Created via [WorkloadsActions::create()](struct.WorkloadsActions.html#method.create)"]
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudAssuredworkloadsV1Beta1Workload,
                    parent: String,
                    external_id: ::std::option::Option<String>,
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
                        let mut output = "https://assuredworkloads.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
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
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                        let access_token = self
                            .auth
                            .access_token()
                            .await
                            .map_err(|err| crate::Error::OAuth2(err))?;
                        req = req.bearer_auth(access_token);
                        Ok(req)
                    }
                }
                #[doc = "Created via [WorkloadsActions::delete()](struct.WorkloadsActions.html#method.delete)"]
                #[derive(Debug, Clone)]
                pub struct DeleteRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    etag: ::std::option::Option<String>,
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
                impl<'a> DeleteRequestBuilder<'a> {
                    #[doc = "Optional. The etag of the workload. If this is provided, it must match the server’s etag."]
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
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://assuredworkloads.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
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
                        let access_token = self
                            .auth
                            .access_token()
                            .await
                            .map_err(|err| crate::Error::OAuth2(err))?;
                        req = req.bearer_auth(access_token);
                        Ok(req)
                    }
                }
                #[doc = "Created via [WorkloadsActions::get()](struct.WorkloadsActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
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
                    ) -> Result<
                        crate::schemas::GoogleCloudAssuredworkloadsV1Beta1Workload,
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
                        crate::schemas::GoogleCloudAssuredworkloadsV1Beta1Workload,
                        crate::Error,
                    > {
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
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://assuredworkloads.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
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
                #[doc = "Created via [WorkloadsActions::list()](struct.WorkloadsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
                    filter: ::std::option::Option<String>,
                    page_size: ::std::option::Option<i32>,
                    page_token: ::std::option::Option<String>,
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
                    #[doc = "\nExecute the request and yield each item in the `workloads` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                    pub fn stream_workloads<T>(
                        self,
                    ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned
                            + ::google_field_selector::FieldSelector
                            + 'a,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: ::std::option::Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.stream_workloads_with_fields(fields)
                    }
                    #[doc = "\nExecute the request and yield each item in the `workloads` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                    pub fn stream_workloads_with_default_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<
                            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1Workload,
                            crate::Error,
                        >,
                    > + 'a {
                        self.stream_workloads_with_fields(None::<String>)
                    }
                    #[doc = "\nExecute the request and yield each item in the `workloads` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                    pub fn stream_workloads_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<
                            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1Workload,
                            crate::Error,
                        >,
                    > + 'a {
                        self.stream_workloads_with_fields(Some("*"))
                    }
                    #[doc = "\nExecute the request and yield each item in the `workloads` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                    pub fn stream_workloads_with_fields<T, F>(
                        mut self,
                        fields: ::std::option::Option<F>,
                    ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + 'a,
                        F: AsRef<str>,
                    {
                        #[derive(:: serde :: Deserialize, :: serde :: Serialize)]
                        struct Page<T> {
                            #[serde(rename = "nextPageToken")]
                            pub next_page_token: ::std::option::Option<String>,
                            #[serde(rename = "workloads")]
                            pub items: Vec<T>,
                        }
                        impl<T> crate::GetNextPageToken<String> for Page<T> {
                            fn next_page_token(&self) -> ::std::option::Option<String> {
                                self.next_page_token.to_owned()
                            }
                        }
                        impl<T> crate::stream::IntoPageItems for Page<T> {
                            type Items = Vec<T>;
                            fn into_page_items(self) -> Self::Items {
                                self.items
                            }
                        }
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
                        crate::stream::page_item_stream::<_, Page<T>>(self)
                    }
                    #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                    #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                    #[doc = r" token is returned."]
                    #[doc = r""]
                    #[doc = r" Requests the field given by the [`FieldSelector`] implementation from the server."]
                    #[doc = r""]
                    #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                    #[doc = r" [`FieldSelector`]: ::google_field_selector::FieldSelector"]
                    pub fn stream<T>(
                        self,
                    ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                    where
                        T: crate::GetNextPageToken<String>
                            + ::serde::de::DeserializeOwned
                            + ::google_field_selector::FieldSelector
                            + 'a,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: ::std::option::Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.stream_with_fields(fields)
                    }
                    #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                    #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                    #[doc = r" repeated until no page token is returned."]
                    #[doc = r""]
                    #[doc = r" Requests the default set of fields from the server."]
                    pub fn stream_with_default_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<
                            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1ListWorkloadsResponse,
                            crate::Error,
                        >,
                    > + 'a {
                        self.stream_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                    #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                    #[doc = r" repeated until no page token is returned."]
                    #[doc = r""]
                    #[doc = r" Requests all fields from the server."]
                    pub fn stream_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<
                            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1ListWorkloadsResponse,
                            crate::Error,
                        >,
                    > + 'a {
                        self.stream_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                    #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                    #[doc = r" token is returned."]
                    #[doc = r""]
                    #[doc = r" Only the given `fields` are requested from the server. If the list of fields is not"]
                    #[doc = r" empty, the `nextPageToken` field will be added to the list."]
                    #[doc = r""]
                    #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                    pub fn stream_with_fields<T, F>(
                        mut self,
                        fields: ::std::option::Option<F>,
                    ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                    where
                        T: crate::GetNextPageToken<String> + ::serde::de::DeserializeOwned + 'a,
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
                        crate::stream::page_stream(self)
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
                    ) -> Result<
                        crate::schemas::GoogleCloudAssuredworkloadsV1Beta1ListWorkloadsResponse,
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
                        crate::schemas::GoogleCloudAssuredworkloadsV1Beta1ListWorkloadsResponse,
                        crate::Error,
                    > {
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
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://assuredworkloads.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
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
                #[async_trait::async_trait]
                impl<'a> crate::stream::StreamableMethod for ListRequestBuilder<'a> {
                    type PageToken = String;
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    async fn execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: crate::GetNextPageToken<String> + ::serde::de::DeserializeOwned,
                    {
                        self._execute().await
                    }
                }
                #[doc = "Created via [WorkloadsActions::patch()](struct.WorkloadsActions.html#method.patch)"]
                #[derive(Debug, Clone)]
                pub struct PatchRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudAssuredworkloadsV1Beta1Workload,
                    name: String,
                    update_mask: ::std::option::Option<String>,
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
                    ) -> Result<
                        crate::schemas::GoogleCloudAssuredworkloadsV1Beta1Workload,
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
                        crate::schemas::GoogleCloudAssuredworkloadsV1Beta1Workload,
                        crate::Error,
                    > {
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
                        let mut output = "https://assuredworkloads.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
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
                        let access_token = self
                            .auth
                            .access_token()
                            .await
                            .map_err(|err| crate::Error::OAuth2(err))?;
                        req = req.bearer_auth(access_token);
                        Ok(req)
                    }
                }
                #[doc = "Created via [WorkloadsActions::restrict_allowed_resources()](struct.WorkloadsActions.html#method.restrict_allowed_resources)"]
                #[derive(Debug, Clone)]
                pub struct RestrictAllowedResourcesRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , request : crate :: schemas :: GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesRequest , name : String , access_token : :: std :: option :: Option < String > , alt : :: std :: option :: Option < crate :: params :: Alt > , callback : :: std :: option :: Option < String > , fields : :: std :: option :: Option < String > , key : :: std :: option :: Option < String > , oauth_token : :: std :: option :: Option < String > , pretty_print : :: std :: option :: Option < bool > , quota_user : :: std :: option :: Option < String > , upload_protocol : :: std :: option :: Option < String > , upload_type : :: std :: option :: Option < String > , xgafv : :: std :: option :: Option < crate :: params :: Xgafv > , }
                impl<'a> RestrictAllowedResourcesRequestBuilder<'a> {
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
                    #[doc = r" the response resource."]                    pub async fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesResponse , crate :: Error >{
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]                    pub async fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudAssuredworkloadsV1Beta1RestrictAllowedResourcesResponse , crate :: Error >{
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
                        let mut output = "https://assuredworkloads.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":restrictAllowedResources");
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
                pub mod organizations {
                    pub mod params {}
                    pub struct OrganizationsActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> OrganizationsActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Actions that can be performed on the locations resource"]                        pub fn locations (& self) -> crate :: resources :: organizations :: locations :: workloads :: organizations :: locations :: LocationsActions{
                            crate :: resources :: organizations :: locations :: workloads :: organizations :: locations :: LocationsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
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
                            #[doc = "Actions that can be performed on the workloads resource"]                            pub fn workloads (& self) -> crate :: resources :: organizations :: locations :: workloads :: organizations :: locations :: workloads :: WorkloadsActions{
                                crate :: resources :: organizations :: locations :: workloads :: organizations :: locations :: workloads :: WorkloadsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                            }
                        }
                        pub mod workloads {
                            pub mod params {}
                            pub struct WorkloadsActions<'a> {
                                pub(crate) reqwest: &'a reqwest::Client,
                                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            }
                            impl<'a> WorkloadsActions<'a> {
                                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                                    self.auth
                                }
                                #[doc = "Analyzes a hypothetical move of a source project or project-based workload to a target (destination) folder-based workload."]
                                pub fn analyze_workload_move(
                                    &self,
                                    source: impl Into<String>,
                                    target: impl Into<String>,
                                ) -> AnalyzeWorkloadMoveRequestBuilder
                                {
                                    AnalyzeWorkloadMoveRequestBuilder {
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
                                        source: source.into(),
                                        target: target.into(),
                                        project: None,
                                    }
                                }
                            }
                            #[doc = "Created via [WorkloadsActions::analyze_workload_move()](struct.WorkloadsActions.html#method.analyze_workload_move)"]
                            #[derive(Debug, Clone)]
                            pub struct AnalyzeWorkloadMoveRequestBuilder<'a> {
                                pub(crate) reqwest: &'a ::reqwest::Client,
                                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                                source: String,
                                target: String,
                                project: ::std::option::Option<String>,
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
                            impl<'a> AnalyzeWorkloadMoveRequestBuilder<'a> {
                                #[doc = "The source type is a project. Specify the project’s relative resource name, formatted as either a project number or a project ID: “projects/{PROJECT_NUMBER}” or “projects/{PROJECT_ID}” For example: “projects/951040570662” when specifying a project number, or “projects/my-project-123” when specifying a project ID."]
                                pub fn project(mut self, value: impl Into<String>) -> Self {
                                    self.project = Some(value.into());
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
                                    T: ::serde::de::DeserializeOwned
                                        + ::google_field_selector::FieldSelector,
                                {
                                    let fields = ::google_field_selector::to_string::<T>();
                                    let fields: ::std::option::Option<String> = if fields.is_empty()
                                    {
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
                                #[doc = r" the response resource."]                                pub async fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudAssuredworkloadsV1Beta1AnalyzeWorkloadMoveResponse , crate :: Error >{
                                    self.execute_with_fields(None::<&str>).await
                                }
                                #[doc = r" Execute the given operation. This will provide a `fields`"]
                                #[doc = r" selector of `*`. This will include every attribute of the"]
                                #[doc = r" response resource and should be limited to use during"]
                                #[doc = r" development or debugging."]                                pub async fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudAssuredworkloadsV1Beta1AnalyzeWorkloadMoveResponse , crate :: Error >{
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
                                    Ok(req.send().await?.error_for_status()?.json().await?)
                                }
                                fn _path(&self) -> String {
                                    let mut output =
                                        "https://assuredworkloads.googleapis.com/".to_owned();
                                    output.push_str("v1beta1/");
                                    {
                                        let var_as_str = &self.source;
                                        output.extend(::percent_encoding::utf8_percent_encode(
                                            &var_as_str,
                                            crate::RESERVED,
                                        ));
                                    }
                                    output.push_str("/");
                                    {
                                        let var_as_str = &self.target;
                                        output.extend(::percent_encoding::utf8_percent_encode(
                                            &var_as_str,
                                            crate::RESERVED,
                                        ));
                                    }
                                    output.push_str(":analyzeWorkloadMove");
                                    output
                                }
                                async fn _request(
                                    &self,
                                    path: &str,
                                ) -> Result<::reqwest::RequestBuilder, crate::Error>
                                {
                                    let mut req =
                                        self.reqwest.request(::reqwest::Method::GET, path);
                                    req = req.query(&[("project", &self.project)]);
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
                pub mod violations {
                    pub mod params {}
                    pub struct ViolationsActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> ViolationsActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Acknowledges an existing violation. By acknowledging a violation, users acknowledge the existence of a compliance violation in their workload and decide to ignore it due to a valid business justification. Acknowledgement is a permanent operation and it cannot be reverted."]
                        pub fn acknowledge(
                            &self,
                            request : crate :: schemas :: GoogleCloudAssuredworkloadsV1Beta1AcknowledgeViolationRequest,
                            name: impl Into<String>,
                        ) -> AcknowledgeRequestBuilder {
                            AcknowledgeRequestBuilder {
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
                        #[doc = "Retrieves Assured Workload Violation based on ID."]
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
                        #[doc = "Lists the Violations in the AssuredWorkload Environment. Callers may also choose to read across multiple Workloads as per [AIP-159](https://google.aip.dev/159) by using ‘-’ (the hyphen or dash character) as a wildcard character instead of workload-id in the parent. Format `organizations/{org_id}/locations/{location}/workloads/-`"]
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
                                interval_end_time: None,
                                interval_start_time: None,
                                page_size: None,
                                page_token: None,
                            }
                        }
                    }
                    #[doc = "Created via [ViolationsActions::acknowledge()](struct.ViolationsActions.html#method.acknowledge)"]
                    #[derive(Debug, Clone)]
                    pub struct AcknowledgeRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , request : crate :: schemas :: GoogleCloudAssuredworkloadsV1Beta1AcknowledgeViolationRequest , name : String , access_token : :: std :: option :: Option < String > , alt : :: std :: option :: Option < crate :: params :: Alt > , callback : :: std :: option :: Option < String > , fields : :: std :: option :: Option < String > , key : :: std :: option :: Option < String > , oauth_token : :: std :: option :: Option < String > , pretty_print : :: std :: option :: Option < bool > , quota_user : :: std :: option :: Option < String > , upload_protocol : :: std :: option :: Option < String > , upload_type : :: std :: option :: Option < String > , xgafv : :: std :: option :: Option < crate :: params :: Xgafv > , }
                    impl<'a> AcknowledgeRequestBuilder<'a> {
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
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
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
                        #[doc = r" the response resource."]                        pub async fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudAssuredworkloadsV1Beta1AcknowledgeViolationResponse , crate :: Error >{
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]                        pub async fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudAssuredworkloadsV1Beta1AcknowledgeViolationResponse , crate :: Error >{
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
                            let mut output = "https://assuredworkloads.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str(":acknowledge");
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
                    #[doc = "Created via [ViolationsActions::get()](struct.ViolationsActions.html#method.get)"]
                    #[derive(Debug, Clone)]
                    pub struct GetRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        name: String,
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
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
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
                        ) -> Result<
                            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1Violation,
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
                            crate::schemas::GoogleCloudAssuredworkloadsV1Beta1Violation,
                            crate::Error,
                        > {
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
                            Ok(req.send().await?.error_for_status()?.json().await?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://assuredworkloads.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
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
                    #[doc = "Created via [ViolationsActions::list()](struct.ViolationsActions.html#method.list)"]
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        parent: String,
                        filter: ::std::option::Option<String>,
                        interval_end_time: ::std::option::Option<String>,
                        interval_start_time: ::std::option::Option<String>,
                        page_size: ::std::option::Option<i32>,
                        page_token: ::std::option::Option<String>,
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
                    impl<'a> ListRequestBuilder<'a> {
                        #[doc = "Optional. A custom filter for filtering by the Violations properties."]
                        pub fn filter(mut self, value: impl Into<String>) -> Self {
                            self.filter = Some(value.into());
                            self
                        }
                        #[doc = "The end of the time window."]
                        pub fn interval_end_time(mut self, value: impl Into<String>) -> Self {
                            self.interval_end_time = Some(value.into());
                            self
                        }
                        #[doc = "The start of the time window."]
                        pub fn interval_start_time(mut self, value: impl Into<String>) -> Self {
                            self.interval_start_time = Some(value.into());
                            self
                        }
                        #[doc = "Optional. Page size."]
                        pub fn page_size(mut self, value: i32) -> Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "Optional. Page token returned from previous request."]
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
                        #[doc = "\nExecute the request and yield each item in the `violations` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                        pub fn stream_violations<T>(
                            self,
                        ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector
                                + 'a,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: ::std::option::Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.stream_violations_with_fields(fields)
                        }
                        #[doc = "\nExecute the request and yield each item in the `violations` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                        pub fn stream_violations_with_default_fields(
                            self,
                        ) -> impl ::futures::Stream<
                            Item = Result<
                                crate::schemas::GoogleCloudAssuredworkloadsV1Beta1Violation,
                                crate::Error,
                            >,
                        > + 'a {
                            self.stream_violations_with_fields(None::<String>)
                        }
                        #[doc = "\nExecute the request and yield each item in the `violations` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                        pub fn stream_violations_with_all_fields(
                            self,
                        ) -> impl ::futures::Stream<
                            Item = Result<
                                crate::schemas::GoogleCloudAssuredworkloadsV1Beta1Violation,
                                crate::Error,
                            >,
                        > + 'a {
                            self.stream_violations_with_fields(Some("*"))
                        }
                        #[doc = "\nExecute the request and yield each item in the `violations` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                        pub fn stream_violations_with_fields<T, F>(
                            mut self,
                            fields: ::std::option::Option<F>,
                        ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                        where
                            T: ::serde::de::DeserializeOwned + 'a,
                            F: AsRef<str>,
                        {
                            #[derive(:: serde :: Deserialize, :: serde :: Serialize)]
                            struct Page<T> {
                                #[serde(rename = "nextPageToken")]
                                pub next_page_token: ::std::option::Option<String>,
                                #[serde(rename = "violations")]
                                pub items: Vec<T>,
                            }
                            impl<T> crate::GetNextPageToken<String> for Page<T> {
                                fn next_page_token(&self) -> ::std::option::Option<String> {
                                    self.next_page_token.to_owned()
                                }
                            }
                            impl<T> crate::stream::IntoPageItems for Page<T> {
                                type Items = Vec<T>;
                                fn into_page_items(self) -> Self::Items {
                                    self.items
                                }
                            }
                            self.fields = Some({
                                let mut selector =
                                    concat!("nextPageToken,", "violations").to_owned();
                                let items_fields =
                                    fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                                if !items_fields.is_empty() {
                                    selector.push_str("(");
                                    selector.push_str(items_fields);
                                    selector.push_str(")");
                                }
                                selector
                            });
                            crate::stream::page_item_stream::<_, Page<T>>(self)
                        }
                        #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                        #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                        #[doc = r" token is returned."]
                        #[doc = r""]
                        #[doc = r" Requests the field given by the [`FieldSelector`] implementation from the server."]
                        #[doc = r""]
                        #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                        #[doc = r" [`FieldSelector`]: ::google_field_selector::FieldSelector"]
                        pub fn stream<T>(
                            self,
                        ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                        where
                            T: crate::GetNextPageToken<String>
                                + ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector
                                + 'a,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: ::std::option::Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.stream_with_fields(fields)
                        }
                        #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                        #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                        #[doc = r" repeated until no page token is returned."]
                        #[doc = r""]
                        #[doc = r" Requests the default set of fields from the server."]                        pub fn stream_with_default_fields (self) -> impl :: futures :: Stream < Item = Result < crate :: schemas :: GoogleCloudAssuredworkloadsV1Beta1ListViolationsResponse , crate :: Error >> + 'a{
                            self.stream_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                        #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                        #[doc = r" repeated until no page token is returned."]
                        #[doc = r""]
                        #[doc = r" Requests all fields from the server."]                        pub fn stream_with_all_fields (self) -> impl :: futures :: Stream < Item = Result < crate :: schemas :: GoogleCloudAssuredworkloadsV1Beta1ListViolationsResponse , crate :: Error >> + 'a{
                            self.stream_with_fields(Some("*"))
                        }
                        #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                        #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                        #[doc = r" token is returned."]
                        #[doc = r""]
                        #[doc = r" Only the given `fields` are requested from the server. If the list of fields is not"]
                        #[doc = r" empty, the `nextPageToken` field will be added to the list."]
                        #[doc = r""]
                        #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                        pub fn stream_with_fields<T, F>(
                            mut self,
                            fields: ::std::option::Option<F>,
                        ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                        where
                            T: crate::GetNextPageToken<String> + ::serde::de::DeserializeOwned + 'a,
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
                            crate::stream::page_stream(self)
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
                        #[doc = r" the response resource."]                        pub async fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudAssuredworkloadsV1Beta1ListViolationsResponse , crate :: Error >{
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]                        pub async fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudAssuredworkloadsV1Beta1ListViolationsResponse , crate :: Error >{
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
                            Ok(req.send().await?.error_for_status()?.json().await?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://assuredworkloads.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/violations");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                            req = req.query(&[("filter", &self.filter)]);
                            req = req.query(&[("interval.endTime", &self.interval_end_time)]);
                            req = req.query(&[("interval.startTime", &self.interval_start_time)]);
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
                    #[async_trait::async_trait]
                    impl<'a> crate::stream::StreamableMethod for ListRequestBuilder<'a> {
                        type PageToken = String;
                        fn set_page_token(&mut self, value: String) {
                            self.page_token = value.into();
                        }
                        async fn execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: crate::GetNextPageToken<String> + ::serde::de::DeserializeOwned,
                        {
                            self._execute().await
                        }
                    }
                }
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
            #[doc = "Actions that can be performed on the organizations resource"]
            pub fn organizations(
                &self,
            ) -> crate::resources::projects::organizations::OrganizationsActions {
                crate::resources::projects::organizations::OrganizationsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod organizations {
            pub mod params {}
            pub struct OrganizationsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> OrganizationsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Actions that can be performed on the locations resource"]
                pub fn locations(
                    &self,
                ) -> crate::resources::projects::organizations::locations::LocationsActions
                {
                    crate::resources::projects::organizations::locations::LocationsActions {
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
                    #[doc = "Actions that can be performed on the workloads resource"]                    pub fn workloads (& self) -> crate :: resources :: projects :: organizations :: locations :: workloads :: WorkloadsActions{
                        crate :: resources :: projects :: organizations :: locations :: workloads :: WorkloadsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                }
                pub mod workloads {
                    pub mod params {}
                    pub struct WorkloadsActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> WorkloadsActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Analyzes a hypothetical move of a source project or project-based workload to a target (destination) folder-based workload."]
                        pub fn analyze_workload_move(
                            &self,
                            project: impl Into<String>,
                            target: impl Into<String>,
                        ) -> AnalyzeWorkloadMoveRequestBuilder {
                            AnalyzeWorkloadMoveRequestBuilder {
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
                                project: project.into(),
                                target: target.into(),
                                source: None,
                            }
                        }
                    }
                    #[doc = "Created via [WorkloadsActions::analyze_workload_move()](struct.WorkloadsActions.html#method.analyze_workload_move)"]
                    #[derive(Debug, Clone)]
                    pub struct AnalyzeWorkloadMoveRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        project: String,
                        target: String,
                        source: ::std::option::Option<String>,
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
                    impl<'a> AnalyzeWorkloadMoveRequestBuilder<'a> {
                        #[doc = "The source type is a project-based workload. Specify the workloads’s relative resource name, formatted as: “organizations/{ORGANIZATION_ID}/locations/{LOCATION_ID}/workloads/{WORKLOAD_ID}” For example: “organizations/123/locations/us-east1/workloads/assured-workload-1”"]
                        pub fn source(mut self, value: impl Into<String>) -> Self {
                            self.source = Some(value.into());
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
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
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
                        #[doc = r" the response resource."]                        pub async fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudAssuredworkloadsV1Beta1AnalyzeWorkloadMoveResponse , crate :: Error >{
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]                        pub async fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudAssuredworkloadsV1Beta1AnalyzeWorkloadMoveResponse , crate :: Error >{
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
                            Ok(req.send().await?.error_for_status()?.json().await?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://assuredworkloads.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            {
                                let var_as_str = &self.project;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/");
                            {
                                let var_as_str = &self.target;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str(":analyzeWorkloadMove");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                            req = req.query(&[("source", &self.source)]);
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
/// Represent the ability to extract the `nextPageToken` from a response.
pub trait GetNextPageToken<T> {
    /// Get the `nextPageToken` from a response if present.
    fn next_page_token(&self) -> ::std::option::Option<T>;
}

impl<T: ::std::convert::From<::std::string::String>> GetNextPageToken<T>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn next_page_token(&self) -> ::std::option::Option<T> {
        self.get("nextPageToken")
            .and_then(|t| t.as_str())
            .map(|s| s.to_owned().into())
    }
}
/// Traits and functions to improve streamable (multiple page) API method handling.
pub mod stream {
    use super::GetNextPageToken;

    /// Extract the items embedded in a page like response.
    pub trait IntoPageItems {
        /// Type of the items list in the page.
        type Items: IntoIterator;

        /// Consume the response and return the embedded items.
        fn into_page_items(self) -> Self::Items;
    }

    /// Represent a API method which can be invoked multiple times to retrieve
    /// multiple pages of items.
    #[async_trait::async_trait]
    pub trait StreamableMethod {
        /// Type of the `pageToken` and `nextPageToken` fields.
        type PageToken;

        /// Update the current page token of the request.
        fn set_page_token(&mut self, value: Self::PageToken);

        /// Execute the request.
        async fn execute<T>(&mut self) -> Result<T, crate::Error>
        where
            T: GetNextPageToken<Self::PageToken> + ::serde::de::DeserializeOwned;
    }

    /// Return a [`Stream`](::futures::Stream) over all pages of the given API
    /// method.
    pub fn page_stream<M, T>(method: M) -> impl ::futures::Stream<Item = Result<T, crate::Error>>
    where
        M: StreamableMethod,
        T: GetNextPageToken<M::PageToken> + ::serde::de::DeserializeOwned,
    {
        ::futures::stream::unfold((method, false), |(mut method, mut finished)| async move {
            if finished {
                return None;
            }
            let response = match method.execute::<T>().await {
                Ok(r) => r,
                Err(err) => return Some((Err(err), (method, false))),
            };
            if let Some(next_page_token) = response.next_page_token() {
                method.set_page_token(next_page_token);
            } else {
                finished = true;
            }

            Some((Ok(response), (method, finished)))
        })
    }

    /// Return a [`Stream`](::futures::Stream) over the items in all pages of
    /// the given API method.
    pub fn page_item_stream<M, T>(
        method: M,
    ) -> impl ::futures::Stream<Item = Result<<T::Items as IntoIterator>::Item, crate::Error>>
    where
        M: StreamableMethod,
        T: GetNextPageToken<M::PageToken> + ::serde::de::DeserializeOwned + IntoPageItems,
    {
        use ::futures::StreamExt;
        use ::futures::TryStreamExt;

        page_stream::<M, T>(method)
            .map_ok(|page| ::futures::stream::iter(page.into_page_items()).map(Ok))
            .try_flatten()
    }
}
