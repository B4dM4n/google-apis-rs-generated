#![allow(rustdoc::bare_urls)]
#![doc = "# Resources and Methods\n* [assets](resources/assets/struct.AssetsActions.html)\n  * [*list*](resources/assets/struct.ListRequestBuilder.html)\n* [effective_iam_policies](resources/effective_iam_policies/struct.EffectiveIamPoliciesActions.html)\n  * [*batchGet*](resources/effective_iam_policies/struct.BatchGetRequestBuilder.html)\n* [feeds](resources/feeds/struct.FeedsActions.html)\n  * [*create*](resources/feeds/struct.CreateRequestBuilder.html), [*delete*](resources/feeds/struct.DeleteRequestBuilder.html), [*get*](resources/feeds/struct.GetRequestBuilder.html), [*list*](resources/feeds/struct.ListRequestBuilder.html), [*patch*](resources/feeds/struct.PatchRequestBuilder.html)\n* [operations](resources/operations/struct.OperationsActions.html)\n  * [*get*](resources/operations/struct.GetRequestBuilder.html)\n* [saved_queries](resources/saved_queries/struct.SavedQueriesActions.html)\n  * [*create*](resources/saved_queries/struct.CreateRequestBuilder.html), [*delete*](resources/saved_queries/struct.DeleteRequestBuilder.html), [*get*](resources/saved_queries/struct.GetRequestBuilder.html), [*list*](resources/saved_queries/struct.ListRequestBuilder.html), [*patch*](resources/saved_queries/struct.PatchRequestBuilder.html)\n* [v_1](resources/v_1/struct.V1Actions.html)\n  * [*analyzeIamPolicy*](resources/v_1/struct.AnalyzeIamPolicyRequestBuilder.html), [*analyzeIamPolicyLongrunning*](resources/v_1/struct.AnalyzeIamPolicyLongrunningRequestBuilder.html), [*analyzeMove*](resources/v_1/struct.AnalyzeMoveRequestBuilder.html), [*analyzeOrgPolicies*](resources/v_1/struct.AnalyzeOrgPoliciesRequestBuilder.html), [*analyzeOrgPolicyGovernedAssets*](resources/v_1/struct.AnalyzeOrgPolicyGovernedAssetsRequestBuilder.html), [*analyzeOrgPolicyGovernedContainers*](resources/v_1/struct.AnalyzeOrgPolicyGovernedContainersRequestBuilder.html), [*batchGetAssetsHistory*](resources/v_1/struct.BatchGetAssetsHistoryRequestBuilder.html), [*exportAssets*](resources/v_1/struct.ExportAssetsRequestBuilder.html), [*queryAssets*](resources/v_1/struct.QueryAssetsRequestBuilder.html), [*searchAllIamPolicies*](resources/v_1/struct.SearchAllIamPoliciesRequestBuilder.html), [*searchAllResources*](resources/v_1/struct.SearchAllResourcesRequestBuilder.html)\n"]
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
    pub struct AccessSelector {
        #[doc = "Optional. The permissions to appear in result."]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The roles to appear in result."]
        #[serde(
            rename = "roles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub roles: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for AccessSelector {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AccessSelector {
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
    pub struct AnalyzeIamPolicyLongrunningMetadata {
        #[doc = "Output only. The time the operation was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AnalyzeIamPolicyLongrunningMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnalyzeIamPolicyLongrunningMetadata {
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
    pub struct AnalyzeIamPolicyLongrunningRequest {
        #[doc = "Required. The request query."]
        #[serde(
            rename = "analysisQuery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_query: ::std::option::Option<crate::schemas::IamPolicyAnalysisQuery>,
        #[doc = "Required. Output configuration indicating where the results will be output to."]
        #[serde(
            rename = "outputConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_config: ::std::option::Option<crate::schemas::IamPolicyAnalysisOutputConfig>,
        #[doc = "Optional. The name of a saved query, which must be in the format of: * projects/project_number/savedQueries/saved_query_id * folders/folder_number/savedQueries/saved_query_id * organizations/organization_number/savedQueries/saved_query_id If both `analysis_query` and `saved_analysis_query` are provided, they will be merged together with the `saved_analysis_query` as base and the `analysis_query` as overrides. For more details of the merge behavior, please refer to the [MergeFrom](https://developers.google.com/protocol-buffers/docs/reference/cpp/google.protobuf.message#Message.MergeFrom.details) doc. Note that you cannot override primitive fields with default value, such as 0 or empty string, etc., because we use proto3, which doesn’t support field presence yet."]
        #[serde(
            rename = "savedAnalysisQuery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub saved_analysis_query: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AnalyzeIamPolicyLongrunningRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnalyzeIamPolicyLongrunningRequest {
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
    pub struct AnalyzeIamPolicyLongrunningResponse {}
    impl ::google_field_selector::FieldSelector for AnalyzeIamPolicyLongrunningResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnalyzeIamPolicyLongrunningResponse {
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
    pub struct AnalyzeIamPolicyResponse {
        #[doc = "Represents whether all entries in the main_analysis and service_account_impersonation_analysis have been fully explored to answer the query in the request."]
        #[serde(
            rename = "fullyExplored",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fully_explored: ::std::option::Option<bool>,
        #[doc = "The main analysis that matches the original request."]
        #[serde(
            rename = "mainAnalysis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub main_analysis: ::std::option::Option<crate::schemas::IamPolicyAnalysis>,
        #[doc = "The service account impersonation analysis if AnalyzeIamPolicyRequest.analyze_service_account_impersonation is enabled."]
        #[serde(
            rename = "serviceAccountImpersonationAnalysis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_account_impersonation_analysis:
            ::std::option::Option<Vec<crate::schemas::IamPolicyAnalysis>>,
    }
    impl ::google_field_selector::FieldSelector for AnalyzeIamPolicyResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnalyzeIamPolicyResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct AnalyzeMoveResponse {
        #[doc = "The list of analyses returned from performing the intended resource move analysis. The analysis is grouped by different Google Cloud services."]
        #[serde(
            rename = "moveAnalysis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub move_analysis: ::std::option::Option<Vec<crate::schemas::MoveAnalysis>>,
    }
    impl ::google_field_selector::FieldSelector for AnalyzeMoveResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnalyzeMoveResponse {
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
    pub struct AnalyzeOrgPoliciesResponse {
        #[doc = "The definition of the constraint in the request."]
        #[serde(
            rename = "constraint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub constraint: ::std::option::Option<crate::schemas::AnalyzerOrgPolicyConstraint>,
        #[doc = "The page token to fetch the next page for AnalyzeOrgPoliciesResponse.org_policy_results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The organization policies under the AnalyzeOrgPoliciesRequest.scope with the AnalyzeOrgPoliciesRequest.constraint."]
        #[serde(
            rename = "orgPolicyResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub org_policy_results: ::std::option::Option<Vec<crate::schemas::OrgPolicyResult>>,
    }
    impl ::google_field_selector::FieldSelector for AnalyzeOrgPoliciesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnalyzeOrgPoliciesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for AnalyzeOrgPoliciesResponse {
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
    pub struct AnalyzeOrgPolicyGovernedAssetsResponse { # [doc = "The definition of the constraint in the request."] # [serde (rename = "constraint" , default , skip_serializing_if = "std::option::Option::is_none")] pub constraint : :: std :: option :: Option < crate :: schemas :: AnalyzerOrgPolicyConstraint > , # [doc = "The list of the analyzed governed assets."] # [serde (rename = "governedAssets" , default , skip_serializing_if = "std::option::Option::is_none")] pub governed_assets : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudAssetV1AnalyzeOrgPolicyGovernedAssetsResponseGovernedAsset > > , # [doc = "The page token to fetch the next page for AnalyzeOrgPolicyGovernedAssetsResponse.governed_assets."] # [serde (rename = "nextPageToken" , default , skip_serializing_if = "std::option::Option::is_none")] pub next_page_token : :: std :: option :: Option < String > , }
    impl ::google_field_selector::FieldSelector for AnalyzeOrgPolicyGovernedAssetsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnalyzeOrgPolicyGovernedAssetsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for AnalyzeOrgPolicyGovernedAssetsResponse {
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
    pub struct AnalyzeOrgPolicyGovernedContainersResponse {
        #[doc = "The definition of the constraint in the request."]
        #[serde(
            rename = "constraint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub constraint: ::std::option::Option<crate::schemas::AnalyzerOrgPolicyConstraint>,
        #[doc = "The list of the analyzed governed containers."]
        #[serde(
            rename = "governedContainers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub governed_containers:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudAssetV1GovernedContainer>>,
        #[doc = "The page token to fetch the next page for AnalyzeOrgPolicyGovernedContainersResponse.governed_containers."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AnalyzeOrgPolicyGovernedContainersResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnalyzeOrgPolicyGovernedContainersResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for AnalyzeOrgPolicyGovernedContainersResponse {
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
    pub struct AnalyzerOrgPolicy {
        #[doc = "The \\[full resource name\\] (https://cloud.google.com/asset-inventory/docs/resource-name-format) of an organization/folder/project resource where this organization policy applies to. For any user defined org policies, this field has the same value as the \\[attached_resource\\] field. Only for default policy, this field has the different value."]
        #[serde(
            rename = "appliedResource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub applied_resource: ::std::option::Option<String>,
        #[doc = "The \\[full resource name\\] (https://cloud.google.com/asset-inventory/docs/resource-name-format) of an organization/folder/project resource where this organization policy is set. Notice that some type of constraints are defined with default policy. This field will be empty for them."]
        #[serde(
            rename = "attachedResource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attached_resource: ::std::option::Option<String>,
        #[doc = "If `inherit_from_parent` is true, Rules set higher up in the hierarchy (up to the closest root) are inherited and present in the effective policy. If it is false, then no rules are inherited, and this policy becomes the effective root for evaluation."]
        #[serde(
            rename = "inheritFromParent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inherit_from_parent: ::std::option::Option<bool>,
        #[doc = "Ignores policies set above this resource and restores the default behavior of the constraint at this resource. This field can be set in policies for either list or boolean constraints. If set, `rules` must be empty and `inherit_from_parent` must be set to false."]
        #[serde(
            rename = "reset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reset: ::std::option::Option<bool>,
        #[doc = "List of rules for this organization policy."]
        #[serde(
            rename = "rules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rules: ::std::option::Option<Vec<crate::schemas::GoogleCloudAssetV1Rule>>,
    }
    impl ::google_field_selector::FieldSelector for AnalyzerOrgPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnalyzerOrgPolicy {
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
    pub struct AnalyzerOrgPolicyConstraint {
        #[doc = "The definition of the custom constraint."]
        #[serde(
            rename = "customConstraint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_constraint:
            ::std::option::Option<crate::schemas::GoogleCloudAssetV1CustomConstraint>,
        #[doc = "The definition of the canned constraint defined by Google."]
        #[serde(
            rename = "googleDefinedConstraint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_defined_constraint:
            ::std::option::Option<crate::schemas::GoogleCloudAssetV1Constraint>,
    }
    impl ::google_field_selector::FieldSelector for AnalyzerOrgPolicyConstraint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnalyzerOrgPolicyConstraint {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Asset {
        #[doc = "Please also refer to the [access level user guide](https://cloud.google.com/access-context-manager/docs/overview#access-levels)."]
        #[serde(
            rename = "accessLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access_level:
            ::std::option::Option<crate::schemas::GoogleIdentityAccesscontextmanagerV1AccessLevel>,
        #[doc = "Please also refer to the [access policy user guide](https://cloud.google.com/access-context-manager/docs/overview#access-policies)."]
        #[serde(
            rename = "accessPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access_policy:
            ::std::option::Option<crate::schemas::GoogleIdentityAccesscontextmanagerV1AccessPolicy>,
        #[doc = "The ancestry path of an asset in Google Cloud [resource hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy), represented as a list of relative resource names. An ancestry path starts with the closest ancestor in the hierarchy and ends at root. If the asset is a project, folder, or organization, the ancestry path starts from the asset itself. Example: `[\"projects/123456789\", \"folders/5432\", \"organizations/1234\"]`"]
        #[serde(
            rename = "ancestors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ancestors: ::std::option::Option<Vec<String>>,
        #[doc = "The type of the asset. Example: `compute.googleapis.com/Disk` See [Supported asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types) for more information."]
        #[serde(
            rename = "assetType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub asset_type: ::std::option::Option<String>,
        #[doc = "A representation of the IAM policy set on a Google Cloud resource. There can be a maximum of one IAM policy set on any given resource. In addition, IAM policies inherit their granted access scope from any policies set on parent resources in the resource hierarchy. Therefore, the effectively policy is the union of both the policy set on this resource and each policy set on all of the resource’s ancestry resource levels in the hierarchy. See [this topic](https://cloud.google.com/iam/help/allow-policies/inheritance) for more information."]
        #[serde(
            rename = "iamPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub iam_policy: ::std::option::Option<crate::schemas::Policy>,
        #[doc = "The full name of the asset. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1` See [Resource names](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more information."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "A representation of an [organization policy](https://cloud.google.com/resource-manager/docs/organization-policy/overview#organization_policy). There can be more than one organization policy with different constraints set on a given resource."]
        #[serde(
            rename = "orgPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub org_policy: ::std::option::Option<Vec<crate::schemas::GoogleCloudOrgpolicyV1Policy>>,
        #[doc = "A representation of runtime OS Inventory information. See [this topic](https://cloud.google.com/compute/docs/instances/os-inventory-management) for more information."]
        #[serde(
            rename = "osInventory",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_inventory: ::std::option::Option<crate::schemas::Inventory>,
        #[doc = "One related asset of the current asset."]
        #[serde(
            rename = "relatedAsset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub related_asset: ::std::option::Option<crate::schemas::RelatedAsset>,
        #[doc = "DEPRECATED. This field only presents for the purpose of backward-compatibility. The server will never generate responses with this field. The related assets of the asset of one relationship type. One asset only represents one type of relationship."]
        #[serde(
            rename = "relatedAssets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub related_assets: ::std::option::Option<crate::schemas::RelatedAssets>,
        #[doc = "A representation of the resource."]
        #[serde(
            rename = "resource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource: ::std::option::Option<crate::schemas::Resource>,
        #[doc = "Please also refer to the [service perimeter user guide](https://cloud.google.com/vpc-service-controls/docs/overview)."]
        #[serde(
            rename = "servicePerimeter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_perimeter: ::std::option::Option<
            crate::schemas::GoogleIdentityAccesscontextmanagerV1ServicePerimeter,
        >,
        #[doc = "The last update timestamp of an asset. update_time is updated when create/update/delete operation is performed."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Asset {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Asset {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct AttachedResource {
        #[doc = "The type of this attached resource. Example: `osconfig.googleapis.com/Inventory` You can find the supported attached asset types of each resource in this table: `https://cloud.google.com/asset-inventory/docs/supported-asset-types#searchable_asset_types`"]
        #[serde(
            rename = "assetType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub asset_type: ::std::option::Option<String>,
        #[doc = "Versioned resource representations of this attached resource. This is repeated because there could be multiple versions of the attached resource representations during version migration."]
        #[serde(
            rename = "versionedResources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub versioned_resources: ::std::option::Option<Vec<crate::schemas::VersionedResource>>,
    }
    impl ::google_field_selector::FieldSelector for AttachedResource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AttachedResource {
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
    pub struct AuditConfig {
        #[doc = "The configuration for logging of each type of permission."]
        #[serde(
            rename = "auditLogConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audit_log_configs: ::std::option::Option<Vec<crate::schemas::AuditLogConfig>>,
        #[doc = "Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services."]
        #[serde(
            rename = "service",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AuditConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AuditConfig {
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
    pub struct AuditLogConfig {
        #[doc = "Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members."]
        #[serde(
            rename = "exemptedMembers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exempted_members: ::std::option::Option<Vec<String>>,
        #[doc = "The log type that this config enables."]
        #[serde(
            rename = "logType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub log_type: ::std::option::Option<crate::schemas::AuditLogConfigLogType>,
    }
    impl ::google_field_selector::FieldSelector for AuditLogConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AuditLogConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AuditLogConfigLogType {
        #[doc = "Admin reads. Example: CloudIAM getIamPolicy"]
        AdminRead,
        #[doc = "Data reads. Example: CloudSQL Users list"]
        DataRead,
        #[doc = "Data writes. Example: CloudSQL Users create"]
        DataWrite,
        #[doc = "Default case. Should never be this."]
        LogTypeUnspecified,
    }
    impl AuditLogConfigLogType {
        pub fn as_str(self) -> &'static str {
            match self {
                AuditLogConfigLogType::AdminRead => "ADMIN_READ",
                AuditLogConfigLogType::DataRead => "DATA_READ",
                AuditLogConfigLogType::DataWrite => "DATA_WRITE",
                AuditLogConfigLogType::LogTypeUnspecified => "LOG_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AuditLogConfigLogType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AuditLogConfigLogType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AuditLogConfigLogType, ()> {
            Ok(match s {
                "ADMIN_READ" => AuditLogConfigLogType::AdminRead,
                "DATA_READ" => AuditLogConfigLogType::DataRead,
                "DATA_WRITE" => AuditLogConfigLogType::DataWrite,
                "LOG_TYPE_UNSPECIFIED" => AuditLogConfigLogType::LogTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AuditLogConfigLogType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AuditLogConfigLogType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AuditLogConfigLogType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADMIN_READ" => AuditLogConfigLogType::AdminRead,
                "DATA_READ" => AuditLogConfigLogType::DataRead,
                "DATA_WRITE" => AuditLogConfigLogType::DataWrite,
                "LOG_TYPE_UNSPECIFIED" => AuditLogConfigLogType::LogTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AuditLogConfigLogType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AuditLogConfigLogType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct BatchGetAssetsHistoryResponse {
        #[doc = "A list of assets with valid time windows."]
        #[serde(
            rename = "assets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub assets: ::std::option::Option<Vec<crate::schemas::TemporalAsset>>,
    }
    impl ::google_field_selector::FieldSelector for BatchGetAssetsHistoryResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchGetAssetsHistoryResponse {
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
    pub struct BatchGetEffectiveIamPoliciesResponse {
        #[doc = "The effective policies for a batch of resources. Note that the results order is the same as the order of BatchGetEffectiveIamPoliciesRequest.names. When a resource does not have any effective IAM policies, its corresponding policy_result will contain empty EffectiveIamPolicy.policies."]
        #[serde(
            rename = "policyResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy_results: ::std::option::Option<Vec<crate::schemas::EffectiveIamPolicy>>,
    }
    impl ::google_field_selector::FieldSelector for BatchGetEffectiveIamPoliciesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchGetEffectiveIamPoliciesResponse {
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
    pub struct BigQueryDestination {
        #[doc = "Required. The BigQuery dataset in format “projects/projectId/datasets/datasetId”, to which the snapshot result should be exported. If this dataset does not exist, the export call returns an INVALID_ARGUMENT error. Setting the `contentType` for `exportAssets` determines the [schema](/asset-inventory/docs/exporting-to-bigquery#bigquery-schema) of the BigQuery table. Setting `separateTablesPerAssetType` to `TRUE` also influences the schema."]
        #[serde(
            rename = "dataset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dataset: ::std::option::Option<String>,
        #[doc = "If the destination table already exists and this flag is `TRUE`, the table will be overwritten by the contents of assets snapshot. If the flag is `FALSE` or unset and the destination table already exists, the export call returns an INVALID_ARGUMEMT error."]
        #[serde(
            rename = "force",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub force: ::std::option::Option<bool>,
        #[doc = "\\[partition_spec\\] determines whether to export to partitioned table(s) and how to partition the data. If \\[partition_spec\\] is unset or \\[partition_spec.partition_key\\] is unset or `PARTITION_KEY_UNSPECIFIED`, the snapshot results will be exported to non-partitioned table(s). \\[force\\] will decide whether to overwrite existing table(s). If \\[partition_spec\\] is specified. First, the snapshot results will be written to partitioned table(s) with two additional timestamp columns, readTime and requestTime, one of which will be the partition key. Secondly, in the case when any destination table already exists, it will first try to update existing table’s schema as necessary by appending additional columns. Then, if \\[force\\] is `TRUE`, the corresponding partition will be overwritten by the snapshot results (data in different partitions will remain intact); if \\[force\\] is unset or `FALSE`, it will append the data. An error will be returned if the schema update or data appension fails."]
        #[serde(
            rename = "partitionSpec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub partition_spec: ::std::option::Option<crate::schemas::PartitionSpec>,
        #[doc = "If this flag is `TRUE`, the snapshot results will be written to one or multiple tables, each of which contains results of one asset type. The \\[force\\] and \\[partition_spec\\] fields will apply to each of them. Field \\[table\\] will be concatenated with “*” and the asset type names (see https://cloud.google.com/asset-inventory/docs/supported-asset-types for supported asset types) to construct per-asset-type table names, in which all non-alphanumeric characters like “.” and “/” will be substituted by “*”. Example: if field \\[table\\] is “mytable” and snapshot results contain “storage.googleapis.com/Bucket” assets, the corresponding table name will be “mytable_storage_googleapis_com_Bucket”. If any of these tables does not exist, a new table with the concatenated name will be created. When \\[content_type\\] in the ExportAssetsRequest is `RESOURCE`, the schema of each table will include RECORD-type columns mapped to the nested fields in the Asset.resource.data field of that asset type (up to the 15 nested level BigQuery supports (https://cloud.google.com/bigquery/docs/nested-repeated#limitations)). The fields in >15 nested levels will be stored in JSON format string as a child column of its parent RECORD column. If error occurs when exporting to any table, the whole export call will return an error but the export results that already succeed will persist. Example: if exporting to table_type_A succeeds when exporting to table_type_B fails during one export call, the results in table_type_A will persist and there will not be partial results persisting in a table."]
        #[serde(
            rename = "separateTablesPerAssetType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub separate_tables_per_asset_type: ::std::option::Option<bool>,
        #[doc = "Required. The BigQuery table to which the snapshot result should be written. If this table does not exist, a new table with the given name will be created."]
        #[serde(
            rename = "table",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BigQueryDestination {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BigQueryDestination {
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
    pub struct Binding {
        #[doc = "The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the principals in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        #[serde(
            rename = "condition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub condition: ::std::option::Option<crate::schemas::Expr>,
        #[doc = "Specifies the principals requesting access for a Google Cloud resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. Does not include identities that come from external identity providers (IdPs) through identity federation. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a Google service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `serviceAccount:{projectid}.svc.id.goog[{namespace}/{kubernetes-sa}]`: An identifier for a [Kubernetes service account](https://cloud.google.com/kubernetes-engine/docs/how-to/kubernetes-service-accounts). For example, `my-project.svc.id.goog[my-namespace/my-kubernetes-sa]`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding."]
        #[serde(
            rename = "members",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub members: ::std::option::Option<Vec<String>>,
        #[doc = "Role that is assigned to the list of `members`, or principals. For example, `roles/viewer`, `roles/editor`, or `roles/owner`."]
        #[serde(
            rename = "role",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub role: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Binding {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Binding {
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
    pub struct ConditionContext {
        #[doc = "The hypothetical access timestamp to evaluate IAM conditions. Note that this value must not be earlier than the current time; otherwise, an INVALID_ARGUMENT error will be returned."]
        #[serde(
            rename = "accessTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ConditionContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConditionContext {
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
    pub struct ConditionEvaluation {
        #[doc = "The evaluation result."]
        #[serde(
            rename = "evaluationValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub evaluation_value:
            ::std::option::Option<crate::schemas::ConditionEvaluationEvaluationValue>,
    }
    impl ::google_field_selector::FieldSelector for ConditionEvaluation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConditionEvaluation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ConditionEvaluationEvaluationValue {
        #[doc = "The evaluation result is `conditional` when the condition expression contains variables that are either missing input values or have not been supported by Analyzer yet."]
        Conditional,
        #[doc = "Reserved for future use."]
        EvaluationValueUnspecified,
        #[doc = "The evaluation result is `false`."]
        False,
        #[doc = "The evaluation result is `true`."]
        True,
    }
    impl ConditionEvaluationEvaluationValue {
        pub fn as_str(self) -> &'static str {
            match self {
                ConditionEvaluationEvaluationValue::Conditional => "CONDITIONAL",
                ConditionEvaluationEvaluationValue::EvaluationValueUnspecified => {
                    "EVALUATION_VALUE_UNSPECIFIED"
                }
                ConditionEvaluationEvaluationValue::False => "FALSE",
                ConditionEvaluationEvaluationValue::True => "TRUE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ConditionEvaluationEvaluationValue {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ConditionEvaluationEvaluationValue {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ConditionEvaluationEvaluationValue, ()> {
            Ok(match s {
                "CONDITIONAL" => ConditionEvaluationEvaluationValue::Conditional,
                "EVALUATION_VALUE_UNSPECIFIED" => {
                    ConditionEvaluationEvaluationValue::EvaluationValueUnspecified
                }
                "FALSE" => ConditionEvaluationEvaluationValue::False,
                "TRUE" => ConditionEvaluationEvaluationValue::True,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ConditionEvaluationEvaluationValue {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ConditionEvaluationEvaluationValue {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ConditionEvaluationEvaluationValue {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONDITIONAL" => ConditionEvaluationEvaluationValue::Conditional,
                "EVALUATION_VALUE_UNSPECIFIED" => {
                    ConditionEvaluationEvaluationValue::EvaluationValueUnspecified
                }
                "FALSE" => ConditionEvaluationEvaluationValue::False,
                "TRUE" => ConditionEvaluationEvaluationValue::True,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ConditionEvaluationEvaluationValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConditionEvaluationEvaluationValue {
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
    pub struct CreateFeedRequest {
        #[doc = "Required. The feed details. The field `name` must be empty and it will be generated in the format of: projects/project_number/feeds/feed_id folders/folder_number/feeds/feed_id organizations/organization_number/feeds/feed_id"]
        #[serde(
            rename = "feed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub feed: ::std::option::Option<crate::schemas::Feed>,
        #[doc = "Required. This is the client-assigned asset feed identifier and it needs to be unique under a specific parent project/folder/organization."]
        #[serde(
            rename = "feedId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub feed_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreateFeedRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateFeedRequest {
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
    pub struct Date {
        #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn’t significant."]
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
    impl ::google_field_selector::FieldSelector for Date {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Date {
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
    pub struct EffectiveIamPolicy {
        #[doc = "The \\[full_resource_name\\] (https://cloud.google.com/asset-inventory/docs/resource-name-format) for which the policies are computed. This is one of the BatchGetEffectiveIamPoliciesRequest.names the caller provides in the request."]
        #[serde(
            rename = "fullResourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub full_resource_name: ::std::option::Option<String>,
        #[doc = "The effective policies for the full_resource_name. These policies include the policy set on the full_resource_name and those set on its parents and ancestors up to the BatchGetEffectiveIamPoliciesRequest.scope. Note that these policies are not filtered according to the resource type of the full_resource_name. These policies are hierarchically ordered by PolicyInfo.attached_resource starting from full_resource_name itself to its parents and ancestors, such that policies\\[i\\]’s PolicyInfo.attached_resource is the child of policies\\[i+1\\]’s PolicyInfo.attached_resource, if policies\\[i+1\\] exists."]
        #[serde(
            rename = "policies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policies: ::std::option::Option<Vec<crate::schemas::PolicyInfo>>,
    }
    impl ::google_field_selector::FieldSelector for EffectiveIamPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EffectiveIamPolicy {
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
    pub struct Empty {}
    impl ::google_field_selector::FieldSelector for Empty {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Empty {
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
    pub struct Explanation {
        #[doc = "The map from roles to their included permissions that match the permission query (i.e., a query containing `policy.role.permissions:`). Example: if query `policy.role.permissions:compute.disk.get` matches a policy binding that contains owner role, the matched_permissions will be `{\"roles/owner\": [\"compute.disk.get\"]}`. The roles can also be found in the returned `policy` bindings. Note that the map is populated only for requests with permission queries."]
        #[serde(
            rename = "matchedPermissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub matched_permissions: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::Permissions>,
        >,
    }
    impl ::google_field_selector::FieldSelector for Explanation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Explanation {
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
    pub struct ExportAssetsRequest {
        #[doc = "A list of asset types to take a snapshot for. For example: “compute.googleapis.com/Disk”. Regular expressions are also supported. For example: * “compute.googleapis.com.\\*” snapshots resources whose asset type starts with “compute.googleapis.com”. * “.\\*Instance” snapshots resources whose asset type ends with “Instance”. * “.*Instance.*” snapshots resources whose asset type contains “Instance”. See [RE2](https://github.com/google/re2/wiki/Syntax) for all supported regular expression syntax. If the regular expression does not match any supported asset type, an INVALID_ARGUMENT error will be returned. If specified, only matching assets will be returned, otherwise, it will snapshot all asset types. See [Introduction to Cloud Asset Inventory](https://cloud.google.com/asset-inventory/docs/overview) for all supported asset types."]
        #[serde(
            rename = "assetTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub asset_types: ::std::option::Option<Vec<String>>,
        #[doc = "Asset content type. If not specified, no content but the asset name will be returned."]
        #[serde(
            rename = "contentType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_type: ::std::option::Option<crate::schemas::ExportAssetsRequestContentType>,
        #[doc = "Required. Output configuration indicating where the results will be output to."]
        #[serde(
            rename = "outputConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_config: ::std::option::Option<crate::schemas::OutputConfig>,
        #[doc = "Timestamp to take an asset snapshot. This can only be set to a timestamp between the current time and the current time minus 35 days (inclusive). If not specified, the current time will be used. Due to delays in resource data collection and indexing, there is a volatile window during which running the same query may get different results."]
        #[serde(
            rename = "readTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub read_time: ::std::option::Option<String>,
        #[doc = "A list of relationship types to export, for example: `INSTANCE_TO_INSTANCEGROUP`. This field should only be specified if content_type=RELATIONSHIP. * If specified: it snapshots specified relationships. It returns an error if any of the \\[relationship_types\\] doesn’t belong to the supported relationship types of the \\[asset_types\\] or if any of the \\[asset_types\\] doesn’t belong to the source types of the \\[relationship_types\\]. * Otherwise: it snapshots the supported relationships for all \\[asset_types\\] or returns an error if any of the \\[asset_types\\] has no relationship support. An unspecified asset types field means all supported asset_types. See [Introduction to Cloud Asset Inventory](https://cloud.google.com/asset-inventory/docs/overview) for all supported asset types and relationship types."]
        #[serde(
            rename = "relationshipTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relationship_types: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for ExportAssetsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExportAssetsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ExportAssetsRequestContentType {
        #[doc = "The Access Context Manager policy set on an asset."]
        AccessPolicy,
        #[doc = "Unspecified content type."]
        ContentTypeUnspecified,
        #[doc = "The actual IAM policy set on a resource."]
        IamPolicy,
        #[doc = "The organization policy set on an asset."]
        OrgPolicy,
        #[doc = "The runtime OS Inventory information."]
        OsInventory,
        #[doc = "The related resources."]
        Relationship,
        #[doc = "Resource metadata."]
        Resource,
    }
    impl ExportAssetsRequestContentType {
        pub fn as_str(self) -> &'static str {
            match self {
                ExportAssetsRequestContentType::AccessPolicy => "ACCESS_POLICY",
                ExportAssetsRequestContentType::ContentTypeUnspecified => {
                    "CONTENT_TYPE_UNSPECIFIED"
                }
                ExportAssetsRequestContentType::IamPolicy => "IAM_POLICY",
                ExportAssetsRequestContentType::OrgPolicy => "ORG_POLICY",
                ExportAssetsRequestContentType::OsInventory => "OS_INVENTORY",
                ExportAssetsRequestContentType::Relationship => "RELATIONSHIP",
                ExportAssetsRequestContentType::Resource => "RESOURCE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ExportAssetsRequestContentType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ExportAssetsRequestContentType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ExportAssetsRequestContentType, ()> {
            Ok(match s {
                "ACCESS_POLICY" => ExportAssetsRequestContentType::AccessPolicy,
                "CONTENT_TYPE_UNSPECIFIED" => {
                    ExportAssetsRequestContentType::ContentTypeUnspecified
                }
                "IAM_POLICY" => ExportAssetsRequestContentType::IamPolicy,
                "ORG_POLICY" => ExportAssetsRequestContentType::OrgPolicy,
                "OS_INVENTORY" => ExportAssetsRequestContentType::OsInventory,
                "RELATIONSHIP" => ExportAssetsRequestContentType::Relationship,
                "RESOURCE" => ExportAssetsRequestContentType::Resource,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ExportAssetsRequestContentType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ExportAssetsRequestContentType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ExportAssetsRequestContentType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCESS_POLICY" => ExportAssetsRequestContentType::AccessPolicy,
                "CONTENT_TYPE_UNSPECIFIED" => {
                    ExportAssetsRequestContentType::ContentTypeUnspecified
                }
                "IAM_POLICY" => ExportAssetsRequestContentType::IamPolicy,
                "ORG_POLICY" => ExportAssetsRequestContentType::OrgPolicy,
                "OS_INVENTORY" => ExportAssetsRequestContentType::OsInventory,
                "RELATIONSHIP" => ExportAssetsRequestContentType::Relationship,
                "RESOURCE" => ExportAssetsRequestContentType::Resource,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ExportAssetsRequestContentType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExportAssetsRequestContentType {
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
    pub struct Expr {
        #[doc = "Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Textual representation of an expression in Common Expression Language syntax."]
        #[serde(
            rename = "expression",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expression: ::std::option::Option<String>,
        #[doc = "Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Expr {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Expr {
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
    pub struct Feed {
        #[doc = "A list of the full names of the assets to receive updates. You must specify either or both of asset_names and asset_types. Only asset updates matching specified asset_names or asset_types are exported to the feed. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`. For a list of the full names for supported asset types, see [Resource name format](/asset-inventory/docs/resource-name-format)."]
        #[serde(
            rename = "assetNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub asset_names: ::std::option::Option<Vec<String>>,
        #[doc = "A list of types of the assets to receive updates. You must specify either or both of asset_names and asset_types. Only asset updates matching specified asset_names or asset_types are exported to the feed. Example: `\"compute.googleapis.com/Disk\"` For a list of all supported asset types, see [Supported asset types](/asset-inventory/docs/supported-asset-types)."]
        #[serde(
            rename = "assetTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub asset_types: ::std::option::Option<Vec<String>>,
        #[doc = "A condition which determines whether an asset update should be published. If specified, an asset will be returned only when the expression evaluates to true. When set, `expression` field in the `Expr` must be a valid \\[CEL expression\\] (https://github.com/google/cel-spec) on a TemporalAsset with name `temporal_asset`. Example: a Feed with expression (“temporal_asset.deleted == true”) will only publish Asset deletions. Other fields of `Expr` are optional. See our [user guide](https://cloud.google.com/asset-inventory/docs/monitoring-asset-changes-with-condition) for detailed instructions."]
        #[serde(
            rename = "condition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub condition: ::std::option::Option<crate::schemas::Expr>,
        #[doc = "Asset content type. If not specified, no content but the asset name and type will be returned."]
        #[serde(
            rename = "contentType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_type: ::std::option::Option<crate::schemas::FeedContentType>,
        #[doc = "Required. Feed output configuration defining where the asset updates are published to."]
        #[serde(
            rename = "feedOutputConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub feed_output_config: ::std::option::Option<crate::schemas::FeedOutputConfig>,
        #[doc = "Required. The format will be projects/{project_number}/feeds/{client-assigned_feed_identifier} or folders/{folder_number}/feeds/{client-assigned_feed_identifier} or organizations/{organization_number}/feeds/{client-assigned_feed_identifier} The client-assigned feed identifier must be unique within the parent project/folder/organization."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "A list of relationship types to output, for example: `INSTANCE_TO_INSTANCEGROUP`. This field should only be specified if content_type=RELATIONSHIP. * If specified: it outputs specified relationship updates on the \\[asset_names\\] or the \\[asset_types\\]. It returns an error if any of the \\[relationship_types\\] doesn’t belong to the supported relationship types of the \\[asset_names\\] or \\[asset_types\\], or any of the \\[asset_names\\] or the \\[asset_types\\] doesn’t belong to the source types of the \\[relationship_types\\]. * Otherwise: it outputs the supported relationships of the types of \\[asset_names\\] and \\[asset_types\\] or returns an error if any of the \\[asset_names\\] or the \\[asset_types\\] has no replationship support. See [Introduction to Cloud Asset Inventory](https://cloud.google.com/asset-inventory/docs/overview) for all supported asset types and relationship types."]
        #[serde(
            rename = "relationshipTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relationship_types: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for Feed {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Feed {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FeedContentType {
        #[doc = "The Access Context Manager policy set on an asset."]
        AccessPolicy,
        #[doc = "Unspecified content type."]
        ContentTypeUnspecified,
        #[doc = "The actual IAM policy set on a resource."]
        IamPolicy,
        #[doc = "The organization policy set on an asset."]
        OrgPolicy,
        #[doc = "The runtime OS Inventory information."]
        OsInventory,
        #[doc = "The related resources."]
        Relationship,
        #[doc = "Resource metadata."]
        Resource,
    }
    impl FeedContentType {
        pub fn as_str(self) -> &'static str {
            match self {
                FeedContentType::AccessPolicy => "ACCESS_POLICY",
                FeedContentType::ContentTypeUnspecified => "CONTENT_TYPE_UNSPECIFIED",
                FeedContentType::IamPolicy => "IAM_POLICY",
                FeedContentType::OrgPolicy => "ORG_POLICY",
                FeedContentType::OsInventory => "OS_INVENTORY",
                FeedContentType::Relationship => "RELATIONSHIP",
                FeedContentType::Resource => "RESOURCE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for FeedContentType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for FeedContentType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<FeedContentType, ()> {
            Ok(match s {
                "ACCESS_POLICY" => FeedContentType::AccessPolicy,
                "CONTENT_TYPE_UNSPECIFIED" => FeedContentType::ContentTypeUnspecified,
                "IAM_POLICY" => FeedContentType::IamPolicy,
                "ORG_POLICY" => FeedContentType::OrgPolicy,
                "OS_INVENTORY" => FeedContentType::OsInventory,
                "RELATIONSHIP" => FeedContentType::Relationship,
                "RESOURCE" => FeedContentType::Resource,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for FeedContentType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FeedContentType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FeedContentType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCESS_POLICY" => FeedContentType::AccessPolicy,
                "CONTENT_TYPE_UNSPECIFIED" => FeedContentType::ContentTypeUnspecified,
                "IAM_POLICY" => FeedContentType::IamPolicy,
                "ORG_POLICY" => FeedContentType::OrgPolicy,
                "OS_INVENTORY" => FeedContentType::OsInventory,
                "RELATIONSHIP" => FeedContentType::Relationship,
                "RESOURCE" => FeedContentType::Resource,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for FeedContentType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FeedContentType {
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
    pub struct FeedOutputConfig {
        #[doc = "Destination on Pub/Sub."]
        #[serde(
            rename = "pubsubDestination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pubsub_destination: ::std::option::Option<crate::schemas::PubsubDestination>,
    }
    impl ::google_field_selector::FieldSelector for FeedOutputConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FeedOutputConfig {
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
    pub struct GcsDestination {
        #[doc = "The URI of the Cloud Storage object. It’s the same URI that is used by gsutil. Example: “gs://bucket_name/object_name”. See [Viewing and Editing Object Metadata](https://cloud.google.com/storage/docs/viewing-editing-metadata) for more information. If the specified Cloud Storage object already exists and there is no [hold](https://cloud.google.com/storage/docs/object-holds), it will be overwritten with the exported result."]
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
        #[doc = "The URI prefix of all generated Cloud Storage objects. Example: “gs://bucket_name/object_name_prefix”. Each object URI is in format: “gs://bucket_name/object_name_prefix// and only contains assets for that type. starts from 0. Example: “gs://bucket_name/object_name_prefix/compute.googleapis.com/Disk/0” is the first shard of output objects containing all compute.googleapis.com/Disk assets. An INVALID_ARGUMENT error will be returned if file with the same name “gs://bucket_name/object_name_prefix” already exists."]
        #[serde(
            rename = "uriPrefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri_prefix: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GcsDestination {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GcsDestination {
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
    pub struct GoogleCloudAssetV1Access {
        #[doc = "The analysis state of this access."]
        #[serde(
            rename = "analysisState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_state: ::std::option::Option<crate::schemas::IamPolicyAnalysisState>,
        #[doc = "The permission."]
        #[serde(
            rename = "permission",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permission: ::std::option::Option<String>,
        #[doc = "The role."]
        #[serde(
            rename = "role",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub role: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1Access {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1Access {
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
    pub struct GoogleCloudAssetV1AccessControlList {
        #[doc = "The accesses that match one of the following conditions: - The access_selector, if it is specified in request; - Otherwise, access specifiers reachable from the policy binding’s role."]
        #[serde(
            rename = "accesses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub accesses: ::std::option::Option<Vec<crate::schemas::GoogleCloudAssetV1Access>>,
        #[doc = "Condition evaluation for this AccessControlList, if there is a condition defined in the above IAM policy binding."]
        #[serde(
            rename = "conditionEvaluation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub condition_evaluation: ::std::option::Option<crate::schemas::ConditionEvaluation>,
        #[doc = "Resource edges of the graph starting from the policy attached resource to any descendant resources. The Edge.source_node contains the full resource name of a parent resource and Edge.target_node contains the full resource name of a child resource. This field is present only if the output_resource_edges option is enabled in request."]
        #[serde(
            rename = "resourceEdges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_edges: ::std::option::Option<Vec<crate::schemas::GoogleCloudAssetV1Edge>>,
        #[doc = "The resources that match one of the following conditions: - The resource_selector, if it is specified in request; - Otherwise, resources reachable from the policy attached resource."]
        #[serde(
            rename = "resources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resources: ::std::option::Option<Vec<crate::schemas::GoogleCloudAssetV1Resource>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1AccessControlList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1AccessControlList {
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
    pub struct GoogleCloudAssetV1AnalyzeOrgPolicyGovernedAssetsResponseGovernedAsset { # [doc = "The consolidated policy for the analyzed asset. The consolidated policy is computed by merging and evaluating AnalyzeOrgPolicyGovernedAssetsResponse.GovernedAsset.policy_bundle. The evaluation will respect the organization policy [hierarchy rules](https://cloud.google.com/resource-manager/docs/organization-policy/understanding-hierarchy)."] # [serde (rename = "consolidatedPolicy" , default , skip_serializing_if = "std::option::Option::is_none")] pub consolidated_policy : :: std :: option :: Option < crate :: schemas :: AnalyzerOrgPolicy > , # [doc = "An IAM policy governed by the organization policies of the AnalyzeOrgPolicyGovernedAssetsRequest.constraint."] # [serde (rename = "governedIamPolicy" , default , skip_serializing_if = "std::option::Option::is_none")] pub governed_iam_policy : :: std :: option :: Option < crate :: schemas :: GoogleCloudAssetV1AnalyzeOrgPolicyGovernedAssetsResponseGovernedIamPolicy > , # [doc = "A Google Cloud resource governed by the organization policies of the AnalyzeOrgPolicyGovernedAssetsRequest.constraint."] # [serde (rename = "governedResource" , default , skip_serializing_if = "std::option::Option::is_none")] pub governed_resource : :: std :: option :: Option < crate :: schemas :: GoogleCloudAssetV1AnalyzeOrgPolicyGovernedAssetsResponseGovernedResource > , # [doc = "The ordered list of all organization policies from the AnalyzeOrgPoliciesResponse.OrgPolicyResult.consolidated_policy.attached_resource to the scope specified in the request. If the constraint is defined with default policy, it will also appear in the list."] # [serde (rename = "policyBundle" , default , skip_serializing_if = "std::option::Option::is_none")] pub policy_bundle : :: std :: option :: Option < Vec < crate :: schemas :: AnalyzerOrgPolicy > > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssetV1AnalyzeOrgPolicyGovernedAssetsResponseGovernedAsset
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssetV1AnalyzeOrgPolicyGovernedAssetsResponseGovernedAsset
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
    pub struct GoogleCloudAssetV1AnalyzeOrgPolicyGovernedAssetsResponseGovernedIamPolicy {
        #[doc = "The full resource name of the resource associated with this IAM policy. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`. See [Cloud Asset Inventory Resource Name Format](https://cloud.google.com/asset-inventory/docs/resource-name-format) for more information."]
        #[serde(
            rename = "attachedResource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attached_resource: ::std::option::Option<String>,
        #[doc = "The folder(s) that this IAM policy belongs to, in the form of folders/{FOLDER_NUMBER}. This field is available when the IAM policy belongs(directly or cascadingly) to one or more folders."]
        #[serde(
            rename = "folders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub folders: ::std::option::Option<Vec<String>>,
        #[doc = "The organization that this IAM policy belongs to, in the form of organizations/{ORGANIZATION_NUMBER}. This field is available when the IAM policy belongs(directly or cascadingly) to an organization."]
        #[serde(
            rename = "organization",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub organization: ::std::option::Option<String>,
        #[doc = "The IAM policy directly set on the given resource."]
        #[serde(
            rename = "policy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy: ::std::option::Option<crate::schemas::Policy>,
        #[doc = "The project that this IAM policy belongs to, in the form of projects/{PROJECT_NUMBER}. This field is available when the IAM policy belongs to a project."]
        #[serde(
            rename = "project",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssetV1AnalyzeOrgPolicyGovernedAssetsResponseGovernedIamPolicy
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssetV1AnalyzeOrgPolicyGovernedAssetsResponseGovernedIamPolicy
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
    pub struct GoogleCloudAssetV1AnalyzeOrgPolicyGovernedAssetsResponseGovernedResource {
        #[doc = "The folder(s) that this resource belongs to, in the form of folders/{FOLDER_NUMBER}. This field is available when the resource belongs(directly or cascadingly) to one or more folders."]
        #[serde(
            rename = "folders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub folders: ::std::option::Option<Vec<String>>,
        #[doc = "The \\[full resource name\\] (https://cloud.google.com/asset-inventory/docs/resource-name-format) of the Google Cloud resource."]
        #[serde(
            rename = "fullResourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub full_resource_name: ::std::option::Option<String>,
        #[doc = "The organization that this resource belongs to, in the form of organizations/{ORGANIZATION_NUMBER}. This field is available when the resource belongs(directly or cascadingly) to an organization."]
        #[serde(
            rename = "organization",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub organization: ::std::option::Option<String>,
        #[doc = "The \\[full resource name\\] (https://cloud.google.com/asset-inventory/docs/resource-name-format) of the parent of AnalyzeOrgPolicyGovernedAssetsResponse.GovernedResource.full_resource_name."]
        #[serde(
            rename = "parent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent: ::std::option::Option<String>,
        #[doc = "The project that this resource belongs to, in the form of projects/{PROJECT_NUMBER}. This field is available when the resource belongs to a project."]
        #[serde(
            rename = "project",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssetV1AnalyzeOrgPolicyGovernedAssetsResponseGovernedResource
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssetV1AnalyzeOrgPolicyGovernedAssetsResponseGovernedResource
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
    pub struct GoogleCloudAssetV1BigQueryDestination {
        #[doc = "Required. The BigQuery dataset in format “projects/projectId/datasets/datasetId”, to which the analysis results should be exported. If this dataset does not exist, the export call will return an INVALID_ARGUMENT error."]
        #[serde(
            rename = "dataset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dataset: ::std::option::Option<String>,
        #[doc = "The partition key for BigQuery partitioned table."]
        #[serde(
            rename = "partitionKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub partition_key: ::std::option::Option<
            crate::schemas::GoogleCloudAssetV1BigQueryDestinationPartitionKey,
        >,
        #[doc = "Required. The prefix of the BigQuery tables to which the analysis results will be written. Tables will be created based on this table_prefix if not exist: * \\_analysis table will contain export operation’s metadata. * \\_analysis_result will contain all the IamPolicyAnalysisResult. When \\[partition_key\\] is specified, both tables will be partitioned based on the \\[partition_key\\]."]
        #[serde(
            rename = "tablePrefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_prefix: ::std::option::Option<String>,
        #[doc = "Optional. Specifies the action that occurs if the destination table or partition already exists. The following values are supported: * WRITE_TRUNCATE: If the table or partition already exists, BigQuery overwrites the entire table or all the partitions data. * WRITE_APPEND: If the table or partition already exists, BigQuery appends the data to the table or the latest partition. * WRITE_EMPTY: If the table already exists and contains data, an error is returned. The default value is WRITE_APPEND. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Details are at https://cloud.google.com/bigquery/docs/loading-data-local#appending_to_or_overwriting_a_table_using_a_local_file."]
        #[serde(
            rename = "writeDisposition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub write_disposition: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1BigQueryDestination {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1BigQueryDestination {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssetV1BigQueryDestinationPartitionKey {
        #[doc = "Unspecified partition key. Tables won’t be partitioned using this option."]
        PartitionKeyUnspecified,
        #[doc = "The time when the request is received. If specified as partition key, the result table(s) is partitoned by the RequestTime column, an additional timestamp column representing when the request was received."]
        RequestTime,
    }
    impl GoogleCloudAssetV1BigQueryDestinationPartitionKey {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudAssetV1BigQueryDestinationPartitionKey::PartitionKeyUnspecified => {
                    "PARTITION_KEY_UNSPECIFIED"
                }
                GoogleCloudAssetV1BigQueryDestinationPartitionKey::RequestTime => "REQUEST_TIME",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudAssetV1BigQueryDestinationPartitionKey {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudAssetV1BigQueryDestinationPartitionKey {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudAssetV1BigQueryDestinationPartitionKey, ()> {
            Ok(match s {
                "PARTITION_KEY_UNSPECIFIED" => {
                    GoogleCloudAssetV1BigQueryDestinationPartitionKey::PartitionKeyUnspecified
                }
                "REQUEST_TIME" => GoogleCloudAssetV1BigQueryDestinationPartitionKey::RequestTime,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudAssetV1BigQueryDestinationPartitionKey {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudAssetV1BigQueryDestinationPartitionKey {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudAssetV1BigQueryDestinationPartitionKey {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PARTITION_KEY_UNSPECIFIED" => {
                    GoogleCloudAssetV1BigQueryDestinationPartitionKey::PartitionKeyUnspecified
                }
                "REQUEST_TIME" => GoogleCloudAssetV1BigQueryDestinationPartitionKey::RequestTime,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1BigQueryDestinationPartitionKey {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1BigQueryDestinationPartitionKey {
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
    pub struct GoogleCloudAssetV1BooleanConstraint {}
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1BooleanConstraint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1BooleanConstraint {
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
    pub struct GoogleCloudAssetV1Constraint {
        #[doc = "Defines this constraint as being a BooleanConstraint."]
        #[serde(
            rename = "booleanConstraint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub boolean_constraint:
            ::std::option::Option<crate::schemas::GoogleCloudAssetV1BooleanConstraint>,
        #[doc = "The evaluation behavior of this constraint in the absence of ‘Policy’."]
        #[serde(
            rename = "constraintDefault",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub constraint_default:
            ::std::option::Option<crate::schemas::GoogleCloudAssetV1ConstraintConstraintDefault>,
        #[doc = "Detailed description of what this `Constraint` controls as well as how and where it is enforced."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The human readable name of the constraint."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Defines this constraint as being a ListConstraint."]
        #[serde(
            rename = "listConstraint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub list_constraint:
            ::std::option::Option<crate::schemas::GoogleCloudAssetV1ListConstraint>,
        #[doc = "The unique name of the constraint. Format of the name should be * `constraints/{constraint_name}` For example, `constraints/compute.disableSerialPortAccess`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1Constraint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1Constraint {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssetV1ConstraintConstraintDefault {
        #[doc = "Indicate that all values are allowed for list constraints. Indicate that enforcement is off for boolean constraints."]
        Allow,
        #[doc = "This is only used for distinguishing unset values and should never be used."]
        ConstraintDefaultUnspecified,
        #[doc = "Indicate that all values are denied for list constraints. Indicate that enforcement is on for boolean constraints."]
        Deny,
    }
    impl GoogleCloudAssetV1ConstraintConstraintDefault {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudAssetV1ConstraintConstraintDefault::Allow => "ALLOW",
                GoogleCloudAssetV1ConstraintConstraintDefault::ConstraintDefaultUnspecified => {
                    "CONSTRAINT_DEFAULT_UNSPECIFIED"
                }
                GoogleCloudAssetV1ConstraintConstraintDefault::Deny => "DENY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudAssetV1ConstraintConstraintDefault {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudAssetV1ConstraintConstraintDefault {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudAssetV1ConstraintConstraintDefault, ()> {
            Ok(match s {
                "ALLOW" => GoogleCloudAssetV1ConstraintConstraintDefault::Allow,
                "CONSTRAINT_DEFAULT_UNSPECIFIED" => {
                    GoogleCloudAssetV1ConstraintConstraintDefault::ConstraintDefaultUnspecified
                }
                "DENY" => GoogleCloudAssetV1ConstraintConstraintDefault::Deny,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudAssetV1ConstraintConstraintDefault {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudAssetV1ConstraintConstraintDefault {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudAssetV1ConstraintConstraintDefault {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALLOW" => GoogleCloudAssetV1ConstraintConstraintDefault::Allow,
                "CONSTRAINT_DEFAULT_UNSPECIFIED" => {
                    GoogleCloudAssetV1ConstraintConstraintDefault::ConstraintDefaultUnspecified
                }
                "DENY" => GoogleCloudAssetV1ConstraintConstraintDefault::Deny,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1ConstraintConstraintDefault {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1ConstraintConstraintDefault {
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
    pub struct GoogleCloudAssetV1CustomConstraint {
        #[doc = "Allow or deny type."]
        #[serde(
            rename = "actionType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action_type:
            ::std::option::Option<crate::schemas::GoogleCloudAssetV1CustomConstraintActionType>,
        #[doc = "Organization Policy condition/expression. For example: `resource.instanceName.matches(\"[production|test]_.*_(\\d)+\")'` or, `resource.management.auto_upgrade == true`"]
        #[serde(
            rename = "condition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub condition: ::std::option::Option<String>,
        #[doc = "Detailed information about this custom policy constraint."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "One line display name for the UI."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "All the operations being applied for this constraint."]
        #[serde(
            rename = "methodTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub method_types: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudAssetV1CustomConstraintMethodTypesItems>,
        >,
        #[doc = "Name of the constraint. This is unique within the organization. Format of the name should be * `organizations/{organization_id}/customConstraints/{custom_constraint_id}` Example : “organizations/123/customConstraints/custom.createOnlyE2TypeVms”"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The Resource Instance type on which this policy applies to. Format will be of the form : “/” Example: * `compute.googleapis.com/Instance`."]
        #[serde(
            rename = "resourceTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_types: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1CustomConstraint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1CustomConstraint {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssetV1CustomConstraintActionType {
        #[doc = "Unspecified. Will results in user error."]
        ActionTypeUnspecified,
        #[doc = "Allowed action type."]
        Allow,
        #[doc = "Deny action type."]
        Deny,
    }
    impl GoogleCloudAssetV1CustomConstraintActionType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudAssetV1CustomConstraintActionType::ActionTypeUnspecified => {
                    "ACTION_TYPE_UNSPECIFIED"
                }
                GoogleCloudAssetV1CustomConstraintActionType::Allow => "ALLOW",
                GoogleCloudAssetV1CustomConstraintActionType::Deny => "DENY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudAssetV1CustomConstraintActionType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudAssetV1CustomConstraintActionType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudAssetV1CustomConstraintActionType, ()> {
            Ok(match s {
                "ACTION_TYPE_UNSPECIFIED" => {
                    GoogleCloudAssetV1CustomConstraintActionType::ActionTypeUnspecified
                }
                "ALLOW" => GoogleCloudAssetV1CustomConstraintActionType::Allow,
                "DENY" => GoogleCloudAssetV1CustomConstraintActionType::Deny,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudAssetV1CustomConstraintActionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudAssetV1CustomConstraintActionType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudAssetV1CustomConstraintActionType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTION_TYPE_UNSPECIFIED" => {
                    GoogleCloudAssetV1CustomConstraintActionType::ActionTypeUnspecified
                }
                "ALLOW" => GoogleCloudAssetV1CustomConstraintActionType::Allow,
                "DENY" => GoogleCloudAssetV1CustomConstraintActionType::Deny,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1CustomConstraintActionType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1CustomConstraintActionType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssetV1CustomConstraintMethodTypesItems {
        #[doc = "Constraint applied when creating the resource."]
        Create,
        #[doc = "Constraint applied when deleting the resource."]
        Delete,
        #[doc = "Unspecified. Will results in user error."]
        MethodTypeUnspecified,
        #[doc = "Constraint applied when updating the resource."]
        Update,
    }
    impl GoogleCloudAssetV1CustomConstraintMethodTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudAssetV1CustomConstraintMethodTypesItems::Create => "CREATE",
                GoogleCloudAssetV1CustomConstraintMethodTypesItems::Delete => "DELETE",
                GoogleCloudAssetV1CustomConstraintMethodTypesItems::MethodTypeUnspecified => {
                    "METHOD_TYPE_UNSPECIFIED"
                }
                GoogleCloudAssetV1CustomConstraintMethodTypesItems::Update => "UPDATE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudAssetV1CustomConstraintMethodTypesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudAssetV1CustomConstraintMethodTypesItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudAssetV1CustomConstraintMethodTypesItems, ()> {
            Ok(match s {
                "CREATE" => GoogleCloudAssetV1CustomConstraintMethodTypesItems::Create,
                "DELETE" => GoogleCloudAssetV1CustomConstraintMethodTypesItems::Delete,
                "METHOD_TYPE_UNSPECIFIED" => {
                    GoogleCloudAssetV1CustomConstraintMethodTypesItems::MethodTypeUnspecified
                }
                "UPDATE" => GoogleCloudAssetV1CustomConstraintMethodTypesItems::Update,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudAssetV1CustomConstraintMethodTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudAssetV1CustomConstraintMethodTypesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudAssetV1CustomConstraintMethodTypesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATE" => GoogleCloudAssetV1CustomConstraintMethodTypesItems::Create,
                "DELETE" => GoogleCloudAssetV1CustomConstraintMethodTypesItems::Delete,
                "METHOD_TYPE_UNSPECIFIED" => {
                    GoogleCloudAssetV1CustomConstraintMethodTypesItems::MethodTypeUnspecified
                }
                "UPDATE" => GoogleCloudAssetV1CustomConstraintMethodTypesItems::Update,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1CustomConstraintMethodTypesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1CustomConstraintMethodTypesItems {
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
    pub struct GoogleCloudAssetV1Edge {
        #[doc = "The source node of the edge. For example, it could be a full resource name for a resource node or an email of an identity."]
        #[serde(
            rename = "sourceNode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_node: ::std::option::Option<String>,
        #[doc = "The target node of the edge. For example, it could be a full resource name for a resource node or an email of an identity."]
        #[serde(
            rename = "targetNode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_node: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1Edge {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1Edge {
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
    pub struct GoogleCloudAssetV1GcsDestination {
        #[doc = "Required. The URI of the Cloud Storage object. It’s the same URI that is used by gsutil. Example: “gs://bucket_name/object_name”. See [Viewing and Editing Object Metadata](https://cloud.google.com/storage/docs/viewing-editing-metadata) for more information. If the specified Cloud Storage object already exists and there is no [hold](https://cloud.google.com/storage/docs/object-holds), it will be overwritten with the analysis result."]
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1GcsDestination {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1GcsDestination {
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
    pub struct GoogleCloudAssetV1GovernedContainer {
        #[doc = "The consolidated organization policy for the analyzed resource. The consolidated organization policy is computed by merging and evaluating AnalyzeOrgPolicyGovernedContainersResponse.GovernedContainer.policy_bundle. The evaluation will respect the organization policy [hierarchy rules](https://cloud.google.com/resource-manager/docs/organization-policy/understanding-hierarchy)."]
        #[serde(
            rename = "consolidatedPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub consolidated_policy: ::std::option::Option<crate::schemas::AnalyzerOrgPolicy>,
        #[doc = "The \\[full resource name\\] (https://cloud.google.com/asset-inventory/docs/resource-name-format) of an organization/folder/project resource."]
        #[serde(
            rename = "fullResourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub full_resource_name: ::std::option::Option<String>,
        #[doc = "The \\[full resource name\\] (https://cloud.google.com/asset-inventory/docs/resource-name-format) of the parent of AnalyzeOrgPolicyGovernedContainersResponse.GovernedContainer.full_resource_name."]
        #[serde(
            rename = "parent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent: ::std::option::Option<String>,
        #[doc = "The ordered list of all organization policies from the AnalyzeOrgPoliciesResponse.OrgPolicyResult.consolidated_policy.attached_resource. to the scope specified in the request. If the constraint is defined with default policy, it will also appear in the list."]
        #[serde(
            rename = "policyBundle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy_bundle: ::std::option::Option<Vec<crate::schemas::AnalyzerOrgPolicy>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1GovernedContainer {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1GovernedContainer {
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
    pub struct GoogleCloudAssetV1Identity {
        #[doc = "The analysis state of this identity."]
        #[serde(
            rename = "analysisState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_state: ::std::option::Option<crate::schemas::IamPolicyAnalysisState>,
        #[doc = "The identity name in any form of members appear in [IAM policy binding](https://cloud.google.com/iam/reference/rest/v1/Binding), such as: - user:foo@google.com - group:group1@google.com - serviceAccount:s1@prj1.iam.gserviceaccount.com - projectOwner:some_project_id - domain:google.com - allUsers - etc."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1Identity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1Identity {
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
    pub struct GoogleCloudAssetV1IdentityList {
        #[doc = "Group identity edges of the graph starting from the binding’s group members to any node of the identities. The Edge.source_node contains a group, such as `group:parent@google.com`. The Edge.target_node contains a member of the group, such as `group:child@google.com` or `user:foo@google.com`. This field is present only if the output_group_edges option is enabled in request."]
        #[serde(
            rename = "groupEdges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group_edges: ::std::option::Option<Vec<crate::schemas::GoogleCloudAssetV1Edge>>,
        #[doc = "Only the identities that match one of the following conditions will be presented: - The identity_selector, if it is specified in request; - Otherwise, identities reachable from the policy binding’s members."]
        #[serde(
            rename = "identities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identities: ::std::option::Option<Vec<crate::schemas::GoogleCloudAssetV1Identity>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1IdentityList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1IdentityList {
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
    pub struct GoogleCloudAssetV1ListConstraint {
        #[doc = "Indicates whether values grouped into categories can be used in `Policy.allowed_values` and `Policy.denied_values`. For example, `\"in:Python\"` would match any value in the ‘Python’ group."]
        #[serde(
            rename = "supportsIn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub supports_in: ::std::option::Option<bool>,
        #[doc = "Indicates whether subtrees of Cloud Resource Manager resource hierarchy can be used in `Policy.allowed_values` and `Policy.denied_values`. For example, `\"under:folders/123\"` would match any resource under the ‘folders/123’ folder."]
        #[serde(
            rename = "supportsUnder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub supports_under: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1ListConstraint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1ListConstraint {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudAssetV1P7Beta1Asset {
        #[doc = "Please also refer to the [access level user guide](https://cloud.google.com/access-context-manager/docs/overview#access-levels)."]
        #[serde(
            rename = "accessLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access_level:
            ::std::option::Option<crate::schemas::GoogleIdentityAccesscontextmanagerV1AccessLevel>,
        #[doc = "Please also refer to the [access policy user guide](https://cloud.google.com/access-context-manager/docs/overview#access-policies)."]
        #[serde(
            rename = "accessPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access_policy:
            ::std::option::Option<crate::schemas::GoogleIdentityAccesscontextmanagerV1AccessPolicy>,
        #[doc = "The ancestry path of an asset in Google Cloud [resource hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy), represented as a list of relative resource names. An ancestry path starts with the closest ancestor in the hierarchy and ends at root. If the asset is a project, folder, or organization, the ancestry path starts from the asset itself. Example: `[\"projects/123456789\", \"folders/5432\", \"organizations/1234\"]`"]
        #[serde(
            rename = "ancestors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ancestors: ::std::option::Option<Vec<String>>,
        #[doc = "The type of the asset. Example: `compute.googleapis.com/Disk` See [Supported asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types) for more information."]
        #[serde(
            rename = "assetType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub asset_type: ::std::option::Option<String>,
        #[doc = "A representation of the IAM policy set on a Google Cloud resource. There can be a maximum of one IAM policy set on any given resource. In addition, IAM policies inherit their granted access scope from any policies set on parent resources in the resource hierarchy. Therefore, the effectively policy is the union of both the policy set on this resource and each policy set on all of the resource’s ancestry resource levels in the hierarchy. See [this topic](https://cloud.google.com/iam/help/allow-policies/inheritance) for more information."]
        #[serde(
            rename = "iamPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub iam_policy: ::std::option::Option<crate::schemas::Policy>,
        #[doc = "The full name of the asset. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1` See [Resource names](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more information."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "A representation of an [organization policy](https://cloud.google.com/resource-manager/docs/organization-policy/overview#organization_policy). There can be more than one organization policy with different constraints set on a given resource."]
        #[serde(
            rename = "orgPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub org_policy: ::std::option::Option<Vec<crate::schemas::GoogleCloudOrgpolicyV1Policy>>,
        #[doc = "The related assets of the asset of one relationship type. One asset only represents one type of relationship."]
        #[serde(
            rename = "relatedAssets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub related_assets:
            ::std::option::Option<crate::schemas::GoogleCloudAssetV1P7Beta1RelatedAssets>,
        #[doc = "A representation of the resource."]
        #[serde(
            rename = "resource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource: ::std::option::Option<crate::schemas::GoogleCloudAssetV1P7Beta1Resource>,
        #[doc = "Please also refer to the [service perimeter user guide](https://cloud.google.com/vpc-service-controls/docs/overview)."]
        #[serde(
            rename = "servicePerimeter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_perimeter: ::std::option::Option<
            crate::schemas::GoogleIdentityAccesscontextmanagerV1ServicePerimeter,
        >,
        #[doc = "The last update timestamp of an asset. update_time is updated when create/update/delete operation is performed."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1P7Beta1Asset {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1P7Beta1Asset {
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
    pub struct GoogleCloudAssetV1P7Beta1RelatedAsset {
        #[doc = "The ancestors of an asset in Google Cloud [resource hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy), represented as a list of relative resource names. An ancestry path starts with the closest ancestor in the hierarchy and ends at root. Example: `[\"projects/123456789\", \"folders/5432\", \"organizations/1234\"]`"]
        #[serde(
            rename = "ancestors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ancestors: ::std::option::Option<Vec<String>>,
        #[doc = "The full name of the asset. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1` See [Resource names](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more information."]
        #[serde(
            rename = "asset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub asset: ::std::option::Option<String>,
        #[doc = "The type of the asset. Example: `compute.googleapis.com/Disk` See [Supported asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types) for more information."]
        #[serde(
            rename = "assetType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub asset_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1P7Beta1RelatedAsset {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1P7Beta1RelatedAsset {
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
    pub struct GoogleCloudAssetV1P7Beta1RelatedAssets {
        #[doc = "The peer resources of the relationship."]
        #[serde(
            rename = "assets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub assets:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudAssetV1P7Beta1RelatedAsset>>,
        #[doc = "The detailed relation attributes."]
        #[serde(
            rename = "relationshipAttributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relationship_attributes:
            ::std::option::Option<crate::schemas::GoogleCloudAssetV1P7Beta1RelationshipAttributes>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1P7Beta1RelatedAssets {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1P7Beta1RelatedAssets {
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
    pub struct GoogleCloudAssetV1P7Beta1RelationshipAttributes {
        #[doc = "The detail of the relationship, e.g. `contains`, `attaches`"]
        #[serde(
            rename = "action",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action: ::std::option::Option<String>,
        #[doc = "The unique identifier of the relationship type. Example: `INSTANCE_TO_INSTANCEGROUP`"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The source asset type. Example: `compute.googleapis.com/Instance`"]
        #[serde(
            rename = "sourceResourceType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_resource_type: ::std::option::Option<String>,
        #[doc = "The target asset type. Example: `compute.googleapis.com/Disk`"]
        #[serde(
            rename = "targetResourceType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_resource_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1P7Beta1RelationshipAttributes {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1P7Beta1RelationshipAttributes {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudAssetV1P7Beta1Resource {
        #[doc = "The content of the resource, in which some sensitive fields are removed and may not be present."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The URL of the discovery document containing the resource’s JSON schema. Example: `https://www.googleapis.com/discovery/v1/apis/compute/v1/rest` This value is unspecified for resources that do not have an API based on a discovery document, such as Cloud Bigtable."]
        #[serde(
            rename = "discoveryDocumentUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub discovery_document_uri: ::std::option::Option<String>,
        #[doc = "The JSON schema name listed in the discovery document. Example: `Project` This value is unspecified for resources that do not have an API based on a discovery document, such as Cloud Bigtable."]
        #[serde(
            rename = "discoveryName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub discovery_name: ::std::option::Option<String>,
        #[doc = "The location of the resource in Google Cloud, such as its zone and region. For more information, see https://cloud.google.com/about/locations/."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "The full name of the immediate parent of this resource. See [Resource Names](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more information. For Google Cloud assets, this value is the parent resource defined in the [IAM policy hierarchy](https://cloud.google.com/iam/docs/overview#policy_hierarchy). Example: `//cloudresourcemanager.googleapis.com/projects/my_project_123` For third-party assets, this field may be set differently."]
        #[serde(
            rename = "parent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent: ::std::option::Option<String>,
        #[doc = "The REST URL for accessing the resource. An HTTP `GET` request using this URL returns the resource itself. Example: `https://cloudresourcemanager.googleapis.com/v1/projects/my-project-123` This value is unspecified for resources without a REST API."]
        #[serde(
            rename = "resourceUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_url: ::std::option::Option<String>,
        #[doc = "The API version. Example: `v1`"]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1P7Beta1Resource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1P7Beta1Resource {
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
    pub struct GoogleCloudAssetV1QueryAssetsOutputConfigBigQueryDestination {
        #[doc = "Required. The BigQuery dataset where the query results will be saved. It has the format of “projects/{projectId}/datasets/{datasetId}”."]
        #[serde(
            rename = "dataset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dataset: ::std::option::Option<String>,
        #[doc = "Required. The BigQuery table where the query results will be saved. If this table does not exist, a new table with the given name will be created."]
        #[serde(
            rename = "table",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table: ::std::option::Option<String>,
        #[doc = "Specifies the action that occurs if the destination table or partition already exists. The following values are supported: * WRITE_TRUNCATE: If the table or partition already exists, BigQuery overwrites the entire table or all the partitions data. * WRITE_APPEND: If the table or partition already exists, BigQuery appends the data to the table or the latest partition. * WRITE_EMPTY: If the table already exists and contains data, a ‘duplicate’ error is returned in the job result. The default value is WRITE_EMPTY."]
        #[serde(
            rename = "writeDisposition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub write_disposition: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudAssetV1QueryAssetsOutputConfigBigQueryDestination
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudAssetV1QueryAssetsOutputConfigBigQueryDestination
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
    pub struct GoogleCloudAssetV1Resource {
        #[doc = "The analysis state of this resource."]
        #[serde(
            rename = "analysisState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_state: ::std::option::Option<crate::schemas::IamPolicyAnalysisState>,
        #[doc = "The [full resource name](https://cloud.google.com/asset-inventory/docs/resource-name-format)"]
        #[serde(
            rename = "fullResourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub full_resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1Resource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1Resource {
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
    pub struct GoogleCloudAssetV1Rule {
        #[doc = "Setting this to true means that all values are allowed. This field can be set only in Policies for list constraints."]
        #[serde(
            rename = "allowAll",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_all: ::std::option::Option<bool>,
        #[doc = "The evaluating condition for this rule."]
        #[serde(
            rename = "condition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub condition: ::std::option::Option<crate::schemas::Expr>,
        #[doc = "Setting this to true means that all values are denied. This field can be set only in Policies for list constraints."]
        #[serde(
            rename = "denyAll",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deny_all: ::std::option::Option<bool>,
        #[doc = "If `true`, then the `Policy` is enforced. If `false`, then any configuration is acceptable. This field can be set only in Policies for boolean constraints."]
        #[serde(
            rename = "enforce",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enforce: ::std::option::Option<bool>,
        #[doc = "List of values to be used for this PolicyRule. This field can be set only in Policies for list constraints."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<crate::schemas::GoogleCloudAssetV1StringValues>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1Rule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1Rule {
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
    pub struct GoogleCloudAssetV1StringValues {
        #[doc = "List of values allowed at this resource."]
        #[serde(
            rename = "allowedValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allowed_values: ::std::option::Option<Vec<String>>,
        #[doc = "List of values denied at this resource."]
        #[serde(
            rename = "deniedValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub denied_values: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1StringValues {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1StringValues {
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
    pub struct GoogleCloudOrgpolicyV1BooleanPolicy {
        #[doc = "If `true`, then the `Policy` is enforced. If `false`, then any configuration is acceptable. Suppose you have a `Constraint` `constraints/compute.disableSerialPortAccess` with `constraint_default` set to `ALLOW`. A `Policy` for that `Constraint` exhibits the following behavior: - If the `Policy` at this resource has enforced set to `false`, serial port connection attempts will be allowed. - If the `Policy` at this resource has enforced set to `true`, serial port connection attempts will be refused. - If the `Policy` at this resource is `RestoreDefault`, serial port connection attempts will be allowed. - If no `Policy` is set at this resource or anywhere higher in the resource hierarchy, serial port connection attempts will be allowed. - If no `Policy` is set at this resource, but one exists higher in the resource hierarchy, the behavior is as if the`Policy` were set at this resource. The following examples demonstrate the different possible layerings: Example 1 (nearest `Constraint` wins): `organizations/foo` has a `Policy` with: {enforced: false} `projects/bar` has no `Policy` set. The constraint at `projects/bar` and `organizations/foo` will not be enforced. Example 2 (enforcement gets replaced): `organizations/foo` has a `Policy` with: {enforced: false} `projects/bar` has a `Policy` with: {enforced: true} The constraint at `organizations/foo` is not enforced. The constraint at `projects/bar` is enforced. Example 3 (RestoreDefault): `organizations/foo` has a `Policy` with: {enforced: true} `projects/bar` has a `Policy` with: {RestoreDefault: {}} The constraint at `organizations/foo` is enforced. The constraint at `projects/bar` is not enforced, because `constraint_default` for the `Constraint` is `ALLOW`."]
        #[serde(
            rename = "enforced",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enforced: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudOrgpolicyV1BooleanPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudOrgpolicyV1BooleanPolicy {
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
    pub struct GoogleCloudOrgpolicyV1ListPolicy {
        #[doc = "The policy all_values state."]
        #[serde(
            rename = "allValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub all_values:
            ::std::option::Option<crate::schemas::GoogleCloudOrgpolicyV1ListPolicyAllValues>,
        #[doc = "List of values allowed at this resource. Can only be set if `all_values` is set to `ALL_VALUES_UNSPECIFIED`."]
        #[serde(
            rename = "allowedValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allowed_values: ::std::option::Option<Vec<String>>,
        #[doc = "List of values denied at this resource. Can only be set if `all_values` is set to `ALL_VALUES_UNSPECIFIED`."]
        #[serde(
            rename = "deniedValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub denied_values: ::std::option::Option<Vec<String>>,
        #[doc = "Determines the inheritance behavior for this `Policy`. By default, a `ListPolicy` set at a resource supersedes any `Policy` set anywhere up the resource hierarchy. However, if `inherit_from_parent` is set to `true`, then the values from the effective `Policy` of the parent resource are inherited, meaning the values set in this `Policy` are added to the values inherited up the hierarchy. Setting `Policy` hierarchies that inherit both allowed values and denied values isn’t recommended in most circumstances to keep the configuration simple and understandable. However, it is possible to set a `Policy` with `allowed_values` set that inherits a `Policy` with `denied_values` set. In this case, the values that are allowed must be in `allowed_values` and not present in `denied_values`. For example, suppose you have a `Constraint` `constraints/serviceuser.services`, which has a `constraint_type` of `list_constraint`, and with `constraint_default` set to `ALLOW`. Suppose that at the Organization level, a `Policy` is applied that restricts the allowed API activations to {`E1`, `E2`}. Then, if a `Policy` is applied to a project below the Organization that has `inherit_from_parent` set to `false` and field all_values set to DENY, then an attempt to activate any API will be denied. The following examples demonstrate different possible layerings for `projects/bar` parented by `organizations/foo`: Example 1 (no inherited values): `organizations/foo` has a `Policy` with values: {allowed_values: “E1” allowed_values:“E2”} `projects/bar` has `inherit_from_parent` `false` and values: {allowed_values: “E3” allowed_values: “E4”} The accepted values at `organizations/foo` are `E1`, `E2`. The accepted values at `projects/bar` are `E3`, and `E4`. Example 2 (inherited values): `organizations/foo` has a `Policy` with values: {allowed_values: “E1” allowed_values:“E2”} `projects/bar` has a `Policy` with values: {value: “E3” value: “E4” inherit_from_parent: true} The accepted values at `organizations/foo` are `E1`, `E2`. The accepted values at `projects/bar` are `E1`, `E2`, `E3`, and `E4`. Example 3 (inheriting both allowed and denied values): `organizations/foo` has a `Policy` with values: {allowed_values: “E1” allowed_values: “E2”} `projects/bar` has a `Policy` with: {denied_values: “E1”} The accepted values at `organizations/foo` are `E1`, `E2`. The value accepted at `projects/bar` is `E2`. Example 4 (RestoreDefault): `organizations/foo` has a `Policy` with values: {allowed_values: “E1” allowed_values:“E2”} `projects/bar` has a `Policy` with values: {RestoreDefault: {}} The accepted values at `organizations/foo` are `E1`, `E2`. The accepted values at `projects/bar` are either all or none depending on the value of `constraint_default` (if `ALLOW`, all; if `DENY`, none). Example 5 (no policy inherits parent policy): `organizations/foo` has no `Policy` set. `projects/bar` has no `Policy` set. The accepted values at both levels are either all or none depending on the value of `constraint_default` (if `ALLOW`, all; if `DENY`, none). Example 6 (ListConstraint allowing all): `organizations/foo` has a `Policy` with values: {allowed_values: “E1” allowed_values: “E2”} `projects/bar` has a `Policy` with: {all: ALLOW} The accepted values at `organizations/foo` are `E1`, E2`. Any value is accepted at `projects/bar`. Example 7 (ListConstraint allowing none): `organizations/foo`has a`Policy`with values: {allowed_values: \"E1\" allowed_values: \"E2\"}`projects/bar`has a`Policy`with: {all: DENY} The accepted values at`organizations/foo`are`E1`, E2`. No value is accepted at `projects/bar`. Example 10 (allowed and denied subtrees of Resource Manager hierarchy): Given the following resource hierarchy O1->{F1, F2}; F1->{P1}; F2->{P2, P3}, `organizations/foo` has a `Policy` with values: {allowed_values: “under:organizations/O1”} `projects/bar` has a `Policy` with: {allowed_values: “under:projects/P3”} {denied_values: “under:folders/F2”} The accepted values at `organizations/foo` are `organizations/O1`, `folders/F1`, `folders/F2`, `projects/P1`, `projects/P2`, `projects/P3`. The accepted values at `projects/bar` are `organizations/O1`, `folders/F1`, `projects/P1`."]
        #[serde(
            rename = "inheritFromParent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inherit_from_parent: ::std::option::Option<bool>,
        #[doc = "Optional. The Google Cloud Console will try to default to a configuration that matches the value specified in this `Policy`. If `suggested_value` is not set, it will inherit the value specified higher in the hierarchy, unless `inherit_from_parent` is `false`."]
        #[serde(
            rename = "suggestedValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudOrgpolicyV1ListPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudOrgpolicyV1ListPolicy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudOrgpolicyV1ListPolicyAllValues {
        #[doc = "Indicates that allowed_values or denied_values must be set."]
        AllValuesUnspecified,
        #[doc = "A policy with this set allows all values."]
        Allow,
        #[doc = "A policy with this set denies all values."]
        Deny,
    }
    impl GoogleCloudOrgpolicyV1ListPolicyAllValues {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudOrgpolicyV1ListPolicyAllValues::AllValuesUnspecified => {
                    "ALL_VALUES_UNSPECIFIED"
                }
                GoogleCloudOrgpolicyV1ListPolicyAllValues::Allow => "ALLOW",
                GoogleCloudOrgpolicyV1ListPolicyAllValues::Deny => "DENY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudOrgpolicyV1ListPolicyAllValues {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudOrgpolicyV1ListPolicyAllValues {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudOrgpolicyV1ListPolicyAllValues, ()> {
            Ok(match s {
                "ALL_VALUES_UNSPECIFIED" => {
                    GoogleCloudOrgpolicyV1ListPolicyAllValues::AllValuesUnspecified
                }
                "ALLOW" => GoogleCloudOrgpolicyV1ListPolicyAllValues::Allow,
                "DENY" => GoogleCloudOrgpolicyV1ListPolicyAllValues::Deny,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudOrgpolicyV1ListPolicyAllValues {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudOrgpolicyV1ListPolicyAllValues {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudOrgpolicyV1ListPolicyAllValues {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL_VALUES_UNSPECIFIED" => {
                    GoogleCloudOrgpolicyV1ListPolicyAllValues::AllValuesUnspecified
                }
                "ALLOW" => GoogleCloudOrgpolicyV1ListPolicyAllValues::Allow,
                "DENY" => GoogleCloudOrgpolicyV1ListPolicyAllValues::Deny,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudOrgpolicyV1ListPolicyAllValues {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudOrgpolicyV1ListPolicyAllValues {
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
    pub struct GoogleCloudOrgpolicyV1Policy {
        #[doc = "For boolean `Constraints`, whether to enforce the `Constraint` or not."]
        #[serde(
            rename = "booleanPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub boolean_policy:
            ::std::option::Option<crate::schemas::GoogleCloudOrgpolicyV1BooleanPolicy>,
        #[doc = "The name of the `Constraint` the `Policy` is configuring, for example, `constraints/serviceuser.services`. A [list of available constraints](/resource-manager/docs/organization-policy/org-policy-constraints) is available. Immutable after creation."]
        #[serde(
            rename = "constraint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub constraint: ::std::option::Option<String>,
        #[doc = "An opaque tag indicating the current version of the `Policy`, used for concurrency control. When the `Policy` is returned from either a `GetPolicy` or a `ListOrgPolicy` request, this `etag` indicates the version of the current `Policy` to use when executing a read-modify-write loop. When the `Policy` is returned from a `GetEffectivePolicy` request, the `etag` will be unset. When the `Policy` is used in a `SetOrgPolicy` method, use the `etag` value that was returned from a `GetOrgPolicy` request as part of a read-modify-write loop for concurrency control. Not setting the `etag`in a `SetOrgPolicy` request will result in an unconditional write of the `Policy`."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "List of values either allowed or disallowed."]
        #[serde(
            rename = "listPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub list_policy: ::std::option::Option<crate::schemas::GoogleCloudOrgpolicyV1ListPolicy>,
        #[doc = "Restores the default behavior of the constraint; independent of `Constraint` type."]
        #[serde(
            rename = "restoreDefault",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub restore_default:
            ::std::option::Option<crate::schemas::GoogleCloudOrgpolicyV1RestoreDefault>,
        #[doc = "The time stamp the `Policy` was previously updated. This is set by the server, not specified by the caller, and represents the last time a call to `SetOrgPolicy` was made for that `Policy`. Any value set by the client will be ignored."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
        #[doc = "Version of the `Policy`. Default version is 0;"]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudOrgpolicyV1Policy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudOrgpolicyV1Policy {
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
    pub struct GoogleCloudOrgpolicyV1RestoreDefault {}
    impl ::google_field_selector::FieldSelector for GoogleCloudOrgpolicyV1RestoreDefault {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudOrgpolicyV1RestoreDefault {
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
    pub struct GoogleIdentityAccesscontextmanagerV1AccessLevel {
        #[doc = "A `BasicLevel` composed of `Conditions`."]
        #[serde(
            rename = "basic",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub basic:
            ::std::option::Option<crate::schemas::GoogleIdentityAccesscontextmanagerV1BasicLevel>,
        #[doc = "A `CustomLevel` written in the Common Expression Language."]
        #[serde(
            rename = "custom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom:
            ::std::option::Option<crate::schemas::GoogleIdentityAccesscontextmanagerV1CustomLevel>,
        #[doc = "Description of the `AccessLevel` and its use. Does not affect behavior."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Resource name for the `AccessLevel`. Format: `accessPolicies/{access_policy}/accessLevels/{access_level}`. The `access_level` component must begin with a letter, followed by alphanumeric characters or `_`. Its maximum length is 50 characters. After you create an `AccessLevel`, you cannot change its `name`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Human readable title. Must be unique within the Policy."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1AccessLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1AccessLevel {
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
    pub struct GoogleIdentityAccesscontextmanagerV1AccessPolicy {
        #[doc = "Output only. An opaque identifier for the current version of the `AccessPolicy`. This will always be a strongly validated etag, meaning that two Access Polices will be identical if and only if their etags are identical. Clients should not expect this to be in any specific format."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "Output only. Resource name of the `AccessPolicy`. Format: `accessPolicies/{access_policy}`"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Required. The parent of this `AccessPolicy` in the Cloud Resource Hierarchy. Currently immutable once created. Format: `organizations/{organization_id}`"]
        #[serde(
            rename = "parent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent: ::std::option::Option<String>,
        #[doc = "The scopes of a policy define which resources an ACM policy can restrict, and where ACM resources can be referenced. For example, a policy with scopes=\\[“folders/123”\\] has the following behavior: - vpcsc perimeters can only restrict projects within folders/123 - access levels can only be referenced by resources within folders/123. If empty, there are no limitations on which resources can be restricted by an ACM policy, and there are no limitations on where ACM resources can be referenced. Only one policy can include a given scope (attempting to create a second policy which includes “folders/123” will result in an error). Currently, scopes cannot be modified after a policy is created. Currently, policies can only have a single scope. Format: list of `folders/{folder_number}` or `projects/{project_number}`"]
        #[serde(
            rename = "scopes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scopes: ::std::option::Option<Vec<String>>,
        #[doc = "Required. Human readable title. Does not affect behavior."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1AccessPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1AccessPolicy {
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
    pub struct GoogleIdentityAccesscontextmanagerV1ApiOperation {
        #[doc = "API methods or permissions to allow. Method or permission must belong to the service specified by `service_name` field. A single MethodSelector entry with `*` specified for the `method` field will allow all methods AND permissions for the service specified in `service_name`."]
        #[serde(
            rename = "methodSelectors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub method_selectors: ::std::option::Option<
            Vec<crate::schemas::GoogleIdentityAccesscontextmanagerV1MethodSelector>,
        >,
        #[doc = "The name of the API whose methods or permissions the IngressPolicy or EgressPolicy want to allow. A single ApiOperation with `service_name` field set to `*` will allow all methods AND permissions for all services."]
        #[serde(
            rename = "serviceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1ApiOperation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1ApiOperation {
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
    pub struct GoogleIdentityAccesscontextmanagerV1BasicLevel {
        #[doc = "How the `conditions` list should be combined to determine if a request is granted this `AccessLevel`. If AND is used, each `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. If OR is used, at least one `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. Default behavior is AND."]
        #[serde(
            rename = "combiningFunction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub combining_function: ::std::option::Option<
            crate::schemas::GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction,
        >,
        #[doc = "Required. A list of requirements for the `AccessLevel` to be granted."]
        #[serde(
            rename = "conditions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conditions: ::std::option::Option<
            Vec<crate::schemas::GoogleIdentityAccesscontextmanagerV1Condition>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1BasicLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1BasicLevel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction {
        #[doc = "All `Conditions` must be true for the `BasicLevel` to be true."]
        And,
        #[doc = "If at least one `Condition` is true, then the `BasicLevel` is true."]
        Or,
    }
    impl GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction::And => "AND",
                GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction::Or => "OR",
            }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction,
            (),
        > {
            Ok(match s {
                "AND" => GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction::And,
                "OR" => GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction::Or,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AND" => GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction::And,
                "OR" => GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction::Or,
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
        for GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction
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
    pub struct GoogleIdentityAccesscontextmanagerV1Condition {
        #[doc = "Device specific restrictions, all restrictions must hold for the Condition to be true. If not specified, all devices are allowed."]
        #[serde(
            rename = "devicePolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_policy:
            ::std::option::Option<crate::schemas::GoogleIdentityAccesscontextmanagerV1DevicePolicy>,
        #[doc = "CIDR block IP subnetwork specification. May be IPv4 or IPv6. Note that for a CIDR IP address block, the specified IP address portion must be properly truncated (i.e. all the host bits must be zero) or the input is considered malformed. For example, “192.0.2.0/24” is accepted but “192.0.2.1/24” is not. Similarly, for IPv6, “2001:db8::/32” is accepted whereas “2001:db8::1/32” is not. The originating IP of a request must be in one of the listed subnets in order for this Condition to be true. If empty, all IP addresses are allowed."]
        #[serde(
            rename = "ipSubnetworks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ip_subnetworks: ::std::option::Option<Vec<String>>,
        #[doc = "The request must be made by one of the provided user or service accounts. Groups are not supported. Syntax: `user:{emailid}` `serviceAccount:{emailid}` If not specified, a request may come from any user."]
        #[serde(
            rename = "members",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub members: ::std::option::Option<Vec<String>>,
        #[doc = "Whether to negate the Condition. If true, the Condition becomes a NAND over its non-empty fields, each field must be false for the Condition overall to be satisfied. Defaults to false."]
        #[serde(
            rename = "negate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub negate: ::std::option::Option<bool>,
        #[doc = "The request must originate from one of the provided countries/regions. Must be valid ISO 3166-1 alpha-2 codes."]
        #[serde(
            rename = "regions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub regions: ::std::option::Option<Vec<String>>,
        #[doc = "A list of other access levels defined in the same `Policy`, referenced by resource name. Referencing an `AccessLevel` which does not exist is an error. All access levels listed must be granted for the Condition to be true. Example: “`accessPolicies/MY_POLICY/accessLevels/LEVEL_NAME\"`"]
        #[serde(
            rename = "requiredAccessLevels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub required_access_levels: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1Condition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1Condition {
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
    pub struct GoogleIdentityAccesscontextmanagerV1CustomLevel {
        #[doc = "Required. A Cloud CEL expression evaluating to a boolean."]
        #[serde(
            rename = "expr",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expr: ::std::option::Option<crate::schemas::Expr>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1CustomLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1CustomLevel {
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
    pub struct GoogleIdentityAccesscontextmanagerV1DevicePolicy { # [doc = "Allowed device management levels, an empty list allows all management levels."] # [serde (rename = "allowedDeviceManagementLevels" , default , skip_serializing_if = "std::option::Option::is_none")] pub allowed_device_management_levels : :: std :: option :: Option < Vec < crate :: schemas :: GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems > > , # [doc = "Allowed encryptions statuses, an empty list allows all statuses."] # [serde (rename = "allowedEncryptionStatuses" , default , skip_serializing_if = "std::option::Option::is_none")] pub allowed_encryption_statuses : :: std :: option :: Option < Vec < crate :: schemas :: GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems > > , # [doc = "Allowed OS versions, an empty list allows all types and all versions."] # [serde (rename = "osConstraints" , default , skip_serializing_if = "std::option::Option::is_none")] pub os_constraints : :: std :: option :: Option < Vec < crate :: schemas :: GoogleIdentityAccesscontextmanagerV1OsConstraint > > , # [doc = "Whether the device needs to be approved by the customer admin."] # [serde (rename = "requireAdminApproval" , default , skip_serializing_if = "std::option::Option::is_none")] pub require_admin_approval : :: std :: option :: Option < bool > , # [doc = "Whether the device needs to be corp owned."] # [serde (rename = "requireCorpOwned" , default , skip_serializing_if = "std::option::Option::is_none")] pub require_corp_owned : :: std :: option :: Option < bool > , # [doc = "Whether or not screenlock is required for the DevicePolicy to be true. Defaults to `false`."] # [serde (rename = "requireScreenlock" , default , skip_serializing_if = "std::option::Option::is_none")] pub require_screenlock : :: std :: option :: Option < bool > , }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1DevicePolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1DevicePolicy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems {
        #[doc = "Basic management is enabled, which is generally limited to monitoring and wiping the corporate account."]
        Basic,
        #[doc = "Complete device management. This includes more thorough monitoring and the ability to directly manage the device (such as remote wiping). This can be enabled through the Android Enterprise Platform."]
        Complete,
        #[doc = "The device’s management level is not specified or not known."]
        ManagementUnspecified,
        #[doc = "The device is not managed."]
        None,
    }
    impl GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems {
        pub fn as_str(self) -> &'static str {
            match self { GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: Basic => "BASIC" , GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: Complete => "COMPLETE" , GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: ManagementUnspecified => "MANAGEMENT_UNSPECIFIED" , GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: None => "NONE" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems,
            (),
        > {
            Ok (match s { "BASIC" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: Basic , "COMPLETE" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: Complete , "MANAGEMENT_UNSPECIFIED" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: ManagementUnspecified , "NONE" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: None , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "BASIC" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: Basic , "COMPLETE" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: Complete , "MANAGEMENT_UNSPECIFIED" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: ManagementUnspecified , "NONE" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: None , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems {
        #[doc = "The device is encrypted."]
        Encrypted,
        #[doc = "The encryption status of the device is not specified or not known."]
        EncryptionUnspecified,
        #[doc = "The device does not support encryption."]
        EncryptionUnsupported,
        #[doc = "The device supports encryption, but is currently unencrypted."]
        Unencrypted,
    }
    impl GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems {
        pub fn as_str(self) -> &'static str {
            match self { GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: Encrypted => "ENCRYPTED" , GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: EncryptionUnspecified => "ENCRYPTION_UNSPECIFIED" , GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: EncryptionUnsupported => "ENCRYPTION_UNSUPPORTED" , GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: Unencrypted => "UNENCRYPTED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems,
            (),
        > {
            Ok (match s { "ENCRYPTED" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: Encrypted , "ENCRYPTION_UNSPECIFIED" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: EncryptionUnspecified , "ENCRYPTION_UNSUPPORTED" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: EncryptionUnsupported , "UNENCRYPTED" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: Unencrypted , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ENCRYPTED" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: Encrypted , "ENCRYPTION_UNSPECIFIED" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: EncryptionUnspecified , "ENCRYPTION_UNSUPPORTED" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: EncryptionUnsupported , "UNENCRYPTED" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: Unencrypted , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems
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
    pub struct GoogleIdentityAccesscontextmanagerV1EgressFrom {
        #[doc = "A list of identities that are allowed access through this \\[EgressPolicy\\]. Should be in the format of email address. The email address should represent individual user or service account only."]
        #[serde(
            rename = "identities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identities: ::std::option::Option<Vec<String>>,
        #[doc = "Specifies the type of identities that are allowed access to outside the perimeter. If left unspecified, then members of `identities` field will be allowed access."]
        #[serde(
            rename = "identityType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identity_type: ::std::option::Option<
            crate::schemas::GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1EgressFrom {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1EgressFrom {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType {
        #[doc = "Authorize access from all identities outside the perimeter."]
        AnyIdentity,
        #[doc = "Authorize access from all service accounts outside the perimeter."]
        AnyServiceAccount,
        #[doc = "Authorize access from all human users outside the perimeter."]
        AnyUserAccount,
        #[doc = "No blanket identity group specified."]
        IdentityTypeUnspecified,
    }
    impl GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: AnyIdentity => "ANY_IDENTITY" , GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: AnyServiceAccount => "ANY_SERVICE_ACCOUNT" , GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: AnyUserAccount => "ANY_USER_ACCOUNT" , GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: IdentityTypeUnspecified => "IDENTITY_TYPE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType, ()>
        {
            Ok (match s { "ANY_IDENTITY" => GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: AnyIdentity , "ANY_SERVICE_ACCOUNT" => GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: AnyServiceAccount , "ANY_USER_ACCOUNT" => GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: AnyUserAccount , "IDENTITY_TYPE_UNSPECIFIED" => GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: IdentityTypeUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ANY_IDENTITY" => GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: AnyIdentity , "ANY_SERVICE_ACCOUNT" => GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: AnyServiceAccount , "ANY_USER_ACCOUNT" => GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: AnyUserAccount , "IDENTITY_TYPE_UNSPECIFIED" => GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: IdentityTypeUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType
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
    pub struct GoogleIdentityAccesscontextmanagerV1EgressPolicy {
        #[doc = "Defines conditions on the source of a request causing this EgressPolicy to apply."]
        #[serde(
            rename = "egressFrom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub egress_from:
            ::std::option::Option<crate::schemas::GoogleIdentityAccesscontextmanagerV1EgressFrom>,
        #[doc = "Defines the conditions on the ApiOperation and destination resources that cause this EgressPolicy to apply."]
        #[serde(
            rename = "egressTo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub egress_to:
            ::std::option::Option<crate::schemas::GoogleIdentityAccesscontextmanagerV1EgressTo>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1EgressPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1EgressPolicy {
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
    pub struct GoogleIdentityAccesscontextmanagerV1EgressTo {
        #[doc = "A list of external resources that are allowed to be accessed. Only AWS and Azure resources are supported. For Amazon S3, the supported format is s3://BUCKET_NAME. For Azure Storage, the supported format is azure://myaccount.blob.core.windows.net/CONTAINER_NAME. A request matches if it contains an external resource in this list (Example: s3://bucket/path). Currently ‘\\*’ is not allowed."]
        #[serde(
            rename = "externalResources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub external_resources: ::std::option::Option<Vec<String>>,
        #[doc = "A list of ApiOperations allowed to be performed by the sources specified in the corresponding EgressFrom. A request matches if it uses an operation/service in this list."]
        #[serde(
            rename = "operations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operations: ::std::option::Option<
            Vec<crate::schemas::GoogleIdentityAccesscontextmanagerV1ApiOperation>,
        >,
        #[doc = "A list of resources, currently only projects in the form `projects/`, that are allowed to be accessed by sources defined in the corresponding EgressFrom. A request matches if it contains a resource in this list. If `*` is specified for `resources`, then this EgressTo rule will authorize access to all resources outside the perimeter."]
        #[serde(
            rename = "resources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resources: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1EgressTo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1EgressTo {
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
    pub struct GoogleIdentityAccesscontextmanagerV1IngressFrom {
        #[doc = "A list of identities that are allowed access through this ingress policy. Should be in the format of email address. The email address should represent individual user or service account only."]
        #[serde(
            rename = "identities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identities: ::std::option::Option<Vec<String>>,
        #[doc = "Specifies the type of identities that are allowed access from outside the perimeter. If left unspecified, then members of `identities` field will be allowed access."]
        #[serde(
            rename = "identityType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identity_type: ::std::option::Option<
            crate::schemas::GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType,
        >,
        #[doc = "Sources that this IngressPolicy authorizes access from."]
        #[serde(
            rename = "sources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sources: ::std::option::Option<
            Vec<crate::schemas::GoogleIdentityAccesscontextmanagerV1IngressSource>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1IngressFrom {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1IngressFrom {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType {
        #[doc = "Authorize access from all identities outside the perimeter."]
        AnyIdentity,
        #[doc = "Authorize access from all service accounts outside the perimeter."]
        AnyServiceAccount,
        #[doc = "Authorize access from all human users outside the perimeter."]
        AnyUserAccount,
        #[doc = "No blanket identity group specified."]
        IdentityTypeUnspecified,
    }
    impl GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: AnyIdentity => "ANY_IDENTITY" , GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: AnyServiceAccount => "ANY_SERVICE_ACCOUNT" , GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: AnyUserAccount => "ANY_USER_ACCOUNT" , GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: IdentityTypeUnspecified => "IDENTITY_TYPE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType, ()>
        {
            Ok (match s { "ANY_IDENTITY" => GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: AnyIdentity , "ANY_SERVICE_ACCOUNT" => GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: AnyServiceAccount , "ANY_USER_ACCOUNT" => GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: AnyUserAccount , "IDENTITY_TYPE_UNSPECIFIED" => GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: IdentityTypeUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ANY_IDENTITY" => GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: AnyIdentity , "ANY_SERVICE_ACCOUNT" => GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: AnyServiceAccount , "ANY_USER_ACCOUNT" => GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: AnyUserAccount , "IDENTITY_TYPE_UNSPECIFIED" => GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: IdentityTypeUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType
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
    pub struct GoogleIdentityAccesscontextmanagerV1IngressPolicy {
        #[doc = "Defines the conditions on the source of a request causing this IngressPolicy to apply."]
        #[serde(
            rename = "ingressFrom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ingress_from:
            ::std::option::Option<crate::schemas::GoogleIdentityAccesscontextmanagerV1IngressFrom>,
        #[doc = "Defines the conditions on the ApiOperation and request destination that cause this IngressPolicy to apply."]
        #[serde(
            rename = "ingressTo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ingress_to:
            ::std::option::Option<crate::schemas::GoogleIdentityAccesscontextmanagerV1IngressTo>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1IngressPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1IngressPolicy {
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
    pub struct GoogleIdentityAccesscontextmanagerV1IngressSource {
        #[doc = "An AccessLevel resource name that allow resources within the ServicePerimeters to be accessed from the internet. AccessLevels listed must be in the same policy as this ServicePerimeter. Referencing a nonexistent AccessLevel will cause an error. If no AccessLevel names are listed, resources within the perimeter can only be accessed via Google Cloud calls with request origins within the perimeter. Example: `accessPolicies/MY_POLICY/accessLevels/MY_LEVEL`. If a single `*` is specified for `access_level`, then all IngressSources will be allowed."]
        #[serde(
            rename = "accessLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access_level: ::std::option::Option<String>,
        #[doc = "A Google Cloud resource that is allowed to ingress the perimeter. Requests from these resources will be allowed to access perimeter data. Currently only projects and VPCs are allowed. Project format: `projects/{project_number}` VPC network format: `//compute.googleapis.com/projects/{PROJECT_ID}/global/networks/{NAME}`. The project may be in any Google Cloud organization, not just the organization that the perimeter is defined in. `*` is not allowed, the case of allowing all Google Cloud resources only is not supported."]
        #[serde(
            rename = "resource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1IngressSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1IngressSource {
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
    pub struct GoogleIdentityAccesscontextmanagerV1IngressTo {
        #[doc = "A list of ApiOperations allowed to be performed by the sources specified in corresponding IngressFrom in this ServicePerimeter."]
        #[serde(
            rename = "operations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operations: ::std::option::Option<
            Vec<crate::schemas::GoogleIdentityAccesscontextmanagerV1ApiOperation>,
        >,
        #[doc = "A list of resources, currently only projects in the form `projects/`, protected by this ServicePerimeter that are allowed to be accessed by sources defined in the corresponding IngressFrom. If a single `*` is specified, then access to all resources inside the perimeter are allowed."]
        #[serde(
            rename = "resources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resources: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1IngressTo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1IngressTo {
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
    pub struct GoogleIdentityAccesscontextmanagerV1MethodSelector {
        #[doc = "Value for `method` should be a valid method name for the corresponding `service_name` in ApiOperation. If `*` used as value for `method`, then ALL methods and permissions are allowed."]
        #[serde(
            rename = "method",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub method: ::std::option::Option<String>,
        #[doc = "Value for `permission` should be a valid Cloud IAM permission for the corresponding `service_name` in ApiOperation."]
        #[serde(
            rename = "permission",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permission: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1MethodSelector {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1MethodSelector {
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
    pub struct GoogleIdentityAccesscontextmanagerV1OsConstraint {
        #[doc = "The minimum allowed OS version. If not set, any version of this OS satisfies the constraint. Format: `\"major.minor.patch\"`. Examples: `\"10.5.301\"`, `\"9.2.1\"`."]
        #[serde(
            rename = "minimumVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub minimum_version: ::std::option::Option<String>,
        #[doc = "Required. The allowed OS type."]
        #[serde(
            rename = "osType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_type: ::std::option::Option<
            crate::schemas::GoogleIdentityAccesscontextmanagerV1OsConstraintOsType,
        >,
        #[doc = "Only allows requests from devices with a verified Chrome OS. Verifications includes requirements that the device is enterprise-managed, conformant to domain policies, and the caller has permission to call the API targeted by the request."]
        #[serde(
            rename = "requireVerifiedChromeOs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub require_verified_chrome_os: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1OsConstraint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1OsConstraint {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleIdentityAccesscontextmanagerV1OsConstraintOsType {
        #[doc = "An Android operating system."]
        Android,
        #[doc = "A desktop ChromeOS operating system."]
        DesktopChromeOs,
        #[doc = "A desktop Linux operating system."]
        DesktopLinux,
        #[doc = "A desktop Mac operating system."]
        DesktopMac,
        #[doc = "A desktop Windows operating system."]
        DesktopWindows,
        #[doc = "An iOS operating system."]
        Ios,
        #[doc = "The operating system of the device is not specified or not known."]
        OsUnspecified,
    }
    impl GoogleIdentityAccesscontextmanagerV1OsConstraintOsType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::Android => "ANDROID",
                GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopChromeOs => {
                    "DESKTOP_CHROME_OS"
                }
                GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopLinux => {
                    "DESKTOP_LINUX"
                }
                GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopMac => "DESKTOP_MAC",
                GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopWindows => {
                    "DESKTOP_WINDOWS"
                }
                GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::Ios => "IOS",
                GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::OsUnspecified => {
                    "OS_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleIdentityAccesscontextmanagerV1OsConstraintOsType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleIdentityAccesscontextmanagerV1OsConstraintOsType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleIdentityAccesscontextmanagerV1OsConstraintOsType, ()>
        {
            Ok(match s {
                "ANDROID" => GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::Android,
                "DESKTOP_CHROME_OS" => {
                    GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopChromeOs
                }
                "DESKTOP_LINUX" => {
                    GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopLinux
                }
                "DESKTOP_MAC" => GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopMac,
                "DESKTOP_WINDOWS" => {
                    GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopWindows
                }
                "IOS" => GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::Ios,
                "OS_UNSPECIFIED" => {
                    GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::OsUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleIdentityAccesscontextmanagerV1OsConstraintOsType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleIdentityAccesscontextmanagerV1OsConstraintOsType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleIdentityAccesscontextmanagerV1OsConstraintOsType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANDROID" => GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::Android,
                "DESKTOP_CHROME_OS" => {
                    GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopChromeOs
                }
                "DESKTOP_LINUX" => {
                    GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopLinux
                }
                "DESKTOP_MAC" => GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopMac,
                "DESKTOP_WINDOWS" => {
                    GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopWindows
                }
                "IOS" => GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::Ios,
                "OS_UNSPECIFIED" => {
                    GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::OsUnspecified
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
        for GoogleIdentityAccesscontextmanagerV1OsConstraintOsType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleIdentityAccesscontextmanagerV1OsConstraintOsType
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
    pub struct GoogleIdentityAccesscontextmanagerV1ServicePerimeter {
        #[doc = "Description of the `ServicePerimeter` and its use. Does not affect behavior."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Resource name for the `ServicePerimeter`. Format: `accessPolicies/{access_policy}/servicePerimeters/{service_perimeter}`. The `service_perimeter` component must begin with a letter, followed by alphanumeric characters or `_`. After you create a `ServicePerimeter`, you cannot change its `name`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Perimeter type indicator. A single project or VPC network is allowed to be a member of single regular perimeter, but multiple service perimeter bridges. A project cannot be a included in a perimeter bridge without being included in regular perimeter. For perimeter bridges, the restricted service list as well as access level lists must be empty."]
        #[serde(
            rename = "perimeterType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub perimeter_type: ::std::option::Option<
            crate::schemas::GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType,
        >,
        #[doc = "Proposed (or dry run) ServicePerimeter configuration. This configuration allows to specify and test ServicePerimeter configuration without enforcing actual access restrictions. Only allowed to be set when the “use_explicit_dry_run_spec” flag is set."]
        #[serde(
            rename = "spec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spec: ::std::option::Option<
            crate::schemas::GoogleIdentityAccesscontextmanagerV1ServicePerimeterConfig,
        >,
        #[doc = "Current ServicePerimeter configuration. Specifies sets of resources, restricted services and access levels that determine perimeter content and boundaries."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<
            crate::schemas::GoogleIdentityAccesscontextmanagerV1ServicePerimeterConfig,
        >,
        #[doc = "Human readable title. Must be unique within the Policy."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "Use explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists for all Service Perimeters, and that spec is identical to the status for those Service Perimeters. When this flag is set, it inhibits the generation of the implicit spec, thereby allowing the user to explicitly provide a configuration (“spec”) to use in a dry-run version of the Service Perimeter. This allows the user to test changes to the enforced config (“status”) without actually enforcing them. This testing is done through analyzing the differences between currently enforced and suggested restrictions. use_explicit_dry_run_spec must bet set to True if any of the fields in the spec are set to non-default values."]
        #[serde(
            rename = "useExplicitDryRunSpec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_explicit_dry_run_spec: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleIdentityAccesscontextmanagerV1ServicePerimeter
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1ServicePerimeter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType {
        #[doc = "Perimeter Bridge."]
        PerimeterTypeBridge,
        #[doc = "Regular Perimeter. When no value is specified, the perimeter uses this type."]
        PerimeterTypeRegular,
    }
    impl GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType :: PerimeterTypeBridge => "PERIMETER_TYPE_BRIDGE" , GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType :: PerimeterTypeRegular => "PERIMETER_TYPE_REGULAR" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType,
            (),
        > {
            Ok (match s { "PERIMETER_TYPE_BRIDGE" => GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType :: PerimeterTypeBridge , "PERIMETER_TYPE_REGULAR" => GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType :: PerimeterTypeRegular , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "PERIMETER_TYPE_BRIDGE" => GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType :: PerimeterTypeBridge , "PERIMETER_TYPE_REGULAR" => GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType :: PerimeterTypeRegular , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType
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
    pub struct GoogleIdentityAccesscontextmanagerV1ServicePerimeterConfig {
        #[doc = "A list of `AccessLevel` resource names that allow resources within the `ServicePerimeter` to be accessed from the internet. `AccessLevels` listed must be in the same policy as this `ServicePerimeter`. Referencing a nonexistent `AccessLevel` is a syntax error. If no `AccessLevel` names are listed, resources within the perimeter can only be accessed via Google Cloud calls with request origins within the perimeter. Example: `\"accessPolicies/MY_POLICY/accessLevels/MY_LEVEL\"`. For Service Perimeter Bridge, must be empty."]
        #[serde(
            rename = "accessLevels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access_levels: ::std::option::Option<Vec<String>>,
        #[doc = "List of EgressPolicies to apply to the perimeter. A perimeter may have multiple EgressPolicies, each of which is evaluated separately. Access is granted if any EgressPolicy grants it. Must be empty for a perimeter bridge."]
        #[serde(
            rename = "egressPolicies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub egress_policies: ::std::option::Option<
            Vec<crate::schemas::GoogleIdentityAccesscontextmanagerV1EgressPolicy>,
        >,
        #[doc = "List of IngressPolicies to apply to the perimeter. A perimeter may have multiple IngressPolicies, each of which is evaluated separately. Access is granted if any Ingress Policy grants it. Must be empty for a perimeter bridge."]
        #[serde(
            rename = "ingressPolicies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ingress_policies: ::std::option::Option<
            Vec<crate::schemas::GoogleIdentityAccesscontextmanagerV1IngressPolicy>,
        >,
        #[doc = "A list of Google Cloud resources that are inside of the service perimeter. Currently only projects and VPCs are allowed. Project format: `projects/{project_number}` VPC network format: `//compute.googleapis.com/projects/{PROJECT_ID}/global/networks/{NAME}`."]
        #[serde(
            rename = "resources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resources: ::std::option::Option<Vec<String>>,
        #[doc = "Google Cloud services that are subject to the Service Perimeter restrictions. For example, if `storage.googleapis.com` is specified, access to the storage buckets inside the perimeter must meet the perimeter’s access restrictions."]
        #[serde(
            rename = "restrictedServices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub restricted_services: ::std::option::Option<Vec<String>>,
        #[doc = "Configuration for APIs allowed within Perimeter."]
        #[serde(
            rename = "vpcAccessibleServices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vpc_accessible_services: ::std::option::Option<
            crate::schemas::GoogleIdentityAccesscontextmanagerV1VpcAccessibleServices,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleIdentityAccesscontextmanagerV1ServicePerimeterConfig
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleIdentityAccesscontextmanagerV1ServicePerimeterConfig
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
    pub struct GoogleIdentityAccesscontextmanagerV1VpcAccessibleServices {
        #[doc = "The list of APIs usable within the Service Perimeter. Must be empty unless ‘enable_restriction’ is True. You can specify a list of individual services, as well as include the ‘RESTRICTED-SERVICES’ value, which automatically includes all of the services protected by the perimeter."]
        #[serde(
            rename = "allowedServices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allowed_services: ::std::option::Option<Vec<String>>,
        #[doc = "Whether to restrict API calls within the Service Perimeter to the list of APIs specified in ‘allowed_services’."]
        #[serde(
            rename = "enableRestriction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_restriction: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleIdentityAccesscontextmanagerV1VpcAccessibleServices
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleIdentityAccesscontextmanagerV1VpcAccessibleServices
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
    pub struct IamPolicyAnalysis {
        #[doc = "The analysis query."]
        #[serde(
            rename = "analysisQuery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_query: ::std::option::Option<crate::schemas::IamPolicyAnalysisQuery>,
        #[doc = "A list of IamPolicyAnalysisResult that matches the analysis query, or empty if no result is found."]
        #[serde(
            rename = "analysisResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_results: ::std::option::Option<Vec<crate::schemas::IamPolicyAnalysisResult>>,
        #[doc = "Represents whether all entries in the analysis_results have been fully explored to answer the query."]
        #[serde(
            rename = "fullyExplored",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fully_explored: ::std::option::Option<bool>,
        #[doc = "A list of non-critical errors happened during the query handling."]
        #[serde(
            rename = "nonCriticalErrors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_critical_errors: ::std::option::Option<Vec<crate::schemas::IamPolicyAnalysisState>>,
    }
    impl ::google_field_selector::FieldSelector for IamPolicyAnalysis {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IamPolicyAnalysis {
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
    pub struct IamPolicyAnalysisOutputConfig {
        #[doc = "Destination on BigQuery."]
        #[serde(
            rename = "bigqueryDestination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bigquery_destination:
            ::std::option::Option<crate::schemas::GoogleCloudAssetV1BigQueryDestination>,
        #[doc = "Destination on Cloud Storage."]
        #[serde(
            rename = "gcsDestination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_destination:
            ::std::option::Option<crate::schemas::GoogleCloudAssetV1GcsDestination>,
    }
    impl ::google_field_selector::FieldSelector for IamPolicyAnalysisOutputConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IamPolicyAnalysisOutputConfig {
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
    pub struct IamPolicyAnalysisQuery {
        #[doc = "Optional. Specifies roles or permissions for analysis. This is optional."]
        #[serde(
            rename = "accessSelector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access_selector: ::std::option::Option<crate::schemas::AccessSelector>,
        #[doc = "Optional. The hypothetical context for IAM conditions evaluation."]
        #[serde(
            rename = "conditionContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub condition_context: ::std::option::Option<crate::schemas::ConditionContext>,
        #[doc = "Optional. Specifies an identity for analysis."]
        #[serde(
            rename = "identitySelector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identity_selector: ::std::option::Option<crate::schemas::IdentitySelector>,
        #[doc = "Optional. The query options."]
        #[serde(
            rename = "options",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub options: ::std::option::Option<crate::schemas::Options>,
        #[doc = "Optional. Specifies a resource for analysis."]
        #[serde(
            rename = "resourceSelector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_selector: ::std::option::Option<crate::schemas::ResourceSelector>,
        #[doc = "Required. The relative name of the root asset. Only resources and IAM policies within the scope will be analyzed. This can only be an organization number (such as “organizations/123”), a folder number (such as “folders/123”), a project ID (such as “projects/my-project-id”), or a project number (such as “projects/12345”). To know how to get organization id, visit [here ](https://cloud.google.com/resource-manager/docs/creating-managing-organization#retrieving_your_organization_id). To know how to get folder or project id, visit [here ](https://cloud.google.com/resource-manager/docs/creating-managing-folders#viewing_or_listing_folders_and_projects)."]
        #[serde(
            rename = "scope",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scope: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for IamPolicyAnalysisQuery {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IamPolicyAnalysisQuery {
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
    pub struct IamPolicyAnalysisResult {
        #[doc = "The access control lists derived from the iam_binding that match or potentially match resource and access selectors specified in the request."]
        #[serde(
            rename = "accessControlLists",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access_control_lists:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudAssetV1AccessControlList>>,
        #[doc = "The [full resource name](https://cloud.google.com/asset-inventory/docs/resource-name-format) of the resource to which the iam_binding policy attaches."]
        #[serde(
            rename = "attachedResourceFullName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attached_resource_full_name: ::std::option::Option<String>,
        #[doc = "Represents whether all analyses on the iam_binding have successfully finished."]
        #[serde(
            rename = "fullyExplored",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fully_explored: ::std::option::Option<bool>,
        #[doc = "The IAM policy binding under analysis."]
        #[serde(
            rename = "iamBinding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub iam_binding: ::std::option::Option<crate::schemas::Binding>,
        #[doc = "The identity list derived from members of the iam_binding that match or potentially match identity selector specified in the request."]
        #[serde(
            rename = "identityList",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identity_list: ::std::option::Option<crate::schemas::GoogleCloudAssetV1IdentityList>,
    }
    impl ::google_field_selector::FieldSelector for IamPolicyAnalysisResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IamPolicyAnalysisResult {
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
    pub struct IamPolicyAnalysisState {
        #[doc = "The human-readable description of the cause of failure."]
        #[serde(
            rename = "cause",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cause: ::std::option::Option<String>,
        #[doc = "The Google standard error code that best describes the state. For example: - OK means the analysis on this entity has been successfully finished; - PERMISSION_DENIED means an access denied error is encountered; - DEADLINE_EXCEEDED means the analysis on this entity hasn’t been started in time;"]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<crate::schemas::IamPolicyAnalysisStateCode>,
    }
    impl ::google_field_selector::FieldSelector for IamPolicyAnalysisState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IamPolicyAnalysisState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum IamPolicyAnalysisStateCode {
        #[doc = "The operation was aborted, typically due to a concurrency issue such as a sequencer check failure or transaction abort. See the guidelines above for deciding between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`. HTTP Mapping: 409 Conflict"]
        Aborted,
        #[doc = "The entity that a client attempted to create (e.g., file or directory) already exists. HTTP Mapping: 409 Conflict"]
        AlreadyExists,
        #[doc = "The operation was cancelled, typically by the caller. HTTP Mapping: 499 Client Closed Request"]
        Cancelled,
        #[doc = "Unrecoverable data loss or corruption. HTTP Mapping: 500 Internal Server Error"]
        DataLoss,
        #[doc = "The deadline expired before the operation could complete. For operations that change the state of the system, this error may be returned even if the operation has completed successfully. For example, a successful response from a server could have been delayed long enough for the deadline to expire. HTTP Mapping: 504 Gateway Timeout"]
        DeadlineExceeded,
        #[doc = "The operation was rejected because the system is not in a state required for the operation’s execution. For example, the directory to be deleted is non-empty, an rmdir operation is applied to a non-directory, etc. Service implementors can use the following guidelines to decide between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`: (a) Use `UNAVAILABLE` if the client can retry just the failing call. (b) Use `ABORTED` if the client should retry at a higher level. For example, when a client-specified test-and-set fails, indicating the client should restart a read-modify-write sequence. (c) Use `FAILED_PRECONDITION` if the client should not retry until the system state has been explicitly fixed. For example, if an “rmdir” fails because the directory is non-empty, `FAILED_PRECONDITION` should be returned since the client should not retry unless the files are deleted from the directory. HTTP Mapping: 400 Bad Request"]
        FailedPrecondition,
        #[doc = "Internal errors. This means that some invariants expected by the underlying system have been broken. This error code is reserved for serious errors. HTTP Mapping: 500 Internal Server Error"]
        Internal,
        #[doc = "The client specified an invalid argument. Note that this differs from `FAILED_PRECONDITION`. `INVALID_ARGUMENT` indicates arguments that are problematic regardless of the state of the system (e.g., a malformed file name). HTTP Mapping: 400 Bad Request"]
        InvalidArgument,
        #[doc = "Some requested entity (e.g., file or directory) was not found. Note to server developers: if a request is denied for an entire class of users, such as gradual feature rollout or undocumented allowlist, `NOT_FOUND` may be used. If a request is denied for some users within a class of users, such as user-based access control, `PERMISSION_DENIED` must be used. HTTP Mapping: 404 Not Found"]
        NotFound,
        #[doc = "Not an error; returned on success. HTTP Mapping: 200 OK"]
        Ok,
        #[doc = "The operation was attempted past the valid range. E.g., seeking or reading past end-of-file. Unlike `INVALID_ARGUMENT`, this error indicates a problem that may be fixed if the system state changes. For example, a 32-bit file system will generate `INVALID_ARGUMENT` if asked to read at an offset that is not in the range \\[0,2^32-1\\], but it will generate `OUT_OF_RANGE` if asked to read from an offset past the current file size. There is a fair bit of overlap between `FAILED_PRECONDITION` and `OUT_OF_RANGE`. We recommend using `OUT_OF_RANGE` (the more specific error) when it applies so that callers who are iterating through a space can easily look for an `OUT_OF_RANGE` error to detect when they are done. HTTP Mapping: 400 Bad Request"]
        OutOfRange,
        #[doc = "The caller does not have permission to execute the specified operation. `PERMISSION_DENIED` must not be used for rejections caused by exhausting some resource (use `RESOURCE_EXHAUSTED` instead for those errors). `PERMISSION_DENIED` must not be used if the caller can not be identified (use `UNAUTHENTICATED` instead for those errors). This error code does not imply the request is valid or the requested entity exists or satisfies other pre-conditions. HTTP Mapping: 403 Forbidden"]
        PermissionDenied,
        #[doc = "Some resource has been exhausted, perhaps a per-user quota, or perhaps the entire file system is out of space. HTTP Mapping: 429 Too Many Requests"]
        ResourceExhausted,
        #[doc = "The request does not have valid authentication credentials for the operation. HTTP Mapping: 401 Unauthorized"]
        Unauthenticated,
        #[doc = "The service is currently unavailable. This is most likely a transient condition, which can be corrected by retrying with a backoff. Note that it is not always safe to retry non-idempotent operations. See the guidelines above for deciding between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`. HTTP Mapping: 503 Service Unavailable"]
        Unavailable,
        #[doc = "The operation is not implemented or is not supported/enabled in this service. HTTP Mapping: 501 Not Implemented"]
        Unimplemented,
        #[doc = "Unknown error. For example, this error may be returned when a `Status` value received from another address space belongs to an error space that is not known in this address space. Also errors raised by APIs that do not return enough error information may be converted to this error. HTTP Mapping: 500 Internal Server Error"]
        Unknown,
    }
    impl IamPolicyAnalysisStateCode {
        pub fn as_str(self) -> &'static str {
            match self {
                IamPolicyAnalysisStateCode::Aborted => "ABORTED",
                IamPolicyAnalysisStateCode::AlreadyExists => "ALREADY_EXISTS",
                IamPolicyAnalysisStateCode::Cancelled => "CANCELLED",
                IamPolicyAnalysisStateCode::DataLoss => "DATA_LOSS",
                IamPolicyAnalysisStateCode::DeadlineExceeded => "DEADLINE_EXCEEDED",
                IamPolicyAnalysisStateCode::FailedPrecondition => "FAILED_PRECONDITION",
                IamPolicyAnalysisStateCode::Internal => "INTERNAL",
                IamPolicyAnalysisStateCode::InvalidArgument => "INVALID_ARGUMENT",
                IamPolicyAnalysisStateCode::NotFound => "NOT_FOUND",
                IamPolicyAnalysisStateCode::Ok => "OK",
                IamPolicyAnalysisStateCode::OutOfRange => "OUT_OF_RANGE",
                IamPolicyAnalysisStateCode::PermissionDenied => "PERMISSION_DENIED",
                IamPolicyAnalysisStateCode::ResourceExhausted => "RESOURCE_EXHAUSTED",
                IamPolicyAnalysisStateCode::Unauthenticated => "UNAUTHENTICATED",
                IamPolicyAnalysisStateCode::Unavailable => "UNAVAILABLE",
                IamPolicyAnalysisStateCode::Unimplemented => "UNIMPLEMENTED",
                IamPolicyAnalysisStateCode::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for IamPolicyAnalysisStateCode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for IamPolicyAnalysisStateCode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<IamPolicyAnalysisStateCode, ()> {
            Ok(match s {
                "ABORTED" => IamPolicyAnalysisStateCode::Aborted,
                "ALREADY_EXISTS" => IamPolicyAnalysisStateCode::AlreadyExists,
                "CANCELLED" => IamPolicyAnalysisStateCode::Cancelled,
                "DATA_LOSS" => IamPolicyAnalysisStateCode::DataLoss,
                "DEADLINE_EXCEEDED" => IamPolicyAnalysisStateCode::DeadlineExceeded,
                "FAILED_PRECONDITION" => IamPolicyAnalysisStateCode::FailedPrecondition,
                "INTERNAL" => IamPolicyAnalysisStateCode::Internal,
                "INVALID_ARGUMENT" => IamPolicyAnalysisStateCode::InvalidArgument,
                "NOT_FOUND" => IamPolicyAnalysisStateCode::NotFound,
                "OK" => IamPolicyAnalysisStateCode::Ok,
                "OUT_OF_RANGE" => IamPolicyAnalysisStateCode::OutOfRange,
                "PERMISSION_DENIED" => IamPolicyAnalysisStateCode::PermissionDenied,
                "RESOURCE_EXHAUSTED" => IamPolicyAnalysisStateCode::ResourceExhausted,
                "UNAUTHENTICATED" => IamPolicyAnalysisStateCode::Unauthenticated,
                "UNAVAILABLE" => IamPolicyAnalysisStateCode::Unavailable,
                "UNIMPLEMENTED" => IamPolicyAnalysisStateCode::Unimplemented,
                "UNKNOWN" => IamPolicyAnalysisStateCode::Unknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for IamPolicyAnalysisStateCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for IamPolicyAnalysisStateCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for IamPolicyAnalysisStateCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ABORTED" => IamPolicyAnalysisStateCode::Aborted,
                "ALREADY_EXISTS" => IamPolicyAnalysisStateCode::AlreadyExists,
                "CANCELLED" => IamPolicyAnalysisStateCode::Cancelled,
                "DATA_LOSS" => IamPolicyAnalysisStateCode::DataLoss,
                "DEADLINE_EXCEEDED" => IamPolicyAnalysisStateCode::DeadlineExceeded,
                "FAILED_PRECONDITION" => IamPolicyAnalysisStateCode::FailedPrecondition,
                "INTERNAL" => IamPolicyAnalysisStateCode::Internal,
                "INVALID_ARGUMENT" => IamPolicyAnalysisStateCode::InvalidArgument,
                "NOT_FOUND" => IamPolicyAnalysisStateCode::NotFound,
                "OK" => IamPolicyAnalysisStateCode::Ok,
                "OUT_OF_RANGE" => IamPolicyAnalysisStateCode::OutOfRange,
                "PERMISSION_DENIED" => IamPolicyAnalysisStateCode::PermissionDenied,
                "RESOURCE_EXHAUSTED" => IamPolicyAnalysisStateCode::ResourceExhausted,
                "UNAUTHENTICATED" => IamPolicyAnalysisStateCode::Unauthenticated,
                "UNAVAILABLE" => IamPolicyAnalysisStateCode::Unavailable,
                "UNIMPLEMENTED" => IamPolicyAnalysisStateCode::Unimplemented,
                "UNKNOWN" => IamPolicyAnalysisStateCode::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for IamPolicyAnalysisStateCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IamPolicyAnalysisStateCode {
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
    pub struct IamPolicySearchResult {
        #[doc = "The type of the resource associated with this IAM policy. Example: `compute.googleapis.com/Disk`. To search against the `asset_type`: * specify the `asset_types` field in your search request."]
        #[serde(
            rename = "assetType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub asset_type: ::std::option::Option<String>,
        #[doc = "Explanation about the IAM policy search result. It contains additional information to explain why the search result matches the query."]
        #[serde(
            rename = "explanation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub explanation: ::std::option::Option<crate::schemas::Explanation>,
        #[doc = "The folder(s) that the IAM policy belongs to, in the form of folders/{FOLDER_NUMBER}. This field is available when the IAM policy belongs to one or more folders. To search against `folders`: * use a field query. Example: `folders:(123 OR 456)` * use a free text query. Example: `123` * specify the `scope` field as this folder in your search request."]
        #[serde(
            rename = "folders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub folders: ::std::option::Option<Vec<String>>,
        #[doc = "The organization that the IAM policy belongs to, in the form of organizations/{ORGANIZATION_NUMBER}. This field is available when the IAM policy belongs to an organization. To search against `organization`: * use a field query. Example: `organization:123` * use a free text query. Example: `123` * specify the `scope` field as this organization in your search request."]
        #[serde(
            rename = "organization",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub organization: ::std::option::Option<String>,
        #[doc = "The IAM policy directly set on the given resource. Note that the original IAM policy can contain multiple bindings. This only contains the bindings that match the given query. For queries that don’t contain a constrain on policies (e.g., an empty query), this contains all the bindings. To search against the `policy` bindings: * use a field query: - query by the policy contained members. Example: `policy:amy@gmail.com` - query by the policy contained roles. Example: `policy:roles/compute.admin` - query by the policy contained roles’ included permissions. Example: `policy.role.permissions:compute.instances.create`"]
        #[serde(
            rename = "policy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy: ::std::option::Option<crate::schemas::Policy>,
        #[doc = "The project that the associated Google Cloud resource belongs to, in the form of projects/{PROJECT_NUMBER}. If an IAM policy is set on a resource (like VM instance, Cloud Storage bucket), the project field will indicate the project that contains the resource. If an IAM policy is set on a folder or orgnization, this field will be empty. To search against the `project`: * specify the `scope` field as this project in your search request."]
        #[serde(
            rename = "project",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project: ::std::option::Option<String>,
        #[doc = "The full resource name of the resource associated with this IAM policy. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`. See [Cloud Asset Inventory Resource Name Format](https://cloud.google.com/asset-inventory/docs/resource-name-format) for more information. To search against the `resource`: * use a field query. Example: `resource:organizations/123`"]
        #[serde(
            rename = "resource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for IamPolicySearchResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IamPolicySearchResult {
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
    pub struct IdentitySelector {
        #[doc = "Required. The identity appear in the form of principals in [IAM policy binding](https://cloud.google.com/iam/reference/rest/v1/Binding). The examples of supported forms are: “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com”. Notice that wildcard characters (such as * and ?) are not supported. You must give a specific identity."]
        #[serde(
            rename = "identity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identity: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for IdentitySelector {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentitySelector {
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
    pub struct Inventory {
        #[doc = "Inventory items related to the VM keyed by an opaque unique identifier for each inventory item. The identifier is unique to each distinct and addressable inventory item and will change, when there is a new package version."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::Item>>,
        #[doc = "Output only. The `Inventory` API resource name. Format: `projects/{project_number}/locations/{location}/instances/{instance_id}/inventory`"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Base level operating system information for the VM."]
        #[serde(
            rename = "osInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_info: ::std::option::Option<crate::schemas::OsInfo>,
        #[doc = "Output only. Timestamp of the last reported inventory for the VM."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Inventory {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Inventory {
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
    pub struct Item {
        #[doc = "Software package available to be installed on the VM instance."]
        #[serde(
            rename = "availablePackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub available_package: ::std::option::Option<crate::schemas::SoftwarePackage>,
        #[doc = "When this inventory item was first detected."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Identifier for this item, unique across items for this VM."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Software package present on the VM instance."]
        #[serde(
            rename = "installedPackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub installed_package: ::std::option::Option<crate::schemas::SoftwarePackage>,
        #[doc = "The origin of this inventory item."]
        #[serde(
            rename = "originType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub origin_type: ::std::option::Option<crate::schemas::ItemOriginType>,
        #[doc = "The specific type of inventory, correlating to its specific details."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::ItemType>,
        #[doc = "When this inventory item was last modified."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Item {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Item {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ItemOriginType {
        #[doc = "This inventory item was discovered as the result of the agent reporting inventory via the reporting API."]
        InventoryReport,
        #[doc = "Invalid. An origin type must be specified."]
        OriginTypeUnspecified,
    }
    impl ItemOriginType {
        pub fn as_str(self) -> &'static str {
            match self {
                ItemOriginType::InventoryReport => "INVENTORY_REPORT",
                ItemOriginType::OriginTypeUnspecified => "ORIGIN_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ItemOriginType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ItemOriginType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ItemOriginType, ()> {
            Ok(match s {
                "INVENTORY_REPORT" => ItemOriginType::InventoryReport,
                "ORIGIN_TYPE_UNSPECIFIED" => ItemOriginType::OriginTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ItemOriginType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ItemOriginType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ItemOriginType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INVENTORY_REPORT" => ItemOriginType::InventoryReport,
                "ORIGIN_TYPE_UNSPECIFIED" => ItemOriginType::OriginTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ItemOriginType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemOriginType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ItemType {
        #[doc = "This represents an update that is available for a package."]
        AvailablePackage,
        #[doc = "This represents a package that is installed on the VM."]
        InstalledPackage,
        #[doc = "Invalid. An type must be specified."]
        TypeUnspecified,
    }
    impl ItemType {
        pub fn as_str(self) -> &'static str {
            match self {
                ItemType::AvailablePackage => "AVAILABLE_PACKAGE",
                ItemType::InstalledPackage => "INSTALLED_PACKAGE",
                ItemType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ItemType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ItemType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ItemType, ()> {
            Ok(match s {
                "AVAILABLE_PACKAGE" => ItemType::AvailablePackage,
                "INSTALLED_PACKAGE" => ItemType::InstalledPackage,
                "TYPE_UNSPECIFIED" => ItemType::TypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ItemType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ItemType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ItemType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AVAILABLE_PACKAGE" => ItemType::AvailablePackage,
                "INSTALLED_PACKAGE" => ItemType::InstalledPackage,
                "TYPE_UNSPECIFIED" => ItemType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ItemType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListAssetsResponse {
        #[doc = "Assets."]
        #[serde(
            rename = "assets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub assets: ::std::option::Option<Vec<crate::schemas::Asset>>,
        #[doc = "Token to retrieve the next page of results. It expires 72 hours after the page token for the first page is generated. Set to empty if there are no remaining results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Time the snapshot was taken."]
        #[serde(
            rename = "readTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub read_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListAssetsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListAssetsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for ListAssetsResponse {
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
    pub struct ListFeedsResponse {
        #[doc = "A list of feeds."]
        #[serde(
            rename = "feeds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub feeds: ::std::option::Option<Vec<crate::schemas::Feed>>,
    }
    impl ::google_field_selector::FieldSelector for ListFeedsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListFeedsResponse {
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
    pub struct ListSavedQueriesResponse {
        #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "A list of savedQueries."]
        #[serde(
            rename = "savedQueries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub saved_queries: ::std::option::Option<Vec<crate::schemas::SavedQuery>>,
    }
    impl ::google_field_selector::FieldSelector for ListSavedQueriesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListSavedQueriesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for ListSavedQueriesResponse {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct MoveAnalysis {
        #[doc = "Analysis result of moving the target resource."]
        #[serde(
            rename = "analysis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis: ::std::option::Option<crate::schemas::MoveAnalysisResult>,
        #[doc = "The user friendly display name of the analysis. E.g. IAM, organization policy etc."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Description of error encountered when performing the analysis."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::Status>,
    }
    impl ::google_field_selector::FieldSelector for MoveAnalysis {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MoveAnalysis {
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
    pub struct MoveAnalysisResult {
        #[doc = "Blocking information that would prevent the target resource from moving to the specified destination at runtime."]
        #[serde(
            rename = "blockers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub blockers: ::std::option::Option<Vec<crate::schemas::MoveImpact>>,
        #[doc = "Warning information indicating that moving the target resource to the specified destination might be unsafe. This can include important policy information and configuration changes, but will not block moves at runtime."]
        #[serde(
            rename = "warnings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub warnings: ::std::option::Option<Vec<crate::schemas::MoveImpact>>,
    }
    impl ::google_field_selector::FieldSelector for MoveAnalysisResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MoveAnalysisResult {
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
    pub struct MoveImpact {
        #[doc = "User friendly impact detail in a free form message."]
        #[serde(
            rename = "detail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detail: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MoveImpact {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MoveImpact {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Operation {
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
    impl ::google_field_selector::FieldSelector for Operation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Operation {
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
    pub struct Options {
        #[doc = "Optional. If true, the response will include access analysis from identities to resources via service account impersonation. This is a very expensive operation, because many derived queries will be executed. We highly recommend you use AssetService.AnalyzeIamPolicyLongrunning RPC instead. For example, if the request analyzes for which resources user A has permission P, and there’s an IAM policy states user A has iam.serviceAccounts.getAccessToken permission to a service account SA, and there’s another IAM policy states service account SA has permission P to a Google Cloud folder F, then user A potentially has access to the Google Cloud folder F. And those advanced analysis results will be included in AnalyzeIamPolicyResponse.service_account_impersonation_analysis. Another example, if the request analyzes for who has permission P to a Google Cloud folder F, and there’s an IAM policy states user A has iam.serviceAccounts.actAs permission to a service account SA, and there’s another IAM policy states service account SA has permission P to the Google Cloud folder F, then user A potentially has access to the Google Cloud folder F. And those advanced analysis results will be included in AnalyzeIamPolicyResponse.service_account_impersonation_analysis. Only the following permissions are considered in this analysis: * `iam.serviceAccounts.actAs` * `iam.serviceAccounts.signBlob` * `iam.serviceAccounts.signJwt` * `iam.serviceAccounts.getAccessToken` * `iam.serviceAccounts.getOpenIdToken` * `iam.serviceAccounts.implicitDelegation` Default is false."]
        #[serde(
            rename = "analyzeServiceAccountImpersonation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analyze_service_account_impersonation: ::std::option::Option<bool>,
        #[doc = "Optional. If true, the identities section of the result will expand any Google groups appearing in an IAM policy binding. If IamPolicyAnalysisQuery.identity_selector is specified, the identity in the result will be determined by the selector, and this flag is not allowed to set. If true, the default max expansion per group is 1000 for AssetService.AnalyzeIamPolicy\\]\\[\\]. Default is false."]
        #[serde(
            rename = "expandGroups",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expand_groups: ::std::option::Option<bool>,
        #[doc = "Optional. If true and IamPolicyAnalysisQuery.resource_selector is not specified, the resource section of the result will expand any resource attached to an IAM policy to include resources lower in the resource hierarchy. For example, if the request analyzes for which resources user A has permission P, and the results include an IAM policy with P on a Google Cloud folder, the results will also include resources in that folder with permission P. If true and IamPolicyAnalysisQuery.resource_selector is specified, the resource section of the result will expand the specified resource to include resources lower in the resource hierarchy. Only project or lower resources are supported. Folder and organization resources cannot be used together with this option. For example, if the request analyzes for which users have permission P on a Google Cloud project with this option enabled, the results will include all users who have permission P on that project or any lower resource. If true, the default max expansion per resource is 1000 for AssetService.AnalyzeIamPolicy\\]\\[\\] and 100000 for AssetService.AnalyzeIamPolicyLongrunning\\]\\[\\]. Default is false."]
        #[serde(
            rename = "expandResources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expand_resources: ::std::option::Option<bool>,
        #[doc = "Optional. If true, the access section of result will expand any roles appearing in IAM policy bindings to include their permissions. If IamPolicyAnalysisQuery.access_selector is specified, the access section of the result will be determined by the selector, and this flag is not allowed to set. Default is false."]
        #[serde(
            rename = "expandRoles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expand_roles: ::std::option::Option<bool>,
        #[doc = "Optional. If true, the result will output the relevant membership relationships between groups and other groups, and between groups and principals. Default is false."]
        #[serde(
            rename = "outputGroupEdges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_group_edges: ::std::option::Option<bool>,
        #[doc = "Optional. If true, the result will output the relevant parent/child relationships between resources. Default is false."]
        #[serde(
            rename = "outputResourceEdges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_resource_edges: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for Options {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Options {
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
    pub struct OrgPolicyResult {
        #[doc = "The consolidated organization policy for the analyzed resource. The consolidated organization policy is computed by merging and evaluating AnalyzeOrgPoliciesResponse.policy_bundle. The evaluation will respect the organization policy [hierarchy rules](https://cloud.google.com/resource-manager/docs/organization-policy/understanding-hierarchy)."]
        #[serde(
            rename = "consolidatedPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub consolidated_policy: ::std::option::Option<crate::schemas::AnalyzerOrgPolicy>,
        #[doc = "The ordered list of all organization policies from the AnalyzeOrgPoliciesResponse.OrgPolicyResult.consolidated_policy.attached_resource. to the scope specified in the request. If the constraint is defined with default policy, it will also appear in the list."]
        #[serde(
            rename = "policyBundle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy_bundle: ::std::option::Option<Vec<crate::schemas::AnalyzerOrgPolicy>>,
    }
    impl ::google_field_selector::FieldSelector for OrgPolicyResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OrgPolicyResult {
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
    pub struct OsInfo {
        #[doc = "The system architecture of the operating system."]
        #[serde(
            rename = "architecture",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub architecture: ::std::option::Option<String>,
        #[doc = "The VM hostname."]
        #[serde(
            rename = "hostname",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hostname: ::std::option::Option<String>,
        #[doc = "The kernel release of the operating system."]
        #[serde(
            rename = "kernelRelease",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kernel_release: ::std::option::Option<String>,
        #[doc = "The kernel version of the operating system."]
        #[serde(
            rename = "kernelVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kernel_version: ::std::option::Option<String>,
        #[doc = "The operating system long name. For example ‘Debian GNU/Linux 9’ or ‘Microsoft Window Server 2019 Datacenter’."]
        #[serde(
            rename = "longName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub long_name: ::std::option::Option<String>,
        #[doc = "The current version of the OS Config agent running on the VM."]
        #[serde(
            rename = "osconfigAgentVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub osconfig_agent_version: ::std::option::Option<String>,
        #[doc = "The operating system short name. For example, ‘windows’ or ‘debian’."]
        #[serde(
            rename = "shortName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub short_name: ::std::option::Option<String>,
        #[doc = "The version of the operating system."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OsInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OsInfo {
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
    pub struct OutputConfig {
        #[doc = "Destination on BigQuery. The output table stores the fields in asset Protobuf as columns in BigQuery."]
        #[serde(
            rename = "bigqueryDestination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bigquery_destination: ::std::option::Option<crate::schemas::BigQueryDestination>,
        #[doc = "Destination on Cloud Storage."]
        #[serde(
            rename = "gcsDestination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_destination: ::std::option::Option<crate::schemas::GcsDestination>,
    }
    impl ::google_field_selector::FieldSelector for OutputConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OutputConfig {
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
    pub struct PartitionSpec {
        #[doc = "The partition key for BigQuery partitioned table."]
        #[serde(
            rename = "partitionKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub partition_key: ::std::option::Option<crate::schemas::PartitionSpecPartitionKey>,
    }
    impl ::google_field_selector::FieldSelector for PartitionSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PartitionSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PartitionSpecPartitionKey {
        #[doc = "Unspecified partition key. If used, it means using non-partitioned table."]
        PartitionKeyUnspecified,
        #[doc = "The time when the snapshot is taken. If specified as partition key, the result table(s) is partitoned by the additional timestamp column, readTime. If \\[read_time\\] in ExportAssetsRequest is specified, the readTime column’s value will be the same as it. Otherwise, its value will be the current time that is used to take the snapshot."]
        ReadTime,
        #[doc = "The time when the request is received and started to be processed. If specified as partition key, the result table(s) is partitoned by the requestTime column, an additional timestamp column representing when the request was received."]
        RequestTime,
    }
    impl PartitionSpecPartitionKey {
        pub fn as_str(self) -> &'static str {
            match self {
                PartitionSpecPartitionKey::PartitionKeyUnspecified => "PARTITION_KEY_UNSPECIFIED",
                PartitionSpecPartitionKey::ReadTime => "READ_TIME",
                PartitionSpecPartitionKey::RequestTime => "REQUEST_TIME",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PartitionSpecPartitionKey {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PartitionSpecPartitionKey {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PartitionSpecPartitionKey, ()> {
            Ok(match s {
                "PARTITION_KEY_UNSPECIFIED" => PartitionSpecPartitionKey::PartitionKeyUnspecified,
                "READ_TIME" => PartitionSpecPartitionKey::ReadTime,
                "REQUEST_TIME" => PartitionSpecPartitionKey::RequestTime,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PartitionSpecPartitionKey {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PartitionSpecPartitionKey {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PartitionSpecPartitionKey {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PARTITION_KEY_UNSPECIFIED" => PartitionSpecPartitionKey::PartitionKeyUnspecified,
                "READ_TIME" => PartitionSpecPartitionKey::ReadTime,
                "REQUEST_TIME" => PartitionSpecPartitionKey::RequestTime,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PartitionSpecPartitionKey {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PartitionSpecPartitionKey {
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
    pub struct Permissions {
        #[doc = "A list of permissions. A sample permission string: `compute.disk.get`."]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for Permissions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Permissions {
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
    pub struct Policy {
        #[doc = "Specifies cloud audit logging configuration for this policy."]
        #[serde(
            rename = "auditConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audit_configs: ::std::option::Option<Vec<crate::schemas::AuditConfig>>,
        #[doc = "Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`."]
        #[serde(
            rename = "bindings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bindings: ::std::option::Option<Vec<crate::schemas::Binding>>,
        #[doc = "`etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Policy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Policy {
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
    pub struct PolicyInfo {
        #[doc = "The full resource name the policy is directly attached to."]
        #[serde(
            rename = "attachedResource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attached_resource: ::std::option::Option<String>,
        #[doc = "The IAM policy that’s directly attached to the attached_resource."]
        #[serde(
            rename = "policy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy: ::std::option::Option<crate::schemas::Policy>,
    }
    impl ::google_field_selector::FieldSelector for PolicyInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyInfo {
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
    pub struct PubsubDestination {
        #[doc = "The name of the Pub/Sub topic to publish to. Example: `projects/PROJECT_ID/topics/TOPIC_ID`."]
        #[serde(
            rename = "topic",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub topic: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PubsubDestination {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PubsubDestination {
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
    pub struct QueryAssetsOutputConfig {
        #[doc = "BigQuery destination where the query results will be saved."]
        #[serde(
            rename = "bigqueryDestination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bigquery_destination: ::std::option::Option<
            crate::schemas::GoogleCloudAssetV1QueryAssetsOutputConfigBigQueryDestination,
        >,
    }
    impl ::google_field_selector::FieldSelector for QueryAssetsOutputConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryAssetsOutputConfig {
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
    pub struct QueryAssetsRequest {
        #[doc = "Optional. Reference to the query job, which is from the `QueryAssetsResponse` of previous `QueryAssets` call."]
        #[serde(
            rename = "jobReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_reference: ::std::option::Option<String>,
        #[doc = "Optional. Destination where the query results will be saved. When this field is specified, the query results won’t be saved in the \\[QueryAssetsResponse.query_result\\]. Instead \\[QueryAssetsResponse.output_config\\] will be set. Meanwhile, \\[QueryAssetsResponse.job_reference\\] will be set and can be used to check the status of the query job when passed to a following \\[QueryAssets\\] API call."]
        #[serde(
            rename = "outputConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_config: ::std::option::Option<crate::schemas::QueryAssetsOutputConfig>,
        #[doc = "Optional. The maximum number of rows to return in the results. Responses are limited to 10 MB and 1000 rows. By default, the maximum row count is 1000. When the byte or row count limit is reached, the rest of the query results will be paginated. The field will be ignored when \\[output_config\\] is specified."]
        #[serde(
            rename = "pageSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_size: ::std::option::Option<i32>,
        #[doc = "Optional. A page token received from previous `QueryAssets`. The field will be ignored when \\[output_config\\] is specified."]
        #[serde(
            rename = "pageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_token: ::std::option::Option<String>,
        #[doc = "Optional. Queries cloud assets as they appeared at the specified point in time."]
        #[serde(
            rename = "readTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub read_time: ::std::option::Option<String>,
        #[doc = "Optional. \\[start_time\\] is required. \\[start_time\\] must be less than \\[end_time\\] Defaults \\[end_time\\] to now if \\[start_time\\] is set and \\[end_time\\] isn’t. Maximum permitted time range is 7 days."]
        #[serde(
            rename = "readTimeWindow",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub read_time_window: ::std::option::Option<crate::schemas::TimeWindow>,
        #[doc = "Optional. A SQL statement that’s compatible with [BigQuery Standard SQL](http://cloud/bigquery/docs/reference/standard-sql/enabling-standard-sql)."]
        #[serde(
            rename = "statement",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub statement: ::std::option::Option<String>,
        #[doc = "Optional. Specifies the maximum amount of time that the client is willing to wait for the query to complete. By default, this limit is 5 min for the first query, and 1 minute for the following queries. If the query is complete, the `done` field in the `QueryAssetsResponse` is true, otherwise false. Like BigQuery [jobs.query API](https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/query#queryrequest) The call is not guaranteed to wait for the specified timeout; it typically returns after around 200 seconds (200,000 milliseconds), even if the query is not complete. The field will be ignored when \\[output_config\\] is specified."]
        #[serde(
            rename = "timeout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timeout: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for QueryAssetsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryAssetsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct QueryAssetsResponse {
        #[doc = "The query response, which can be either an `error` or a valid `response`. If `done` == `false` and the query result is being saved in a output, the output_config field will be set. If `done` == `true`, exactly one of `error`, `query_result` or `output_config` will be set."]
        #[serde(
            rename = "done",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub done: ::std::option::Option<bool>,
        #[doc = "Error status."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::Status>,
        #[doc = "Reference to a query job."]
        #[serde(
            rename = "jobReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_reference: ::std::option::Option<String>,
        #[doc = "Output configuration which indicates instead of being returned in API response on the fly, the query result will be saved in a specific output."]
        #[serde(
            rename = "outputConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_config: ::std::option::Option<crate::schemas::QueryAssetsOutputConfig>,
        #[doc = "Result of the query."]
        #[serde(
            rename = "queryResult",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query_result: ::std::option::Option<crate::schemas::QueryResult>,
    }
    impl ::google_field_selector::FieldSelector for QueryAssetsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryAssetsResponse {
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
    pub struct QueryContent {
        #[doc = "An IAM Policy Analysis query, which could be used in the AssetService.AnalyzeIamPolicy RPC or the AssetService.AnalyzeIamPolicyLongrunning RPC."]
        #[serde(
            rename = "iamPolicyAnalysisQuery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub iam_policy_analysis_query:
            ::std::option::Option<crate::schemas::IamPolicyAnalysisQuery>,
    }
    impl ::google_field_selector::FieldSelector for QueryContent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryContent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct QueryResult {
        #[doc = "Token to retrieve the next page of the results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Each row hold a query result in the format of `Struct`."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "Describes the format of the \\[rows\\]."]
        #[serde(
            rename = "schema",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schema: ::std::option::Option<crate::schemas::TableSchema>,
        #[doc = "Total rows of the whole query results."]
        #[serde(
            rename = "totalRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_rows: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for QueryResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryResult {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for QueryResult {
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
    pub struct RelatedAsset {
        #[doc = "The ancestors of an asset in Google Cloud [resource hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy), represented as a list of relative resource names. An ancestry path starts with the closest ancestor in the hierarchy and ends at root. Example: `[\"projects/123456789\", \"folders/5432\", \"organizations/1234\"]`"]
        #[serde(
            rename = "ancestors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ancestors: ::std::option::Option<Vec<String>>,
        #[doc = "The full name of the asset. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1` See [Resource names](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more information."]
        #[serde(
            rename = "asset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub asset: ::std::option::Option<String>,
        #[doc = "The type of the asset. Example: `compute.googleapis.com/Disk` See [Supported asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types) for more information."]
        #[serde(
            rename = "assetType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub asset_type: ::std::option::Option<String>,
        #[doc = "The unique identifier of the relationship type. Example: `INSTANCE_TO_INSTANCEGROUP`"]
        #[serde(
            rename = "relationshipType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relationship_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RelatedAsset {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RelatedAsset {
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
    pub struct RelatedAssets {
        #[doc = "The peer resources of the relationship."]
        #[serde(
            rename = "assets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub assets: ::std::option::Option<Vec<crate::schemas::RelatedAsset>>,
        #[doc = "The detailed relationship attributes."]
        #[serde(
            rename = "relationshipAttributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relationship_attributes: ::std::option::Option<crate::schemas::RelationshipAttributes>,
    }
    impl ::google_field_selector::FieldSelector for RelatedAssets {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RelatedAssets {
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
    pub struct RelatedResource {
        #[doc = "The type of the asset. Example: `compute.googleapis.com/Instance`"]
        #[serde(
            rename = "assetType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub asset_type: ::std::option::Option<String>,
        #[doc = "The full resource name of the related resource. Example: `//compute.googleapis.com/projects/my_proj_123/zones/instance/instance123`"]
        #[serde(
            rename = "fullResourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub full_resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RelatedResource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RelatedResource {
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
    pub struct RelatedResources {
        #[doc = "The detailed related resources of the primary resource."]
        #[serde(
            rename = "relatedResources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub related_resources: ::std::option::Option<Vec<crate::schemas::RelatedResource>>,
    }
    impl ::google_field_selector::FieldSelector for RelatedResources {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RelatedResources {
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
    pub struct RelationshipAttributes {
        #[doc = "The detail of the relationship, e.g. `contains`, `attaches`"]
        #[serde(
            rename = "action",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action: ::std::option::Option<String>,
        #[doc = "The unique identifier of the relationship type. Example: `INSTANCE_TO_INSTANCEGROUP`"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The source asset type. Example: `compute.googleapis.com/Instance`"]
        #[serde(
            rename = "sourceResourceType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_resource_type: ::std::option::Option<String>,
        #[doc = "The target asset type. Example: `compute.googleapis.com/Disk`"]
        #[serde(
            rename = "targetResourceType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_resource_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RelationshipAttributes {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RelationshipAttributes {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Resource {
        #[doc = "The content of the resource, in which some sensitive fields are removed and may not be present."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The URL of the discovery document containing the resource’s JSON schema. Example: `https://www.googleapis.com/discovery/v1/apis/compute/v1/rest` This value is unspecified for resources that do not have an API based on a discovery document, such as Cloud Bigtable."]
        #[serde(
            rename = "discoveryDocumentUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub discovery_document_uri: ::std::option::Option<String>,
        #[doc = "The JSON schema name listed in the discovery document. Example: `Project` This value is unspecified for resources that do not have an API based on a discovery document, such as Cloud Bigtable."]
        #[serde(
            rename = "discoveryName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub discovery_name: ::std::option::Option<String>,
        #[doc = "The location of the resource in Google Cloud, such as its zone and region. For more information, see https://cloud.google.com/about/locations/."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "The full name of the immediate parent of this resource. See [Resource Names](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more information. For Google Cloud assets, this value is the parent resource defined in the [IAM policy hierarchy](https://cloud.google.com/iam/docs/overview#policy_hierarchy). Example: `//cloudresourcemanager.googleapis.com/projects/my_project_123` For third-party assets, this field may be set differently."]
        #[serde(
            rename = "parent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent: ::std::option::Option<String>,
        #[doc = "The REST URL for accessing the resource. An HTTP `GET` request using this URL returns the resource itself. Example: `https://cloudresourcemanager.googleapis.com/v1/projects/my-project-123` This value is unspecified for resources without a REST API."]
        #[serde(
            rename = "resourceUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_url: ::std::option::Option<String>,
        #[doc = "The API version. Example: `v1`"]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Resource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Resource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ResourceSearchResult {
        #[doc = "The additional searchable attributes of this resource. The attributes may vary from one resource type to another. Examples: `projectId` for Project, `dnsName` for DNS ManagedZone. This field contains a subset of the resource metadata fields that are returned by the List or Get APIs provided by the corresponding Google Cloud service (e.g., Compute Engine). see [API references and supported searchable attributes](https://cloud.google.com/asset-inventory/docs/supported-asset-types#searchable_asset_types) to see which fields are included. You can search values of these fields through free text search. However, you should not consume the field programically as the field names and values may change as the Google Cloud service updates to a new incompatible API version. To search against the `additional_attributes`: * Use a free text query to match the attributes values. Example: to search `additional_attributes = { dnsName: \"foobar\" }`, you can issue a query `foobar`."]
        #[serde(
            rename = "additionalAttributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub additional_attributes:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The type of this resource. Example: `compute.googleapis.com/Disk`. To search against the `asset_type`: * Specify the `asset_type` field in your search request."]
        #[serde(
            rename = "assetType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub asset_type: ::std::option::Option<String>,
        #[doc = "Attached resources of this resource. For example, an OSConfig Inventory is an attached resource of a Compute Instance. This field is repeated because a resource could have multiple attached resources. This `attached_resources` field is not searchable. Some attributes of the attached resources are exposed in `additional_attributes` field, so as to allow users to search on them."]
        #[serde(
            rename = "attachedResources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attached_resources: ::std::option::Option<Vec<crate::schemas::AttachedResource>>,
        #[doc = "The create timestamp of this resource, at which the resource was created. The granularity is in seconds. Timestamp.nanos will always be 0. This field is available only when the resource’s Protobuf contains it. To search against `create_time`: * Use a field query. - value in seconds since unix epoch. Example: `createTime > 1609459200` - value in date string. Example: `createTime > 2021-01-01` - value in date-time string (must be quoted). Example: `createTime > \"2021-01-01T00:00:00\"`"]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "One or more paragraphs of text description of this resource. Maximum length could be up to 1M bytes. This field is available only when the resource’s Protobuf contains it. To search against the `description`: * Use a field query. Example: `description:\"important instance\"` * Use a free text query. Example: `\"important instance\"`"]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The display name of this resource. This field is available only when the resource’s Protobuf contains it. To search against the `display_name`: * Use a field query. Example: `displayName:\"My Instance\"` * Use a free text query. Example: `\"My Instance\"`"]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The folder(s) that this resource belongs to, in the form of folders/{FOLDER_NUMBER}. This field is available when the resource belongs to one or more folders. To search against `folders`: * Use a field query. Example: `folders:(123 OR 456)` * Use a free text query. Example: `123` * Specify the `scope` field as this folder in your search request."]
        #[serde(
            rename = "folders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub folders: ::std::option::Option<Vec<String>>,
        #[doc = "The Cloud KMS [CryptoKey](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys) name or [CryptoKeyVersion](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys.cryptoKeyVersions) name. This field only presents for the purpose of backward compatibility. Please use the `kms_keys` field to retrieve Cloud KMS key information. This field is available only when the resource’s Protobuf contains it and will only be populated for [these resource types](https://cloud.google.com/asset-inventory/docs/legacy-field-names#resource_types_with_the_to_be_deprecated_kmskey_field) for backward compatible purposes. To search against the `kms_key`: * Use a field query. Example: `kmsKey:key` * Use a free text query. Example: `key`"]
        #[serde(
            rename = "kmsKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kms_key: ::std::option::Option<String>,
        #[doc = "The Cloud KMS [CryptoKey](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys) names or [CryptoKeyVersion](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys.cryptoKeyVersions) names. This field is available only when the resource’s Protobuf contains it. To search against the `kms_keys`: * Use a field query. Example: `kmsKeys:key` * Use a free text query. Example: `key`"]
        #[serde(
            rename = "kmsKeys",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kms_keys: ::std::option::Option<Vec<String>>,
        #[doc = "Labels associated with this resource. See [Labelling and grouping Google Cloud resources](https://cloud.google.com/blog/products/gcp/labelling-and-grouping-your-google-cloud-platform-resources) for more information. This field is available only when the resource’s Protobuf contains it. To search against the `labels`: * Use a field query: - query on any label’s key or value. Example: `labels:prod` - query by a given label. Example: `labels.env:prod` - query by a given label’s existence. Example: `labels.env:*` * Use a free text query. Example: `prod`"]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Location can be `global`, regional like `us-east1`, or zonal like `us-west1-b`. This field is available only when the resource’s Protobuf contains it. To search against the `location`: * Use a field query. Example: `location:us-west*` * Use a free text query. Example: `us-west*`"]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "The full resource name of this resource. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`. See [Cloud Asset Inventory Resource Name Format](https://cloud.google.com/asset-inventory/docs/resource-name-format) for more information. To search against the `name`: * Use a field query. Example: `name:instance1` * Use a free text query. Example: `instance1`"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Network tags associated with this resource. Like labels, network tags are a type of annotations used to group Google Cloud resources. See [Labelling Google Cloud resources](https://cloud.google.com/blog/products/gcp/labelling-and-grouping-your-google-cloud-platform-resources) for more information. This field is available only when the resource’s Protobuf contains it. To search against the `network_tags`: * Use a field query. Example: `networkTags:internal` * Use a free text query. Example: `internal`"]
        #[serde(
            rename = "networkTags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub network_tags: ::std::option::Option<Vec<String>>,
        #[doc = "The organization that this resource belongs to, in the form of organizations/{ORGANIZATION_NUMBER}. This field is available when the resource belongs to an organization. To search against `organization`: * Use a field query. Example: `organization:123` * Use a free text query. Example: `123` * Specify the `scope` field as this organization in your search request."]
        #[serde(
            rename = "organization",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub organization: ::std::option::Option<String>,
        #[doc = "The type of this resource’s immediate parent, if there is one. To search against the `parent_asset_type`: * Use a field query. Example: `parentAssetType:\"cloudresourcemanager.googleapis.com/Project\"` * Use a free text query. Example: `cloudresourcemanager.googleapis.com/Project`"]
        #[serde(
            rename = "parentAssetType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent_asset_type: ::std::option::Option<String>,
        #[doc = "The full resource name of this resource’s parent, if it has one. To search against the `parent_full_resource_name`: * Use a field query. Example: `parentFullResourceName:\"project-name\"` * Use a free text query. Example: `project-name`"]
        #[serde(
            rename = "parentFullResourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent_full_resource_name: ::std::option::Option<String>,
        #[doc = "The project that this resource belongs to, in the form of projects/{PROJECT_NUMBER}. This field is available when the resource belongs to a project. To search against `project`: * Use a field query. Example: `project:12345` * Use a free text query. Example: `12345` * Specify the `scope` field as this project in your search request."]
        #[serde(
            rename = "project",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project: ::std::option::Option<String>,
        #[doc = "A map of related resources of this resource, keyed by the relationship type. A relationship type is in the format of {SourceType}*{ACTION}*{DestType}. Example: `DISK_TO_INSTANCE`, `DISK_TO_NETWORK`, `INSTANCE_TO_INSTANCEGROUP`. See [supported relationship types](https://cloud.google.com/asset-inventory/docs/supported-asset-types#supported_relationship_types)."]
        #[serde(
            rename = "relationships",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relationships: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::RelatedResources>,
        >,
        #[doc = "The state of this resource. Different resources types have different state definitions that are mapped from various fields of different resource types. This field is available only when the resource’s Protobuf contains it. Example: If the resource is an instance provided by Compute Engine, its state will include PROVISIONING, STAGING, RUNNING, STOPPING, SUSPENDING, SUSPENDED, REPAIRING, and TERMINATED. See `status` definition in [API Reference](https://cloud.google.com/compute/docs/reference/rest/v1/instances). If the resource is a project provided by Resource Manager, its state will include LIFECYCLE_STATE_UNSPECIFIED, ACTIVE, DELETE_REQUESTED and DELETE_IN_PROGRESS. See `lifecycleState` definition in [API Reference](https://cloud.google.com/resource-manager/reference/rest/v1/projects). To search against the `state`: * Use a field query. Example: `state:RUNNING` * Use a free text query. Example: `RUNNING`"]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<String>,
        #[doc = "TagKey namespaced names, in the format of {ORG_ID}/{TAG_KEY_SHORT_NAME}. To search against the `tagKeys`: * Use a field query. Example: - `tagKeys:\"123456789/env*\"` - `tagKeys=\"123456789/env\"` - `tagKeys:\"env\"` * Use a free text query. Example: - `env`"]
        #[serde(
            rename = "tagKeys",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tag_keys: ::std::option::Option<Vec<String>>,
        #[doc = "TagValue IDs, in the format of tagValues/{TAG_VALUE_ID}. To search against the `tagValueIds`: * Use a field query. Example: - `tagValueIds:\"456\"` - `tagValueIds=\"tagValues/456\"` * Use a free text query. Example: - `456`"]
        #[serde(
            rename = "tagValueIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tag_value_ids: ::std::option::Option<Vec<String>>,
        #[doc = "TagValue namespaced names, in the format of {ORG_ID}/{TAG_KEY_SHORT_NAME}/{TAG_VALUE_SHORT_NAME}. To search against the `tagValues`: * Use a field query. Example: - `tagValues:\"env\"` - `tagValues:\"env/prod\"` - `tagValues:\"123456789/env/prod*\"` - `tagValues=\"123456789/env/prod\"` * Use a free text query. Example: - `prod`"]
        #[serde(
            rename = "tagValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tag_values: ::std::option::Option<Vec<String>>,
        #[doc = "The last update timestamp of this resource, at which the resource was last modified or deleted. The granularity is in seconds. Timestamp.nanos will always be 0. This field is available only when the resource’s Protobuf contains it. To search against `update_time`: * Use a field query. - value in seconds since unix epoch. Example: `updateTime < 1609459200` - value in date string. Example: `updateTime < 2021-01-01` - value in date-time string (must be quoted). Example: `updateTime < \"2021-01-01T00:00:00\"`"]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
        #[doc = "Versioned resource representations of this resource. This is repeated because there could be multiple versions of resource representations during version migration. This `versioned_resources` field is not searchable. Some attributes of the resource representations are exposed in `additional_attributes` field, so as to allow users to search on them."]
        #[serde(
            rename = "versionedResources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub versioned_resources: ::std::option::Option<Vec<crate::schemas::VersionedResource>>,
    }
    impl ::google_field_selector::FieldSelector for ResourceSearchResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResourceSearchResult {
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
    pub struct ResourceSelector {
        #[doc = "Required. The \\[full resource name\\] (https://cloud.google.com/asset-inventory/docs/resource-name-format) of a resource of [supported resource types](https://cloud.google.com/asset-inventory/docs/supported-asset-types#analyzable_asset_types)."]
        #[serde(
            rename = "fullResourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub full_resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ResourceSelector {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResourceSelector {
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
    pub struct SavedQuery {
        #[doc = "The query content."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<crate::schemas::QueryContent>,
        #[doc = "Output only. The create time of this saved query."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Output only. The account’s email address who has created this saved query."]
        #[serde(
            rename = "creator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creator: ::std::option::Option<String>,
        #[doc = "The description of this saved query. This value should be fewer than 255 characters."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Labels applied on the resource. This value should not contain more than 10 entries. The key and value of each entry must be non-empty and fewer than 64 characters."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Output only. The last update time of this saved query."]
        #[serde(
            rename = "lastUpdateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_update_time: ::std::option::Option<String>,
        #[doc = "Output only. The account’s email address who has updated this saved query most recently."]
        #[serde(
            rename = "lastUpdater",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_updater: ::std::option::Option<String>,
        #[doc = "The resource name of the saved query. The format must be: * projects/project_number/savedQueries/saved_query_id * folders/folder_number/savedQueries/saved_query_id * organizations/organization_number/savedQueries/saved_query_id"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SavedQuery {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SavedQuery {
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
    pub struct SearchAllIamPoliciesResponse {
        #[doc = "Set if there are more results than those appearing in this response; to get the next set of results, call this method again, using this value as the `page_token`."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "A list of IAM policies that match the search query. Related information such as the associated resource is returned along with the policy."]
        #[serde(
            rename = "results",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub results: ::std::option::Option<Vec<crate::schemas::IamPolicySearchResult>>,
    }
    impl ::google_field_selector::FieldSelector for SearchAllIamPoliciesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchAllIamPoliciesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for SearchAllIamPoliciesResponse {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SearchAllResourcesResponse {
        #[doc = "If there are more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "A list of Resources that match the search query. It contains the resource standard metadata information."]
        #[serde(
            rename = "results",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub results: ::std::option::Option<Vec<crate::schemas::ResourceSearchResult>>,
    }
    impl ::google_field_selector::FieldSelector for SearchAllResourcesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchAllResourcesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for SearchAllResourcesResponse {
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
    pub struct SoftwarePackage {
        #[doc = "Details of an APT package. For details about the apt package manager, see https://wiki.debian.org/Apt."]
        #[serde(
            rename = "aptPackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub apt_package: ::std::option::Option<crate::schemas::VersionedPackage>,
        #[doc = "Details of a COS package."]
        #[serde(
            rename = "cosPackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cos_package: ::std::option::Option<crate::schemas::VersionedPackage>,
        #[doc = "Details of a Googet package. For details about the googet package manager, see https://github.com/google/googet."]
        #[serde(
            rename = "googetPackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub googet_package: ::std::option::Option<crate::schemas::VersionedPackage>,
        #[doc = "Details of a Windows Quick Fix engineering package. See https://docs.microsoft.com/en-us/windows/win32/cimwin32prov/win32-quickfixengineering for info in Windows Quick Fix Engineering."]
        #[serde(
            rename = "qfePackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub qfe_package: ::std::option::Option<crate::schemas::WindowsQuickFixEngineeringPackage>,
        #[doc = "Details of Windows Application."]
        #[serde(
            rename = "windowsApplication",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub windows_application: ::std::option::Option<crate::schemas::WindowsApplication>,
        #[doc = "Details of a Windows Update package. See https://docs.microsoft.com/en-us/windows/win32/api/\\_wua/ for information about Windows Update."]
        #[serde(
            rename = "wuaPackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub wua_package: ::std::option::Option<crate::schemas::WindowsUpdatePackage>,
        #[doc = "Yum package info. For details about the yum package manager, see https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/6/html/deployment_guide/ch-yum."]
        #[serde(
            rename = "yumPackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub yum_package: ::std::option::Option<crate::schemas::VersionedPackage>,
        #[doc = "Details of a Zypper package. For details about the Zypper package manager, see https://en.opensuse.org/SDB:Zypper_manual."]
        #[serde(
            rename = "zypperPackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub zypper_package: ::std::option::Option<crate::schemas::VersionedPackage>,
        #[doc = "Details of a Zypper patch. For details about the Zypper package manager, see https://en.opensuse.org/SDB:Zypper_manual."]
        #[serde(
            rename = "zypperPatch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub zypper_patch: ::std::option::Option<crate::schemas::ZypperPatch>,
    }
    impl ::google_field_selector::FieldSelector for SoftwarePackage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SoftwarePackage {
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
    pub struct TableFieldSchema {
        #[doc = "The field name. The name must contain only letters (a-z, A-Z), numbers (0-9), or underscores (\\_), and must start with a letter or underscore. The maximum length is 128 characters."]
        #[serde(
            rename = "field",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field: ::std::option::Option<String>,
        #[doc = "Describes the nested schema fields if the type property is set to RECORD."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<Vec<crate::schemas::TableFieldSchema>>,
        #[doc = "The field mode. Possible values include NULLABLE, REQUIRED and REPEATED. The default value is NULLABLE."]
        #[serde(
            rename = "mode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mode: ::std::option::Option<String>,
        #[doc = "The field data type. Possible values include * STRING * BYTES * INTEGER * FLOAT * BOOLEAN * TIMESTAMP * DATE * TIME * DATETIME * GEOGRAPHY, * NUMERIC, * BIGNUMERIC, * RECORD (where RECORD indicates that the field contains a nested schema)."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TableFieldSchema {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableFieldSchema {
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
    pub struct TableSchema {
        #[doc = "Describes the fields in a table."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<Vec<crate::schemas::TableFieldSchema>>,
    }
    impl ::google_field_selector::FieldSelector for TableSchema {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableSchema {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct TemporalAsset {
        #[doc = "An asset in Google Cloud."]
        #[serde(
            rename = "asset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub asset: ::std::option::Option<crate::schemas::Asset>,
        #[doc = "Whether the asset has been deleted or not."]
        #[serde(
            rename = "deleted",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deleted: ::std::option::Option<bool>,
        #[doc = "Prior copy of the asset. Populated if prior_asset_state is PRESENT. Currently this is only set for responses in Real-Time Feed."]
        #[serde(
            rename = "priorAsset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub prior_asset: ::std::option::Option<crate::schemas::Asset>,
        #[doc = "State of prior_asset."]
        #[serde(
            rename = "priorAssetState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub prior_asset_state: ::std::option::Option<crate::schemas::TemporalAssetPriorAssetState>,
        #[doc = "The time window when the asset data and state was observed."]
        #[serde(
            rename = "window",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub window: ::std::option::Option<crate::schemas::TimeWindow>,
    }
    impl ::google_field_selector::FieldSelector for TemporalAsset {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TemporalAsset {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TemporalAssetPriorAssetState {
        #[doc = "prior_asset is a deletion."]
        Deleted,
        #[doc = "Current asset is the first known state."]
        DoesNotExist,
        #[doc = "Failed to set prior_asset."]
        Invalid,
        #[doc = "prior_asset is populated correctly."]
        Present,
        #[doc = "prior_asset is not applicable for the current asset."]
        PriorAssetStateUnspecified,
    }
    impl TemporalAssetPriorAssetState {
        pub fn as_str(self) -> &'static str {
            match self {
                TemporalAssetPriorAssetState::Deleted => "DELETED",
                TemporalAssetPriorAssetState::DoesNotExist => "DOES_NOT_EXIST",
                TemporalAssetPriorAssetState::Invalid => "INVALID",
                TemporalAssetPriorAssetState::Present => "PRESENT",
                TemporalAssetPriorAssetState::PriorAssetStateUnspecified => {
                    "PRIOR_ASSET_STATE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for TemporalAssetPriorAssetState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TemporalAssetPriorAssetState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TemporalAssetPriorAssetState, ()> {
            Ok(match s {
                "DELETED" => TemporalAssetPriorAssetState::Deleted,
                "DOES_NOT_EXIST" => TemporalAssetPriorAssetState::DoesNotExist,
                "INVALID" => TemporalAssetPriorAssetState::Invalid,
                "PRESENT" => TemporalAssetPriorAssetState::Present,
                "PRIOR_ASSET_STATE_UNSPECIFIED" => {
                    TemporalAssetPriorAssetState::PriorAssetStateUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TemporalAssetPriorAssetState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TemporalAssetPriorAssetState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TemporalAssetPriorAssetState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DELETED" => TemporalAssetPriorAssetState::Deleted,
                "DOES_NOT_EXIST" => TemporalAssetPriorAssetState::DoesNotExist,
                "INVALID" => TemporalAssetPriorAssetState::Invalid,
                "PRESENT" => TemporalAssetPriorAssetState::Present,
                "PRIOR_ASSET_STATE_UNSPECIFIED" => {
                    TemporalAssetPriorAssetState::PriorAssetStateUnspecified
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
    impl ::google_field_selector::FieldSelector for TemporalAssetPriorAssetState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TemporalAssetPriorAssetState {
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
    pub struct TimeWindow {
        #[doc = "End time of the time window (inclusive). If not specified, the current timestamp is used instead."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Start time of the time window (exclusive)."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TimeWindow {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TimeWindow {
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
    pub struct UpdateFeedRequest {
        #[doc = "Required. The new values of feed details. It must match an existing feed and the field `name` must be in the format of: projects/project_number/feeds/feed_id or folders/folder_number/feeds/feed_id or organizations/organization_number/feeds/feed_id."]
        #[serde(
            rename = "feed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub feed: ::std::option::Option<crate::schemas::Feed>,
        #[doc = "Required. Only updates the `feed` fields indicated by this mask. The field mask must not be empty, and it must not contain fields that are immutable or only set by the server."]
        #[serde(
            rename = "updateMask",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_mask: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UpdateFeedRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateFeedRequest {
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
    pub struct VersionedPackage {
        #[doc = "The system architecture this package is intended for."]
        #[serde(
            rename = "architecture",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub architecture: ::std::option::Option<String>,
        #[doc = "The name of the package."]
        #[serde(
            rename = "packageName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_name: ::std::option::Option<String>,
        #[doc = "The version of the package."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VersionedPackage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VersionedPackage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct VersionedResource {
        #[doc = "JSON representation of the resource as defined by the corresponding service providing this resource. Example: If the resource is an instance provided by Compute Engine, this field will contain the JSON representation of the instance as defined by Compute Engine: `https://cloud.google.com/compute/docs/reference/rest/v1/instances`. You can find the resource definition for each supported resource type in this table: `https://cloud.google.com/asset-inventory/docs/supported-asset-types#searchable_asset_types`"]
        #[serde(
            rename = "resource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "API version of the resource. Example: If the resource is an instance provided by Compute Engine v1 API as defined in `https://cloud.google.com/compute/docs/reference/rest/v1/instances`, version will be “v1”."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VersionedResource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VersionedResource {
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
    pub struct WindowsApplication {
        #[doc = "The name of the application or product."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The version of the product or application in string format."]
        #[serde(
            rename = "displayVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_version: ::std::option::Option<String>,
        #[doc = "The internet address for technical support."]
        #[serde(
            rename = "helpLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub help_link: ::std::option::Option<String>,
        #[doc = "The last time this product received service. The value of this property is replaced each time a patch is applied or removed from the product or the command-line option is used to repair the product."]
        #[serde(
            rename = "installDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub install_date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "The name of the manufacturer for the product or application."]
        #[serde(
            rename = "publisher",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publisher: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for WindowsApplication {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WindowsApplication {
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
    pub struct WindowsQuickFixEngineeringPackage {
        #[doc = "A short textual description of the QFE update."]
        #[serde(
            rename = "caption",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub caption: ::std::option::Option<String>,
        #[doc = "A textual description of the QFE update."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Unique identifier associated with a particular QFE update."]
        #[serde(
            rename = "hotFixId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hot_fix_id: ::std::option::Option<String>,
        #[doc = "Date that the QFE update was installed. Mapped from installed_on field."]
        #[serde(
            rename = "installTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub install_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for WindowsQuickFixEngineeringPackage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WindowsQuickFixEngineeringPackage {
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
    pub struct WindowsUpdateCategory {
        #[doc = "The identifier of the windows update category."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The name of the windows update category."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for WindowsUpdateCategory {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WindowsUpdateCategory {
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
    pub struct WindowsUpdatePackage {
        #[doc = "The categories that are associated with this update package."]
        #[serde(
            rename = "categories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub categories: ::std::option::Option<Vec<crate::schemas::WindowsUpdateCategory>>,
        #[doc = "The localized description of the update package."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "A collection of Microsoft Knowledge Base article IDs that are associated with the update package."]
        #[serde(
            rename = "kbArticleIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kb_article_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The last published date of the update, in (UTC) date and time."]
        #[serde(
            rename = "lastDeploymentChangeTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_deployment_change_time: ::std::option::Option<String>,
        #[doc = "A collection of URLs that provide more information about the update package."]
        #[serde(
            rename = "moreInfoUrls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub more_info_urls: ::std::option::Option<Vec<String>>,
        #[doc = "The revision number of this update package."]
        #[serde(
            rename = "revisionNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_number: ::std::option::Option<i32>,
        #[doc = "A hyperlink to the language-specific support information for the update."]
        #[serde(
            rename = "supportUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub support_url: ::std::option::Option<String>,
        #[doc = "The localized title of the update package."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "Gets the identifier of an update package. Stays the same across revisions."]
        #[serde(
            rename = "updateId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for WindowsUpdatePackage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WindowsUpdatePackage {
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
    pub struct ZypperPatch {
        #[doc = "The category of the patch."]
        #[serde(
            rename = "category",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category: ::std::option::Option<String>,
        #[doc = "The name of the patch."]
        #[serde(
            rename = "patchName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub patch_name: ::std::option::Option<String>,
        #[doc = "The severity specified for this patch"]
        #[serde(
            rename = "severity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub severity: ::std::option::Option<String>,
        #[doc = "Any summary information provided about this patch."]
        #[serde(
            rename = "summary",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub summary: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ZypperPatch {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ZypperPatch {
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
    #[doc = "Actions that can be performed on the assets resource"]
    pub fn assets(&self) -> crate::resources::assets::AssetsActions {
        crate::resources::assets::AssetsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the effective_iam_policies resource"]
    pub fn effective_iam_policies(
        &self,
    ) -> crate::resources::effective_iam_policies::EffectiveIamPoliciesActions {
        crate::resources::effective_iam_policies::EffectiveIamPoliciesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the feeds resource"]
    pub fn feeds(&self) -> crate::resources::feeds::FeedsActions {
        crate::resources::feeds::FeedsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the operations resource"]
    pub fn operations(&self) -> crate::resources::operations::OperationsActions {
        crate::resources::operations::OperationsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the saved_queries resource"]
    pub fn saved_queries(&self) -> crate::resources::saved_queries::SavedQueriesActions {
        crate::resources::saved_queries::SavedQueriesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the v_1 resource"]
    pub fn v_1(&self) -> crate::resources::v_1::V1Actions {
        crate::resources::v_1::V1Actions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod assets {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListContentType {
                #[doc = "The Access Context Manager policy set on an asset."]
                AccessPolicy,
                #[doc = "Unspecified content type."]
                ContentTypeUnspecified,
                #[doc = "The actual IAM policy set on a resource."]
                IamPolicy,
                #[doc = "The organization policy set on an asset."]
                OrgPolicy,
                #[doc = "The runtime OS Inventory information."]
                OsInventory,
                #[doc = "The related resources."]
                Relationship,
                #[doc = "Resource metadata."]
                Resource,
            }
            impl ListContentType {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListContentType::AccessPolicy => "ACCESS_POLICY",
                        ListContentType::ContentTypeUnspecified => "CONTENT_TYPE_UNSPECIFIED",
                        ListContentType::IamPolicy => "IAM_POLICY",
                        ListContentType::OrgPolicy => "ORG_POLICY",
                        ListContentType::OsInventory => "OS_INVENTORY",
                        ListContentType::Relationship => "RELATIONSHIP",
                        ListContentType::Resource => "RESOURCE",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListContentType {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListContentType {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListContentType, ()> {
                    Ok(match s {
                        "ACCESS_POLICY" => ListContentType::AccessPolicy,
                        "CONTENT_TYPE_UNSPECIFIED" => ListContentType::ContentTypeUnspecified,
                        "IAM_POLICY" => ListContentType::IamPolicy,
                        "ORG_POLICY" => ListContentType::OrgPolicy,
                        "OS_INVENTORY" => ListContentType::OsInventory,
                        "RELATIONSHIP" => ListContentType::Relationship,
                        "RESOURCE" => ListContentType::Resource,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListContentType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListContentType {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListContentType {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "ACCESS_POLICY" => ListContentType::AccessPolicy,
                        "CONTENT_TYPE_UNSPECIFIED" => ListContentType::ContentTypeUnspecified,
                        "IAM_POLICY" => ListContentType::IamPolicy,
                        "ORG_POLICY" => ListContentType::OrgPolicy,
                        "OS_INVENTORY" => ListContentType::OsInventory,
                        "RELATIONSHIP" => ListContentType::Relationship,
                        "RESOURCE" => ListContentType::Resource,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListContentType {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListContentType {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct AssetsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> AssetsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Lists assets with time and resource types and returns paged results in response."]
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
                    asset_types: None,
                    content_type: None,
                    page_size: None,
                    page_token: None,
                    read_time: None,
                    relationship_types: None,
                }
            }
        }
        #[doc = "Created via [AssetsActions::list()](struct.AssetsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            parent: String,
            asset_types: ::std::option::Option<Vec<String>>,
            content_type: ::std::option::Option<crate::resources::assets::params::ListContentType>,
            page_size: ::std::option::Option<i32>,
            page_token: ::std::option::Option<String>,
            read_time: ::std::option::Option<String>,
            relationship_types: ::std::option::Option<Vec<String>>,
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
            #[doc = "A list of asset types to take a snapshot for. For example: “compute.googleapis.com/Disk”. Regular expression is also supported. For example: * “compute.googleapis.com.\\*” snapshots resources whose asset type starts with “compute.googleapis.com”. * “.\\*Instance” snapshots resources whose asset type ends with “Instance”. * “.*Instance.*” snapshots resources whose asset type contains “Instance”. See [RE2](https://github.com/google/re2/wiki/Syntax) for all supported regular expression syntax. If the regular expression does not match any supported asset type, an INVALID_ARGUMENT error will be returned. If specified, only matching assets will be returned, otherwise, it will snapshot all asset types. See [Introduction to Cloud Asset Inventory](https://cloud.google.com/asset-inventory/docs/overview) for all supported asset types."]
            pub fn asset_types(mut self, value: impl Into<Vec<String>>) -> Self {
                self.asset_types = Some(value.into());
                self
            }
            #[doc = "Asset content type. If not specified, no content but the asset name will be returned."]
            pub fn content_type(
                mut self,
                value: crate::resources::assets::params::ListContentType,
            ) -> Self {
                self.content_type = Some(value);
                self
            }
            #[doc = "The maximum number of assets to be returned in a single response. Default is 100, minimum is 1, and maximum is 1000."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "The `next_page_token` returned from the previous `ListAssetsResponse`, or unspecified for the first `ListAssetsRequest`. It is a continuation of a prior `ListAssets` call, and the API should return the next page of assets."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "Timestamp to take an asset snapshot. This can only be set to a timestamp between the current time and the current time minus 35 days (inclusive). If not specified, the current time will be used. Due to delays in resource data collection and indexing, there is a volatile window during which running the same query may get different results."]
            pub fn read_time(mut self, value: impl Into<String>) -> Self {
                self.read_time = Some(value.into());
                self
            }
            #[doc = "A list of relationship types to output, for example: `INSTANCE_TO_INSTANCEGROUP`. This field should only be specified if content_type=RELATIONSHIP. * If specified: it snapshots specified relationships. It returns an error if any of the \\[relationship_types\\] doesn’t belong to the supported relationship types of the \\[asset_types\\] or if any of the \\[asset_types\\] doesn’t belong to the source types of the \\[relationship_types\\]. * Otherwise: it snapshots the supported relationships for all \\[asset_types\\] or returns an error if any of the \\[asset_types\\] has no relationship support. An unspecified asset types field means all supported asset_types. See [Introduction to Cloud Asset Inventory](https://cloud.google.com/asset-inventory/docs/overview) for all supported asset types and relationship types."]
            pub fn relationship_types(mut self, value: impl Into<Vec<String>>) -> Self {
                self.relationship_types = Some(value.into());
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
            #[doc = "\nExecute the request and yield each item in the `assets` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
            pub fn stream_assets<T>(
                self,
            ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector + 'a,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.stream_assets_with_fields(fields)
            }
            #[doc = "\nExecute the request and yield each item in the `assets` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
            pub fn stream_assets_with_default_fields(
                self,
            ) -> impl ::futures::Stream<Item = Result<crate::schemas::Asset, crate::Error>> + 'a
            {
                self.stream_assets_with_fields(None::<String>)
            }
            #[doc = "\nExecute the request and yield each item in the `assets` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
            pub fn stream_assets_with_all_fields(
                self,
            ) -> impl ::futures::Stream<Item = Result<crate::schemas::Asset, crate::Error>> + 'a
            {
                self.stream_assets_with_fields(Some("*"))
            }
            #[doc = "\nExecute the request and yield each item in the `assets` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
            pub fn stream_assets_with_fields<T, F>(
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
                    #[serde(rename = "assets")]
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
                    let mut selector = concat!("nextPageToken,", "assets").to_owned();
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
            pub fn stream<T>(self) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
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
                Item = Result<crate::schemas::ListAssetsResponse, crate::Error>,
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
                Item = Result<crate::schemas::ListAssetsResponse, crate::Error>,
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
                let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
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
            ) -> Result<crate::schemas::ListAssetsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListAssetsResponse, crate::Error> {
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/assets");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                for value in self.asset_types.iter().flatten() {
                    req = req.query(&[("assetTypes", value)]);
                }
                req = req.query(&[("contentType", &self.content_type)]);
                req = req.query(&[("pageSize", &self.page_size)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("readTime", &self.read_time)]);
                for value in self.relationship_types.iter().flatten() {
                    req = req.query(&[("relationshipTypes", value)]);
                }
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
    pub mod effective_iam_policies {
        pub mod params {}
        pub struct EffectiveIamPoliciesActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> EffectiveIamPoliciesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Gets effective IAM policies for a batch of resources."]
            pub fn batch_get(&self, scope: impl Into<String>) -> BatchGetRequestBuilder {
                BatchGetRequestBuilder {
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
                    scope: scope.into(),
                    names: None,
                }
            }
        }
        #[doc = "Created via [EffectiveIamPoliciesActions::batch_get()](struct.EffectiveIamPoliciesActions.html#method.batch_get)"]
        #[derive(Debug, Clone)]
        pub struct BatchGetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            scope: String,
            names: ::std::option::Option<Vec<String>>,
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
        impl<'a> BatchGetRequestBuilder<'a> {
            #[doc = "Required. The names refer to the \\[full_resource_names\\] (https://cloud.google.com/asset-inventory/docs/resource-name-format) of [searchable asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types#searchable_asset_types). A maximum of 20 resources’ effective policies can be retrieved in a batch."]
            pub fn names(mut self, value: impl Into<Vec<String>>) -> Self {
                self.names = Some(value.into());
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
            ) -> Result<crate::schemas::BatchGetEffectiveIamPoliciesResponse, crate::Error>
            {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::BatchGetEffectiveIamPoliciesResponse, crate::Error>
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.scope;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/effectiveIamPolicies:batchGet");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                for value in self.names.iter().flatten() {
                    req = req.query(&[("names", value)]);
                }
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
    pub mod feeds {
        pub mod params {}
        pub struct FeedsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> FeedsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Creates a feed in a parent project/folder/organization to listen to its asset updates."]
            pub fn create(
                &self,
                request: crate::schemas::CreateFeedRequest,
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
            #[doc = "Deletes an asset feed."]
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
            #[doc = "Gets details about an asset feed."]
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
            #[doc = "Lists all asset feeds in a parent project/folder/organization."]
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
                }
            }
            #[doc = "Updates an asset feed configuration."]
            pub fn patch(
                &self,
                request: crate::schemas::UpdateFeedRequest,
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
                }
            }
        }
        #[doc = "Created via [FeedsActions::create()](struct.FeedsActions.html#method.create)"]
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::CreateFeedRequest,
            parent: String,
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
            ) -> Result<crate::schemas::Feed, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Feed, crate::Error> {
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/feeds");
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
        #[doc = "Created via [FeedsActions::delete()](struct.FeedsActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Empty, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Empty, crate::Error> {
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
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
        #[doc = "Created via [FeedsActions::get()](struct.FeedsActions.html#method.get)"]
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
            ) -> Result<crate::schemas::Feed, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Feed, crate::Error> {
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
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
        #[doc = "Created via [FeedsActions::list()](struct.FeedsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            parent: String,
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
            ) -> Result<crate::schemas::ListFeedsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListFeedsResponse, crate::Error> {
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/feeds");
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
        #[doc = "Created via [FeedsActions::patch()](struct.FeedsActions.html#method.patch)"]
        #[derive(Debug, Clone)]
        pub struct PatchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::UpdateFeedRequest,
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
        impl<'a> PatchRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Feed, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Feed, crate::Error> {
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
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
                let mut req = self.reqwest.request(::reqwest::Method::PATCH, path);
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
            ) -> Result<crate::schemas::Operation, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Operation, crate::Error> {
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
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
    pub mod saved_queries {
        pub mod params {}
        pub struct SavedQueriesActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> SavedQueriesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Creates a saved query in a parent project/folder/organization."]
            pub fn create(
                &self,
                request: crate::schemas::SavedQuery,
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
                    saved_query_id: None,
                }
            }
            #[doc = "Deletes a saved query."]
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
            #[doc = "Gets details about a saved query."]
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
            #[doc = "Lists all saved queries in a parent project/folder/organization."]
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
            #[doc = "Updates a saved query."]
            pub fn patch(
                &self,
                request: crate::schemas::SavedQuery,
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
        #[doc = "Created via [SavedQueriesActions::create()](struct.SavedQueriesActions.html#method.create)"]
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::SavedQuery,
            parent: String,
            saved_query_id: ::std::option::Option<String>,
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
            #[doc = "Required. The ID to use for the saved query, which must be unique in the specified parent. It will become the final component of the saved query’s resource name. This value should be 4-63 characters, and valid characters are `a-z-`. Notice that this field is required in the saved query creation, and the `name` field of the `saved_query` will be ignored."]
            pub fn saved_query_id(mut self, value: impl Into<String>) -> Self {
                self.saved_query_id = Some(value.into());
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
            ) -> Result<crate::schemas::SavedQuery, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::SavedQuery, crate::Error> {
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/savedQueries");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("savedQueryId", &self.saved_query_id)]);
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
        #[doc = "Created via [SavedQueriesActions::delete()](struct.SavedQueriesActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Empty, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Empty, crate::Error> {
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
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
        #[doc = "Created via [SavedQueriesActions::get()](struct.SavedQueriesActions.html#method.get)"]
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
            ) -> Result<crate::schemas::SavedQuery, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::SavedQuery, crate::Error> {
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
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
        #[doc = "Created via [SavedQueriesActions::list()](struct.SavedQueriesActions.html#method.list)"]
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
            #[doc = "Optional. The expression to filter resources. The expression is a list of zero or more restrictions combined via logical operators `AND` and `OR`. When `AND` and `OR` are both used in the expression, parentheses must be appropriately used to group the combinations. The expression may also contain regular expressions. See https://google.aip.dev/160 for more information on the grammar."]
            pub fn filter(mut self, value: impl Into<String>) -> Self {
                self.filter = Some(value.into());
                self
            }
            #[doc = "Optional. The maximum number of saved queries to return per page. The service may return fewer than this value. If unspecified, at most 50 will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "Optional. A page token, received from a previous `ListSavedQueries` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListSavedQueries` must match the call that provided the page token."]
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
            #[doc = "\nExecute the request and yield each item in the `savedQueries` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
            pub fn stream_saved_queries<T>(
                self,
            ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector + 'a,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.stream_saved_queries_with_fields(fields)
            }
            #[doc = "\nExecute the request and yield each item in the `savedQueries` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
            pub fn stream_saved_queries_with_default_fields(
                self,
            ) -> impl ::futures::Stream<Item = Result<crate::schemas::SavedQuery, crate::Error>> + 'a
            {
                self.stream_saved_queries_with_fields(None::<String>)
            }
            #[doc = "\nExecute the request and yield each item in the `savedQueries` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
            pub fn stream_saved_queries_with_all_fields(
                self,
            ) -> impl ::futures::Stream<Item = Result<crate::schemas::SavedQuery, crate::Error>> + 'a
            {
                self.stream_saved_queries_with_fields(Some("*"))
            }
            #[doc = "\nExecute the request and yield each item in the `savedQueries` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
            pub fn stream_saved_queries_with_fields<T, F>(
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
                    #[serde(rename = "savedQueries")]
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
                    let mut selector = concat!("nextPageToken,", "savedQueries").to_owned();
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
            pub fn stream<T>(self) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
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
                Item = Result<crate::schemas::ListSavedQueriesResponse, crate::Error>,
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
                Item = Result<crate::schemas::ListSavedQueriesResponse, crate::Error>,
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
                let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
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
            ) -> Result<crate::schemas::ListSavedQueriesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListSavedQueriesResponse, crate::Error> {
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/savedQueries");
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
        #[doc = "Created via [SavedQueriesActions::patch()](struct.SavedQueriesActions.html#method.patch)"]
        #[derive(Debug, Clone)]
        pub struct PatchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::SavedQuery,
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
            #[doc = "Required. The list of fields to update."]
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
            ) -> Result<crate::schemas::SavedQuery, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::SavedQuery, crate::Error> {
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
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
    }
    pub mod v_1 {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum AnalyzeMoveView {
                #[doc = "The default/unset value. The API will default to the FULL view."]
                AnalysisViewUnspecified,
                #[doc = "Basic analysis only including blockers which will prevent the specified resource move at runtime."]
                Basic,
                #[doc = "Full analysis including all level of impacts of the specified resource move."]
                Full,
            }
            impl AnalyzeMoveView {
                pub fn as_str(self) -> &'static str {
                    match self {
                        AnalyzeMoveView::AnalysisViewUnspecified => "ANALYSIS_VIEW_UNSPECIFIED",
                        AnalyzeMoveView::Basic => "BASIC",
                        AnalyzeMoveView::Full => "FULL",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for AnalyzeMoveView {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for AnalyzeMoveView {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<AnalyzeMoveView, ()> {
                    Ok(match s {
                        "ANALYSIS_VIEW_UNSPECIFIED" => AnalyzeMoveView::AnalysisViewUnspecified,
                        "BASIC" => AnalyzeMoveView::Basic,
                        "FULL" => AnalyzeMoveView::Full,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for AnalyzeMoveView {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for AnalyzeMoveView {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for AnalyzeMoveView {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "ANALYSIS_VIEW_UNSPECIFIED" => AnalyzeMoveView::AnalysisViewUnspecified,
                        "BASIC" => AnalyzeMoveView::Basic,
                        "FULL" => AnalyzeMoveView::Full,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for AnalyzeMoveView {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for AnalyzeMoveView {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum BatchGetAssetsHistoryContentType {
                #[doc = "The Access Context Manager policy set on an asset."]
                AccessPolicy,
                #[doc = "Unspecified content type."]
                ContentTypeUnspecified,
                #[doc = "The actual IAM policy set on a resource."]
                IamPolicy,
                #[doc = "The organization policy set on an asset."]
                OrgPolicy,
                #[doc = "The runtime OS Inventory information."]
                OsInventory,
                #[doc = "The related resources."]
                Relationship,
                #[doc = "Resource metadata."]
                Resource,
            }
            impl BatchGetAssetsHistoryContentType {
                pub fn as_str(self) -> &'static str {
                    match self {
                        BatchGetAssetsHistoryContentType::AccessPolicy => "ACCESS_POLICY",
                        BatchGetAssetsHistoryContentType::ContentTypeUnspecified => {
                            "CONTENT_TYPE_UNSPECIFIED"
                        }
                        BatchGetAssetsHistoryContentType::IamPolicy => "IAM_POLICY",
                        BatchGetAssetsHistoryContentType::OrgPolicy => "ORG_POLICY",
                        BatchGetAssetsHistoryContentType::OsInventory => "OS_INVENTORY",
                        BatchGetAssetsHistoryContentType::Relationship => "RELATIONSHIP",
                        BatchGetAssetsHistoryContentType::Resource => "RESOURCE",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for BatchGetAssetsHistoryContentType {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for BatchGetAssetsHistoryContentType {
                type Err = ();
                fn from_str(
                    s: &str,
                ) -> ::std::result::Result<BatchGetAssetsHistoryContentType, ()> {
                    Ok(match s {
                        "ACCESS_POLICY" => BatchGetAssetsHistoryContentType::AccessPolicy,
                        "CONTENT_TYPE_UNSPECIFIED" => {
                            BatchGetAssetsHistoryContentType::ContentTypeUnspecified
                        }
                        "IAM_POLICY" => BatchGetAssetsHistoryContentType::IamPolicy,
                        "ORG_POLICY" => BatchGetAssetsHistoryContentType::OrgPolicy,
                        "OS_INVENTORY" => BatchGetAssetsHistoryContentType::OsInventory,
                        "RELATIONSHIP" => BatchGetAssetsHistoryContentType::Relationship,
                        "RESOURCE" => BatchGetAssetsHistoryContentType::Resource,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for BatchGetAssetsHistoryContentType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for BatchGetAssetsHistoryContentType {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for BatchGetAssetsHistoryContentType {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "ACCESS_POLICY" => BatchGetAssetsHistoryContentType::AccessPolicy,
                        "CONTENT_TYPE_UNSPECIFIED" => {
                            BatchGetAssetsHistoryContentType::ContentTypeUnspecified
                        }
                        "IAM_POLICY" => BatchGetAssetsHistoryContentType::IamPolicy,
                        "ORG_POLICY" => BatchGetAssetsHistoryContentType::OrgPolicy,
                        "OS_INVENTORY" => BatchGetAssetsHistoryContentType::OsInventory,
                        "RELATIONSHIP" => BatchGetAssetsHistoryContentType::Relationship,
                        "RESOURCE" => BatchGetAssetsHistoryContentType::Resource,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for BatchGetAssetsHistoryContentType {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for BatchGetAssetsHistoryContentType {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct V1Actions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> V1Actions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Analyzes IAM policies to answer which identities have what accesses on which resources."]
            pub fn analyze_iam_policy(
                &self,
                scope: impl Into<String>,
            ) -> AnalyzeIamPolicyRequestBuilder {
                AnalyzeIamPolicyRequestBuilder {
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
                    scope: scope.into(),
                    analysis_query_access_selector_permissions: None,
                    analysis_query_access_selector_roles: None,
                    analysis_query_condition_context_access_time: None,
                    analysis_query_identity_selector_identity: None,
                    analysis_query_options_analyze_service_account_impersonation: None,
                    analysis_query_options_expand_groups: None,
                    analysis_query_options_expand_resources: None,
                    analysis_query_options_expand_roles: None,
                    analysis_query_options_output_group_edges: None,
                    analysis_query_options_output_resource_edges: None,
                    analysis_query_resource_selector_full_resource_name: None,
                    execution_timeout: None,
                    saved_analysis_query: None,
                }
            }
            #[doc = "Analyzes IAM policies asynchronously to answer which identities have what accesses on which resources, and writes the analysis results to a Google Cloud Storage or a BigQuery destination. For Cloud Storage destination, the output format is the JSON format that represents a AnalyzeIamPolicyResponse. This method implements the google.longrunning.Operation, which allows you to track the operation status. We recommend intervals of at least 2 seconds with exponential backoff retry to poll the operation result. The metadata contains the metadata for the long-running operation."]
            pub fn analyze_iam_policy_longrunning(
                &self,
                request: crate::schemas::AnalyzeIamPolicyLongrunningRequest,
                scope: impl Into<String>,
            ) -> AnalyzeIamPolicyLongrunningRequestBuilder {
                AnalyzeIamPolicyLongrunningRequestBuilder {
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
                    scope: scope.into(),
                }
            }
            #[doc = "Analyze moving a resource to a specified destination without kicking off the actual move. The analysis is best effort depending on the user’s permissions of viewing different hierarchical policies and configurations. The policies and configuration are subject to change before the actual resource migration takes place."]
            pub fn analyze_move(&self, resource: impl Into<String>) -> AnalyzeMoveRequestBuilder {
                AnalyzeMoveRequestBuilder {
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
                    resource: resource.into(),
                    destination_parent: None,
                    view: None,
                }
            }
            #[doc = "Analyzes organization policies under a scope."]
            pub fn analyze_org_policies(
                &self,
                scope: impl Into<String>,
            ) -> AnalyzeOrgPoliciesRequestBuilder {
                AnalyzeOrgPoliciesRequestBuilder {
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
                    scope: scope.into(),
                    constraint: None,
                    filter: None,
                    page_size: None,
                    page_token: None,
                }
            }
            #[doc = "Analyzes organization policies governed assets (Google Cloud resources or policies) under a scope. This RPC supports custom constraints and the following 10 canned constraints: * storage.uniformBucketLevelAccess * iam.disableServiceAccountKeyCreation * iam.allowedPolicyMemberDomains * compute.vmExternalIpAccess * appengine.enforceServiceAccountActAsCheck * gcp.resourceLocations * compute.trustedImageProjects * compute.skipDefaultNetworkCreation * compute.requireOsLogin * compute.disableNestedVirtualization This RPC only returns either resources of types supported by [searchable asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types#searchable_asset_types), or IAM policies."]
            pub fn analyze_org_policy_governed_assets(
                &self,
                scope: impl Into<String>,
            ) -> AnalyzeOrgPolicyGovernedAssetsRequestBuilder {
                AnalyzeOrgPolicyGovernedAssetsRequestBuilder {
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
                    scope: scope.into(),
                    constraint: None,
                    filter: None,
                    page_size: None,
                    page_token: None,
                }
            }
            #[doc = "Analyzes organization policies governed containers (projects, folders or organization) under a scope."]
            pub fn analyze_org_policy_governed_containers(
                &self,
                scope: impl Into<String>,
            ) -> AnalyzeOrgPolicyGovernedContainersRequestBuilder {
                AnalyzeOrgPolicyGovernedContainersRequestBuilder {
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
                    scope: scope.into(),
                    constraint: None,
                    filter: None,
                    page_size: None,
                    page_token: None,
                }
            }
            #[doc = "Batch gets the update history of assets that overlap a time window. For IAM_POLICY content, this API outputs history when the asset and its attached IAM POLICY both exist. This can create gaps in the output history. Otherwise, this API outputs history with asset in both non-delete or deleted status. If a specified asset does not exist, this API returns an INVALID_ARGUMENT error."]
            pub fn batch_get_assets_history(
                &self,
                parent: impl Into<String>,
            ) -> BatchGetAssetsHistoryRequestBuilder {
                BatchGetAssetsHistoryRequestBuilder {
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
                    asset_names: None,
                    content_type: None,
                    read_time_window_end_time: None,
                    read_time_window_start_time: None,
                    relationship_types: None,
                }
            }
            #[doc = "Exports assets with time and resource types to a given Cloud Storage location/BigQuery table. For Cloud Storage location destinations, the output format is newline-delimited JSON. Each line represents a google.cloud.asset.v1.Asset in the JSON format; for BigQuery table destinations, the output table stores the fields in asset Protobuf as columns. This API implements the google.longrunning.Operation API, which allows you to keep track of the export. We recommend intervals of at least 2 seconds with exponential retry to poll the export operation result. For regular-size resource parent, the export operation usually finishes within 5 minutes."]
            pub fn export_assets(
                &self,
                request: crate::schemas::ExportAssetsRequest,
                parent: impl Into<String>,
            ) -> ExportAssetsRequestBuilder {
                ExportAssetsRequestBuilder {
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
            #[doc = "Issue a job that queries assets using a SQL statement compatible with [BigQuery Standard SQL](http://cloud/bigquery/docs/reference/standard-sql/enabling-standard-sql). If the query execution finishes within timeout and there’s no pagination, the full query results will be returned in the `QueryAssetsResponse`. Otherwise, full query results can be obtained by issuing extra requests with the `job_reference` from the a previous `QueryAssets` call. Note, the query result has approximately 10 GB limitation enforced by BigQuery https://cloud.google.com/bigquery/docs/best-practices-performance-output, queries return larger results will result in errors."]
            pub fn query_assets(
                &self,
                request: crate::schemas::QueryAssetsRequest,
                parent: impl Into<String>,
            ) -> QueryAssetsRequestBuilder {
                QueryAssetsRequestBuilder {
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
            #[doc = "Searches all IAM policies within the specified scope, such as a project, folder, or organization. The caller must be granted the `cloudasset.assets.searchAllIamPolicies` permission on the desired scope, otherwise the request will be rejected."]
            pub fn search_all_iam_policies(
                &self,
                scope: impl Into<String>,
            ) -> SearchAllIamPoliciesRequestBuilder {
                SearchAllIamPoliciesRequestBuilder {
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
                    scope: scope.into(),
                    asset_types: None,
                    order_by: None,
                    page_size: None,
                    page_token: None,
                    query: None,
                }
            }
            #[doc = "Searches all Google Cloud resources within the specified scope, such as a project, folder, or organization. The caller must be granted the `cloudasset.assets.searchAllResources` permission on the desired scope, otherwise the request will be rejected."]
            pub fn search_all_resources(
                &self,
                scope: impl Into<String>,
            ) -> SearchAllResourcesRequestBuilder {
                SearchAllResourcesRequestBuilder {
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
                    scope: scope.into(),
                    asset_types: None,
                    order_by: None,
                    page_size: None,
                    page_token: None,
                    query: None,
                    read_mask: None,
                }
            }
        }
        #[doc = "Created via [V1Actions::analyze_iam_policy()](struct.V1Actions.html#method.analyze_iam_policy)"]
        #[derive(Debug, Clone)]
        pub struct AnalyzeIamPolicyRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            scope: String,
            analysis_query_access_selector_permissions: ::std::option::Option<Vec<String>>,
            analysis_query_access_selector_roles: ::std::option::Option<Vec<String>>,
            analysis_query_condition_context_access_time: ::std::option::Option<String>,
            analysis_query_identity_selector_identity: ::std::option::Option<String>,
            analysis_query_options_analyze_service_account_impersonation:
                ::std::option::Option<bool>,
            analysis_query_options_expand_groups: ::std::option::Option<bool>,
            analysis_query_options_expand_resources: ::std::option::Option<bool>,
            analysis_query_options_expand_roles: ::std::option::Option<bool>,
            analysis_query_options_output_group_edges: ::std::option::Option<bool>,
            analysis_query_options_output_resource_edges: ::std::option::Option<bool>,
            analysis_query_resource_selector_full_resource_name: ::std::option::Option<String>,
            execution_timeout: ::std::option::Option<String>,
            saved_analysis_query: ::std::option::Option<String>,
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
        impl<'a> AnalyzeIamPolicyRequestBuilder<'a> {
            #[doc = "Optional. The permissions to appear in result."]
            pub fn analysis_query_access_selector_permissions(
                mut self,
                value: impl Into<Vec<String>>,
            ) -> Self {
                self.analysis_query_access_selector_permissions = Some(value.into());
                self
            }
            #[doc = "Optional. The roles to appear in result."]
            pub fn analysis_query_access_selector_roles(
                mut self,
                value: impl Into<Vec<String>>,
            ) -> Self {
                self.analysis_query_access_selector_roles = Some(value.into());
                self
            }
            #[doc = "The hypothetical access timestamp to evaluate IAM conditions. Note that this value must not be earlier than the current time; otherwise, an INVALID_ARGUMENT error will be returned."]
            pub fn analysis_query_condition_context_access_time(
                mut self,
                value: impl Into<String>,
            ) -> Self {
                self.analysis_query_condition_context_access_time = Some(value.into());
                self
            }
            #[doc = "Required. The identity appear in the form of principals in [IAM policy binding](https://cloud.google.com/iam/reference/rest/v1/Binding). The examples of supported forms are: “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com”. Notice that wildcard characters (such as * and ?) are not supported. You must give a specific identity."]
            pub fn analysis_query_identity_selector_identity(
                mut self,
                value: impl Into<String>,
            ) -> Self {
                self.analysis_query_identity_selector_identity = Some(value.into());
                self
            }
            #[doc = "Optional. If true, the response will include access analysis from identities to resources via service account impersonation. This is a very expensive operation, because many derived queries will be executed. We highly recommend you use AssetService.AnalyzeIamPolicyLongrunning RPC instead. For example, if the request analyzes for which resources user A has permission P, and there’s an IAM policy states user A has iam.serviceAccounts.getAccessToken permission to a service account SA, and there’s another IAM policy states service account SA has permission P to a Google Cloud folder F, then user A potentially has access to the Google Cloud folder F. And those advanced analysis results will be included in AnalyzeIamPolicyResponse.service_account_impersonation_analysis. Another example, if the request analyzes for who has permission P to a Google Cloud folder F, and there’s an IAM policy states user A has iam.serviceAccounts.actAs permission to a service account SA, and there’s another IAM policy states service account SA has permission P to the Google Cloud folder F, then user A potentially has access to the Google Cloud folder F. And those advanced analysis results will be included in AnalyzeIamPolicyResponse.service_account_impersonation_analysis. Only the following permissions are considered in this analysis: * `iam.serviceAccounts.actAs` * `iam.serviceAccounts.signBlob` * `iam.serviceAccounts.signJwt` * `iam.serviceAccounts.getAccessToken` * `iam.serviceAccounts.getOpenIdToken` * `iam.serviceAccounts.implicitDelegation` Default is false."]
            pub fn analysis_query_options_analyze_service_account_impersonation(
                mut self,
                value: bool,
            ) -> Self {
                self.analysis_query_options_analyze_service_account_impersonation = Some(value);
                self
            }
            #[doc = "Optional. If true, the identities section of the result will expand any Google groups appearing in an IAM policy binding. If IamPolicyAnalysisQuery.identity_selector is specified, the identity in the result will be determined by the selector, and this flag is not allowed to set. If true, the default max expansion per group is 1000 for AssetService.AnalyzeIamPolicy\\]\\[\\]. Default is false."]
            pub fn analysis_query_options_expand_groups(mut self, value: bool) -> Self {
                self.analysis_query_options_expand_groups = Some(value);
                self
            }
            #[doc = "Optional. If true and IamPolicyAnalysisQuery.resource_selector is not specified, the resource section of the result will expand any resource attached to an IAM policy to include resources lower in the resource hierarchy. For example, if the request analyzes for which resources user A has permission P, and the results include an IAM policy with P on a Google Cloud folder, the results will also include resources in that folder with permission P. If true and IamPolicyAnalysisQuery.resource_selector is specified, the resource section of the result will expand the specified resource to include resources lower in the resource hierarchy. Only project or lower resources are supported. Folder and organization resources cannot be used together with this option. For example, if the request analyzes for which users have permission P on a Google Cloud project with this option enabled, the results will include all users who have permission P on that project or any lower resource. If true, the default max expansion per resource is 1000 for AssetService.AnalyzeIamPolicy\\]\\[\\] and 100000 for AssetService.AnalyzeIamPolicyLongrunning\\]\\[\\]. Default is false."]
            pub fn analysis_query_options_expand_resources(mut self, value: bool) -> Self {
                self.analysis_query_options_expand_resources = Some(value);
                self
            }
            #[doc = "Optional. If true, the access section of result will expand any roles appearing in IAM policy bindings to include their permissions. If IamPolicyAnalysisQuery.access_selector is specified, the access section of the result will be determined by the selector, and this flag is not allowed to set. Default is false."]
            pub fn analysis_query_options_expand_roles(mut self, value: bool) -> Self {
                self.analysis_query_options_expand_roles = Some(value);
                self
            }
            #[doc = "Optional. If true, the result will output the relevant membership relationships between groups and other groups, and between groups and principals. Default is false."]
            pub fn analysis_query_options_output_group_edges(mut self, value: bool) -> Self {
                self.analysis_query_options_output_group_edges = Some(value);
                self
            }
            #[doc = "Optional. If true, the result will output the relevant parent/child relationships between resources. Default is false."]
            pub fn analysis_query_options_output_resource_edges(mut self, value: bool) -> Self {
                self.analysis_query_options_output_resource_edges = Some(value);
                self
            }
            #[doc = "Required. The \\[full resource name\\] (https://cloud.google.com/asset-inventory/docs/resource-name-format) of a resource of [supported resource types](https://cloud.google.com/asset-inventory/docs/supported-asset-types#analyzable_asset_types)."]
            pub fn analysis_query_resource_selector_full_resource_name(
                mut self,
                value: impl Into<String>,
            ) -> Self {
                self.analysis_query_resource_selector_full_resource_name = Some(value.into());
                self
            }
            #[doc = "Optional. Amount of time executable has to complete. See JSON representation of [Duration](https://developers.google.com/protocol-buffers/docs/proto3#json). If this field is set with a value less than the RPC deadline, and the execution of your query hasn’t finished in the specified execution timeout, you will get a response with partial result. Otherwise, your query’s execution will continue until the RPC deadline. If it’s not finished until then, you will get a DEADLINE_EXCEEDED error. Default is empty."]
            pub fn execution_timeout(mut self, value: impl Into<String>) -> Self {
                self.execution_timeout = Some(value.into());
                self
            }
            #[doc = "Optional. The name of a saved query, which must be in the format of: * projects/project_number/savedQueries/saved_query_id * folders/folder_number/savedQueries/saved_query_id * organizations/organization_number/savedQueries/saved_query_id If both `analysis_query` and `saved_analysis_query` are provided, they will be merged together with the `saved_analysis_query` as base and the `analysis_query` as overrides. For more details of the merge behavior, please refer to the [MergeFrom](https://developers.google.com/protocol-buffers/docs/reference/cpp/google.protobuf.message#Message.MergeFrom.details) page. Note that you cannot override primitive fields with default value, such as 0 or empty string, etc., because we use proto3, which doesn’t support field presence yet."]
            pub fn saved_analysis_query(mut self, value: impl Into<String>) -> Self {
                self.saved_analysis_query = Some(value.into());
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
            ) -> Result<crate::schemas::AnalyzeIamPolicyResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::AnalyzeIamPolicyResponse, crate::Error> {
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.scope;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":analyzeIamPolicy");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                for value in self
                    .analysis_query_access_selector_permissions
                    .iter()
                    .flatten()
                {
                    req = req.query(&[("analysisQuery.accessSelector.permissions", value)]);
                }
                for value in self.analysis_query_access_selector_roles.iter().flatten() {
                    req = req.query(&[("analysisQuery.accessSelector.roles", value)]);
                }
                req = req.query(&[(
                    "analysisQuery.conditionContext.accessTime",
                    &self.analysis_query_condition_context_access_time,
                )]);
                req = req.query(&[(
                    "analysisQuery.identitySelector.identity",
                    &self.analysis_query_identity_selector_identity,
                )]);
                req = req.query(&[(
                    "analysisQuery.options.analyzeServiceAccountImpersonation",
                    &self.analysis_query_options_analyze_service_account_impersonation,
                )]);
                req = req.query(&[(
                    "analysisQuery.options.expandGroups",
                    &self.analysis_query_options_expand_groups,
                )]);
                req = req.query(&[(
                    "analysisQuery.options.expandResources",
                    &self.analysis_query_options_expand_resources,
                )]);
                req = req.query(&[(
                    "analysisQuery.options.expandRoles",
                    &self.analysis_query_options_expand_roles,
                )]);
                req = req.query(&[(
                    "analysisQuery.options.outputGroupEdges",
                    &self.analysis_query_options_output_group_edges,
                )]);
                req = req.query(&[(
                    "analysisQuery.options.outputResourceEdges",
                    &self.analysis_query_options_output_resource_edges,
                )]);
                req = req.query(&[(
                    "analysisQuery.resourceSelector.fullResourceName",
                    &self.analysis_query_resource_selector_full_resource_name,
                )]);
                req = req.query(&[("executionTimeout", &self.execution_timeout)]);
                req = req.query(&[("savedAnalysisQuery", &self.saved_analysis_query)]);
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
        #[doc = "Created via [V1Actions::analyze_iam_policy_longrunning()](struct.V1Actions.html#method.analyze_iam_policy_longrunning)"]
        #[derive(Debug, Clone)]
        pub struct AnalyzeIamPolicyLongrunningRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::AnalyzeIamPolicyLongrunningRequest,
            scope: String,
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
        impl<'a> AnalyzeIamPolicyLongrunningRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Operation, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Operation, crate::Error> {
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.scope;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":analyzeIamPolicyLongrunning");
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
        #[doc = "Created via [V1Actions::analyze_move()](struct.V1Actions.html#method.analyze_move)"]
        #[derive(Debug, Clone)]
        pub struct AnalyzeMoveRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            resource: String,
            destination_parent: ::std::option::Option<String>,
            view: ::std::option::Option<crate::resources::v_1::params::AnalyzeMoveView>,
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
        impl<'a> AnalyzeMoveRequestBuilder<'a> {
            #[doc = "Required. Name of the Google Cloud folder or organization to reparent the target resource. The analysis will be performed against hypothetically moving the resource to this specified desitination parent. This can only be a folder number (such as “folders/123”) or an organization number (such as “organizations/123”)."]
            pub fn destination_parent(mut self, value: impl Into<String>) -> Self {
                self.destination_parent = Some(value.into());
                self
            }
            #[doc = "Analysis view indicating what information should be included in the analysis response. If unspecified, the default view is FULL."]
            pub fn view(mut self, value: crate::resources::v_1::params::AnalyzeMoveView) -> Self {
                self.view = Some(value);
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
            ) -> Result<crate::schemas::AnalyzeMoveResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::AnalyzeMoveResponse, crate::Error> {
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.resource;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":analyzeMove");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("destinationParent", &self.destination_parent)]);
                req = req.query(&[("view", &self.view)]);
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
        #[doc = "Created via [V1Actions::analyze_org_policies()](struct.V1Actions.html#method.analyze_org_policies)"]
        #[derive(Debug, Clone)]
        pub struct AnalyzeOrgPoliciesRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            scope: String,
            constraint: ::std::option::Option<String>,
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
        impl<'a> AnalyzeOrgPoliciesRequestBuilder<'a> {
            #[doc = "Required. The name of the constraint to analyze organization policies for. The response only contains analyzed organization policies for the provided constraint."]
            pub fn constraint(mut self, value: impl Into<String>) -> Self {
                self.constraint = Some(value.into());
                self
            }
            #[doc = "The expression to filter AnalyzeOrgPoliciesResponse.org_policy_results. The only supported field is `consolidated_policy.attached_resource`, and the only supported operator is `=`. Example: consolidated_policy.attached_resource=“//cloudresourcemanager.googleapis.com/folders/001” will return the org policy results of“folders/001“."]
            pub fn filter(mut self, value: impl Into<String>) -> Self {
                self.filter = Some(value.into());
                self
            }
            #[doc = "The maximum number of items to return per page. If unspecified, AnalyzeOrgPoliciesResponse.org_policy_results will contain 20 items with a maximum of 200."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "The pagination token to retrieve the next page."]
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
            #[doc = "\nExecute the request and yield each item in the `orgPolicyResults` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
            pub fn stream_org_policy_results<T>(
                self,
            ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector + 'a,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.stream_org_policy_results_with_fields(fields)
            }
            #[doc = "\nExecute the request and yield each item in the `orgPolicyResults` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
            pub fn stream_org_policy_results_with_default_fields(
                self,
            ) -> impl ::futures::Stream<Item = Result<crate::schemas::OrgPolicyResult, crate::Error>> + 'a
            {
                self.stream_org_policy_results_with_fields(None::<String>)
            }
            #[doc = "\nExecute the request and yield each item in the `orgPolicyResults` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
            pub fn stream_org_policy_results_with_all_fields(
                self,
            ) -> impl ::futures::Stream<Item = Result<crate::schemas::OrgPolicyResult, crate::Error>> + 'a
            {
                self.stream_org_policy_results_with_fields(Some("*"))
            }
            #[doc = "\nExecute the request and yield each item in the `orgPolicyResults` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
            pub fn stream_org_policy_results_with_fields<T, F>(
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
                    #[serde(rename = "orgPolicyResults")]
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
                    let mut selector = concat!("nextPageToken,", "orgPolicyResults").to_owned();
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
            pub fn stream<T>(self) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
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
                Item = Result<crate::schemas::AnalyzeOrgPoliciesResponse, crate::Error>,
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
                Item = Result<crate::schemas::AnalyzeOrgPoliciesResponse, crate::Error>,
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
                let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
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
            ) -> Result<crate::schemas::AnalyzeOrgPoliciesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::AnalyzeOrgPoliciesResponse, crate::Error> {
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.scope;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":analyzeOrgPolicies");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("constraint", &self.constraint)]);
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
        impl<'a> crate::stream::StreamableMethod for AnalyzeOrgPoliciesRequestBuilder<'a> {
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
        #[doc = "Created via [V1Actions::analyze_org_policy_governed_assets()](struct.V1Actions.html#method.analyze_org_policy_governed_assets)"]
        #[derive(Debug, Clone)]
        pub struct AnalyzeOrgPolicyGovernedAssetsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            scope: String,
            constraint: ::std::option::Option<String>,
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
        impl<'a> AnalyzeOrgPolicyGovernedAssetsRequestBuilder<'a> {
            #[doc = "Required. The name of the constraint to analyze governed assets for. The analysis only contains analyzed organization policies for the provided constraint."]
            pub fn constraint(mut self, value: impl Into<String>) -> Self {
                self.constraint = Some(value.into());
                self
            }
            #[doc = "The expression to filter the governed assets in result. The only supported fields for governed resources are `governed_resource.project` and `governed_resource.folders`. The only supported fields for governed iam policies are `governed_iam_policy.project` and `governed_iam_policy.folders`. The only supported operator is `=`. Example 1: governed_resource.project=“projects/12345678” filter will return all governed resources under projects/12345678 including the project ifself, if applicable. Example 2: governed_iam_policy.folders=“folders/12345678” filter will return all governed iam policies under folders/12345678, if applicable."]
            pub fn filter(mut self, value: impl Into<String>) -> Self {
                self.filter = Some(value.into());
                self
            }
            #[doc = "The maximum number of items to return per page. If unspecified, AnalyzeOrgPolicyGovernedAssetsResponse.governed_assets will contain 100 items with a maximum of 200."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "The pagination token to retrieve the next page."]
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
            #[doc = "\nExecute the request and yield each item in the `governedAssets` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
            pub fn stream_governed_assets<T>(
                self,
            ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector + 'a,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.stream_governed_assets_with_fields(fields)
            }
            #[doc = "\nExecute the request and yield each item in the `governedAssets` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]            pub fn stream_governed_assets_with_default_fields (self) -> impl :: futures :: Stream < Item = Result < crate :: schemas :: GoogleCloudAssetV1AnalyzeOrgPolicyGovernedAssetsResponseGovernedAsset , crate :: Error >> + 'a{
                self.stream_governed_assets_with_fields(None::<String>)
            }
            #[doc = "\nExecute the request and yield each item in the `governedAssets` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]            pub fn stream_governed_assets_with_all_fields (self) -> impl :: futures :: Stream < Item = Result < crate :: schemas :: GoogleCloudAssetV1AnalyzeOrgPolicyGovernedAssetsResponseGovernedAsset , crate :: Error >> + 'a{
                self.stream_governed_assets_with_fields(Some("*"))
            }
            #[doc = "\nExecute the request and yield each item in the `governedAssets` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
            pub fn stream_governed_assets_with_fields<T, F>(
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
                    #[serde(rename = "governedAssets")]
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
                    let mut selector = concat!("nextPageToken,", "governedAssets").to_owned();
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
            pub fn stream<T>(self) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
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
                Item = Result<crate::schemas::AnalyzeOrgPolicyGovernedAssetsResponse, crate::Error>,
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
                Item = Result<crate::schemas::AnalyzeOrgPolicyGovernedAssetsResponse, crate::Error>,
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
                let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
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
            ) -> Result<crate::schemas::AnalyzeOrgPolicyGovernedAssetsResponse, crate::Error>
            {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::AnalyzeOrgPolicyGovernedAssetsResponse, crate::Error>
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.scope;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":analyzeOrgPolicyGovernedAssets");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("constraint", &self.constraint)]);
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
        impl<'a> crate::stream::StreamableMethod for AnalyzeOrgPolicyGovernedAssetsRequestBuilder<'a> {
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
        #[doc = "Created via [V1Actions::analyze_org_policy_governed_containers()](struct.V1Actions.html#method.analyze_org_policy_governed_containers)"]
        #[derive(Debug, Clone)]
        pub struct AnalyzeOrgPolicyGovernedContainersRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            scope: String,
            constraint: ::std::option::Option<String>,
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
        impl<'a> AnalyzeOrgPolicyGovernedContainersRequestBuilder<'a> {
            #[doc = "Required. The name of the constraint to analyze governed containers for. The analysis only contains organization policies for the provided constraint."]
            pub fn constraint(mut self, value: impl Into<String>) -> Self {
                self.constraint = Some(value.into());
                self
            }
            #[doc = "The expression to filter the governed containers in result. The only supported field is `parent`, and the only supported operator is `=`. Example: parent=“//cloudresourcemanager.googleapis.com/folders/001” will return all containers under “folders/001”."]
            pub fn filter(mut self, value: impl Into<String>) -> Self {
                self.filter = Some(value.into());
                self
            }
            #[doc = "The maximum number of items to return per page. If unspecified, AnalyzeOrgPolicyGovernedContainersResponse.governed_containers will contain 100 items with a maximum of 200."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "The pagination token to retrieve the next page."]
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
            #[doc = "\nExecute the request and yield each item in the `governedContainers` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
            pub fn stream_governed_containers<T>(
                self,
            ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector + 'a,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.stream_governed_containers_with_fields(fields)
            }
            #[doc = "\nExecute the request and yield each item in the `governedContainers` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
            pub fn stream_governed_containers_with_default_fields(
                self,
            ) -> impl ::futures::Stream<
                Item = Result<crate::schemas::GoogleCloudAssetV1GovernedContainer, crate::Error>,
            > + 'a {
                self.stream_governed_containers_with_fields(None::<String>)
            }
            #[doc = "\nExecute the request and yield each item in the `governedContainers` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
            pub fn stream_governed_containers_with_all_fields(
                self,
            ) -> impl ::futures::Stream<
                Item = Result<crate::schemas::GoogleCloudAssetV1GovernedContainer, crate::Error>,
            > + 'a {
                self.stream_governed_containers_with_fields(Some("*"))
            }
            #[doc = "\nExecute the request and yield each item in the `governedContainers` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
            pub fn stream_governed_containers_with_fields<T, F>(
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
                    #[serde(rename = "governedContainers")]
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
                    let mut selector = concat!("nextPageToken,", "governedContainers").to_owned();
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
            pub fn stream<T>(self) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
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
                    crate::schemas::AnalyzeOrgPolicyGovernedContainersResponse,
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
                    crate::schemas::AnalyzeOrgPolicyGovernedContainersResponse,
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
                let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
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
            ) -> Result<crate::schemas::AnalyzeOrgPolicyGovernedContainersResponse, crate::Error>
            {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::AnalyzeOrgPolicyGovernedContainersResponse, crate::Error>
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.scope;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":analyzeOrgPolicyGovernedContainers");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("constraint", &self.constraint)]);
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
        impl<'a> crate::stream::StreamableMethod for AnalyzeOrgPolicyGovernedContainersRequestBuilder<'a> {
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
        #[doc = "Created via [V1Actions::batch_get_assets_history()](struct.V1Actions.html#method.batch_get_assets_history)"]
        #[derive(Debug, Clone)]
        pub struct BatchGetAssetsHistoryRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            parent: String,
            asset_names: ::std::option::Option<Vec<String>>,
            content_type: ::std::option::Option<
                crate::resources::v_1::params::BatchGetAssetsHistoryContentType,
            >,
            read_time_window_end_time: ::std::option::Option<String>,
            read_time_window_start_time: ::std::option::Option<String>,
            relationship_types: ::std::option::Option<Vec<String>>,
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
        impl<'a> BatchGetAssetsHistoryRequestBuilder<'a> {
            #[doc = "A list of the full names of the assets. See: https://cloud.google.com/asset-inventory/docs/resource-name-format Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`. The request becomes a no-op if the asset name list is empty, and the max size of the asset name list is 100 in one request."]
            pub fn asset_names(mut self, value: impl Into<Vec<String>>) -> Self {
                self.asset_names = Some(value.into());
                self
            }
            #[doc = "Optional. The content type."]
            pub fn content_type(
                mut self,
                value: crate::resources::v_1::params::BatchGetAssetsHistoryContentType,
            ) -> Self {
                self.content_type = Some(value);
                self
            }
            #[doc = "End time of the time window (inclusive). If not specified, the current timestamp is used instead."]
            pub fn read_time_window_end_time(mut self, value: impl Into<String>) -> Self {
                self.read_time_window_end_time = Some(value.into());
                self
            }
            #[doc = "Start time of the time window (exclusive)."]
            pub fn read_time_window_start_time(mut self, value: impl Into<String>) -> Self {
                self.read_time_window_start_time = Some(value.into());
                self
            }
            #[doc = "Optional. A list of relationship types to output, for example: `INSTANCE_TO_INSTANCEGROUP`. This field should only be specified if content_type=RELATIONSHIP. * If specified: it outputs specified relationships’ history on the \\[asset_names\\]. It returns an error if any of the \\[relationship_types\\] doesn’t belong to the supported relationship types of the \\[asset_names\\] or if any of the \\[asset_names\\]’s types doesn’t belong to the source types of the \\[relationship_types\\]. * Otherwise: it outputs the supported relationships’ history on the \\[asset_names\\] or returns an error if any of the \\[asset_names\\]’s types has no relationship support. See [Introduction to Cloud Asset Inventory](https://cloud.google.com/asset-inventory/docs/overview) for all supported asset types and relationship types."]
            pub fn relationship_types(mut self, value: impl Into<Vec<String>>) -> Self {
                self.relationship_types = Some(value.into());
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
            ) -> Result<crate::schemas::BatchGetAssetsHistoryResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::BatchGetAssetsHistoryResponse, crate::Error> {
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":batchGetAssetsHistory");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                for value in self.asset_names.iter().flatten() {
                    req = req.query(&[("assetNames", value)]);
                }
                req = req.query(&[("contentType", &self.content_type)]);
                req = req.query(&[("readTimeWindow.endTime", &self.read_time_window_end_time)]);
                req = req.query(&[(
                    "readTimeWindow.startTime",
                    &self.read_time_window_start_time,
                )]);
                for value in self.relationship_types.iter().flatten() {
                    req = req.query(&[("relationshipTypes", value)]);
                }
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
        #[doc = "Created via [V1Actions::export_assets()](struct.V1Actions.html#method.export_assets)"]
        #[derive(Debug, Clone)]
        pub struct ExportAssetsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::ExportAssetsRequest,
            parent: String,
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
        impl<'a> ExportAssetsRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Operation, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Operation, crate::Error> {
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":exportAssets");
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
        #[doc = "Created via [V1Actions::query_assets()](struct.V1Actions.html#method.query_assets)"]
        #[derive(Debug, Clone)]
        pub struct QueryAssetsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::QueryAssetsRequest,
            parent: String,
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
        impl<'a> QueryAssetsRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::QueryAssetsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::QueryAssetsResponse, crate::Error> {
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":queryAssets");
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
        #[doc = "Created via [V1Actions::search_all_iam_policies()](struct.V1Actions.html#method.search_all_iam_policies)"]
        #[derive(Debug, Clone)]
        pub struct SearchAllIamPoliciesRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            scope: String,
            asset_types: ::std::option::Option<Vec<String>>,
            order_by: ::std::option::Option<String>,
            page_size: ::std::option::Option<i32>,
            page_token: ::std::option::Option<String>,
            query: ::std::option::Option<String>,
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
        impl<'a> SearchAllIamPoliciesRequestBuilder<'a> {
            #[doc = "Optional. A list of asset types that the IAM policies are attached to. If empty, it will search the IAM policies that are attached to all the [searchable asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types#searchable_asset_types). Regular expressions are also supported. For example: * “compute.googleapis.com.\\*” snapshots IAM policies attached to asset type starts with “compute.googleapis.com”. * “.\\*Instance” snapshots IAM policies attached to asset type ends with “Instance”. * “.*Instance.*” snapshots IAM policies attached to asset type contains “Instance”. See [RE2](https://github.com/google/re2/wiki/Syntax) for all supported regular expression syntax. If the regular expression does not match any supported asset type, an INVALID_ARGUMENT error will be returned."]
            pub fn asset_types(mut self, value: impl Into<Vec<String>>) -> Self {
                self.asset_types = Some(value.into());
                self
            }
            #[doc = "Optional. A comma-separated list of fields specifying the sorting order of the results. The default order is ascending. Add “ DESC“ after the field name to indicate descending order. Redundant space characters are ignored. Example: “assetType DESC, resource”. Only singular primitive fields in the response are sortable: * resource * assetType * project All the other fields such as repeated fields (e.g., `folders`) and non-primitive fields (e.g., `policy`) are not supported."]
            pub fn order_by(mut self, value: impl Into<String>) -> Self {
                self.order_by = Some(value.into());
                self
            }
            #[doc = "Optional. The page size for search result pagination. Page size is capped at 500 even if a larger value is given. If set to zero, server will pick an appropriate default. Returned results may be fewer than requested. When this happens, there could be more results as long as `next_page_token` is returned."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "Optional. If present, retrieve the next batch of results from the preceding call to this method. `page_token` must be the value of `next_page_token` from the previous response. The values of all other method parameters must be identical to those in the previous call."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "Optional. The query statement. See [how to construct a query](https://cloud.google.com/asset-inventory/docs/searching-iam-policies#how_to_construct_a_query) for more information. If not specified or empty, it will search all the IAM policies within the specified `scope`. Note that the query string is compared against each IAM policy binding, including its principals, roles, and IAM conditions. The returned IAM policies will only contain the bindings that match your query. To learn more about the IAM policy structure, see the [IAM policy documentation](https://cloud.google.com/iam/help/allow-policies/structure). Examples: * `policy:amy@gmail.com` to find IAM policy bindings that specify user “amy@gmail.com”. * `policy:roles/compute.admin` to find IAM policy bindings that specify the Compute Admin role. * `policy:comp*` to find IAM policy bindings that contain “comp” as a prefix of any word in the binding. * `policy.role.permissions:storage.buckets.update` to find IAM policy bindings that specify a role containing “storage.buckets.update” permission. Note that if callers don’t have `iam.roles.get` access to a role’s included permissions, policy bindings that specify this role will be dropped from the search results. * `policy.role.permissions:upd*` to find IAM policy bindings that specify a role containing “upd” as a prefix of any word in the role permission. Note that if callers don’t have `iam.roles.get` access to a role’s included permissions, policy bindings that specify this role will be dropped from the search results. * `resource:organizations/123456` to find IAM policy bindings that are set on “organizations/123456”. * `resource=//cloudresourcemanager.googleapis.com/projects/myproject` to find IAM policy bindings that are set on the project named “myproject”. * `Important` to find IAM policy bindings that contain “Important” as a word in any of the searchable fields (except for the included permissions). * `resource:(instance1 OR instance2) policy:amy` to find IAM policy bindings that are set on resources “instance1” or “instance2” and also specify user “amy”. * `roles:roles/compute.admin` to find IAM policy bindings that specify the Compute Admin role. * `memberTypes:user` to find IAM policy bindings that contain the principal type “user”."]
            pub fn query(mut self, value: impl Into<String>) -> Self {
                self.query = Some(value.into());
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
            #[doc = "\nExecute the request and yield each item in the `results` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
            pub fn stream_results<T>(
                self,
            ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector + 'a,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.stream_results_with_fields(fields)
            }
            #[doc = "\nExecute the request and yield each item in the `results` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
            pub fn stream_results_with_default_fields(
                self,
            ) -> impl ::futures::Stream<
                Item = Result<crate::schemas::IamPolicySearchResult, crate::Error>,
            > + 'a {
                self.stream_results_with_fields(None::<String>)
            }
            #[doc = "\nExecute the request and yield each item in the `results` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
            pub fn stream_results_with_all_fields(
                self,
            ) -> impl ::futures::Stream<
                Item = Result<crate::schemas::IamPolicySearchResult, crate::Error>,
            > + 'a {
                self.stream_results_with_fields(Some("*"))
            }
            #[doc = "\nExecute the request and yield each item in the `results` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
            pub fn stream_results_with_fields<T, F>(
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
                    #[serde(rename = "results")]
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
                    let mut selector = concat!("nextPageToken,", "results").to_owned();
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
            pub fn stream<T>(self) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
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
                Item = Result<crate::schemas::SearchAllIamPoliciesResponse, crate::Error>,
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
                Item = Result<crate::schemas::SearchAllIamPoliciesResponse, crate::Error>,
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
                let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
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
            ) -> Result<crate::schemas::SearchAllIamPoliciesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::SearchAllIamPoliciesResponse, crate::Error> {
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.scope;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":searchAllIamPolicies");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                for value in self.asset_types.iter().flatten() {
                    req = req.query(&[("assetTypes", value)]);
                }
                req = req.query(&[("orderBy", &self.order_by)]);
                req = req.query(&[("pageSize", &self.page_size)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("query", &self.query)]);
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
        impl<'a> crate::stream::StreamableMethod for SearchAllIamPoliciesRequestBuilder<'a> {
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
        #[doc = "Created via [V1Actions::search_all_resources()](struct.V1Actions.html#method.search_all_resources)"]
        #[derive(Debug, Clone)]
        pub struct SearchAllResourcesRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            scope: String,
            asset_types: ::std::option::Option<Vec<String>>,
            order_by: ::std::option::Option<String>,
            page_size: ::std::option::Option<i32>,
            page_token: ::std::option::Option<String>,
            query: ::std::option::Option<String>,
            read_mask: ::std::option::Option<String>,
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
        impl<'a> SearchAllResourcesRequestBuilder<'a> {
            #[doc = "Optional. A list of asset types that this request searches for. If empty, it will search all the [searchable asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types#searchable_asset_types). Regular expressions are also supported. For example: * “compute.googleapis.com.\\*” snapshots resources whose asset type starts with “compute.googleapis.com”. * “.\\*Instance” snapshots resources whose asset type ends with “Instance”. * “.*Instance.*” snapshots resources whose asset type contains “Instance”. See [RE2](https://github.com/google/re2/wiki/Syntax) for all supported regular expression syntax. If the regular expression does not match any supported asset type, an INVALID_ARGUMENT error will be returned."]
            pub fn asset_types(mut self, value: impl Into<Vec<String>>) -> Self {
                self.asset_types = Some(value.into());
                self
            }
            #[doc = "Optional. A comma-separated list of fields specifying the sorting order of the results. The default order is ascending. Add “ DESC“ after the field name to indicate descending order. Redundant space characters are ignored. Example: “location DESC, name”. Only singular primitive fields in the response are sortable: * name * assetType * project * displayName * description * location * createTime * updateTime * state * parentFullResourceName * parentAssetType All the other fields such as repeated fields (e.g., `networkTags`, `kmsKeys`), map fields (e.g., `labels`) and struct fields (e.g., `additionalAttributes`) are not supported."]
            pub fn order_by(mut self, value: impl Into<String>) -> Self {
                self.order_by = Some(value.into());
                self
            }
            #[doc = "Optional. The page size for search result pagination. Page size is capped at 500 even if a larger value is given. If set to zero, server will pick an appropriate default. Returned results may be fewer than requested. When this happens, there could be more results as long as `next_page_token` is returned."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "Optional. If present, then retrieve the next batch of results from the preceding call to this method. `page_token` must be the value of `next_page_token` from the previous response. The values of all other method parameters, must be identical to those in the previous call."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "Optional. The query statement. See [how to construct a query](https://cloud.google.com/asset-inventory/docs/searching-resources#how_to_construct_a_query) for more information. If not specified or empty, it will search all the resources within the specified `scope`. Examples: * `name:Important` to find Google Cloud resources whose name contains “Important” as a word. * `name=Important` to find the Google Cloud resource whose name is exactly “Important”. * `displayName:Impor*` to find Google Cloud resources whose display name contains “Impor” as a prefix of any word in the field. * `location:us-west*` to find Google Cloud resources whose location contains both “us” and “west” as prefixes. * `labels:prod` to find Google Cloud resources whose labels contain “prod” as a key or value. * `labels.env:prod` to find Google Cloud resources that have a label “env” and its value is “prod”. * `labels.env:*` to find Google Cloud resources that have a label “env”. * `kmsKey:key` to find Google Cloud resources encrypted with a customer-managed encryption key whose name contains “key” as a word. This field is deprecated. Please use the `kmsKeys` field to retrieve Cloud KMS key information. * `kmsKeys:key` to find Google Cloud resources encrypted with customer-managed encryption keys whose name contains the word “key”. * `relationships:instance-group-1` to find Google Cloud resources that have relationships with “instance-group-1” in the related resource name. * `relationships:INSTANCE_TO_INSTANCEGROUP` to find Compute Engine instances that have relationships of type “INSTANCE_TO_INSTANCEGROUP”. * `relationships.INSTANCE_TO_INSTANCEGROUP:instance-group-1` to find Compute Engine instances that have relationships with “instance-group-1” in the Compute Engine instance group resource name, for relationship type “INSTANCE_TO_INSTANCEGROUP”. * `state:ACTIVE` to find Google Cloud resources whose state contains “ACTIVE” as a word. * `NOT state:ACTIVE` to find Google Cloud resources whose state doesn’t contain “ACTIVE” as a word. * `createTime<1609459200` to find Google Cloud resources that were created before “2021-01-01 00:00:00 UTC”. 1609459200 is the epoch timestamp of “2021-01-01 00:00:00 UTC” in seconds. * `updateTime>1609459200` to find Google Cloud resources that were updated after “2021-01-01 00:00:00 UTC”. 1609459200 is the epoch timestamp of “2021-01-01 00:00:00 UTC” in seconds. * `Important` to find Google Cloud resources that contain “Important” as a word in any of the searchable fields. * `Impor*` to find Google Cloud resources that contain “Impor” as a prefix of any word in any of the searchable fields. * `Important location:(us-west1 OR global)` to find Google Cloud resources that contain “Important” as a word in any of the searchable fields and are also located in the “us-west1” region or the “global” location."]
            pub fn query(mut self, value: impl Into<String>) -> Self {
                self.query = Some(value.into());
                self
            }
            #[doc = "Optional. A comma-separated list of fields specifying which fields to be returned in ResourceSearchResult. Only ‘*’ or combination of top level fields can be specified. Field names of both snake_case and camelCase are supported. Examples: `\"*\"`, `\"name,location\"`, `\"name,versionedResources\"`. The read_mask paths must be valid field paths listed but not limited to (both snake_case and camelCase are supported): * name * assetType * project * displayName * description * location * tagKeys * tagValues * tagValueIds * labels * networkTags * kmsKey (This field is deprecated. Please use the `kmsKeys` field to retrieve Cloud KMS key information.) * kmsKeys * createTime * updateTime * state * additionalAttributes * versionedResources If read_mask is not specified, all fields except versionedResources will be returned. If only ‘*’ is specified, all fields including versionedResources will be returned. Any invalid field path will trigger INVALID_ARGUMENT error."]
            pub fn read_mask(mut self, value: impl Into<String>) -> Self {
                self.read_mask = Some(value.into());
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
            #[doc = "\nExecute the request and yield each item in the `results` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
            pub fn stream_results<T>(
                self,
            ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector + 'a,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.stream_results_with_fields(fields)
            }
            #[doc = "\nExecute the request and yield each item in the `results` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
            pub fn stream_results_with_default_fields(
                self,
            ) -> impl ::futures::Stream<
                Item = Result<crate::schemas::ResourceSearchResult, crate::Error>,
            > + 'a {
                self.stream_results_with_fields(None::<String>)
            }
            #[doc = "\nExecute the request and yield each item in the `results` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
            pub fn stream_results_with_all_fields(
                self,
            ) -> impl ::futures::Stream<
                Item = Result<crate::schemas::ResourceSearchResult, crate::Error>,
            > + 'a {
                self.stream_results_with_fields(Some("*"))
            }
            #[doc = "\nExecute the request and yield each item in the `results` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
            pub fn stream_results_with_fields<T, F>(
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
                    #[serde(rename = "results")]
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
                    let mut selector = concat!("nextPageToken,", "results").to_owned();
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
            pub fn stream<T>(self) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
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
                Item = Result<crate::schemas::SearchAllResourcesResponse, crate::Error>,
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
                Item = Result<crate::schemas::SearchAllResourcesResponse, crate::Error>,
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
                let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
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
            ) -> Result<crate::schemas::SearchAllResourcesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::SearchAllResourcesResponse, crate::Error> {
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
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.scope;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":searchAllResources");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                for value in self.asset_types.iter().flatten() {
                    req = req.query(&[("assetTypes", value)]);
                }
                req = req.query(&[("orderBy", &self.order_by)]);
                req = req.query(&[("pageSize", &self.page_size)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("query", &self.query)]);
                req = req.query(&[("readMask", &self.read_mask)]);
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
        impl<'a> crate::stream::StreamableMethod for SearchAllResourcesRequestBuilder<'a> {
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
