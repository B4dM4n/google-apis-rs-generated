#![allow(rustdoc::bare_urls)]
#![doc = "# Resources and Methods\n* [iam](resources/iam/struct.IamActions.html)\n  * [*troubleshoot*](resources/iam/struct.TroubleshootRequestBuilder.html)\n"]
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
    pub struct GoogleCloudPolicytroubleshooterV1AccessTuple {
        #[doc = "Required. The full resource name that identifies the resource. For example, `//compute.googleapis.com/projects/my-project/zones/us-central1-a/instances/my-instance`. For examples of full resource names for Google Cloud services, see https://cloud.google.com/iam/help/troubleshooter/full-resource-names."]
        #[serde(
            rename = "fullResourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub full_resource_name: ::std::option::Option<String>,
        #[doc = "Required. The IAM permission to check for the specified principal and resource. For a complete list of IAM permissions, see https://cloud.google.com/iam/help/permissions/reference. For a complete list of predefined IAM roles and the permissions in each role, see https://cloud.google.com/iam/help/roles/reference."]
        #[serde(
            rename = "permission",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permission: ::std::option::Option<String>,
        #[doc = "Required. The principal whose access you want to check, in the form of the email address that represents that principal. For example, `alice@example.com` or `my-service-account@my-project.iam.gserviceaccount.com`. The principal must be a Google Account or a service account. Other types of principals are not supported."]
        #[serde(
            rename = "principal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub principal: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudPolicytroubleshooterV1AccessTuple {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudPolicytroubleshooterV1AccessTuple {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudPolicytroubleshooterV1BindingExplanation { # [doc = "Required. Indicates whether *this binding* provides the specified permission to the specified principal for the specified resource. This field does *not* indicate whether the principal actually has the permission for the resource. There might be another binding that overrides this binding. To determine whether the principal actually has the permission, use the `access` field in the TroubleshootIamPolicyResponse."] # [serde (rename = "access" , default , skip_serializing_if = "std::option::Option::is_none")] pub access : :: std :: option :: Option < crate :: schemas :: GoogleCloudPolicytroubleshooterV1BindingExplanationAccess > , # [doc = "A condition expression that prevents this binding from granting access unless the expression evaluates to `true`. To learn about IAM Conditions, see https://cloud.google.com/iam/help/conditions/overview."] # [serde (rename = "condition" , default , skip_serializing_if = "std::option::Option::is_none")] pub condition : :: std :: option :: Option < crate :: schemas :: GoogleTypeExpr > , # [doc = "Indicates whether each principal in the binding includes the principal specified in the request, either directly or indirectly. Each key identifies a principal in the binding, and each value indicates whether the principal in the binding includes the principal in the request. For example, suppose that a binding includes the following principals: * `user:alice@example.com` * `group:product-eng@example.com` You want to troubleshoot access for `user:bob@example.com`. This user is a principal of the group `group:product-eng@example.com`. For the first principal in the binding, the key is `user:alice@example.com`, and the `membership` field in the value is set to `MEMBERSHIP_NOT_INCLUDED`. For the second principal in the binding, the key is `group:product-eng@example.com`, and the `membership` field in the value is set to `MEMBERSHIP_INCLUDED`."] # [serde (rename = "memberships" , default , skip_serializing_if = "std::option::Option::is_none")] pub memberships : :: std :: option :: Option < :: std :: collections :: BTreeMap < String , crate :: schemas :: GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembership > > , # [doc = "The relevance of this binding to the overall determination for the entire policy."] # [serde (rename = "relevance" , default , skip_serializing_if = "std::option::Option::is_none")] pub relevance : :: std :: option :: Option < crate :: schemas :: GoogleCloudPolicytroubleshooterV1BindingExplanationRelevance > , # [doc = "The role that this binding grants. For example, `roles/compute.serviceAgent`. For a complete list of predefined IAM roles, as well as the permissions in each role, see https://cloud.google.com/iam/help/roles/reference."] # [serde (rename = "role" , default , skip_serializing_if = "std::option::Option::is_none")] pub role : :: std :: option :: Option < String > , # [doc = "Indicates whether the role granted by this binding contains the specified permission."] # [serde (rename = "rolePermission" , default , skip_serializing_if = "std::option::Option::is_none")] pub role_permission : :: std :: option :: Option < crate :: schemas :: GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermission > , # [doc = "The relevance of the permission’s existence, or nonexistence, in the role to the overall determination for the entire policy."] # [serde (rename = "rolePermissionRelevance" , default , skip_serializing_if = "std::option::Option::is_none")] pub role_permission_relevance : :: std :: option :: Option < crate :: schemas :: GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevance > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1BindingExplanation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudPolicytroubleshooterV1BindingExplanation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPolicytroubleshooterV1BindingExplanationAccess {
        #[doc = "Default value. This value is unused."]
        AccessStateUnspecified,
        #[doc = "The principal has the permission."]
        Granted,
        #[doc = "The principal does not have the permission."]
        NotGranted,
        #[doc = "The principal has the permission only if a condition expression evaluates to `true`."]
        UnknownConditional,
        #[doc = "The sender of the request does not have access to all of the policies that Policy Troubleshooter needs to evaluate."]
        UnknownInfoDenied,
    }
    impl GoogleCloudPolicytroubleshooterV1BindingExplanationAccess {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPolicytroubleshooterV1BindingExplanationAccess :: AccessStateUnspecified => "ACCESS_STATE_UNSPECIFIED" , GoogleCloudPolicytroubleshooterV1BindingExplanationAccess :: Granted => "GRANTED" , GoogleCloudPolicytroubleshooterV1BindingExplanationAccess :: NotGranted => "NOT_GRANTED" , GoogleCloudPolicytroubleshooterV1BindingExplanationAccess :: UnknownConditional => "UNKNOWN_CONDITIONAL" , GoogleCloudPolicytroubleshooterV1BindingExplanationAccess :: UnknownInfoDenied => "UNKNOWN_INFO_DENIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudPolicytroubleshooterV1BindingExplanationAccess {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudPolicytroubleshooterV1BindingExplanationAccess {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudPolicytroubleshooterV1BindingExplanationAccess, ()>
        {
            Ok (match s { "ACCESS_STATE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BindingExplanationAccess :: AccessStateUnspecified , "GRANTED" => GoogleCloudPolicytroubleshooterV1BindingExplanationAccess :: Granted , "NOT_GRANTED" => GoogleCloudPolicytroubleshooterV1BindingExplanationAccess :: NotGranted , "UNKNOWN_CONDITIONAL" => GoogleCloudPolicytroubleshooterV1BindingExplanationAccess :: UnknownConditional , "UNKNOWN_INFO_DENIED" => GoogleCloudPolicytroubleshooterV1BindingExplanationAccess :: UnknownInfoDenied , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudPolicytroubleshooterV1BindingExplanationAccess {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudPolicytroubleshooterV1BindingExplanationAccess {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudPolicytroubleshooterV1BindingExplanationAccess {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ACCESS_STATE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BindingExplanationAccess :: AccessStateUnspecified , "GRANTED" => GoogleCloudPolicytroubleshooterV1BindingExplanationAccess :: Granted , "NOT_GRANTED" => GoogleCloudPolicytroubleshooterV1BindingExplanationAccess :: NotGranted , "UNKNOWN_CONDITIONAL" => GoogleCloudPolicytroubleshooterV1BindingExplanationAccess :: UnknownConditional , "UNKNOWN_INFO_DENIED" => GoogleCloudPolicytroubleshooterV1BindingExplanationAccess :: UnknownInfoDenied , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1BindingExplanationAccess
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1BindingExplanationAccess
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPolicytroubleshooterV1BindingExplanationRelevance {
        #[doc = "Default value. This value is unused."]
        HeuristicRelevanceUnspecified,
        #[doc = "The data point has a strong effect on the result. Changing the data point is likely to affect the overall determination."]
        High,
        #[doc = "The data point has a limited effect on the result. Changing the data point is unlikely to affect the overall determination."]
        Normal,
    }
    impl GoogleCloudPolicytroubleshooterV1BindingExplanationRelevance {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPolicytroubleshooterV1BindingExplanationRelevance :: HeuristicRelevanceUnspecified => "HEURISTIC_RELEVANCE_UNSPECIFIED" , GoogleCloudPolicytroubleshooterV1BindingExplanationRelevance :: High => "HIGH" , GoogleCloudPolicytroubleshooterV1BindingExplanationRelevance :: Normal => "NORMAL" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudPolicytroubleshooterV1BindingExplanationRelevance {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudPolicytroubleshooterV1BindingExplanationRelevance {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudPolicytroubleshooterV1BindingExplanationRelevance, ()>
        {
            Ok (match s { "HEURISTIC_RELEVANCE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BindingExplanationRelevance :: HeuristicRelevanceUnspecified , "HIGH" => GoogleCloudPolicytroubleshooterV1BindingExplanationRelevance :: High , "NORMAL" => GoogleCloudPolicytroubleshooterV1BindingExplanationRelevance :: Normal , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudPolicytroubleshooterV1BindingExplanationRelevance {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudPolicytroubleshooterV1BindingExplanationRelevance {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudPolicytroubleshooterV1BindingExplanationRelevance
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "HEURISTIC_RELEVANCE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BindingExplanationRelevance :: HeuristicRelevanceUnspecified , "HIGH" => GoogleCloudPolicytroubleshooterV1BindingExplanationRelevance :: High , "NORMAL" => GoogleCloudPolicytroubleshooterV1BindingExplanationRelevance :: Normal , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1BindingExplanationRelevance
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1BindingExplanationRelevance
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermission {
        #[doc = "The permission is included in the role."]
        RolePermissionIncluded,
        #[doc = "The permission is not included in the role."]
        RolePermissionNotIncluded,
        #[doc = "The sender of the request is not allowed to access the binding."]
        RolePermissionUnknownInfoDenied,
        #[doc = "Default value. This value is unused."]
        RolePermissionUnspecified,
    }
    impl GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermission {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermission :: RolePermissionIncluded => "ROLE_PERMISSION_INCLUDED" , GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermission :: RolePermissionNotIncluded => "ROLE_PERMISSION_NOT_INCLUDED" , GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermission :: RolePermissionUnknownInfoDenied => "ROLE_PERMISSION_UNKNOWN_INFO_DENIED" , GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermission :: RolePermissionUnspecified => "ROLE_PERMISSION_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermission
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermission {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermission,
            (),
        > {
            Ok (match s { "ROLE_PERMISSION_INCLUDED" => GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermission :: RolePermissionIncluded , "ROLE_PERMISSION_NOT_INCLUDED" => GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermission :: RolePermissionNotIncluded , "ROLE_PERMISSION_UNKNOWN_INFO_DENIED" => GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermission :: RolePermissionUnknownInfoDenied , "ROLE_PERMISSION_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermission :: RolePermissionUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermission {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermission {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermission
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ROLE_PERMISSION_INCLUDED" => GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermission :: RolePermissionIncluded , "ROLE_PERMISSION_NOT_INCLUDED" => GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermission :: RolePermissionNotIncluded , "ROLE_PERMISSION_UNKNOWN_INFO_DENIED" => GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermission :: RolePermissionUnknownInfoDenied , "ROLE_PERMISSION_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermission :: RolePermissionUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermission
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermission
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevance {
        #[doc = "Default value. This value is unused."]
        HeuristicRelevanceUnspecified,
        #[doc = "The data point has a strong effect on the result. Changing the data point is likely to affect the overall determination."]
        High,
        #[doc = "The data point has a limited effect on the result. Changing the data point is unlikely to affect the overall determination."]
        Normal,
    }
    impl GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevance {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevance :: HeuristicRelevanceUnspecified => "HEURISTIC_RELEVANCE_UNSPECIFIED" , GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevance :: High => "HIGH" , GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevance :: Normal => "NORMAL" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevance
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevance
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevance,
            (),
        > {
            Ok (match s { "HEURISTIC_RELEVANCE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevance :: HeuristicRelevanceUnspecified , "HIGH" => GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevance :: High , "NORMAL" => GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevance :: Normal , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevance
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevance
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevance
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "HEURISTIC_RELEVANCE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevance :: HeuristicRelevanceUnspecified , "HIGH" => GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevance :: High , "NORMAL" => GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevance :: Normal , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevance
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1BindingExplanationRolePermissionRelevance
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
    pub struct GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembership { # [doc = "Indicates whether the binding includes the principal."] # [serde (rename = "membership" , default , skip_serializing_if = "std::option::Option::is_none")] pub membership : :: std :: option :: Option < crate :: schemas :: GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership > , # [doc = "The relevance of the principal’s status to the overall determination for the binding."] # [serde (rename = "relevance" , default , skip_serializing_if = "std::option::Option::is_none")] pub relevance : :: std :: option :: Option < crate :: schemas :: GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevance > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembership
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembership
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership {
        #[doc = "The binding includes the principal. The principal can be included directly or indirectly. For example: * A principal is included directly if that principal is listed in the binding. * A principal is included indirectly if that principal is in a Google group or Google Workspace domain that is listed in the binding."]
        MembershipIncluded,
        #[doc = "The binding does not include the principal."]
        MembershipNotIncluded,
        #[doc = "The sender of the request is not allowed to access the binding."]
        MembershipUnknownInfoDenied,
        #[doc = "The principal is an unsupported type. Only Google Accounts and service accounts are supported."]
        MembershipUnknownUnsupported,
        #[doc = "Default value. This value is unused."]
        MembershipUnspecified,
    }
    impl GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership :: MembershipIncluded => "MEMBERSHIP_INCLUDED" , GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership :: MembershipNotIncluded => "MEMBERSHIP_NOT_INCLUDED" , GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership :: MembershipUnknownInfoDenied => "MEMBERSHIP_UNKNOWN_INFO_DENIED" , GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership :: MembershipUnknownUnsupported => "MEMBERSHIP_UNKNOWN_UNSUPPORTED" , GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership :: MembershipUnspecified => "MEMBERSHIP_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership,
            (),
        > {
            Ok (match s { "MEMBERSHIP_INCLUDED" => GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership :: MembershipIncluded , "MEMBERSHIP_NOT_INCLUDED" => GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership :: MembershipNotIncluded , "MEMBERSHIP_UNKNOWN_INFO_DENIED" => GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership :: MembershipUnknownInfoDenied , "MEMBERSHIP_UNKNOWN_UNSUPPORTED" => GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership :: MembershipUnknownUnsupported , "MEMBERSHIP_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership :: MembershipUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "MEMBERSHIP_INCLUDED" => GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership :: MembershipIncluded , "MEMBERSHIP_NOT_INCLUDED" => GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership :: MembershipNotIncluded , "MEMBERSHIP_UNKNOWN_INFO_DENIED" => GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership :: MembershipUnknownInfoDenied , "MEMBERSHIP_UNKNOWN_UNSUPPORTED" => GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership :: MembershipUnknownUnsupported , "MEMBERSHIP_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership :: MembershipUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipMembership
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevance {
        #[doc = "Default value. This value is unused."]
        HeuristicRelevanceUnspecified,
        #[doc = "The data point has a strong effect on the result. Changing the data point is likely to affect the overall determination."]
        High,
        #[doc = "The data point has a limited effect on the result. Changing the data point is unlikely to affect the overall determination."]
        Normal,
    }
    impl GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevance {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevance :: HeuristicRelevanceUnspecified => "HEURISTIC_RELEVANCE_UNSPECIFIED" , GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevance :: High => "HIGH" , GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevance :: Normal => "NORMAL" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevance
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevance
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevance,
            (),
        > {
            Ok (match s { "HEURISTIC_RELEVANCE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevance :: HeuristicRelevanceUnspecified , "HIGH" => GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevance :: High , "NORMAL" => GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevance :: Normal , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevance
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevance
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevance
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "HEURISTIC_RELEVANCE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevance :: HeuristicRelevanceUnspecified , "HIGH" => GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevance :: High , "NORMAL" => GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevance :: Normal , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevance
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1BindingExplanationAnnotatedMembershipRelevance
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
    pub struct GoogleCloudPolicytroubleshooterV1ExplainedPolicy {
        #[doc = "Indicates whether *this policy* provides the specified permission to the specified principal for the specified resource. This field does *not* indicate whether the principal actually has the permission for the resource. There might be another policy that overrides this policy. To determine whether the principal actually has the permission, use the `access` field in the TroubleshootIamPolicyResponse."]
        #[serde(
            rename = "access",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access: ::std::option::Option<
            crate::schemas::GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess,
        >,
        #[doc = "Details about how each binding in the policy affects the principal’s ability, or inability, to use the permission for the resource. If the sender of the request does not have access to the policy, this field is omitted."]
        #[serde(
            rename = "bindingExplanations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub binding_explanations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudPolicytroubleshooterV1BindingExplanation>,
        >,
        #[doc = "The full resource name that identifies the resource. For example, `//compute.googleapis.com/projects/my-project/zones/us-central1-a/instances/my-instance`. If the sender of the request does not have access to the policy, this field is omitted. For examples of full resource names for Google Cloud services, see https://cloud.google.com/iam/help/troubleshooter/full-resource-names."]
        #[serde(
            rename = "fullResourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub full_resource_name: ::std::option::Option<String>,
        #[doc = "The IAM policy attached to the resource. If the sender of the request does not have access to the policy, this field is empty."]
        #[serde(
            rename = "policy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy: ::std::option::Option<crate::schemas::GoogleIamV1Policy>,
        #[doc = "The relevance of this policy to the overall determination in the TroubleshootIamPolicyResponse. If the sender of the request does not have access to the policy, this field is omitted."]
        #[serde(
            rename = "relevance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relevance: ::std::option::Option<
            crate::schemas::GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevance,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudPolicytroubleshooterV1ExplainedPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudPolicytroubleshooterV1ExplainedPolicy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess {
        #[doc = "Default value. This value is unused."]
        AccessStateUnspecified,
        #[doc = "The principal has the permission."]
        Granted,
        #[doc = "The principal does not have the permission."]
        NotGranted,
        #[doc = "The principal has the permission only if a condition expression evaluates to `true`."]
        UnknownConditional,
        #[doc = "The sender of the request does not have access to all of the policies that Policy Troubleshooter needs to evaluate."]
        UnknownInfoDenied,
    }
    impl GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess::AccessStateUnspecified => {
                    "ACCESS_STATE_UNSPECIFIED"
                }
                GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess::Granted => "GRANTED",
                GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess::NotGranted => "NOT_GRANTED",
                GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess::UnknownConditional => {
                    "UNKNOWN_CONDITIONAL"
                }
                GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess::UnknownInfoDenied => {
                    "UNKNOWN_INFO_DENIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess, ()>
        {
            Ok(match s {
                "ACCESS_STATE_UNSPECIFIED" => {
                    GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess::AccessStateUnspecified
                }
                "GRANTED" => GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess::Granted,
                "NOT_GRANTED" => GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess::NotGranted,
                "UNKNOWN_CONDITIONAL" => {
                    GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess::UnknownConditional
                }
                "UNKNOWN_INFO_DENIED" => {
                    GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess::UnknownInfoDenied
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCESS_STATE_UNSPECIFIED" => {
                    GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess::AccessStateUnspecified
                }
                "GRANTED" => GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess::Granted,
                "NOT_GRANTED" => GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess::NotGranted,
                "UNKNOWN_CONDITIONAL" => {
                    GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess::UnknownConditional
                }
                "UNKNOWN_INFO_DENIED" => {
                    GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess::UnknownInfoDenied
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
        for GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1ExplainedPolicyAccess
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevance {
        #[doc = "Default value. This value is unused."]
        HeuristicRelevanceUnspecified,
        #[doc = "The data point has a strong effect on the result. Changing the data point is likely to affect the overall determination."]
        High,
        #[doc = "The data point has a limited effect on the result. Changing the data point is unlikely to affect the overall determination."]
        Normal,
    }
    impl GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevance {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevance :: HeuristicRelevanceUnspecified => "HEURISTIC_RELEVANCE_UNSPECIFIED" , GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevance :: High => "HIGH" , GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevance :: Normal => "NORMAL" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevance {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevance {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevance, ()>
        {
            Ok (match s { "HEURISTIC_RELEVANCE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevance :: HeuristicRelevanceUnspecified , "HIGH" => GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevance :: High , "NORMAL" => GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevance :: Normal , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevance {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevance {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevance {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "HEURISTIC_RELEVANCE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevance :: HeuristicRelevanceUnspecified , "HIGH" => GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevance :: High , "NORMAL" => GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevance :: Normal , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevance
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1ExplainedPolicyRelevance
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
    pub struct GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyRequest {
        #[doc = "The information to use for checking whether a principal has a permission for a resource."]
        #[serde(
            rename = "accessTuple",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access_tuple:
            ::std::option::Option<crate::schemas::GoogleCloudPolicytroubleshooterV1AccessTuple>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyRequest
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponse {
        #[doc = "Indicates whether the principal has the specified permission for the specified resource, based on evaluating all of the applicable IAM policies."]
        #[serde(
            rename = "access",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access: ::std::option::Option<
            crate::schemas::GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess,
        >,
        #[doc = "The general errors contained in the troubleshooting response."]
        #[serde(
            rename = "errors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub errors: ::std::option::Option<Vec<crate::schemas::GoogleRpcStatus>>,
        #[doc = "List of IAM policies that were evaluated to check the principal’s permissions, with annotations to indicate how each policy contributed to the final result. The list of policies can include the policy for the resource itself. It can also include policies that are inherited from higher levels of the resource hierarchy, including the organization, the folder, and the project. To learn more about the resource hierarchy, see https://cloud.google.com/iam/help/resource-hierarchy."]
        #[serde(
            rename = "explainedPolicies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub explained_policies: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudPolicytroubleshooterV1ExplainedPolicy>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess {
        #[doc = "Default value. This value is unused."]
        AccessStateUnspecified,
        #[doc = "The principal has the permission."]
        Granted,
        #[doc = "The principal does not have the permission."]
        NotGranted,
        #[doc = "The principal has the permission only if a condition expression evaluates to `true`."]
        UnknownConditional,
        #[doc = "The sender of the request does not have access to all of the policies that Policy Troubleshooter needs to evaluate."]
        UnknownInfoDenied,
    }
    impl GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess :: AccessStateUnspecified => "ACCESS_STATE_UNSPECIFIED" , GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess :: Granted => "GRANTED" , GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess :: NotGranted => "NOT_GRANTED" , GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess :: UnknownConditional => "UNKNOWN_CONDITIONAL" , GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess :: UnknownInfoDenied => "UNKNOWN_INFO_DENIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess,
            (),
        > {
            Ok (match s { "ACCESS_STATE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess :: AccessStateUnspecified , "GRANTED" => GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess :: Granted , "NOT_GRANTED" => GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess :: NotGranted , "UNKNOWN_CONDITIONAL" => GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess :: UnknownConditional , "UNKNOWN_INFO_DENIED" => GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess :: UnknownInfoDenied , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ACCESS_STATE_UNSPECIFIED" => GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess :: AccessStateUnspecified , "GRANTED" => GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess :: Granted , "NOT_GRANTED" => GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess :: NotGranted , "UNKNOWN_CONDITIONAL" => GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess :: UnknownConditional , "UNKNOWN_INFO_DENIED" => GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess :: UnknownInfoDenied , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponseAccess
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
    pub struct GoogleIamV1AuditConfig {
        #[doc = "The configuration for logging of each type of permission."]
        #[serde(
            rename = "auditLogConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audit_log_configs:
            ::std::option::Option<Vec<crate::schemas::GoogleIamV1AuditLogConfig>>,
        #[doc = "Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services."]
        #[serde(
            rename = "service",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1AuditConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1AuditConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleIamV1AuditLogConfig {
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
        pub log_type: ::std::option::Option<crate::schemas::GoogleIamV1AuditLogConfigLogType>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1AuditLogConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1AuditLogConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleIamV1AuditLogConfigLogType {
        #[doc = "Admin reads. Example: CloudIAM getIamPolicy"]
        AdminRead,
        #[doc = "Data reads. Example: CloudSQL Users list"]
        DataRead,
        #[doc = "Data writes. Example: CloudSQL Users create"]
        DataWrite,
        #[doc = "Default case. Should never be this."]
        LogTypeUnspecified,
    }
    impl GoogleIamV1AuditLogConfigLogType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleIamV1AuditLogConfigLogType::AdminRead => "ADMIN_READ",
                GoogleIamV1AuditLogConfigLogType::DataRead => "DATA_READ",
                GoogleIamV1AuditLogConfigLogType::DataWrite => "DATA_WRITE",
                GoogleIamV1AuditLogConfigLogType::LogTypeUnspecified => "LOG_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleIamV1AuditLogConfigLogType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleIamV1AuditLogConfigLogType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleIamV1AuditLogConfigLogType, ()> {
            Ok(match s {
                "ADMIN_READ" => GoogleIamV1AuditLogConfigLogType::AdminRead,
                "DATA_READ" => GoogleIamV1AuditLogConfigLogType::DataRead,
                "DATA_WRITE" => GoogleIamV1AuditLogConfigLogType::DataWrite,
                "LOG_TYPE_UNSPECIFIED" => GoogleIamV1AuditLogConfigLogType::LogTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleIamV1AuditLogConfigLogType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleIamV1AuditLogConfigLogType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleIamV1AuditLogConfigLogType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADMIN_READ" => GoogleIamV1AuditLogConfigLogType::AdminRead,
                "DATA_READ" => GoogleIamV1AuditLogConfigLogType::DataRead,
                "DATA_WRITE" => GoogleIamV1AuditLogConfigLogType::DataWrite,
                "LOG_TYPE_UNSPECIFIED" => GoogleIamV1AuditLogConfigLogType::LogTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1AuditLogConfigLogType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1AuditLogConfigLogType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleIamV1Binding {
        #[doc = "The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the principals in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        #[serde(
            rename = "condition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub condition: ::std::option::Option<crate::schemas::GoogleTypeExpr>,
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
    impl ::google_field_selector::FieldSelector for GoogleIamV1Binding {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1Binding {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleIamV1Policy {
        #[doc = "Specifies cloud audit logging configuration for this policy."]
        #[serde(
            rename = "auditConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audit_configs: ::std::option::Option<Vec<crate::schemas::GoogleIamV1AuditConfig>>,
        #[doc = "Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`."]
        #[serde(
            rename = "bindings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bindings: ::std::option::Option<Vec<crate::schemas::GoogleIamV1Binding>>,
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
    impl ::google_field_selector::FieldSelector for GoogleIamV1Policy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1Policy {
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
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleTypeExpr {
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
    impl ::google_field_selector::FieldSelector for GoogleTypeExpr {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleTypeExpr {
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
    #[doc = "Actions that can be performed on the iam resource"]
    pub fn iam(&self) -> crate::resources::iam::IamActions {
        crate::resources::iam::IamActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod iam {
        pub mod params {}
        pub struct IamActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> IamActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Checks whether a principal has a specific permission for a specific resource, and explains why the principal does or does not have that permission."]
            pub fn troubleshoot(
                &self,
                request : crate :: schemas :: GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyRequest,
            ) -> TroubleshootRequestBuilder {
                TroubleshootRequestBuilder {
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
                }
            }
        }
        #[doc = "Created via [IamActions::troubleshoot()](struct.IamActions.html#method.troubleshoot)"]
        #[derive(Debug, Clone)]
        pub struct TroubleshootRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyRequest,
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
        impl<'a> TroubleshootRequestBuilder<'a> {
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
                crate::schemas::GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponse,
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
                crate::schemas::GoogleCloudPolicytroubleshooterV1TroubleshootIamPolicyResponse,
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
                let mut output = "https://policytroubleshooter.googleapis.com/".to_owned();
                output.push_str("v1/iam:troubleshoot");
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
