#![allow(rustdoc::bare_urls)]
#![doc = "# Resources and Methods\n* [projects](resources/projects/struct.ProjectsActions.html)\n  * [locations](resources/projects/locations/struct.LocationsActions.html)\n    * [*get*](resources/projects/locations/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/struct.ListRequestBuilder.html)\n    * [features](resources/projects/locations/features/struct.FeaturesActions.html)\n      * [*create*](resources/projects/locations/features/struct.CreateRequestBuilder.html), [*delete*](resources/projects/locations/features/struct.DeleteRequestBuilder.html), [*get*](resources/projects/locations/features/struct.GetRequestBuilder.html), [*getIamPolicy*](resources/projects/locations/features/struct.GetIamPolicyRequestBuilder.html), [*list*](resources/projects/locations/features/struct.ListRequestBuilder.html), [*patch*](resources/projects/locations/features/struct.PatchRequestBuilder.html), [*setIamPolicy*](resources/projects/locations/features/struct.SetIamPolicyRequestBuilder.html), [*testIamPermissions*](resources/projects/locations/features/struct.TestIamPermissionsRequestBuilder.html)\n    * [memberships](resources/projects/locations/memberships/struct.MembershipsActions.html)\n      * [*getIamPolicy*](resources/projects/locations/memberships/struct.GetIamPolicyRequestBuilder.html), [*setIamPolicy*](resources/projects/locations/memberships/struct.SetIamPolicyRequestBuilder.html), [*testIamPermissions*](resources/projects/locations/memberships/struct.TestIamPermissionsRequestBuilder.html)\n    * [operations](resources/projects/locations/operations/struct.OperationsActions.html)\n      * [*cancel*](resources/projects/locations/operations/struct.CancelRequestBuilder.html), [*delete*](resources/projects/locations/operations/struct.DeleteRequestBuilder.html), [*get*](resources/projects/locations/operations/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/operations/struct.ListRequestBuilder.html)\n"]
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
    pub struct AnthosObservabilityFeatureSpec {
        #[doc = "Default membership spec for unconfigured memberships"]
        #[serde(
            rename = "defaultMembershipSpec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_membership_spec:
            ::std::option::Option<crate::schemas::AnthosObservabilityMembershipSpec>,
    }
    impl ::google_field_selector::FieldSelector for AnthosObservabilityFeatureSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnthosObservabilityFeatureSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AnthosObservabilityMembershipSpec {
        #[doc = "Use full of metrics rather than optimized metrics. See https://cloud.google.com/anthos/clusters/docs/on-prem/1.8/concepts/logging-and-monitoring#optimized_metrics_default_metrics"]
        #[serde(
            rename = "doNotOptimizeMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub do_not_optimize_metrics: ::std::option::Option<bool>,
        #[doc = "Enable collecting and reporting metrics and logs from user apps."]
        #[serde(
            rename = "enableStackdriverOnApplications",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_stackdriver_on_applications: ::std::option::Option<bool>,
        #[doc = "the version of stackdriver operator used by this feature"]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AnthosObservabilityMembershipSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnthosObservabilityMembershipSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct AppDevExperienceFeatureSpec {}
    impl ::google_field_selector::FieldSelector for AppDevExperienceFeatureSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AppDevExperienceFeatureSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AppDevExperienceFeatureState {
        #[doc = "Status of subcomponent that detects configured Service Mesh resources."]
        #[serde(
            rename = "networkingInstallSucceeded",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub networking_install_succeeded: ::std::option::Option<crate::schemas::Status>,
    }
    impl ::google_field_selector::FieldSelector for AppDevExperienceFeatureState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AppDevExperienceFeatureState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
    #[derive(
        Debug,
        Clone,
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
        #[doc = "Specifies the principals requesting access for a Google Cloud resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. Does not include identities that come from external identity providers (IdPs) through identity federation. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a Google service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `serviceAccount:{projectid}.svc.id.goog[{namespace}/{kubernetes-sa}]`: An identifier for a [Kubernetes service account](https://cloud.google.com/kubernetes-engine/docs/how-to/kubernetes-service-accounts). For example, `my-project.svc.id.goog[my-namespace/my-kubernetes-sa]`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. "]
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CancelOperationRequest {}
    impl ::google_field_selector::FieldSelector for CancelOperationRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CancelOperationRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CommonFeatureSpec {
        #[doc = "Anthos Observability spec"]
        #[serde(
            rename = "anthosobservability",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub anthosobservability:
            ::std::option::Option<crate::schemas::AnthosObservabilityFeatureSpec>,
        #[doc = "Appdevexperience specific spec."]
        #[serde(
            rename = "appdevexperience",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub appdevexperience: ::std::option::Option<crate::schemas::AppDevExperienceFeatureSpec>,
        #[doc = "FleetObservability feature spec."]
        #[serde(
            rename = "fleetobservability",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fleetobservability:
            ::std::option::Option<crate::schemas::FleetObservabilityFeatureSpec>,
        #[doc = "Multicluster Ingress-specific spec."]
        #[serde(
            rename = "multiclusteringress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub multiclusteringress:
            ::std::option::Option<crate::schemas::MultiClusterIngressFeatureSpec>,
    }
    impl ::google_field_selector::FieldSelector for CommonFeatureSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CommonFeatureSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CommonFeatureState {
        #[doc = "Appdevexperience specific state."]
        #[serde(
            rename = "appdevexperience",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub appdevexperience: ::std::option::Option<crate::schemas::AppDevExperienceFeatureState>,
        #[doc = "FleetObservability feature state."]
        #[serde(
            rename = "fleetobservability",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fleetobservability:
            ::std::option::Option<crate::schemas::FleetObservabilityFeatureState>,
        #[doc = "Output only. The “running state” of the Feature in this Hub."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::FeatureState>,
    }
    impl ::google_field_selector::FieldSelector for CommonFeatureState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CommonFeatureState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CommonFleetDefaultMemberConfigSpec {
        #[doc = "Identity Service-specific spec."]
        #[serde(
            rename = "identityservice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identityservice: ::std::option::Option<crate::schemas::IdentityServiceMembershipSpec>,
    }
    impl ::google_field_selector::FieldSelector for CommonFleetDefaultMemberConfigSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CommonFleetDefaultMemberConfigSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigManagementBinauthzConfig {
        #[doc = "Whether binauthz is enabled in this cluster."]
        #[serde(
            rename = "enabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enabled: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementBinauthzConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementBinauthzConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigManagementBinauthzState {
        #[doc = "The version of binauthz that is installed."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<crate::schemas::ConfigManagementBinauthzVersion>,
        #[doc = "The state of the binauthz webhook."]
        #[serde(
            rename = "webhook",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub webhook: ::std::option::Option<crate::schemas::ConfigManagementBinauthzStateWebhook>,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementBinauthzState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementBinauthzState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ConfigManagementBinauthzStateWebhook {
        #[doc = "Deployment’s state cannot be determined"]
        DeploymentStateUnspecified,
        #[doc = "Deployment was attempted to be installed, but has errors"]
        Error,
        #[doc = "Deployment is installed"]
        Installed,
        #[doc = "Deployment is not installed"]
        NotInstalled,
    }
    impl ConfigManagementBinauthzStateWebhook {
        pub fn as_str(self) -> &'static str {
            match self {
                ConfigManagementBinauthzStateWebhook::DeploymentStateUnspecified => {
                    "DEPLOYMENT_STATE_UNSPECIFIED"
                }
                ConfigManagementBinauthzStateWebhook::Error => "ERROR",
                ConfigManagementBinauthzStateWebhook::Installed => "INSTALLED",
                ConfigManagementBinauthzStateWebhook::NotInstalled => "NOT_INSTALLED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ConfigManagementBinauthzStateWebhook {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ConfigManagementBinauthzStateWebhook {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ConfigManagementBinauthzStateWebhook, ()> {
            Ok(match s {
                "DEPLOYMENT_STATE_UNSPECIFIED" => {
                    ConfigManagementBinauthzStateWebhook::DeploymentStateUnspecified
                }
                "ERROR" => ConfigManagementBinauthzStateWebhook::Error,
                "INSTALLED" => ConfigManagementBinauthzStateWebhook::Installed,
                "NOT_INSTALLED" => ConfigManagementBinauthzStateWebhook::NotInstalled,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ConfigManagementBinauthzStateWebhook {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ConfigManagementBinauthzStateWebhook {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ConfigManagementBinauthzStateWebhook {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEPLOYMENT_STATE_UNSPECIFIED" => {
                    ConfigManagementBinauthzStateWebhook::DeploymentStateUnspecified
                }
                "ERROR" => ConfigManagementBinauthzStateWebhook::Error,
                "INSTALLED" => ConfigManagementBinauthzStateWebhook::Installed,
                "NOT_INSTALLED" => ConfigManagementBinauthzStateWebhook::NotInstalled,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementBinauthzStateWebhook {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementBinauthzStateWebhook {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigManagementBinauthzVersion {
        #[doc = "The version of the binauthz webhook."]
        #[serde(
            rename = "webhookVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub webhook_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementBinauthzVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementBinauthzVersion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigManagementConfigSync {
        #[doc = "Set to true to allow the vertical scaling. Defaults to false which disallows vertical scaling. This field is deprecated."]
        #[serde(
            rename = "allowVerticalScale",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_vertical_scale: ::std::option::Option<bool>,
        #[doc = "Enables the installation of ConfigSync. If set to true, ConfigSync resources will be created and the other ConfigSync fields will be applied if exist. If set to false, all other ConfigSync fields will be ignored, ConfigSync resources will be deleted. If omitted, ConfigSync resources will be managed depends on the presence of git field."]
        #[serde(
            rename = "enabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enabled: ::std::option::Option<bool>,
        #[doc = "Git repo configuration for the cluster."]
        #[serde(
            rename = "git",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub git: ::std::option::Option<crate::schemas::ConfigManagementGitConfig>,
        #[doc = "OCI repo configuration for the cluster"]
        #[serde(
            rename = "oci",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oci: ::std::option::Option<crate::schemas::ConfigManagementOciConfig>,
        #[doc = "Set to true to enable the Config Sync admission webhook to prevent drifts. If set to `false`, disables the Config Sync admission webhook and does not prevent drifts."]
        #[serde(
            rename = "preventDrift",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub prevent_drift: ::std::option::Option<bool>,
        #[doc = "Specifies whether the Config Sync Repo is in “hierarchical” or “unstructured” mode."]
        #[serde(
            rename = "sourceFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_format: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementConfigSync {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementConfigSync {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigManagementConfigSyncDeploymentState {
        #[doc = "Deployment state of admission-webhook"]
        #[serde(
            rename = "admissionWebhook",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub admission_webhook: ::std::option::Option<
            crate::schemas::ConfigManagementConfigSyncDeploymentStateAdmissionWebhook,
        >,
        #[doc = "Deployment state of the git-sync pod"]
        #[serde(
            rename = "gitSync",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub git_sync:
            ::std::option::Option<crate::schemas::ConfigManagementConfigSyncDeploymentStateGitSync>,
        #[doc = "Deployment state of the importer pod"]
        #[serde(
            rename = "importer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub importer: ::std::option::Option<
            crate::schemas::ConfigManagementConfigSyncDeploymentStateImporter,
        >,
        #[doc = "Deployment state of the monitor pod"]
        #[serde(
            rename = "monitor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub monitor:
            ::std::option::Option<crate::schemas::ConfigManagementConfigSyncDeploymentStateMonitor>,
        #[doc = "Deployment state of reconciler-manager pod"]
        #[serde(
            rename = "reconcilerManager",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reconciler_manager: ::std::option::Option<
            crate::schemas::ConfigManagementConfigSyncDeploymentStateReconcilerManager,
        >,
        #[doc = "Deployment state of root-reconciler"]
        #[serde(
            rename = "rootReconciler",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub root_reconciler: ::std::option::Option<
            crate::schemas::ConfigManagementConfigSyncDeploymentStateRootReconciler,
        >,
        #[doc = "Deployment state of the syncer pod"]
        #[serde(
            rename = "syncer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub syncer:
            ::std::option::Option<crate::schemas::ConfigManagementConfigSyncDeploymentStateSyncer>,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementConfigSyncDeploymentState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementConfigSyncDeploymentState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ConfigManagementConfigSyncDeploymentStateAdmissionWebhook {
        #[doc = "Deployment’s state cannot be determined"]
        DeploymentStateUnspecified,
        #[doc = "Deployment was attempted to be installed, but has errors"]
        Error,
        #[doc = "Deployment is installed"]
        Installed,
        #[doc = "Deployment is not installed"]
        NotInstalled,
    }
    impl ConfigManagementConfigSyncDeploymentStateAdmissionWebhook {
        pub fn as_str(self) -> &'static str {
            match self { ConfigManagementConfigSyncDeploymentStateAdmissionWebhook :: DeploymentStateUnspecified => "DEPLOYMENT_STATE_UNSPECIFIED" , ConfigManagementConfigSyncDeploymentStateAdmissionWebhook :: Error => "ERROR" , ConfigManagementConfigSyncDeploymentStateAdmissionWebhook :: Installed => "INSTALLED" , ConfigManagementConfigSyncDeploymentStateAdmissionWebhook :: NotInstalled => "NOT_INSTALLED" , }
        }
    }
    impl ::std::convert::AsRef<str> for ConfigManagementConfigSyncDeploymentStateAdmissionWebhook {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ConfigManagementConfigSyncDeploymentStateAdmissionWebhook {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ConfigManagementConfigSyncDeploymentStateAdmissionWebhook, ()>
        {
            Ok (match s { "DEPLOYMENT_STATE_UNSPECIFIED" => ConfigManagementConfigSyncDeploymentStateAdmissionWebhook :: DeploymentStateUnspecified , "ERROR" => ConfigManagementConfigSyncDeploymentStateAdmissionWebhook :: Error , "INSTALLED" => ConfigManagementConfigSyncDeploymentStateAdmissionWebhook :: Installed , "NOT_INSTALLED" => ConfigManagementConfigSyncDeploymentStateAdmissionWebhook :: NotInstalled , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for ConfigManagementConfigSyncDeploymentStateAdmissionWebhook {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ConfigManagementConfigSyncDeploymentStateAdmissionWebhook {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ConfigManagementConfigSyncDeploymentStateAdmissionWebhook {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "DEPLOYMENT_STATE_UNSPECIFIED" => ConfigManagementConfigSyncDeploymentStateAdmissionWebhook :: DeploymentStateUnspecified , "ERROR" => ConfigManagementConfigSyncDeploymentStateAdmissionWebhook :: Error , "INSTALLED" => ConfigManagementConfigSyncDeploymentStateAdmissionWebhook :: Installed , "NOT_INSTALLED" => ConfigManagementConfigSyncDeploymentStateAdmissionWebhook :: NotInstalled , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for ConfigManagementConfigSyncDeploymentStateAdmissionWebhook
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ConfigManagementConfigSyncDeploymentStateAdmissionWebhook
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ConfigManagementConfigSyncDeploymentStateGitSync {
        #[doc = "Deployment’s state cannot be determined"]
        DeploymentStateUnspecified,
        #[doc = "Deployment was attempted to be installed, but has errors"]
        Error,
        #[doc = "Deployment is installed"]
        Installed,
        #[doc = "Deployment is not installed"]
        NotInstalled,
    }
    impl ConfigManagementConfigSyncDeploymentStateGitSync {
        pub fn as_str(self) -> &'static str {
            match self {
                ConfigManagementConfigSyncDeploymentStateGitSync::DeploymentStateUnspecified => {
                    "DEPLOYMENT_STATE_UNSPECIFIED"
                }
                ConfigManagementConfigSyncDeploymentStateGitSync::Error => "ERROR",
                ConfigManagementConfigSyncDeploymentStateGitSync::Installed => "INSTALLED",
                ConfigManagementConfigSyncDeploymentStateGitSync::NotInstalled => "NOT_INSTALLED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ConfigManagementConfigSyncDeploymentStateGitSync {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ConfigManagementConfigSyncDeploymentStateGitSync {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ConfigManagementConfigSyncDeploymentStateGitSync, ()> {
            Ok(match s {
                "DEPLOYMENT_STATE_UNSPECIFIED" => {
                    ConfigManagementConfigSyncDeploymentStateGitSync::DeploymentStateUnspecified
                }
                "ERROR" => ConfigManagementConfigSyncDeploymentStateGitSync::Error,
                "INSTALLED" => ConfigManagementConfigSyncDeploymentStateGitSync::Installed,
                "NOT_INSTALLED" => ConfigManagementConfigSyncDeploymentStateGitSync::NotInstalled,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ConfigManagementConfigSyncDeploymentStateGitSync {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ConfigManagementConfigSyncDeploymentStateGitSync {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ConfigManagementConfigSyncDeploymentStateGitSync {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEPLOYMENT_STATE_UNSPECIFIED" => {
                    ConfigManagementConfigSyncDeploymentStateGitSync::DeploymentStateUnspecified
                }
                "ERROR" => ConfigManagementConfigSyncDeploymentStateGitSync::Error,
                "INSTALLED" => ConfigManagementConfigSyncDeploymentStateGitSync::Installed,
                "NOT_INSTALLED" => ConfigManagementConfigSyncDeploymentStateGitSync::NotInstalled,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementConfigSyncDeploymentStateGitSync {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementConfigSyncDeploymentStateGitSync {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ConfigManagementConfigSyncDeploymentStateImporter {
        #[doc = "Deployment’s state cannot be determined"]
        DeploymentStateUnspecified,
        #[doc = "Deployment was attempted to be installed, but has errors"]
        Error,
        #[doc = "Deployment is installed"]
        Installed,
        #[doc = "Deployment is not installed"]
        NotInstalled,
    }
    impl ConfigManagementConfigSyncDeploymentStateImporter {
        pub fn as_str(self) -> &'static str {
            match self {
                ConfigManagementConfigSyncDeploymentStateImporter::DeploymentStateUnspecified => {
                    "DEPLOYMENT_STATE_UNSPECIFIED"
                }
                ConfigManagementConfigSyncDeploymentStateImporter::Error => "ERROR",
                ConfigManagementConfigSyncDeploymentStateImporter::Installed => "INSTALLED",
                ConfigManagementConfigSyncDeploymentStateImporter::NotInstalled => "NOT_INSTALLED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ConfigManagementConfigSyncDeploymentStateImporter {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ConfigManagementConfigSyncDeploymentStateImporter {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ConfigManagementConfigSyncDeploymentStateImporter, ()> {
            Ok(match s {
                "DEPLOYMENT_STATE_UNSPECIFIED" => {
                    ConfigManagementConfigSyncDeploymentStateImporter::DeploymentStateUnspecified
                }
                "ERROR" => ConfigManagementConfigSyncDeploymentStateImporter::Error,
                "INSTALLED" => ConfigManagementConfigSyncDeploymentStateImporter::Installed,
                "NOT_INSTALLED" => ConfigManagementConfigSyncDeploymentStateImporter::NotInstalled,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ConfigManagementConfigSyncDeploymentStateImporter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ConfigManagementConfigSyncDeploymentStateImporter {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ConfigManagementConfigSyncDeploymentStateImporter {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEPLOYMENT_STATE_UNSPECIFIED" => {
                    ConfigManagementConfigSyncDeploymentStateImporter::DeploymentStateUnspecified
                }
                "ERROR" => ConfigManagementConfigSyncDeploymentStateImporter::Error,
                "INSTALLED" => ConfigManagementConfigSyncDeploymentStateImporter::Installed,
                "NOT_INSTALLED" => ConfigManagementConfigSyncDeploymentStateImporter::NotInstalled,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementConfigSyncDeploymentStateImporter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementConfigSyncDeploymentStateImporter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ConfigManagementConfigSyncDeploymentStateMonitor {
        #[doc = "Deployment’s state cannot be determined"]
        DeploymentStateUnspecified,
        #[doc = "Deployment was attempted to be installed, but has errors"]
        Error,
        #[doc = "Deployment is installed"]
        Installed,
        #[doc = "Deployment is not installed"]
        NotInstalled,
    }
    impl ConfigManagementConfigSyncDeploymentStateMonitor {
        pub fn as_str(self) -> &'static str {
            match self {
                ConfigManagementConfigSyncDeploymentStateMonitor::DeploymentStateUnspecified => {
                    "DEPLOYMENT_STATE_UNSPECIFIED"
                }
                ConfigManagementConfigSyncDeploymentStateMonitor::Error => "ERROR",
                ConfigManagementConfigSyncDeploymentStateMonitor::Installed => "INSTALLED",
                ConfigManagementConfigSyncDeploymentStateMonitor::NotInstalled => "NOT_INSTALLED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ConfigManagementConfigSyncDeploymentStateMonitor {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ConfigManagementConfigSyncDeploymentStateMonitor {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ConfigManagementConfigSyncDeploymentStateMonitor, ()> {
            Ok(match s {
                "DEPLOYMENT_STATE_UNSPECIFIED" => {
                    ConfigManagementConfigSyncDeploymentStateMonitor::DeploymentStateUnspecified
                }
                "ERROR" => ConfigManagementConfigSyncDeploymentStateMonitor::Error,
                "INSTALLED" => ConfigManagementConfigSyncDeploymentStateMonitor::Installed,
                "NOT_INSTALLED" => ConfigManagementConfigSyncDeploymentStateMonitor::NotInstalled,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ConfigManagementConfigSyncDeploymentStateMonitor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ConfigManagementConfigSyncDeploymentStateMonitor {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ConfigManagementConfigSyncDeploymentStateMonitor {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEPLOYMENT_STATE_UNSPECIFIED" => {
                    ConfigManagementConfigSyncDeploymentStateMonitor::DeploymentStateUnspecified
                }
                "ERROR" => ConfigManagementConfigSyncDeploymentStateMonitor::Error,
                "INSTALLED" => ConfigManagementConfigSyncDeploymentStateMonitor::Installed,
                "NOT_INSTALLED" => ConfigManagementConfigSyncDeploymentStateMonitor::NotInstalled,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementConfigSyncDeploymentStateMonitor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementConfigSyncDeploymentStateMonitor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ConfigManagementConfigSyncDeploymentStateReconcilerManager {
        #[doc = "Deployment’s state cannot be determined"]
        DeploymentStateUnspecified,
        #[doc = "Deployment was attempted to be installed, but has errors"]
        Error,
        #[doc = "Deployment is installed"]
        Installed,
        #[doc = "Deployment is not installed"]
        NotInstalled,
    }
    impl ConfigManagementConfigSyncDeploymentStateReconcilerManager {
        pub fn as_str(self) -> &'static str {
            match self { ConfigManagementConfigSyncDeploymentStateReconcilerManager :: DeploymentStateUnspecified => "DEPLOYMENT_STATE_UNSPECIFIED" , ConfigManagementConfigSyncDeploymentStateReconcilerManager :: Error => "ERROR" , ConfigManagementConfigSyncDeploymentStateReconcilerManager :: Installed => "INSTALLED" , ConfigManagementConfigSyncDeploymentStateReconcilerManager :: NotInstalled => "NOT_INSTALLED" , }
        }
    }
    impl ::std::convert::AsRef<str> for ConfigManagementConfigSyncDeploymentStateReconcilerManager {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ConfigManagementConfigSyncDeploymentStateReconcilerManager {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ConfigManagementConfigSyncDeploymentStateReconcilerManager, ()>
        {
            Ok (match s { "DEPLOYMENT_STATE_UNSPECIFIED" => ConfigManagementConfigSyncDeploymentStateReconcilerManager :: DeploymentStateUnspecified , "ERROR" => ConfigManagementConfigSyncDeploymentStateReconcilerManager :: Error , "INSTALLED" => ConfigManagementConfigSyncDeploymentStateReconcilerManager :: Installed , "NOT_INSTALLED" => ConfigManagementConfigSyncDeploymentStateReconcilerManager :: NotInstalled , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for ConfigManagementConfigSyncDeploymentStateReconcilerManager {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ConfigManagementConfigSyncDeploymentStateReconcilerManager {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ConfigManagementConfigSyncDeploymentStateReconcilerManager {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "DEPLOYMENT_STATE_UNSPECIFIED" => ConfigManagementConfigSyncDeploymentStateReconcilerManager :: DeploymentStateUnspecified , "ERROR" => ConfigManagementConfigSyncDeploymentStateReconcilerManager :: Error , "INSTALLED" => ConfigManagementConfigSyncDeploymentStateReconcilerManager :: Installed , "NOT_INSTALLED" => ConfigManagementConfigSyncDeploymentStateReconcilerManager :: NotInstalled , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for ConfigManagementConfigSyncDeploymentStateReconcilerManager
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ConfigManagementConfigSyncDeploymentStateReconcilerManager
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ConfigManagementConfigSyncDeploymentStateRootReconciler {
        #[doc = "Deployment’s state cannot be determined"]
        DeploymentStateUnspecified,
        #[doc = "Deployment was attempted to be installed, but has errors"]
        Error,
        #[doc = "Deployment is installed"]
        Installed,
        #[doc = "Deployment is not installed"]
        NotInstalled,
    }
    impl ConfigManagementConfigSyncDeploymentStateRootReconciler {
        pub fn as_str(self) -> &'static str {
            match self { ConfigManagementConfigSyncDeploymentStateRootReconciler :: DeploymentStateUnspecified => "DEPLOYMENT_STATE_UNSPECIFIED" , ConfigManagementConfigSyncDeploymentStateRootReconciler :: Error => "ERROR" , ConfigManagementConfigSyncDeploymentStateRootReconciler :: Installed => "INSTALLED" , ConfigManagementConfigSyncDeploymentStateRootReconciler :: NotInstalled => "NOT_INSTALLED" , }
        }
    }
    impl ::std::convert::AsRef<str> for ConfigManagementConfigSyncDeploymentStateRootReconciler {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ConfigManagementConfigSyncDeploymentStateRootReconciler {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ConfigManagementConfigSyncDeploymentStateRootReconciler, ()>
        {
            Ok (match s { "DEPLOYMENT_STATE_UNSPECIFIED" => ConfigManagementConfigSyncDeploymentStateRootReconciler :: DeploymentStateUnspecified , "ERROR" => ConfigManagementConfigSyncDeploymentStateRootReconciler :: Error , "INSTALLED" => ConfigManagementConfigSyncDeploymentStateRootReconciler :: Installed , "NOT_INSTALLED" => ConfigManagementConfigSyncDeploymentStateRootReconciler :: NotInstalled , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for ConfigManagementConfigSyncDeploymentStateRootReconciler {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ConfigManagementConfigSyncDeploymentStateRootReconciler {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ConfigManagementConfigSyncDeploymentStateRootReconciler {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "DEPLOYMENT_STATE_UNSPECIFIED" => ConfigManagementConfigSyncDeploymentStateRootReconciler :: DeploymentStateUnspecified , "ERROR" => ConfigManagementConfigSyncDeploymentStateRootReconciler :: Error , "INSTALLED" => ConfigManagementConfigSyncDeploymentStateRootReconciler :: Installed , "NOT_INSTALLED" => ConfigManagementConfigSyncDeploymentStateRootReconciler :: NotInstalled , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for ConfigManagementConfigSyncDeploymentStateRootReconciler
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ConfigManagementConfigSyncDeploymentStateRootReconciler
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ConfigManagementConfigSyncDeploymentStateSyncer {
        #[doc = "Deployment’s state cannot be determined"]
        DeploymentStateUnspecified,
        #[doc = "Deployment was attempted to be installed, but has errors"]
        Error,
        #[doc = "Deployment is installed"]
        Installed,
        #[doc = "Deployment is not installed"]
        NotInstalled,
    }
    impl ConfigManagementConfigSyncDeploymentStateSyncer {
        pub fn as_str(self) -> &'static str {
            match self {
                ConfigManagementConfigSyncDeploymentStateSyncer::DeploymentStateUnspecified => {
                    "DEPLOYMENT_STATE_UNSPECIFIED"
                }
                ConfigManagementConfigSyncDeploymentStateSyncer::Error => "ERROR",
                ConfigManagementConfigSyncDeploymentStateSyncer::Installed => "INSTALLED",
                ConfigManagementConfigSyncDeploymentStateSyncer::NotInstalled => "NOT_INSTALLED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ConfigManagementConfigSyncDeploymentStateSyncer {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ConfigManagementConfigSyncDeploymentStateSyncer {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ConfigManagementConfigSyncDeploymentStateSyncer, ()> {
            Ok(match s {
                "DEPLOYMENT_STATE_UNSPECIFIED" => {
                    ConfigManagementConfigSyncDeploymentStateSyncer::DeploymentStateUnspecified
                }
                "ERROR" => ConfigManagementConfigSyncDeploymentStateSyncer::Error,
                "INSTALLED" => ConfigManagementConfigSyncDeploymentStateSyncer::Installed,
                "NOT_INSTALLED" => ConfigManagementConfigSyncDeploymentStateSyncer::NotInstalled,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ConfigManagementConfigSyncDeploymentStateSyncer {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ConfigManagementConfigSyncDeploymentStateSyncer {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ConfigManagementConfigSyncDeploymentStateSyncer {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEPLOYMENT_STATE_UNSPECIFIED" => {
                    ConfigManagementConfigSyncDeploymentStateSyncer::DeploymentStateUnspecified
                }
                "ERROR" => ConfigManagementConfigSyncDeploymentStateSyncer::Error,
                "INSTALLED" => ConfigManagementConfigSyncDeploymentStateSyncer::Installed,
                "NOT_INSTALLED" => ConfigManagementConfigSyncDeploymentStateSyncer::NotInstalled,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementConfigSyncDeploymentStateSyncer {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementConfigSyncDeploymentStateSyncer {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigManagementConfigSyncState {
        #[doc = "Information about the deployment of ConfigSync, including the version of the various Pods deployed"]
        #[serde(
            rename = "deploymentState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deployment_state:
            ::std::option::Option<crate::schemas::ConfigManagementConfigSyncDeploymentState>,
        #[doc = "The state of ConfigSync’s process to sync configs to a cluster"]
        #[serde(
            rename = "syncState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sync_state: ::std::option::Option<crate::schemas::ConfigManagementSyncState>,
        #[doc = "The version of ConfigSync deployed"]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<crate::schemas::ConfigManagementConfigSyncVersion>,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementConfigSyncState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementConfigSyncState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigManagementConfigSyncVersion {
        #[doc = "Version of the deployed admission_webhook pod"]
        #[serde(
            rename = "admissionWebhook",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub admission_webhook: ::std::option::Option<String>,
        #[doc = "Version of the deployed git-sync pod"]
        #[serde(
            rename = "gitSync",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub git_sync: ::std::option::Option<String>,
        #[doc = "Version of the deployed importer pod"]
        #[serde(
            rename = "importer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub importer: ::std::option::Option<String>,
        #[doc = "Version of the deployed monitor pod"]
        #[serde(
            rename = "monitor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub monitor: ::std::option::Option<String>,
        #[doc = "Version of the deployed reconciler-manager pod"]
        #[serde(
            rename = "reconcilerManager",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reconciler_manager: ::std::option::Option<String>,
        #[doc = "Version of the deployed reconciler container in root-reconciler pod"]
        #[serde(
            rename = "rootReconciler",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub root_reconciler: ::std::option::Option<String>,
        #[doc = "Version of the deployed syncer pod"]
        #[serde(
            rename = "syncer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub syncer: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementConfigSyncVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementConfigSyncVersion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigManagementErrorResource {
        #[doc = "Group/version/kind of the resource that is causing an error"]
        #[serde(
            rename = "resourceGvk",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_gvk: ::std::option::Option<crate::schemas::ConfigManagementGroupVersionKind>,
        #[doc = "Metadata name of the resource that is causing an error"]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
        #[doc = "Namespace of the resource that is causing an error"]
        #[serde(
            rename = "resourceNamespace",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_namespace: ::std::option::Option<String>,
        #[doc = "Path in the git repo of the erroneous config"]
        #[serde(
            rename = "sourcePath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_path: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementErrorResource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementErrorResource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigManagementGatekeeperDeploymentState { # [doc = "Status of gatekeeper-audit deployment."] # [serde (rename = "gatekeeperAudit" , default , skip_serializing_if = "std::option::Option::is_none")] pub gatekeeper_audit : :: std :: option :: Option < crate :: schemas :: ConfigManagementGatekeeperDeploymentStateGatekeeperAudit > , # [doc = "Status of gatekeeper-controller-manager pod."] # [serde (rename = "gatekeeperControllerManagerState" , default , skip_serializing_if = "std::option::Option::is_none")] pub gatekeeper_controller_manager_state : :: std :: option :: Option < crate :: schemas :: ConfigManagementGatekeeperDeploymentStateGatekeeperControllerManagerState > , # [doc = "Status of the pod serving the mutation webhook."] # [serde (rename = "gatekeeperMutation" , default , skip_serializing_if = "std::option::Option::is_none")] pub gatekeeper_mutation : :: std :: option :: Option < crate :: schemas :: ConfigManagementGatekeeperDeploymentStateGatekeeperMutation > , }
    impl ::google_field_selector::FieldSelector for ConfigManagementGatekeeperDeploymentState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementGatekeeperDeploymentState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ConfigManagementGatekeeperDeploymentStateGatekeeperAudit {
        #[doc = "Deployment’s state cannot be determined"]
        DeploymentStateUnspecified,
        #[doc = "Deployment was attempted to be installed, but has errors"]
        Error,
        #[doc = "Deployment is installed"]
        Installed,
        #[doc = "Deployment is not installed"]
        NotInstalled,
    }
    impl ConfigManagementGatekeeperDeploymentStateGatekeeperAudit {
        pub fn as_str(self) -> &'static str {
            match self { ConfigManagementGatekeeperDeploymentStateGatekeeperAudit :: DeploymentStateUnspecified => "DEPLOYMENT_STATE_UNSPECIFIED" , ConfigManagementGatekeeperDeploymentStateGatekeeperAudit :: Error => "ERROR" , ConfigManagementGatekeeperDeploymentStateGatekeeperAudit :: Installed => "INSTALLED" , ConfigManagementGatekeeperDeploymentStateGatekeeperAudit :: NotInstalled => "NOT_INSTALLED" , }
        }
    }
    impl ::std::convert::AsRef<str> for ConfigManagementGatekeeperDeploymentStateGatekeeperAudit {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ConfigManagementGatekeeperDeploymentStateGatekeeperAudit {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ConfigManagementGatekeeperDeploymentStateGatekeeperAudit, ()>
        {
            Ok (match s { "DEPLOYMENT_STATE_UNSPECIFIED" => ConfigManagementGatekeeperDeploymentStateGatekeeperAudit :: DeploymentStateUnspecified , "ERROR" => ConfigManagementGatekeeperDeploymentStateGatekeeperAudit :: Error , "INSTALLED" => ConfigManagementGatekeeperDeploymentStateGatekeeperAudit :: Installed , "NOT_INSTALLED" => ConfigManagementGatekeeperDeploymentStateGatekeeperAudit :: NotInstalled , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for ConfigManagementGatekeeperDeploymentStateGatekeeperAudit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ConfigManagementGatekeeperDeploymentStateGatekeeperAudit {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ConfigManagementGatekeeperDeploymentStateGatekeeperAudit {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "DEPLOYMENT_STATE_UNSPECIFIED" => ConfigManagementGatekeeperDeploymentStateGatekeeperAudit :: DeploymentStateUnspecified , "ERROR" => ConfigManagementGatekeeperDeploymentStateGatekeeperAudit :: Error , "INSTALLED" => ConfigManagementGatekeeperDeploymentStateGatekeeperAudit :: Installed , "NOT_INSTALLED" => ConfigManagementGatekeeperDeploymentStateGatekeeperAudit :: NotInstalled , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for ConfigManagementGatekeeperDeploymentStateGatekeeperAudit
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ConfigManagementGatekeeperDeploymentStateGatekeeperAudit
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ConfigManagementGatekeeperDeploymentStateGatekeeperControllerManagerState {
        #[doc = "Deployment’s state cannot be determined"]
        DeploymentStateUnspecified,
        #[doc = "Deployment was attempted to be installed, but has errors"]
        Error,
        #[doc = "Deployment is installed"]
        Installed,
        #[doc = "Deployment is not installed"]
        NotInstalled,
    }
    impl ConfigManagementGatekeeperDeploymentStateGatekeeperControllerManagerState {
        pub fn as_str(self) -> &'static str {
            match self { ConfigManagementGatekeeperDeploymentStateGatekeeperControllerManagerState :: DeploymentStateUnspecified => "DEPLOYMENT_STATE_UNSPECIFIED" , ConfigManagementGatekeeperDeploymentStateGatekeeperControllerManagerState :: Error => "ERROR" , ConfigManagementGatekeeperDeploymentStateGatekeeperControllerManagerState :: Installed => "INSTALLED" , ConfigManagementGatekeeperDeploymentStateGatekeeperControllerManagerState :: NotInstalled => "NOT_INSTALLED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for ConfigManagementGatekeeperDeploymentStateGatekeeperControllerManagerState
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for ConfigManagementGatekeeperDeploymentStateGatekeeperControllerManagerState
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            ConfigManagementGatekeeperDeploymentStateGatekeeperControllerManagerState,
            (),
        > {
            Ok (match s { "DEPLOYMENT_STATE_UNSPECIFIED" => ConfigManagementGatekeeperDeploymentStateGatekeeperControllerManagerState :: DeploymentStateUnspecified , "ERROR" => ConfigManagementGatekeeperDeploymentStateGatekeeperControllerManagerState :: Error , "INSTALLED" => ConfigManagementGatekeeperDeploymentStateGatekeeperControllerManagerState :: Installed , "NOT_INSTALLED" => ConfigManagementGatekeeperDeploymentStateGatekeeperControllerManagerState :: NotInstalled , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for ConfigManagementGatekeeperDeploymentStateGatekeeperControllerManagerState
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for ConfigManagementGatekeeperDeploymentStateGatekeeperControllerManagerState
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for ConfigManagementGatekeeperDeploymentStateGatekeeperControllerManagerState
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "DEPLOYMENT_STATE_UNSPECIFIED" => ConfigManagementGatekeeperDeploymentStateGatekeeperControllerManagerState :: DeploymentStateUnspecified , "ERROR" => ConfigManagementGatekeeperDeploymentStateGatekeeperControllerManagerState :: Error , "INSTALLED" => ConfigManagementGatekeeperDeploymentStateGatekeeperControllerManagerState :: Installed , "NOT_INSTALLED" => ConfigManagementGatekeeperDeploymentStateGatekeeperControllerManagerState :: NotInstalled , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for ConfigManagementGatekeeperDeploymentStateGatekeeperControllerManagerState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ConfigManagementGatekeeperDeploymentStateGatekeeperControllerManagerState
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ConfigManagementGatekeeperDeploymentStateGatekeeperMutation {
        #[doc = "Deployment’s state cannot be determined"]
        DeploymentStateUnspecified,
        #[doc = "Deployment was attempted to be installed, but has errors"]
        Error,
        #[doc = "Deployment is installed"]
        Installed,
        #[doc = "Deployment is not installed"]
        NotInstalled,
    }
    impl ConfigManagementGatekeeperDeploymentStateGatekeeperMutation {
        pub fn as_str(self) -> &'static str {
            match self { ConfigManagementGatekeeperDeploymentStateGatekeeperMutation :: DeploymentStateUnspecified => "DEPLOYMENT_STATE_UNSPECIFIED" , ConfigManagementGatekeeperDeploymentStateGatekeeperMutation :: Error => "ERROR" , ConfigManagementGatekeeperDeploymentStateGatekeeperMutation :: Installed => "INSTALLED" , ConfigManagementGatekeeperDeploymentStateGatekeeperMutation :: NotInstalled => "NOT_INSTALLED" , }
        }
    }
    impl ::std::convert::AsRef<str> for ConfigManagementGatekeeperDeploymentStateGatekeeperMutation {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ConfigManagementGatekeeperDeploymentStateGatekeeperMutation {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ConfigManagementGatekeeperDeploymentStateGatekeeperMutation, ()>
        {
            Ok (match s { "DEPLOYMENT_STATE_UNSPECIFIED" => ConfigManagementGatekeeperDeploymentStateGatekeeperMutation :: DeploymentStateUnspecified , "ERROR" => ConfigManagementGatekeeperDeploymentStateGatekeeperMutation :: Error , "INSTALLED" => ConfigManagementGatekeeperDeploymentStateGatekeeperMutation :: Installed , "NOT_INSTALLED" => ConfigManagementGatekeeperDeploymentStateGatekeeperMutation :: NotInstalled , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for ConfigManagementGatekeeperDeploymentStateGatekeeperMutation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ConfigManagementGatekeeperDeploymentStateGatekeeperMutation {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for ConfigManagementGatekeeperDeploymentStateGatekeeperMutation
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "DEPLOYMENT_STATE_UNSPECIFIED" => ConfigManagementGatekeeperDeploymentStateGatekeeperMutation :: DeploymentStateUnspecified , "ERROR" => ConfigManagementGatekeeperDeploymentStateGatekeeperMutation :: Error , "INSTALLED" => ConfigManagementGatekeeperDeploymentStateGatekeeperMutation :: Installed , "NOT_INSTALLED" => ConfigManagementGatekeeperDeploymentStateGatekeeperMutation :: NotInstalled , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for ConfigManagementGatekeeperDeploymentStateGatekeeperMutation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ConfigManagementGatekeeperDeploymentStateGatekeeperMutation
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
    pub struct ConfigManagementGitConfig {
        #[doc = "The GCP Service Account Email used for auth when secret_type is gcpServiceAccount."]
        #[serde(
            rename = "gcpServiceAccountEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcp_service_account_email: ::std::option::Option<String>,
        #[doc = "URL for the HTTPS proxy to be used when communicating with the Git repo."]
        #[serde(
            rename = "httpsProxy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub https_proxy: ::std::option::Option<String>,
        #[doc = "The path within the Git repository that represents the top level of the repo to sync. Default: the root directory of the repository."]
        #[serde(
            rename = "policyDir",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy_dir: ::std::option::Option<String>,
        #[doc = "Type of secret configured for access to the Git repo. Must be one of ssh, cookiefile, gcenode, token, gcpserviceaccount or none. The validation of this is case-sensitive. Required."]
        #[serde(
            rename = "secretType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub secret_type: ::std::option::Option<String>,
        #[doc = "The branch of the repository to sync from. Default: master."]
        #[serde(
            rename = "syncBranch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sync_branch: ::std::option::Option<String>,
        #[doc = "The URL of the Git repository to use as the source of truth."]
        #[serde(
            rename = "syncRepo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sync_repo: ::std::option::Option<String>,
        #[doc = "Git revision (tag or hash) to check out. Default HEAD."]
        #[serde(
            rename = "syncRev",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sync_rev: ::std::option::Option<String>,
        #[doc = "Period in seconds between consecutive syncs. Default: 15."]
        #[serde(
            rename = "syncWaitSecs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub sync_wait_secs: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementGitConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementGitConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigManagementGroupVersionKind {
        #[doc = "Kubernetes Group"]
        #[serde(
            rename = "group",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<String>,
        #[doc = "Kubernetes Kind"]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Kubernetes Version"]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementGroupVersionKind {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementGroupVersionKind {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigManagementHierarchyControllerConfig {
        #[doc = "Whether hierarchical resource quota is enabled in this cluster."]
        #[serde(
            rename = "enableHierarchicalResourceQuota",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_hierarchical_resource_quota: ::std::option::Option<bool>,
        #[doc = "Whether pod tree labels are enabled in this cluster."]
        #[serde(
            rename = "enablePodTreeLabels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_pod_tree_labels: ::std::option::Option<bool>,
        #[doc = "Whether Hierarchy Controller is enabled in this cluster."]
        #[serde(
            rename = "enabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enabled: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementHierarchyControllerConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementHierarchyControllerConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigManagementHierarchyControllerDeploymentState {
        #[doc = "The deployment state for Hierarchy Controller extension (e.g. v0.7.0-hc.1)"]
        #[serde(
            rename = "extension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub extension: ::std::option::Option<
            crate::schemas::ConfigManagementHierarchyControllerDeploymentStateExtension,
        >,
        #[doc = "The deployment state for open source HNC (e.g. v0.7.0-hc.0)"]
        #[serde(
            rename = "hnc",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hnc: ::std::option::Option<
            crate::schemas::ConfigManagementHierarchyControllerDeploymentStateHnc,
        >,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementHierarchyControllerDeploymentState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementHierarchyControllerDeploymentState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ConfigManagementHierarchyControllerDeploymentStateExtension {
        #[doc = "Deployment’s state cannot be determined"]
        DeploymentStateUnspecified,
        #[doc = "Deployment was attempted to be installed, but has errors"]
        Error,
        #[doc = "Deployment is installed"]
        Installed,
        #[doc = "Deployment is not installed"]
        NotInstalled,
    }
    impl ConfigManagementHierarchyControllerDeploymentStateExtension {
        pub fn as_str(self) -> &'static str {
            match self { ConfigManagementHierarchyControllerDeploymentStateExtension :: DeploymentStateUnspecified => "DEPLOYMENT_STATE_UNSPECIFIED" , ConfigManagementHierarchyControllerDeploymentStateExtension :: Error => "ERROR" , ConfigManagementHierarchyControllerDeploymentStateExtension :: Installed => "INSTALLED" , ConfigManagementHierarchyControllerDeploymentStateExtension :: NotInstalled => "NOT_INSTALLED" , }
        }
    }
    impl ::std::convert::AsRef<str> for ConfigManagementHierarchyControllerDeploymentStateExtension {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ConfigManagementHierarchyControllerDeploymentStateExtension {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ConfigManagementHierarchyControllerDeploymentStateExtension, ()>
        {
            Ok (match s { "DEPLOYMENT_STATE_UNSPECIFIED" => ConfigManagementHierarchyControllerDeploymentStateExtension :: DeploymentStateUnspecified , "ERROR" => ConfigManagementHierarchyControllerDeploymentStateExtension :: Error , "INSTALLED" => ConfigManagementHierarchyControllerDeploymentStateExtension :: Installed , "NOT_INSTALLED" => ConfigManagementHierarchyControllerDeploymentStateExtension :: NotInstalled , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for ConfigManagementHierarchyControllerDeploymentStateExtension {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ConfigManagementHierarchyControllerDeploymentStateExtension {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for ConfigManagementHierarchyControllerDeploymentStateExtension
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "DEPLOYMENT_STATE_UNSPECIFIED" => ConfigManagementHierarchyControllerDeploymentStateExtension :: DeploymentStateUnspecified , "ERROR" => ConfigManagementHierarchyControllerDeploymentStateExtension :: Error , "INSTALLED" => ConfigManagementHierarchyControllerDeploymentStateExtension :: Installed , "NOT_INSTALLED" => ConfigManagementHierarchyControllerDeploymentStateExtension :: NotInstalled , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for ConfigManagementHierarchyControllerDeploymentStateExtension
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ConfigManagementHierarchyControllerDeploymentStateExtension
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ConfigManagementHierarchyControllerDeploymentStateHnc {
        #[doc = "Deployment’s state cannot be determined"]
        DeploymentStateUnspecified,
        #[doc = "Deployment was attempted to be installed, but has errors"]
        Error,
        #[doc = "Deployment is installed"]
        Installed,
        #[doc = "Deployment is not installed"]
        NotInstalled,
    }
    impl ConfigManagementHierarchyControllerDeploymentStateHnc {
        pub fn as_str(self) -> &'static str {
            match self { ConfigManagementHierarchyControllerDeploymentStateHnc :: DeploymentStateUnspecified => "DEPLOYMENT_STATE_UNSPECIFIED" , ConfigManagementHierarchyControllerDeploymentStateHnc :: Error => "ERROR" , ConfigManagementHierarchyControllerDeploymentStateHnc :: Installed => "INSTALLED" , ConfigManagementHierarchyControllerDeploymentStateHnc :: NotInstalled => "NOT_INSTALLED" , }
        }
    }
    impl ::std::convert::AsRef<str> for ConfigManagementHierarchyControllerDeploymentStateHnc {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ConfigManagementHierarchyControllerDeploymentStateHnc {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ConfigManagementHierarchyControllerDeploymentStateHnc, ()>
        {
            Ok (match s { "DEPLOYMENT_STATE_UNSPECIFIED" => ConfigManagementHierarchyControllerDeploymentStateHnc :: DeploymentStateUnspecified , "ERROR" => ConfigManagementHierarchyControllerDeploymentStateHnc :: Error , "INSTALLED" => ConfigManagementHierarchyControllerDeploymentStateHnc :: Installed , "NOT_INSTALLED" => ConfigManagementHierarchyControllerDeploymentStateHnc :: NotInstalled , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for ConfigManagementHierarchyControllerDeploymentStateHnc {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ConfigManagementHierarchyControllerDeploymentStateHnc {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ConfigManagementHierarchyControllerDeploymentStateHnc {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "DEPLOYMENT_STATE_UNSPECIFIED" => ConfigManagementHierarchyControllerDeploymentStateHnc :: DeploymentStateUnspecified , "ERROR" => ConfigManagementHierarchyControllerDeploymentStateHnc :: Error , "INSTALLED" => ConfigManagementHierarchyControllerDeploymentStateHnc :: Installed , "NOT_INSTALLED" => ConfigManagementHierarchyControllerDeploymentStateHnc :: NotInstalled , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for ConfigManagementHierarchyControllerDeploymentStateHnc
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ConfigManagementHierarchyControllerDeploymentStateHnc
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
    pub struct ConfigManagementHierarchyControllerState {
        #[doc = "The deployment state for Hierarchy Controller"]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<
            crate::schemas::ConfigManagementHierarchyControllerDeploymentState,
        >,
        #[doc = "The version for Hierarchy Controller"]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version:
            ::std::option::Option<crate::schemas::ConfigManagementHierarchyControllerVersion>,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementHierarchyControllerState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementHierarchyControllerState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigManagementHierarchyControllerVersion {
        #[doc = "Version for Hierarchy Controller extension"]
        #[serde(
            rename = "extension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub extension: ::std::option::Option<String>,
        #[doc = "Version for open source HNC"]
        #[serde(
            rename = "hnc",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hnc: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementHierarchyControllerVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementHierarchyControllerVersion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigManagementInstallError {
        #[doc = "A string representing the user facing error message"]
        #[serde(
            rename = "errorMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementInstallError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementInstallError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigManagementMembershipSpec {
        #[doc = "Binauthz conifguration for the cluster."]
        #[serde(
            rename = "binauthz",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub binauthz: ::std::option::Option<crate::schemas::ConfigManagementBinauthzConfig>,
        #[doc = "Config Sync configuration for the cluster."]
        #[serde(
            rename = "configSync",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config_sync: ::std::option::Option<crate::schemas::ConfigManagementConfigSync>,
        #[doc = "Hierarchy Controller configuration for the cluster."]
        #[serde(
            rename = "hierarchyController",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hierarchy_controller:
            ::std::option::Option<crate::schemas::ConfigManagementHierarchyControllerConfig>,
        #[doc = "Policy Controller configuration for the cluster."]
        #[serde(
            rename = "policyController",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy_controller:
            ::std::option::Option<crate::schemas::ConfigManagementPolicyController>,
        #[doc = "Version of ACM installed."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementMembershipSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementMembershipSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigManagementMembershipState {
        #[doc = "Binauthz status"]
        #[serde(
            rename = "binauthzState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub binauthz_state: ::std::option::Option<crate::schemas::ConfigManagementBinauthzState>,
        #[doc = "The user-defined name for the cluster used by ClusterSelectors to group clusters together. This should match Membership’s membership_name, unless the user installed ACM on the cluster manually prior to enabling the ACM hub feature. Unique within a Anthos Config Management installation."]
        #[serde(
            rename = "clusterName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cluster_name: ::std::option::Option<String>,
        #[doc = "Current sync status"]
        #[serde(
            rename = "configSyncState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config_sync_state:
            ::std::option::Option<crate::schemas::ConfigManagementConfigSyncState>,
        #[doc = "Hierarchy Controller status"]
        #[serde(
            rename = "hierarchyControllerState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hierarchy_controller_state:
            ::std::option::Option<crate::schemas::ConfigManagementHierarchyControllerState>,
        #[doc = "Membership configuration in the cluster. This represents the actual state in the cluster, while the MembershipSpec in the FeatureSpec represents the intended state"]
        #[serde(
            rename = "membershipSpec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub membership_spec: ::std::option::Option<crate::schemas::ConfigManagementMembershipSpec>,
        #[doc = "Current install status of ACM’s Operator"]
        #[serde(
            rename = "operatorState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator_state: ::std::option::Option<crate::schemas::ConfigManagementOperatorState>,
        #[doc = "PolicyController status"]
        #[serde(
            rename = "policyControllerState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy_controller_state:
            ::std::option::Option<crate::schemas::ConfigManagementPolicyControllerState>,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementMembershipState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementMembershipState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigManagementOciConfig {
        #[doc = "The GCP Service Account Email used for auth when secret_type is gcpServiceAccount."]
        #[serde(
            rename = "gcpServiceAccountEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcp_service_account_email: ::std::option::Option<String>,
        #[doc = "The absolute path of the directory that contains the local resources. Default: the root directory of the image."]
        #[serde(
            rename = "policyDir",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy_dir: ::std::option::Option<String>,
        #[doc = "Type of secret configured for access to the Git repo."]
        #[serde(
            rename = "secretType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub secret_type: ::std::option::Option<String>,
        #[doc = "The OCI image repository URL for the package to sync from. e.g. `LOCATION-docker.pkg.dev/PROJECT_ID/REPOSITORY_NAME/PACKAGE_NAME`."]
        #[serde(
            rename = "syncRepo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sync_repo: ::std::option::Option<String>,
        #[doc = "Period in seconds between consecutive syncs. Default: 15."]
        #[serde(
            rename = "syncWaitSecs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub sync_wait_secs: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementOciConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementOciConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigManagementOperatorState {
        #[doc = "The state of the Operator’s deployment"]
        #[serde(
            rename = "deploymentState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deployment_state:
            ::std::option::Option<crate::schemas::ConfigManagementOperatorStateDeploymentState>,
        #[doc = "Install errors."]
        #[serde(
            rename = "errors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub errors: ::std::option::Option<Vec<crate::schemas::ConfigManagementInstallError>>,
        #[doc = "The semenatic version number of the operator"]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementOperatorState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementOperatorState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ConfigManagementOperatorStateDeploymentState {
        #[doc = "Deployment’s state cannot be determined"]
        DeploymentStateUnspecified,
        #[doc = "Deployment was attempted to be installed, but has errors"]
        Error,
        #[doc = "Deployment is installed"]
        Installed,
        #[doc = "Deployment is not installed"]
        NotInstalled,
    }
    impl ConfigManagementOperatorStateDeploymentState {
        pub fn as_str(self) -> &'static str {
            match self {
                ConfigManagementOperatorStateDeploymentState::DeploymentStateUnspecified => {
                    "DEPLOYMENT_STATE_UNSPECIFIED"
                }
                ConfigManagementOperatorStateDeploymentState::Error => "ERROR",
                ConfigManagementOperatorStateDeploymentState::Installed => "INSTALLED",
                ConfigManagementOperatorStateDeploymentState::NotInstalled => "NOT_INSTALLED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ConfigManagementOperatorStateDeploymentState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ConfigManagementOperatorStateDeploymentState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ConfigManagementOperatorStateDeploymentState, ()> {
            Ok(match s {
                "DEPLOYMENT_STATE_UNSPECIFIED" => {
                    ConfigManagementOperatorStateDeploymentState::DeploymentStateUnspecified
                }
                "ERROR" => ConfigManagementOperatorStateDeploymentState::Error,
                "INSTALLED" => ConfigManagementOperatorStateDeploymentState::Installed,
                "NOT_INSTALLED" => ConfigManagementOperatorStateDeploymentState::NotInstalled,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ConfigManagementOperatorStateDeploymentState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ConfigManagementOperatorStateDeploymentState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ConfigManagementOperatorStateDeploymentState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEPLOYMENT_STATE_UNSPECIFIED" => {
                    ConfigManagementOperatorStateDeploymentState::DeploymentStateUnspecified
                }
                "ERROR" => ConfigManagementOperatorStateDeploymentState::Error,
                "INSTALLED" => ConfigManagementOperatorStateDeploymentState::Installed,
                "NOT_INSTALLED" => ConfigManagementOperatorStateDeploymentState::NotInstalled,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementOperatorStateDeploymentState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementOperatorStateDeploymentState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigManagementPolicyController {
        #[doc = "Sets the interval for Policy Controller Audit Scans (in seconds). When set to 0, this disables audit functionality altogether."]
        #[serde(
            rename = "auditIntervalSeconds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub audit_interval_seconds: ::std::option::Option<i64>,
        #[doc = "Enables the installation of Policy Controller. If false, the rest of PolicyController fields take no effect."]
        #[serde(
            rename = "enabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enabled: ::std::option::Option<bool>,
        #[doc = "The set of namespaces that are excluded from Policy Controller checks. Namespaces do not need to currently exist on the cluster."]
        #[serde(
            rename = "exemptableNamespaces",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exemptable_namespaces: ::std::option::Option<Vec<String>>,
        #[doc = "Logs all denies and dry run failures."]
        #[serde(
            rename = "logDeniesEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub log_denies_enabled: ::std::option::Option<bool>,
        #[doc = "Monitoring specifies the configuration of monitoring."]
        #[serde(
            rename = "monitoring",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub monitoring:
            ::std::option::Option<crate::schemas::ConfigManagementPolicyControllerMonitoring>,
        #[doc = "Enable or disable mutation in policy controller. If true, mutation CRDs, webhook and controller deployment will be deployed to the cluster."]
        #[serde(
            rename = "mutationEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mutation_enabled: ::std::option::Option<bool>,
        #[doc = "Enables the ability to use Constraint Templates that reference to objects other than the object currently being evaluated."]
        #[serde(
            rename = "referentialRulesEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub referential_rules_enabled: ::std::option::Option<bool>,
        #[doc = "Installs the default template library along with Policy Controller."]
        #[serde(
            rename = "templateLibraryInstalled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub template_library_installed: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementPolicyController {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementPolicyController {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigManagementPolicyControllerMonitoring {
        #[doc = "Specifies the list of backends Policy Controller will export to. An empty list would effectively disable metrics export."]
        #[serde(
            rename = "backends",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub backends: ::std::option::Option<
            Vec<crate::schemas::ConfigManagementPolicyControllerMonitoringBackendsItems>,
        >,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementPolicyControllerMonitoring {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementPolicyControllerMonitoring {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ConfigManagementPolicyControllerMonitoringBackendsItems {
        #[doc = "Stackdriver/Cloud Monitoring backend for monitoring"]
        CloudMonitoring,
        #[doc = "Backend cannot be determined"]
        MonitoringBackendUnspecified,
        #[doc = "Prometheus backend for monitoring"]
        Prometheus,
    }
    impl ConfigManagementPolicyControllerMonitoringBackendsItems {
        pub fn as_str(self) -> &'static str {
            match self { ConfigManagementPolicyControllerMonitoringBackendsItems :: CloudMonitoring => "CLOUD_MONITORING" , ConfigManagementPolicyControllerMonitoringBackendsItems :: MonitoringBackendUnspecified => "MONITORING_BACKEND_UNSPECIFIED" , ConfigManagementPolicyControllerMonitoringBackendsItems :: Prometheus => "PROMETHEUS" , }
        }
    }
    impl ::std::convert::AsRef<str> for ConfigManagementPolicyControllerMonitoringBackendsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ConfigManagementPolicyControllerMonitoringBackendsItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ConfigManagementPolicyControllerMonitoringBackendsItems, ()>
        {
            Ok (match s { "CLOUD_MONITORING" => ConfigManagementPolicyControllerMonitoringBackendsItems :: CloudMonitoring , "MONITORING_BACKEND_UNSPECIFIED" => ConfigManagementPolicyControllerMonitoringBackendsItems :: MonitoringBackendUnspecified , "PROMETHEUS" => ConfigManagementPolicyControllerMonitoringBackendsItems :: Prometheus , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for ConfigManagementPolicyControllerMonitoringBackendsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ConfigManagementPolicyControllerMonitoringBackendsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ConfigManagementPolicyControllerMonitoringBackendsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CLOUD_MONITORING" => ConfigManagementPolicyControllerMonitoringBackendsItems :: CloudMonitoring , "MONITORING_BACKEND_UNSPECIFIED" => ConfigManagementPolicyControllerMonitoringBackendsItems :: MonitoringBackendUnspecified , "PROMETHEUS" => ConfigManagementPolicyControllerMonitoringBackendsItems :: Prometheus , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for ConfigManagementPolicyControllerMonitoringBackendsItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ConfigManagementPolicyControllerMonitoringBackendsItems
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
    pub struct ConfigManagementPolicyControllerState {
        #[doc = "The state about the policy controller installation."]
        #[serde(
            rename = "deploymentState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deployment_state:
            ::std::option::Option<crate::schemas::ConfigManagementGatekeeperDeploymentState>,
        #[doc = "The version of Gatekeeper Policy Controller deployed."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<crate::schemas::ConfigManagementPolicyControllerVersion>,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementPolicyControllerState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementPolicyControllerState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigManagementPolicyControllerVersion {
        #[doc = "The gatekeeper image tag that is composed of ACM version, git tag, build number."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementPolicyControllerVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementPolicyControllerVersion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigManagementSyncError {
        #[doc = "An ACM defined error code"]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<String>,
        #[doc = "A description of the error"]
        #[serde(
            rename = "errorMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_message: ::std::option::Option<String>,
        #[doc = "A list of config(s) associated with the error, if any"]
        #[serde(
            rename = "errorResources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_resources:
            ::std::option::Option<Vec<crate::schemas::ConfigManagementErrorResource>>,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementSyncError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementSyncError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConfigManagementSyncState {
        #[doc = "Sync status code"]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<crate::schemas::ConfigManagementSyncStateCode>,
        #[doc = "A list of errors resulting from problematic configs. This list will be truncated after 100 errors, although it is unlikely for that many errors to simultaneously exist."]
        #[serde(
            rename = "errors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub errors: ::std::option::Option<Vec<crate::schemas::ConfigManagementSyncError>>,
        #[doc = "Token indicating the state of the importer."]
        #[serde(
            rename = "importToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub import_token: ::std::option::Option<String>,
        #[doc = "Deprecated: use last_sync_time instead. Timestamp of when ACM last successfully synced the repo The time format is specified in https://golang.org/pkg/time/#Time.String"]
        #[serde(
            rename = "lastSync",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_sync: ::std::option::Option<String>,
        #[doc = "Timestamp type of when ACM last successfully synced the repo"]
        #[serde(
            rename = "lastSyncTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_sync_time: ::std::option::Option<String>,
        #[doc = "Token indicating the state of the repo."]
        #[serde(
            rename = "sourceToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_token: ::std::option::Option<String>,
        #[doc = "Token indicating the state of the syncer."]
        #[serde(
            rename = "syncToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sync_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementSyncState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementSyncState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ConfigManagementSyncStateCode {
        #[doc = "Indicates an error configuring ACM, and user action is required"]
        Error,
        #[doc = "ACM has been installed (operator manifest deployed), but not configured."]
        NotConfigured,
        #[doc = "ACM has not been installed (no operator pod found)"]
        NotInstalled,
        #[doc = "ACM is in the progress of syncing a new change"]
        Pending,
        #[doc = "ACM cannot determine a sync code"]
        SyncCodeUnspecified,
        #[doc = "ACM successfully synced the git Repo with the cluster"]
        Synced,
        #[doc = "Error authorizing with the cluster"]
        Unauthorized,
        #[doc = "Cluster could not be reached"]
        Unreachable,
    }
    impl ConfigManagementSyncStateCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ConfigManagementSyncStateCode::Error => "ERROR",
                ConfigManagementSyncStateCode::NotConfigured => "NOT_CONFIGURED",
                ConfigManagementSyncStateCode::NotInstalled => "NOT_INSTALLED",
                ConfigManagementSyncStateCode::Pending => "PENDING",
                ConfigManagementSyncStateCode::SyncCodeUnspecified => "SYNC_CODE_UNSPECIFIED",
                ConfigManagementSyncStateCode::Synced => "SYNCED",
                ConfigManagementSyncStateCode::Unauthorized => "UNAUTHORIZED",
                ConfigManagementSyncStateCode::Unreachable => "UNREACHABLE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ConfigManagementSyncStateCode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ConfigManagementSyncStateCode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ConfigManagementSyncStateCode, ()> {
            Ok(match s {
                "ERROR" => ConfigManagementSyncStateCode::Error,
                "NOT_CONFIGURED" => ConfigManagementSyncStateCode::NotConfigured,
                "NOT_INSTALLED" => ConfigManagementSyncStateCode::NotInstalled,
                "PENDING" => ConfigManagementSyncStateCode::Pending,
                "SYNC_CODE_UNSPECIFIED" => ConfigManagementSyncStateCode::SyncCodeUnspecified,
                "SYNCED" => ConfigManagementSyncStateCode::Synced,
                "UNAUTHORIZED" => ConfigManagementSyncStateCode::Unauthorized,
                "UNREACHABLE" => ConfigManagementSyncStateCode::Unreachable,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ConfigManagementSyncStateCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ConfigManagementSyncStateCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ConfigManagementSyncStateCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ERROR" => ConfigManagementSyncStateCode::Error,
                "NOT_CONFIGURED" => ConfigManagementSyncStateCode::NotConfigured,
                "NOT_INSTALLED" => ConfigManagementSyncStateCode::NotInstalled,
                "PENDING" => ConfigManagementSyncStateCode::Pending,
                "SYNC_CODE_UNSPECIFIED" => ConfigManagementSyncStateCode::SyncCodeUnspecified,
                "SYNCED" => ConfigManagementSyncStateCode::Synced,
                "UNAUTHORIZED" => ConfigManagementSyncStateCode::Unauthorized,
                "UNREACHABLE" => ConfigManagementSyncStateCode::Unreachable,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ConfigManagementSyncStateCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConfigManagementSyncStateCode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Feature {
        #[doc = "Output only. When the Feature resource was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Output only. When the Feature resource was deleted."]
        #[serde(
            rename = "deleteTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_time: ::std::option::Option<String>,
        #[doc = "Optional. Feature configuration applicable to all memberships of the fleet."]
        #[serde(
            rename = "fleetDefaultMemberConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fleet_default_member_config:
            ::std::option::Option<crate::schemas::CommonFleetDefaultMemberConfigSpec>,
        #[doc = "GCP labels for this Feature."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Optional. Membership-specific configuration for this Feature. If this Feature does not support any per-Membership configuration, this field may be unused. The keys indicate which Membership the configuration is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} WILL match the Feature’s project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Membership is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature."]
        #[serde(
            rename = "membershipSpecs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub membership_specs: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::MembershipFeatureSpec>,
        >,
        #[doc = "Output only. Membership-specific Feature status. If this Feature does report any per-Membership status, this field may be unused. The keys indicate which Membership the state is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project number, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} MUST match the Feature’s project number."]
        #[serde(
            rename = "membershipStates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub membership_states: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::MembershipFeatureState>,
        >,
        #[doc = "Output only. The full, unique name of this Feature resource in the format `projects/*/locations/*/features/*`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. State of the Feature resource itself."]
        #[serde(
            rename = "resourceState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_state: ::std::option::Option<crate::schemas::FeatureResourceState>,
        #[doc = "Optional. Scope-specific configuration for this Feature. If this Feature does not support any per-Scope configuration, this field may be unused. The keys indicate which Scope the configuration is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature’s project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Scope is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature."]
        #[serde(
            rename = "scopeSpecs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scope_specs: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::ScopeFeatureSpec>,
        >,
        #[doc = "Output only. Scope-specific Feature status. If this Feature does report any per-Scope status, this field may be unused. The keys indicate which Scope the state is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature’s project."]
        #[serde(
            rename = "scopeStates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scope_states: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::ScopeFeatureState>,
        >,
        #[doc = "Optional. Hub-wide Feature configuration. If this Feature does not support any Hub-wide configuration, this field may be unused."]
        #[serde(
            rename = "spec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spec: ::std::option::Option<crate::schemas::CommonFeatureSpec>,
        #[doc = "Output only. The Hub-wide Feature state."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::CommonFeatureState>,
        #[doc = "Output only. When the Feature resource was last updated."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Feature {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Feature {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FeatureResourceState {
        #[doc = "The current state of the Feature resource in the Hub API."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::FeatureResourceStateState>,
    }
    impl ::google_field_selector::FieldSelector for FeatureResourceState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FeatureResourceState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FeatureResourceStateState {
        #[doc = "The Feature is enabled in this Hub, and the Feature resource is fully available."]
        Active,
        #[doc = "The Feature is being disabled in this Hub, and the Feature resource is being deleted."]
        Disabling,
        #[doc = "The Feature is being enabled, and the Feature resource is being created. Once complete, the corresponding Feature will be enabled in this Hub."]
        Enabling,
        #[doc = "The Feature resource is being updated by the Hub Service."]
        ServiceUpdating,
        #[doc = "State is unknown or not set."]
        StateUnspecified,
        #[doc = "The Feature resource is being updated."]
        Updating,
    }
    impl FeatureResourceStateState {
        pub fn as_str(self) -> &'static str {
            match self {
                FeatureResourceStateState::Active => "ACTIVE",
                FeatureResourceStateState::Disabling => "DISABLING",
                FeatureResourceStateState::Enabling => "ENABLING",
                FeatureResourceStateState::ServiceUpdating => "SERVICE_UPDATING",
                FeatureResourceStateState::StateUnspecified => "STATE_UNSPECIFIED",
                FeatureResourceStateState::Updating => "UPDATING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for FeatureResourceStateState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for FeatureResourceStateState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<FeatureResourceStateState, ()> {
            Ok(match s {
                "ACTIVE" => FeatureResourceStateState::Active,
                "DISABLING" => FeatureResourceStateState::Disabling,
                "ENABLING" => FeatureResourceStateState::Enabling,
                "SERVICE_UPDATING" => FeatureResourceStateState::ServiceUpdating,
                "STATE_UNSPECIFIED" => FeatureResourceStateState::StateUnspecified,
                "UPDATING" => FeatureResourceStateState::Updating,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for FeatureResourceStateState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FeatureResourceStateState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FeatureResourceStateState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVE" => FeatureResourceStateState::Active,
                "DISABLING" => FeatureResourceStateState::Disabling,
                "ENABLING" => FeatureResourceStateState::Enabling,
                "SERVICE_UPDATING" => FeatureResourceStateState::ServiceUpdating,
                "STATE_UNSPECIFIED" => FeatureResourceStateState::StateUnspecified,
                "UPDATING" => FeatureResourceStateState::Updating,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for FeatureResourceStateState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FeatureResourceStateState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FeatureState {
        #[doc = "The high-level, machine-readable status of this Feature."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<crate::schemas::FeatureStateCode>,
        #[doc = "A human-readable description of the current status."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The time this status and any related Feature-specific details were updated."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FeatureState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FeatureState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FeatureStateCode {
        #[doc = "Unknown or not set."]
        CodeUnspecified,
        #[doc = "The Feature is not operating or is in a severely degraded state. The Feature may need intervention to return to normal operation. See the description and any associated Feature-specific details for more information."]
        Error,
        #[doc = "The Feature is operating normally."]
        Ok,
        #[doc = "The Feature has encountered an issue, and is operating in a degraded state. The Feature may need intervention to return to normal operation. See the description and any associated Feature-specific details for more information."]
        Warning,
    }
    impl FeatureStateCode {
        pub fn as_str(self) -> &'static str {
            match self {
                FeatureStateCode::CodeUnspecified => "CODE_UNSPECIFIED",
                FeatureStateCode::Error => "ERROR",
                FeatureStateCode::Ok => "OK",
                FeatureStateCode::Warning => "WARNING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for FeatureStateCode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for FeatureStateCode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<FeatureStateCode, ()> {
            Ok(match s {
                "CODE_UNSPECIFIED" => FeatureStateCode::CodeUnspecified,
                "ERROR" => FeatureStateCode::Error,
                "OK" => FeatureStateCode::Ok,
                "WARNING" => FeatureStateCode::Warning,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for FeatureStateCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FeatureStateCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FeatureStateCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CODE_UNSPECIFIED" => FeatureStateCode::CodeUnspecified,
                "ERROR" => FeatureStateCode::Error,
                "OK" => FeatureStateCode::Ok,
                "WARNING" => FeatureStateCode::Warning,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for FeatureStateCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FeatureStateCode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct FleetObservabilityFeatureSpec {}
    impl ::google_field_selector::FieldSelector for FleetObservabilityFeatureSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FleetObservabilityFeatureSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct FleetObservabilityFeatureState {}
    impl ::google_field_selector::FieldSelector for FleetObservabilityFeatureState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FleetObservabilityFeatureState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct FleetObservabilityMembershipSpec {}
    impl ::google_field_selector::FieldSelector for FleetObservabilityMembershipSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FleetObservabilityMembershipSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct FleetObservabilityMembershipState {}
    impl ::google_field_selector::FieldSelector for FleetObservabilityMembershipState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FleetObservabilityMembershipState {
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
    pub struct IdentityServiceAuthMethod {
        #[doc = "AzureAD specific Configuration."]
        #[serde(
            rename = "azureadConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub azuread_config: ::std::option::Option<crate::schemas::IdentityServiceAzureADConfig>,
        #[doc = "GoogleConfig specific configuration"]
        #[serde(
            rename = "googleConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_config: ::std::option::Option<crate::schemas::IdentityServiceGoogleConfig>,
        #[doc = "Identifier for auth config."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "OIDC specific configuration."]
        #[serde(
            rename = "oidcConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oidc_config: ::std::option::Option<crate::schemas::IdentityServiceOidcConfig>,
        #[doc = "Proxy server address to use for auth method."]
        #[serde(
            rename = "proxy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub proxy: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for IdentityServiceAuthMethod {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentityServiceAuthMethod {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentityServiceAzureADConfig {
        #[doc = "ID for the registered client application that makes authentication requests to the Azure AD identity provider."]
        #[serde(
            rename = "clientId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_id: ::std::option::Option<String>,
        #[doc = "Input only. Unencrypted AzureAD client secret will be passed to the GKE Hub CLH."]
        #[serde(
            rename = "clientSecret",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_secret: ::std::option::Option<String>,
        #[doc = "Output only. Encrypted AzureAD client secret."]
        #[serde(
            rename = "encryptedClientSecret",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub encrypted_client_secret: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "The redirect URL that kubectl uses for authorization."]
        #[serde(
            rename = "kubectlRedirectUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kubectl_redirect_uri: ::std::option::Option<String>,
        #[doc = "Kind of Azure AD account to be authenticated. Supported values are or for accounts belonging to a specific tenant."]
        #[serde(
            rename = "tenant",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tenant: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for IdentityServiceAzureADConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentityServiceAzureADConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentityServiceGoogleConfig {
        #[doc = "Disable automatic configuration of Google Plugin on supported platforms."]
        #[serde(
            rename = "disable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for IdentityServiceGoogleConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentityServiceGoogleConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentityServiceMembershipSpec {
        #[doc = "A member may support multiple auth methods."]
        #[serde(
            rename = "authMethods",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auth_methods: ::std::option::Option<Vec<crate::schemas::IdentityServiceAuthMethod>>,
    }
    impl ::google_field_selector::FieldSelector for IdentityServiceMembershipSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentityServiceMembershipSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentityServiceMembershipState {
        #[doc = "The reason of the failure."]
        #[serde(
            rename = "failureReason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub failure_reason: ::std::option::Option<String>,
        #[doc = "Installed AIS version. This is the AIS version installed on this member. The values makes sense iff state is OK."]
        #[serde(
            rename = "installedVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub installed_version: ::std::option::Option<String>,
        #[doc = "Last reconciled membership configuration"]
        #[serde(
            rename = "memberConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub member_config: ::std::option::Option<crate::schemas::IdentityServiceMembershipSpec>,
        #[doc = "Deployment state on this member"]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::IdentityServiceMembershipStateState>,
    }
    impl ::google_field_selector::FieldSelector for IdentityServiceMembershipState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentityServiceMembershipState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum IdentityServiceMembershipStateState {
        #[doc = "Unspecified state"]
        DeploymentStateUnspecified,
        #[doc = "Failure with error."]
        Error,
        #[doc = "deployment succeeds"]
        Ok,
    }
    impl IdentityServiceMembershipStateState {
        pub fn as_str(self) -> &'static str {
            match self {
                IdentityServiceMembershipStateState::DeploymentStateUnspecified => {
                    "DEPLOYMENT_STATE_UNSPECIFIED"
                }
                IdentityServiceMembershipStateState::Error => "ERROR",
                IdentityServiceMembershipStateState::Ok => "OK",
            }
        }
    }
    impl ::std::convert::AsRef<str> for IdentityServiceMembershipStateState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for IdentityServiceMembershipStateState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<IdentityServiceMembershipStateState, ()> {
            Ok(match s {
                "DEPLOYMENT_STATE_UNSPECIFIED" => {
                    IdentityServiceMembershipStateState::DeploymentStateUnspecified
                }
                "ERROR" => IdentityServiceMembershipStateState::Error,
                "OK" => IdentityServiceMembershipStateState::Ok,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for IdentityServiceMembershipStateState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for IdentityServiceMembershipStateState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for IdentityServiceMembershipStateState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEPLOYMENT_STATE_UNSPECIFIED" => {
                    IdentityServiceMembershipStateState::DeploymentStateUnspecified
                }
                "ERROR" => IdentityServiceMembershipStateState::Error,
                "OK" => IdentityServiceMembershipStateState::Ok,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for IdentityServiceMembershipStateState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentityServiceMembershipStateState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IdentityServiceOidcConfig {
        #[doc = "PEM-encoded CA for OIDC provider."]
        #[serde(
            rename = "certificateAuthorityData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub certificate_authority_data: ::std::option::Option<String>,
        #[doc = "ID for OIDC client application."]
        #[serde(
            rename = "clientId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_id: ::std::option::Option<String>,
        #[doc = "Input only. Unencrypted OIDC client secret will be passed to the GKE Hub CLH."]
        #[serde(
            rename = "clientSecret",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_secret: ::std::option::Option<String>,
        #[doc = "Flag to denote if reverse proxy is used to connect to auth provider. This flag should be set to true when provider is not reachable by Google Cloud Console."]
        #[serde(
            rename = "deployCloudConsoleProxy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deploy_cloud_console_proxy: ::std::option::Option<bool>,
        #[doc = "Enable access token."]
        #[serde(
            rename = "enableAccessToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_access_token: ::std::option::Option<bool>,
        #[doc = "Output only. Encrypted OIDC Client secret"]
        #[serde(
            rename = "encryptedClientSecret",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub encrypted_client_secret: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Comma-separated list of key-value pairs."]
        #[serde(
            rename = "extraParams",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub extra_params: ::std::option::Option<String>,
        #[doc = "Prefix to prepend to group name."]
        #[serde(
            rename = "groupPrefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group_prefix: ::std::option::Option<String>,
        #[doc = "Claim in OIDC ID token that holds group information."]
        #[serde(
            rename = "groupsClaim",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub groups_claim: ::std::option::Option<String>,
        #[doc = "URI for the OIDC provider. This should point to the level below .well-known/openid-configuration."]
        #[serde(
            rename = "issuerUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub issuer_uri: ::std::option::Option<String>,
        #[doc = "Registered redirect uri to redirect users going through OAuth flow using kubectl plugin."]
        #[serde(
            rename = "kubectlRedirectUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kubectl_redirect_uri: ::std::option::Option<String>,
        #[doc = "Comma-separated list of identifiers."]
        #[serde(
            rename = "scopes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scopes: ::std::option::Option<String>,
        #[doc = "Claim in OIDC ID token that holds username."]
        #[serde(
            rename = "userClaim",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_claim: ::std::option::Option<String>,
        #[doc = "Prefix to prepend to user name."]
        #[serde(
            rename = "userPrefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_prefix: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for IdentityServiceOidcConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentityServiceOidcConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListFeaturesResponse {
        #[doc = "A token to request the next page of resources from the `ListFeatures` method. The value of an empty string means that there are no more resources to return."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The list of matching Features"]
        #[serde(
            rename = "resources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resources: ::std::option::Option<Vec<crate::schemas::Feature>>,
    }
    impl ::google_field_selector::FieldSelector for ListFeaturesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListFeaturesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for ListFeaturesResponse {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListLocationsResponse {
        #[doc = "A list of locations that matches the specified filter in the request."]
        #[serde(
            rename = "locations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locations: ::std::option::Option<Vec<crate::schemas::Location>>,
        #[doc = "The standard List next-page token."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListLocationsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListLocationsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for ListLocationsResponse {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListOperationsResponse {
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
        pub operations: ::std::option::Option<Vec<crate::schemas::Operation>>,
    }
    impl ::google_field_selector::FieldSelector for ListOperationsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListOperationsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for ListOperationsResponse {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Location {
        #[doc = "The friendly name for this location, typically a nearby city name. For example, “Tokyo”."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Cross-service attributes for the location. For example {“cloud.googleapis.com/region”: “us-east1”}"]
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
    impl ::google_field_selector::FieldSelector for Location {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Location {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MembershipFeatureSpec {
        #[doc = "Anthos Observability-specific spec"]
        #[serde(
            rename = "anthosobservability",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub anthosobservability:
            ::std::option::Option<crate::schemas::AnthosObservabilityMembershipSpec>,
        #[doc = "Cloud Build-specific spec"]
        #[serde(
            rename = "cloudbuild",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cloudbuild: ::std::option::Option<crate::schemas::MembershipSpec>,
        #[doc = "Config Management-specific spec."]
        #[serde(
            rename = "configmanagement",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub configmanagement: ::std::option::Option<crate::schemas::ConfigManagementMembershipSpec>,
        #[doc = "True if value of `feature_spec` was inherited from a fleet-level default."]
        #[serde(
            rename = "fleetInherited",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fleet_inherited: ::std::option::Option<bool>,
        #[doc = "Fleet observability membership spec"]
        #[serde(
            rename = "fleetobservability",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fleetobservability:
            ::std::option::Option<crate::schemas::FleetObservabilityMembershipSpec>,
        #[doc = "Identity Service-specific spec."]
        #[serde(
            rename = "identityservice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identityservice: ::std::option::Option<crate::schemas::IdentityServiceMembershipSpec>,
        #[doc = "Anthos Service Mesh-specific spec"]
        #[serde(
            rename = "mesh",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mesh: ::std::option::Option<crate::schemas::ServiceMeshMembershipSpec>,
        #[doc = "Policy Controller spec."]
        #[serde(
            rename = "policycontroller",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policycontroller: ::std::option::Option<crate::schemas::PolicyControllerMembershipSpec>,
    }
    impl ::google_field_selector::FieldSelector for MembershipFeatureSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MembershipFeatureSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct MembershipFeatureState {
        #[doc = "Appdevexperience specific state."]
        #[serde(
            rename = "appdevexperience",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub appdevexperience: ::std::option::Option<crate::schemas::AppDevExperienceFeatureState>,
        #[doc = "Config Management-specific state."]
        #[serde(
            rename = "configmanagement",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub configmanagement:
            ::std::option::Option<crate::schemas::ConfigManagementMembershipState>,
        #[doc = "Fleet observability membership state."]
        #[serde(
            rename = "fleetobservability",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fleetobservability:
            ::std::option::Option<crate::schemas::FleetObservabilityMembershipState>,
        #[doc = "Identity Service-specific state."]
        #[serde(
            rename = "identityservice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identityservice: ::std::option::Option<crate::schemas::IdentityServiceMembershipState>,
        #[doc = "Metering-specific state."]
        #[serde(
            rename = "metering",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metering: ::std::option::Option<crate::schemas::MeteringMembershipState>,
        #[doc = "Policycontroller-specific state."]
        #[serde(
            rename = "policycontroller",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policycontroller:
            ::std::option::Option<crate::schemas::PolicyControllerMembershipState>,
        #[doc = "Service Mesh-specific state."]
        #[serde(
            rename = "servicemesh",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub servicemesh: ::std::option::Option<crate::schemas::ServiceMeshMembershipState>,
        #[doc = "The high-level state of this Feature for a single membership."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::FeatureState>,
    }
    impl ::google_field_selector::FieldSelector for MembershipFeatureState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MembershipFeatureState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MembershipSpec {
        #[doc = "Whether it is allowed to run the privileged builds on the cluster or not."]
        #[serde(
            rename = "securityPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub security_policy: ::std::option::Option<crate::schemas::MembershipSpecSecurityPolicy>,
        #[doc = "Version of the cloud build software on the cluster."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MembershipSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MembershipSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MembershipSpecSecurityPolicy {
        #[doc = "Privileged build pods are disallowed"]
        NonPrivileged,
        #[doc = "Privileged build pods are allowed"]
        Privileged,
        #[doc = "Unspecified policy"]
        SecurityPolicyUnspecified,
    }
    impl MembershipSpecSecurityPolicy {
        pub fn as_str(self) -> &'static str {
            match self {
                MembershipSpecSecurityPolicy::NonPrivileged => "NON_PRIVILEGED",
                MembershipSpecSecurityPolicy::Privileged => "PRIVILEGED",
                MembershipSpecSecurityPolicy::SecurityPolicyUnspecified => {
                    "SECURITY_POLICY_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for MembershipSpecSecurityPolicy {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MembershipSpecSecurityPolicy {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MembershipSpecSecurityPolicy, ()> {
            Ok(match s {
                "NON_PRIVILEGED" => MembershipSpecSecurityPolicy::NonPrivileged,
                "PRIVILEGED" => MembershipSpecSecurityPolicy::Privileged,
                "SECURITY_POLICY_UNSPECIFIED" => {
                    MembershipSpecSecurityPolicy::SecurityPolicyUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MembershipSpecSecurityPolicy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MembershipSpecSecurityPolicy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MembershipSpecSecurityPolicy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NON_PRIVILEGED" => MembershipSpecSecurityPolicy::NonPrivileged,
                "PRIVILEGED" => MembershipSpecSecurityPolicy::Privileged,
                "SECURITY_POLICY_UNSPECIFIED" => {
                    MembershipSpecSecurityPolicy::SecurityPolicyUnspecified
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
    impl ::google_field_selector::FieldSelector for MembershipSpecSecurityPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MembershipSpecSecurityPolicy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct MeteringMembershipState {
        #[doc = "The time stamp of the most recent measurement of the number of vCPUs in the cluster."]
        #[serde(
            rename = "lastMeasurementTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_measurement_time: ::std::option::Option<String>,
        #[doc = "The vCPUs capacity in the cluster according to the most recent measurement (1/1000 precision)."]
        #[serde(
            rename = "preciseLastMeasuredClusterVcpuCapacity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub precise_last_measured_cluster_vcpu_capacity: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for MeteringMembershipState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MeteringMembershipState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MultiClusterIngressFeatureSpec {
        #[doc = "Deprecated: This field will be ignored and should not be set. Customer’s billing structure."]
        #[serde(
            rename = "billing",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub billing: ::std::option::Option<crate::schemas::MultiClusterIngressFeatureSpecBilling>,
        #[doc = "Fully-qualified Membership name which hosts the MultiClusterIngress CRD. Example: `projects/foo-proj/locations/global/memberships/bar`"]
        #[serde(
            rename = "configMembership",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config_membership: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MultiClusterIngressFeatureSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MultiClusterIngressFeatureSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MultiClusterIngressFeatureSpecBilling {
        #[doc = "User is paying for Anthos as a whole."]
        AnthosLicense,
        #[doc = "Unknown"]
        BillingUnspecified,
        #[doc = "User pays a fee per-endpoint."]
        PayAsYouGo,
    }
    impl MultiClusterIngressFeatureSpecBilling {
        pub fn as_str(self) -> &'static str {
            match self {
                MultiClusterIngressFeatureSpecBilling::AnthosLicense => "ANTHOS_LICENSE",
                MultiClusterIngressFeatureSpecBilling::BillingUnspecified => "BILLING_UNSPECIFIED",
                MultiClusterIngressFeatureSpecBilling::PayAsYouGo => "PAY_AS_YOU_GO",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MultiClusterIngressFeatureSpecBilling {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MultiClusterIngressFeatureSpecBilling {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MultiClusterIngressFeatureSpecBilling, ()> {
            Ok(match s {
                "ANTHOS_LICENSE" => MultiClusterIngressFeatureSpecBilling::AnthosLicense,
                "BILLING_UNSPECIFIED" => MultiClusterIngressFeatureSpecBilling::BillingUnspecified,
                "PAY_AS_YOU_GO" => MultiClusterIngressFeatureSpecBilling::PayAsYouGo,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MultiClusterIngressFeatureSpecBilling {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MultiClusterIngressFeatureSpecBilling {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MultiClusterIngressFeatureSpecBilling {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANTHOS_LICENSE" => MultiClusterIngressFeatureSpecBilling::AnthosLicense,
                "BILLING_UNSPECIFIED" => MultiClusterIngressFeatureSpecBilling::BillingUnspecified,
                "PAY_AS_YOU_GO" => MultiClusterIngressFeatureSpecBilling::PayAsYouGo,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MultiClusterIngressFeatureSpecBilling {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MultiClusterIngressFeatureSpecBilling {
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
    pub struct OperationMetadata {
        #[doc = "Output only. API version used to start the operation."]
        #[serde(
            rename = "apiVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub api_version: ::std::option::Option<String>,
        #[doc = "Output only. Identifies whether the user has requested cancellation of the operation. Operations that have successfully been cancelled have Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."]
        #[serde(
            rename = "cancelRequested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cancel_requested: ::std::option::Option<bool>,
        #[doc = "Output only. The time the operation was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Output only. The time the operation finished running."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Output only. Human-readable status of the operation, if any."]
        #[serde(
            rename = "statusDetail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status_detail: ::std::option::Option<String>,
        #[doc = "Output only. Server-defined resource path for the target of the operation."]
        #[serde(
            rename = "target",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target: ::std::option::Option<String>,
        #[doc = "Output only. Name of the verb executed by the operation."]
        #[serde(
            rename = "verb",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub verb: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OperationMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OperationMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
    pub struct PolicyControllerBundleInstallSpec {
        #[doc = "the set of namespaces to be exempted from the bundle"]
        #[serde(
            rename = "exemptedNamespaces",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exempted_namespaces: ::std::option::Option<Vec<String>>,
        #[doc = "Management specifies how the bundle will be managed by the controller."]
        #[serde(
            rename = "management",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub management:
            ::std::option::Option<crate::schemas::PolicyControllerBundleInstallSpecManagement>,
    }
    impl ::google_field_selector::FieldSelector for PolicyControllerBundleInstallSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyControllerBundleInstallSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PolicyControllerBundleInstallSpecManagement {
        #[doc = "The entity should be insistently reconciled by the Hub controller"]
        Installed,
        #[doc = "No Management strategy has been specified."]
        ManagementUnspecified,
    }
    impl PolicyControllerBundleInstallSpecManagement {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyControllerBundleInstallSpecManagement::Installed => "INSTALLED",
                PolicyControllerBundleInstallSpecManagement::ManagementUnspecified => {
                    "MANAGEMENT_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for PolicyControllerBundleInstallSpecManagement {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PolicyControllerBundleInstallSpecManagement {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<PolicyControllerBundleInstallSpecManagement, ()> {
            Ok(match s {
                "INSTALLED" => PolicyControllerBundleInstallSpecManagement::Installed,
                "MANAGEMENT_UNSPECIFIED" => {
                    PolicyControllerBundleInstallSpecManagement::ManagementUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PolicyControllerBundleInstallSpecManagement {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyControllerBundleInstallSpecManagement {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyControllerBundleInstallSpecManagement {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INSTALLED" => PolicyControllerBundleInstallSpecManagement::Installed,
                "MANAGEMENT_UNSPECIFIED" => {
                    PolicyControllerBundleInstallSpecManagement::ManagementUnspecified
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
    impl ::google_field_selector::FieldSelector for PolicyControllerBundleInstallSpecManagement {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyControllerBundleInstallSpecManagement {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PolicyControllerHubConfig {
        #[doc = "Sets the interval for Policy Controller Audit Scans (in seconds). When set to 0, this disables audit functionality altogether."]
        #[serde(
            rename = "auditIntervalSeconds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub audit_interval_seconds: ::std::option::Option<i64>,
        #[doc = "The set of namespaces that are excluded from Policy Controller checks. Namespaces do not need to currently exist on the cluster."]
        #[serde(
            rename = "exemptableNamespaces",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exemptable_namespaces: ::std::option::Option<Vec<String>>,
        #[doc = "The install_spec represents the intended state specified by the latest request that mutated install_spec in the feature spec, not the lifecycle state of the feature observed by the Hub feature controller that is reported in the feature state."]
        #[serde(
            rename = "installSpec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub install_spec:
            ::std::option::Option<crate::schemas::PolicyControllerHubConfigInstallSpec>,
        #[doc = "Logs all denies and dry run failures."]
        #[serde(
            rename = "logDeniesEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub log_denies_enabled: ::std::option::Option<bool>,
        #[doc = "Monitoring specifies the configuration of monitoring."]
        #[serde(
            rename = "monitoring",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub monitoring: ::std::option::Option<crate::schemas::PolicyControllerMonitoringConfig>,
        #[doc = "Enables the ability to mutate resources using Policy Controller."]
        #[serde(
            rename = "mutationEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mutation_enabled: ::std::option::Option<bool>,
        #[doc = "Specifies the desired policy content on the cluster"]
        #[serde(
            rename = "policyContent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy_content:
            ::std::option::Option<crate::schemas::PolicyControllerPolicyContentSpec>,
        #[doc = "Enables the ability to use Constraint Templates that reference to objects other than the object currently being evaluated."]
        #[serde(
            rename = "referentialRulesEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub referential_rules_enabled: ::std::option::Option<bool>,
        #[doc = "Configures the library templates to install along with Policy Controller."]
        #[serde(
            rename = "templateLibraryConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub template_library_config:
            ::std::option::Option<crate::schemas::PolicyControllerTemplateLibraryConfig>,
    }
    impl ::google_field_selector::FieldSelector for PolicyControllerHubConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyControllerHubConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PolicyControllerHubConfigInstallSpec {
        #[doc = "Request to install and enable Policy Controller."]
        InstallSpecEnabled,
        #[doc = "Request to uninstall Policy Controller."]
        InstallSpecNotInstalled,
        #[doc = "Request to suspend Policy Controller i.e. its webhooks. If Policy Controller is not installed, it will be installed but suspended."]
        InstallSpecSuspended,
        #[doc = "Spec is unknown."]
        InstallSpecUnspecified,
    }
    impl PolicyControllerHubConfigInstallSpec {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyControllerHubConfigInstallSpec::InstallSpecEnabled => "INSTALL_SPEC_ENABLED",
                PolicyControllerHubConfigInstallSpec::InstallSpecNotInstalled => {
                    "INSTALL_SPEC_NOT_INSTALLED"
                }
                PolicyControllerHubConfigInstallSpec::InstallSpecSuspended => {
                    "INSTALL_SPEC_SUSPENDED"
                }
                PolicyControllerHubConfigInstallSpec::InstallSpecUnspecified => {
                    "INSTALL_SPEC_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for PolicyControllerHubConfigInstallSpec {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PolicyControllerHubConfigInstallSpec {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PolicyControllerHubConfigInstallSpec, ()> {
            Ok(match s {
                "INSTALL_SPEC_ENABLED" => PolicyControllerHubConfigInstallSpec::InstallSpecEnabled,
                "INSTALL_SPEC_NOT_INSTALLED" => {
                    PolicyControllerHubConfigInstallSpec::InstallSpecNotInstalled
                }
                "INSTALL_SPEC_SUSPENDED" => {
                    PolicyControllerHubConfigInstallSpec::InstallSpecSuspended
                }
                "INSTALL_SPEC_UNSPECIFIED" => {
                    PolicyControllerHubConfigInstallSpec::InstallSpecUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PolicyControllerHubConfigInstallSpec {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyControllerHubConfigInstallSpec {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyControllerHubConfigInstallSpec {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INSTALL_SPEC_ENABLED" => PolicyControllerHubConfigInstallSpec::InstallSpecEnabled,
                "INSTALL_SPEC_NOT_INSTALLED" => {
                    PolicyControllerHubConfigInstallSpec::InstallSpecNotInstalled
                }
                "INSTALL_SPEC_SUSPENDED" => {
                    PolicyControllerHubConfigInstallSpec::InstallSpecSuspended
                }
                "INSTALL_SPEC_UNSPECIFIED" => {
                    PolicyControllerHubConfigInstallSpec::InstallSpecUnspecified
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
    impl ::google_field_selector::FieldSelector for PolicyControllerHubConfigInstallSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyControllerHubConfigInstallSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PolicyControllerMembershipSpec {
        #[doc = "Policy Controller configuration for the cluster."]
        #[serde(
            rename = "policyControllerHubConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy_controller_hub_config:
            ::std::option::Option<crate::schemas::PolicyControllerHubConfig>,
        #[doc = "Version of Policy Controller installed."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PolicyControllerMembershipSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyControllerMembershipSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PolicyControllerMembershipState {
        #[doc = "Currently these include (also serving as map keys): 1. “admission” 2. “audit” 3. “mutation”"]
        #[serde(
            rename = "componentStates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub component_states: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::PolicyControllerOnClusterState>,
        >,
        #[doc = "The state of the template library and any bundles included in the chosen version of the manifest"]
        #[serde(
            rename = "contentStates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_states: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::PolicyControllerOnClusterState>,
        >,
        #[doc = "The overall Policy Controller lifecycle state observed by the Hub Feature controller."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::PolicyControllerMembershipStateState>,
    }
    impl ::google_field_selector::FieldSelector for PolicyControllerMembershipState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyControllerMembershipState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PolicyControllerMembershipStateState {
        #[doc = "The PC is fully installed on the cluster and in an operational mode. In this state PCH will be reconciling state with the PC, and the PC will be performing it’s operational tasks per that software. Entering a READY state requires that the hub has confirmed the PC is installed and its pods are operational with the version of the PC the PCH expects."]
        Active,
        #[doc = "The PC is not operational, and the PCH is unable to act to make it operational. Entering a CLUSTER_ERROR state happens automatically when the PCH determines that a PC installed on the cluster is non-operative or that the cluster does not meet requirements set for the PCH to administer the cluster but has nevertheless been given an instruction to do so (such as ‘install’)."]
        ClusterError,
        #[doc = "The PC may have resources on the cluster, but the PCH wishes to remove the Membership. The Membership still exists."]
        Decommissioning,
        #[doc = "In this state, the PC may still be operational, and only the PCH is unable to act. The hub should not issue instructions to change the PC state, or otherwise interfere with the on-cluster resources. Entering a HUB_ERROR state happens automatically when the PCH determines the hub is in an unhealthy state and it wishes to ‘take hands off’ to avoid corrupting the PC or other data."]
        HubError,
        #[doc = "The PCH possesses a Membership, however the PC is not fully installed on the cluster. In this state the hub can be expected to be taking actions to install the PC on the cluster."]
        Installing,
        #[doc = "The lifecycle state is unspecified."]
        LifecycleStateUnspecified,
        #[doc = "The PC does not exist on the given cluster, and no k8s resources of any type that are associated with the PC should exist there. The cluster does not possess a membership with the PCH."]
        NotInstalled,
        #[doc = "Policy Controller (PC) is installed but suspended. This means that the policies are not enforced, but violations are still recorded (through audit)."]
        Suspended,
        #[doc = "The PC is fully installed, but in the process of changing the configuration (including changing the version of PC either up and down, or modifying the manifests of PC) of the resources running on the cluster. The PCH has a Membership, is aware of the version the cluster should be running in, but has not confirmed for itself that the PC is running with that version."]
        Updating,
    }
    impl PolicyControllerMembershipStateState {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyControllerMembershipStateState::Active => "ACTIVE",
                PolicyControllerMembershipStateState::ClusterError => "CLUSTER_ERROR",
                PolicyControllerMembershipStateState::Decommissioning => "DECOMMISSIONING",
                PolicyControllerMembershipStateState::HubError => "HUB_ERROR",
                PolicyControllerMembershipStateState::Installing => "INSTALLING",
                PolicyControllerMembershipStateState::LifecycleStateUnspecified => {
                    "LIFECYCLE_STATE_UNSPECIFIED"
                }
                PolicyControllerMembershipStateState::NotInstalled => "NOT_INSTALLED",
                PolicyControllerMembershipStateState::Suspended => "SUSPENDED",
                PolicyControllerMembershipStateState::Updating => "UPDATING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PolicyControllerMembershipStateState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PolicyControllerMembershipStateState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PolicyControllerMembershipStateState, ()> {
            Ok(match s {
                "ACTIVE" => PolicyControllerMembershipStateState::Active,
                "CLUSTER_ERROR" => PolicyControllerMembershipStateState::ClusterError,
                "DECOMMISSIONING" => PolicyControllerMembershipStateState::Decommissioning,
                "HUB_ERROR" => PolicyControllerMembershipStateState::HubError,
                "INSTALLING" => PolicyControllerMembershipStateState::Installing,
                "LIFECYCLE_STATE_UNSPECIFIED" => {
                    PolicyControllerMembershipStateState::LifecycleStateUnspecified
                }
                "NOT_INSTALLED" => PolicyControllerMembershipStateState::NotInstalled,
                "SUSPENDED" => PolicyControllerMembershipStateState::Suspended,
                "UPDATING" => PolicyControllerMembershipStateState::Updating,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PolicyControllerMembershipStateState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyControllerMembershipStateState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyControllerMembershipStateState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVE" => PolicyControllerMembershipStateState::Active,
                "CLUSTER_ERROR" => PolicyControllerMembershipStateState::ClusterError,
                "DECOMMISSIONING" => PolicyControllerMembershipStateState::Decommissioning,
                "HUB_ERROR" => PolicyControllerMembershipStateState::HubError,
                "INSTALLING" => PolicyControllerMembershipStateState::Installing,
                "LIFECYCLE_STATE_UNSPECIFIED" => {
                    PolicyControllerMembershipStateState::LifecycleStateUnspecified
                }
                "NOT_INSTALLED" => PolicyControllerMembershipStateState::NotInstalled,
                "SUSPENDED" => PolicyControllerMembershipStateState::Suspended,
                "UPDATING" => PolicyControllerMembershipStateState::Updating,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PolicyControllerMembershipStateState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyControllerMembershipStateState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PolicyControllerMonitoringConfig {
        #[doc = "Specifies the list of backends Policy Controller will export to. An empty list would effectively disable metrics export."]
        #[serde(
            rename = "backends",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub backends: ::std::option::Option<
            Vec<crate::schemas::PolicyControllerMonitoringConfigBackendsItems>,
        >,
    }
    impl ::google_field_selector::FieldSelector for PolicyControllerMonitoringConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyControllerMonitoringConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PolicyControllerMonitoringConfigBackendsItems {
        #[doc = "Stackdriver/Cloud Monitoring backend for monitoring"]
        CloudMonitoring,
        #[doc = "Backend cannot be determined"]
        MonitoringBackendUnspecified,
        #[doc = "Prometheus backend for monitoring"]
        Prometheus,
    }
    impl PolicyControllerMonitoringConfigBackendsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyControllerMonitoringConfigBackendsItems::CloudMonitoring => {
                    "CLOUD_MONITORING"
                }
                PolicyControllerMonitoringConfigBackendsItems::MonitoringBackendUnspecified => {
                    "MONITORING_BACKEND_UNSPECIFIED"
                }
                PolicyControllerMonitoringConfigBackendsItems::Prometheus => "PROMETHEUS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PolicyControllerMonitoringConfigBackendsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PolicyControllerMonitoringConfigBackendsItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<PolicyControllerMonitoringConfigBackendsItems, ()> {
            Ok(match s {
                "CLOUD_MONITORING" => {
                    PolicyControllerMonitoringConfigBackendsItems::CloudMonitoring
                }
                "MONITORING_BACKEND_UNSPECIFIED" => {
                    PolicyControllerMonitoringConfigBackendsItems::MonitoringBackendUnspecified
                }
                "PROMETHEUS" => PolicyControllerMonitoringConfigBackendsItems::Prometheus,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PolicyControllerMonitoringConfigBackendsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyControllerMonitoringConfigBackendsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyControllerMonitoringConfigBackendsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CLOUD_MONITORING" => {
                    PolicyControllerMonitoringConfigBackendsItems::CloudMonitoring
                }
                "MONITORING_BACKEND_UNSPECIFIED" => {
                    PolicyControllerMonitoringConfigBackendsItems::MonitoringBackendUnspecified
                }
                "PROMETHEUS" => PolicyControllerMonitoringConfigBackendsItems::Prometheus,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PolicyControllerMonitoringConfigBackendsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyControllerMonitoringConfigBackendsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PolicyControllerOnClusterState {
        #[doc = "Surface potential errors or information logs."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details: ::std::option::Option<String>,
        #[doc = "The lifecycle state of this component."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::PolicyControllerOnClusterStateState>,
    }
    impl ::google_field_selector::FieldSelector for PolicyControllerOnClusterState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyControllerOnClusterState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PolicyControllerOnClusterStateState {
        #[doc = "The PC is fully installed on the cluster and in an operational mode. In this state PCH will be reconciling state with the PC, and the PC will be performing it’s operational tasks per that software. Entering a READY state requires that the hub has confirmed the PC is installed and its pods are operational with the version of the PC the PCH expects."]
        Active,
        #[doc = "The PC is not operational, and the PCH is unable to act to make it operational. Entering a CLUSTER_ERROR state happens automatically when the PCH determines that a PC installed on the cluster is non-operative or that the cluster does not meet requirements set for the PCH to administer the cluster but has nevertheless been given an instruction to do so (such as ‘install’)."]
        ClusterError,
        #[doc = "The PC may have resources on the cluster, but the PCH wishes to remove the Membership. The Membership still exists."]
        Decommissioning,
        #[doc = "In this state, the PC may still be operational, and only the PCH is unable to act. The hub should not issue instructions to change the PC state, or otherwise interfere with the on-cluster resources. Entering a HUB_ERROR state happens automatically when the PCH determines the hub is in an unhealthy state and it wishes to ‘take hands off’ to avoid corrupting the PC or other data."]
        HubError,
        #[doc = "The PCH possesses a Membership, however the PC is not fully installed on the cluster. In this state the hub can be expected to be taking actions to install the PC on the cluster."]
        Installing,
        #[doc = "The lifecycle state is unspecified."]
        LifecycleStateUnspecified,
        #[doc = "The PC does not exist on the given cluster, and no k8s resources of any type that are associated with the PC should exist there. The cluster does not possess a membership with the PCH."]
        NotInstalled,
        #[doc = "Policy Controller (PC) is installed but suspended. This means that the policies are not enforced, but violations are still recorded (through audit)."]
        Suspended,
        #[doc = "The PC is fully installed, but in the process of changing the configuration (including changing the version of PC either up and down, or modifying the manifests of PC) of the resources running on the cluster. The PCH has a Membership, is aware of the version the cluster should be running in, but has not confirmed for itself that the PC is running with that version."]
        Updating,
    }
    impl PolicyControllerOnClusterStateState {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyControllerOnClusterStateState::Active => "ACTIVE",
                PolicyControllerOnClusterStateState::ClusterError => "CLUSTER_ERROR",
                PolicyControllerOnClusterStateState::Decommissioning => "DECOMMISSIONING",
                PolicyControllerOnClusterStateState::HubError => "HUB_ERROR",
                PolicyControllerOnClusterStateState::Installing => "INSTALLING",
                PolicyControllerOnClusterStateState::LifecycleStateUnspecified => {
                    "LIFECYCLE_STATE_UNSPECIFIED"
                }
                PolicyControllerOnClusterStateState::NotInstalled => "NOT_INSTALLED",
                PolicyControllerOnClusterStateState::Suspended => "SUSPENDED",
                PolicyControllerOnClusterStateState::Updating => "UPDATING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PolicyControllerOnClusterStateState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PolicyControllerOnClusterStateState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PolicyControllerOnClusterStateState, ()> {
            Ok(match s {
                "ACTIVE" => PolicyControllerOnClusterStateState::Active,
                "CLUSTER_ERROR" => PolicyControllerOnClusterStateState::ClusterError,
                "DECOMMISSIONING" => PolicyControllerOnClusterStateState::Decommissioning,
                "HUB_ERROR" => PolicyControllerOnClusterStateState::HubError,
                "INSTALLING" => PolicyControllerOnClusterStateState::Installing,
                "LIFECYCLE_STATE_UNSPECIFIED" => {
                    PolicyControllerOnClusterStateState::LifecycleStateUnspecified
                }
                "NOT_INSTALLED" => PolicyControllerOnClusterStateState::NotInstalled,
                "SUSPENDED" => PolicyControllerOnClusterStateState::Suspended,
                "UPDATING" => PolicyControllerOnClusterStateState::Updating,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PolicyControllerOnClusterStateState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyControllerOnClusterStateState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyControllerOnClusterStateState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVE" => PolicyControllerOnClusterStateState::Active,
                "CLUSTER_ERROR" => PolicyControllerOnClusterStateState::ClusterError,
                "DECOMMISSIONING" => PolicyControllerOnClusterStateState::Decommissioning,
                "HUB_ERROR" => PolicyControllerOnClusterStateState::HubError,
                "INSTALLING" => PolicyControllerOnClusterStateState::Installing,
                "LIFECYCLE_STATE_UNSPECIFIED" => {
                    PolicyControllerOnClusterStateState::LifecycleStateUnspecified
                }
                "NOT_INSTALLED" => PolicyControllerOnClusterStateState::NotInstalled,
                "SUSPENDED" => PolicyControllerOnClusterStateState::Suspended,
                "UPDATING" => PolicyControllerOnClusterStateState::Updating,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PolicyControllerOnClusterStateState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyControllerOnClusterStateState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PolicyControllerPolicyContentSpec {
        #[doc = "map of bundle name to BundleInstallSpec. The bundle name maps to the `bundleName` key in the `policycontroller.gke.io/constraintData` annotation on a constraint."]
        #[serde(
            rename = "bundles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bundles: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::PolicyControllerBundleInstallSpec>,
        >,
    }
    impl ::google_field_selector::FieldSelector for PolicyControllerPolicyContentSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyControllerPolicyContentSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PolicyControllerTemplateLibraryConfig {
        #[doc = "Whether the standard template library should be installed or not."]
        #[serde(
            rename = "included",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub included: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for PolicyControllerTemplateLibraryConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyControllerTemplateLibraryConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct ScopeFeatureSpec {}
    impl ::google_field_selector::FieldSelector for ScopeFeatureSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScopeFeatureSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ScopeFeatureState {
        #[doc = "Output only. The “running state” of the Feature in this Scope."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::FeatureState>,
    }
    impl ::google_field_selector::FieldSelector for ScopeFeatureState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScopeFeatureState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ServiceMeshControlPlaneManagement {
        #[doc = "Explanation of state."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details: ::std::option::Option<Vec<crate::schemas::ServiceMeshStatusDetails>>,
        #[doc = "LifecycleState of control plane management."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::ServiceMeshControlPlaneManagementState>,
    }
    impl ::google_field_selector::FieldSelector for ServiceMeshControlPlaneManagement {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ServiceMeshControlPlaneManagement {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ServiceMeshControlPlaneManagementState {
        #[doc = "ACTIVE means that the component is ready for use."]
        Active,
        #[doc = "DEGRADED means that the component is ready, but operating in a degraded state."]
        Degraded,
        #[doc = "DISABLED means that the component is not enabled."]
        Disabled,
        #[doc = "FAILED_PRECONDITION means that provisioning cannot proceed because of some characteristic of the member cluster."]
        FailedPrecondition,
        #[doc = "Unspecified"]
        LifecycleStateUnspecified,
        #[doc = "NEEDS_ATTENTION means that the component is ready, but some user intervention is required. (For example that the user should migrate workloads to a new control plane revision.)"]
        NeedsAttention,
        #[doc = "PROVISIONING means that provisioning is in progress."]
        Provisioning,
        #[doc = "STALLED means that provisioning could not be done."]
        Stalled,
    }
    impl ServiceMeshControlPlaneManagementState {
        pub fn as_str(self) -> &'static str {
            match self {
                ServiceMeshControlPlaneManagementState::Active => "ACTIVE",
                ServiceMeshControlPlaneManagementState::Degraded => "DEGRADED",
                ServiceMeshControlPlaneManagementState::Disabled => "DISABLED",
                ServiceMeshControlPlaneManagementState::FailedPrecondition => "FAILED_PRECONDITION",
                ServiceMeshControlPlaneManagementState::LifecycleStateUnspecified => {
                    "LIFECYCLE_STATE_UNSPECIFIED"
                }
                ServiceMeshControlPlaneManagementState::NeedsAttention => "NEEDS_ATTENTION",
                ServiceMeshControlPlaneManagementState::Provisioning => "PROVISIONING",
                ServiceMeshControlPlaneManagementState::Stalled => "STALLED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ServiceMeshControlPlaneManagementState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ServiceMeshControlPlaneManagementState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ServiceMeshControlPlaneManagementState, ()> {
            Ok(match s {
                "ACTIVE" => ServiceMeshControlPlaneManagementState::Active,
                "DEGRADED" => ServiceMeshControlPlaneManagementState::Degraded,
                "DISABLED" => ServiceMeshControlPlaneManagementState::Disabled,
                "FAILED_PRECONDITION" => ServiceMeshControlPlaneManagementState::FailedPrecondition,
                "LIFECYCLE_STATE_UNSPECIFIED" => {
                    ServiceMeshControlPlaneManagementState::LifecycleStateUnspecified
                }
                "NEEDS_ATTENTION" => ServiceMeshControlPlaneManagementState::NeedsAttention,
                "PROVISIONING" => ServiceMeshControlPlaneManagementState::Provisioning,
                "STALLED" => ServiceMeshControlPlaneManagementState::Stalled,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ServiceMeshControlPlaneManagementState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ServiceMeshControlPlaneManagementState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ServiceMeshControlPlaneManagementState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVE" => ServiceMeshControlPlaneManagementState::Active,
                "DEGRADED" => ServiceMeshControlPlaneManagementState::Degraded,
                "DISABLED" => ServiceMeshControlPlaneManagementState::Disabled,
                "FAILED_PRECONDITION" => ServiceMeshControlPlaneManagementState::FailedPrecondition,
                "LIFECYCLE_STATE_UNSPECIFIED" => {
                    ServiceMeshControlPlaneManagementState::LifecycleStateUnspecified
                }
                "NEEDS_ATTENTION" => ServiceMeshControlPlaneManagementState::NeedsAttention,
                "PROVISIONING" => ServiceMeshControlPlaneManagementState::Provisioning,
                "STALLED" => ServiceMeshControlPlaneManagementState::Stalled,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ServiceMeshControlPlaneManagementState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ServiceMeshControlPlaneManagementState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ServiceMeshDataPlaneManagement {
        #[doc = "Explanation of the status."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details: ::std::option::Option<Vec<crate::schemas::ServiceMeshStatusDetails>>,
        #[doc = "Lifecycle status of data plane management."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::ServiceMeshDataPlaneManagementState>,
    }
    impl ::google_field_selector::FieldSelector for ServiceMeshDataPlaneManagement {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ServiceMeshDataPlaneManagement {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ServiceMeshDataPlaneManagementState {
        #[doc = "ACTIVE means that the component is ready for use."]
        Active,
        #[doc = "DEGRADED means that the component is ready, but operating in a degraded state."]
        Degraded,
        #[doc = "DISABLED means that the component is not enabled."]
        Disabled,
        #[doc = "FAILED_PRECONDITION means that provisioning cannot proceed because of some characteristic of the member cluster."]
        FailedPrecondition,
        #[doc = "Unspecified"]
        LifecycleStateUnspecified,
        #[doc = "NEEDS_ATTENTION means that the component is ready, but some user intervention is required. (For example that the user should migrate workloads to a new control plane revision.)"]
        NeedsAttention,
        #[doc = "PROVISIONING means that provisioning is in progress."]
        Provisioning,
        #[doc = "STALLED means that provisioning could not be done."]
        Stalled,
    }
    impl ServiceMeshDataPlaneManagementState {
        pub fn as_str(self) -> &'static str {
            match self {
                ServiceMeshDataPlaneManagementState::Active => "ACTIVE",
                ServiceMeshDataPlaneManagementState::Degraded => "DEGRADED",
                ServiceMeshDataPlaneManagementState::Disabled => "DISABLED",
                ServiceMeshDataPlaneManagementState::FailedPrecondition => "FAILED_PRECONDITION",
                ServiceMeshDataPlaneManagementState::LifecycleStateUnspecified => {
                    "LIFECYCLE_STATE_UNSPECIFIED"
                }
                ServiceMeshDataPlaneManagementState::NeedsAttention => "NEEDS_ATTENTION",
                ServiceMeshDataPlaneManagementState::Provisioning => "PROVISIONING",
                ServiceMeshDataPlaneManagementState::Stalled => "STALLED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ServiceMeshDataPlaneManagementState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ServiceMeshDataPlaneManagementState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ServiceMeshDataPlaneManagementState, ()> {
            Ok(match s {
                "ACTIVE" => ServiceMeshDataPlaneManagementState::Active,
                "DEGRADED" => ServiceMeshDataPlaneManagementState::Degraded,
                "DISABLED" => ServiceMeshDataPlaneManagementState::Disabled,
                "FAILED_PRECONDITION" => ServiceMeshDataPlaneManagementState::FailedPrecondition,
                "LIFECYCLE_STATE_UNSPECIFIED" => {
                    ServiceMeshDataPlaneManagementState::LifecycleStateUnspecified
                }
                "NEEDS_ATTENTION" => ServiceMeshDataPlaneManagementState::NeedsAttention,
                "PROVISIONING" => ServiceMeshDataPlaneManagementState::Provisioning,
                "STALLED" => ServiceMeshDataPlaneManagementState::Stalled,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ServiceMeshDataPlaneManagementState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ServiceMeshDataPlaneManagementState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ServiceMeshDataPlaneManagementState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVE" => ServiceMeshDataPlaneManagementState::Active,
                "DEGRADED" => ServiceMeshDataPlaneManagementState::Degraded,
                "DISABLED" => ServiceMeshDataPlaneManagementState::Disabled,
                "FAILED_PRECONDITION" => ServiceMeshDataPlaneManagementState::FailedPrecondition,
                "LIFECYCLE_STATE_UNSPECIFIED" => {
                    ServiceMeshDataPlaneManagementState::LifecycleStateUnspecified
                }
                "NEEDS_ATTENTION" => ServiceMeshDataPlaneManagementState::NeedsAttention,
                "PROVISIONING" => ServiceMeshDataPlaneManagementState::Provisioning,
                "STALLED" => ServiceMeshDataPlaneManagementState::Stalled,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ServiceMeshDataPlaneManagementState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ServiceMeshDataPlaneManagementState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ServiceMeshMembershipSpec {
        #[doc = "Enables automatic control plane management."]
        #[serde(
            rename = "controlPlane",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub control_plane:
            ::std::option::Option<crate::schemas::ServiceMeshMembershipSpecControlPlane>,
        #[doc = "Enables automatic Service Mesh management."]
        #[serde(
            rename = "management",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub management: ::std::option::Option<crate::schemas::ServiceMeshMembershipSpecManagement>,
    }
    impl ::google_field_selector::FieldSelector for ServiceMeshMembershipSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ServiceMeshMembershipSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ServiceMeshMembershipSpecControlPlane {
        #[doc = "Google should provision a control plane revision and make it available in the cluster. Google will enroll this revision in a release channel and keep it up to date. The control plane revision may be a managed service, or a managed install."]
        Automatic,
        #[doc = "Unspecified"]
        ControlPlaneManagementUnspecified,
        #[doc = "User will manually configure the control plane (e.g. via CLI, or via the ControlPlaneRevision KRM API)"]
        Manual,
    }
    impl ServiceMeshMembershipSpecControlPlane {
        pub fn as_str(self) -> &'static str {
            match self {
                ServiceMeshMembershipSpecControlPlane::Automatic => "AUTOMATIC",
                ServiceMeshMembershipSpecControlPlane::ControlPlaneManagementUnspecified => {
                    "CONTROL_PLANE_MANAGEMENT_UNSPECIFIED"
                }
                ServiceMeshMembershipSpecControlPlane::Manual => "MANUAL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ServiceMeshMembershipSpecControlPlane {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ServiceMeshMembershipSpecControlPlane {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ServiceMeshMembershipSpecControlPlane, ()> {
            Ok(match s {
                "AUTOMATIC" => ServiceMeshMembershipSpecControlPlane::Automatic,
                "CONTROL_PLANE_MANAGEMENT_UNSPECIFIED" => {
                    ServiceMeshMembershipSpecControlPlane::ControlPlaneManagementUnspecified
                }
                "MANUAL" => ServiceMeshMembershipSpecControlPlane::Manual,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ServiceMeshMembershipSpecControlPlane {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ServiceMeshMembershipSpecControlPlane {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ServiceMeshMembershipSpecControlPlane {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AUTOMATIC" => ServiceMeshMembershipSpecControlPlane::Automatic,
                "CONTROL_PLANE_MANAGEMENT_UNSPECIFIED" => {
                    ServiceMeshMembershipSpecControlPlane::ControlPlaneManagementUnspecified
                }
                "MANUAL" => ServiceMeshMembershipSpecControlPlane::Manual,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ServiceMeshMembershipSpecControlPlane {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ServiceMeshMembershipSpecControlPlane {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ServiceMeshMembershipSpecManagement {
        #[doc = "Google should manage my Service Mesh for the cluster."]
        ManagementAutomatic,
        #[doc = "User will manually configure their service mesh components."]
        ManagementManual,
        #[doc = "Unspecified"]
        ManagementUnspecified,
    }
    impl ServiceMeshMembershipSpecManagement {
        pub fn as_str(self) -> &'static str {
            match self {
                ServiceMeshMembershipSpecManagement::ManagementAutomatic => "MANAGEMENT_AUTOMATIC",
                ServiceMeshMembershipSpecManagement::ManagementManual => "MANAGEMENT_MANUAL",
                ServiceMeshMembershipSpecManagement::ManagementUnspecified => {
                    "MANAGEMENT_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for ServiceMeshMembershipSpecManagement {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ServiceMeshMembershipSpecManagement {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ServiceMeshMembershipSpecManagement, ()> {
            Ok(match s {
                "MANAGEMENT_AUTOMATIC" => ServiceMeshMembershipSpecManagement::ManagementAutomatic,
                "MANAGEMENT_MANUAL" => ServiceMeshMembershipSpecManagement::ManagementManual,
                "MANAGEMENT_UNSPECIFIED" => {
                    ServiceMeshMembershipSpecManagement::ManagementUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ServiceMeshMembershipSpecManagement {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ServiceMeshMembershipSpecManagement {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ServiceMeshMembershipSpecManagement {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MANAGEMENT_AUTOMATIC" => ServiceMeshMembershipSpecManagement::ManagementAutomatic,
                "MANAGEMENT_MANUAL" => ServiceMeshMembershipSpecManagement::ManagementManual,
                "MANAGEMENT_UNSPECIFIED" => {
                    ServiceMeshMembershipSpecManagement::ManagementUnspecified
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
    impl ::google_field_selector::FieldSelector for ServiceMeshMembershipSpecManagement {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ServiceMeshMembershipSpecManagement {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ServiceMeshMembershipState {
        #[doc = "Output only. Status of control plane management"]
        #[serde(
            rename = "controlPlaneManagement",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub control_plane_management:
            ::std::option::Option<crate::schemas::ServiceMeshControlPlaneManagement>,
        #[doc = "Output only. Status of data plane management."]
        #[serde(
            rename = "dataPlaneManagement",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_plane_management:
            ::std::option::Option<crate::schemas::ServiceMeshDataPlaneManagement>,
    }
    impl ::google_field_selector::FieldSelector for ServiceMeshMembershipState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ServiceMeshMembershipState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ServiceMeshStatusDetails {
        #[doc = "A machine-readable code that further describes a broad status."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<String>,
        #[doc = "Human-readable explanation of code."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ServiceMeshStatusDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ServiceMeshStatusDetails {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SetIamPolicyRequest {
        #[doc = "REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them."]
        #[serde(
            rename = "policy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy: ::std::option::Option<crate::schemas::Policy>,
        #[doc = "OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: \"bindings, etag\"`"]
        #[serde(
            rename = "updateMask",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_mask: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SetIamPolicyRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SetIamPolicyRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Status {
        #[doc = "Code specifies AppDevExperienceFeature’s subcomponent ready state."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<crate::schemas::StatusCode>,
        #[doc = "Description is populated if Code is Failed, explaining why it has failed."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
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
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum StatusCode {
        #[doc = "Not set."]
        CodeUnspecified,
        #[doc = "AppDevExperienceFeature’s specified subcomponent ready state is false. This means AppDevExperienceFeature has encountered an issue that blocks all, or a portion, of its normal operation. See the `description` for more details."]
        Failed,
        #[doc = "AppDevExperienceFeature’s specified subcomponent is ready."]
        Ok,
        #[doc = "AppDevExperienceFeature’s specified subcomponent has a pending or unknown state."]
        Unknown,
    }
    impl StatusCode {
        pub fn as_str(self) -> &'static str {
            match self {
                StatusCode::CodeUnspecified => "CODE_UNSPECIFIED",
                StatusCode::Failed => "FAILED",
                StatusCode::Ok => "OK",
                StatusCode::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for StatusCode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for StatusCode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<StatusCode, ()> {
            Ok(match s {
                "CODE_UNSPECIFIED" => StatusCode::CodeUnspecified,
                "FAILED" => StatusCode::Failed,
                "OK" => StatusCode::Ok,
                "UNKNOWN" => StatusCode::Unknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for StatusCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for StatusCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for StatusCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CODE_UNSPECIFIED" => StatusCode::CodeUnspecified,
                "FAILED" => StatusCode::Failed,
                "OK" => StatusCode::Ok,
                "UNKNOWN" => StatusCode::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for StatusCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StatusCode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TestIamPermissionsRequest {
        #[doc = "The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions)."]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for TestIamPermissionsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestIamPermissionsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TestIamPermissionsResponse {
        #[doc = "A subset of `TestPermissionsRequest.permissions` that the caller is allowed."]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for TestIamPermissionsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestIamPermissionsResponse {
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
            #[doc = "Actions that can be performed on the locations resource"]
            pub fn locations(&self) -> crate::resources::projects::locations::LocationsActions {
                crate::resources::projects::locations::LocationsActions {
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
                #[doc = "Actions that can be performed on the features resource"]
                pub fn features(
                    &self,
                ) -> crate::resources::projects::locations::features::FeaturesActions
                {
                    crate::resources::projects::locations::features::FeaturesActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
                #[doc = "Actions that can be performed on the memberships resource"]
                pub fn memberships(
                    &self,
                ) -> crate::resources::projects::locations::memberships::MembershipsActions
                {
                    crate::resources::projects::locations::memberships::MembershipsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
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
            }
            #[doc = "Created via [LocationsActions::get()](struct.LocationsActions.html#method.get)"]
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
                ) -> Result<crate::schemas::Location, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Location, crate::Error> {
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
                    let mut output = "https://gkehub.googleapis.com/".to_owned();
                    output.push_str("v1beta/");
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
                #[doc = "\nExecute the request and yield each item in the `locations` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                pub fn stream_locations<T>(
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
                    self.stream_locations_with_fields(fields)
                }
                #[doc = "\nExecute the request and yield each item in the `locations` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                pub fn stream_locations_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<Item = Result<crate::schemas::Location, crate::Error>> + 'a
                {
                    self.stream_locations_with_fields(None::<String>)
                }
                #[doc = "\nExecute the request and yield each item in the `locations` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                pub fn stream_locations_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<Item = Result<crate::schemas::Location, crate::Error>> + 'a
                {
                    self.stream_locations_with_fields(Some("*"))
                }
                #[doc = "\nExecute the request and yield each item in the `locations` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                pub fn stream_locations_with_fields<T, F>(
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
                        #[serde(rename = "locations")]
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
                        let mut selector = concat!("nextPageToken,", "locations").to_owned();
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
                    Item = Result<crate::schemas::ListLocationsResponse, crate::Error>,
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
                    Item = Result<crate::schemas::ListLocationsResponse, crate::Error>,
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
                ) -> Result<crate::schemas::ListLocationsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListLocationsResponse, crate::Error> {
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
                    let mut output = "https://gkehub.googleapis.com/".to_owned();
                    output.push_str("v1beta/");
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
            pub mod features {
                pub mod params {}
                pub struct FeaturesActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> FeaturesActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Adds a new Feature."]
                    pub fn create(
                        &self,
                        request: crate::schemas::Feature,
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
                            feature_id: None,
                            request_id: None,
                        }
                    }
                    #[doc = "Removes a Feature."]
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
                            force: None,
                            request_id: None,
                        }
                    }
                    #[doc = "Gets details of a single Feature."]
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
                    #[doc = "Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."]
                    pub fn get_iam_policy(
                        &self,
                        resource: impl Into<String>,
                    ) -> GetIamPolicyRequestBuilder {
                        GetIamPolicyRequestBuilder {
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
                            options_requested_policy_version: None,
                        }
                    }
                    #[doc = "Lists Features in a given project and location."]
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
                            order_by: None,
                            page_size: None,
                            page_token: None,
                        }
                    }
                    #[doc = "Updates an existing Feature."]
                    pub fn patch(
                        &self,
                        request: crate::schemas::Feature,
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
                            request_id: None,
                            update_mask: None,
                        }
                    }
                    #[doc = "Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors."]
                    pub fn set_iam_policy(
                        &self,
                        request: crate::schemas::SetIamPolicyRequest,
                        resource: impl Into<String>,
                    ) -> SetIamPolicyRequestBuilder {
                        SetIamPolicyRequestBuilder {
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
                            resource: resource.into(),
                        }
                    }
                    #[doc = "Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may “fail open” without warning."]
                    pub fn test_iam_permissions(
                        &self,
                        request: crate::schemas::TestIamPermissionsRequest,
                        resource: impl Into<String>,
                    ) -> TestIamPermissionsRequestBuilder {
                        TestIamPermissionsRequestBuilder {
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
                            resource: resource.into(),
                        }
                    }
                }
                #[doc = "Created via [FeaturesActions::create()](struct.FeaturesActions.html#method.create)"]
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::Feature,
                    parent: String,
                    feature_id: ::std::option::Option<String>,
                    request_id: ::std::option::Option<String>,
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
                    #[doc = "The ID of the feature to create."]
                    pub fn feature_id(mut self, value: impl Into<String>) -> Self {
                        self.feature_id = Some(value.into());
                        self
                    }
                    #[doc = "A request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. The server will guarantee that for at least 60 minutes after the first request. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments. The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000)."]
                    pub fn request_id(mut self, value: impl Into<String>) -> Self {
                        self.request_id = Some(value.into());
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
                        let mut output = "https://gkehub.googleapis.com/".to_owned();
                        output.push_str("v1beta/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/features");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                        req = req.query(&[("featureId", &self.feature_id)]);
                        req = req.query(&[("requestId", &self.request_id)]);
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
                #[doc = "Created via [FeaturesActions::delete()](struct.FeaturesActions.html#method.delete)"]
                #[derive(Debug, Clone)]
                pub struct DeleteRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    force: ::std::option::Option<bool>,
                    request_id: ::std::option::Option<String>,
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
                    #[doc = "If set to true, the delete will ignore any outstanding resources for this Feature (that is, `FeatureState.has_resources` is set to true). These resources will NOT be cleaned up or modified in any way."]
                    pub fn force(mut self, value: bool) -> Self {
                        self.force = Some(value);
                        self
                    }
                    #[doc = "Optional. A request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. The server will guarantee that for at least 60 minutes after the first request. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments. The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000)."]
                    pub fn request_id(mut self, value: impl Into<String>) -> Self {
                        self.request_id = Some(value.into());
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
                        let mut output = "https://gkehub.googleapis.com/".to_owned();
                        output.push_str("v1beta/");
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
                        req = req.query(&[("force", &self.force)]);
                        req = req.query(&[("requestId", &self.request_id)]);
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
                #[doc = "Created via [FeaturesActions::get()](struct.FeaturesActions.html#method.get)"]
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
                    ) -> Result<crate::schemas::Feature, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Feature, crate::Error> {
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
                        let mut output = "https://gkehub.googleapis.com/".to_owned();
                        output.push_str("v1beta/");
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
                #[doc = "Created via [FeaturesActions::get_iam_policy()](struct.FeaturesActions.html#method.get_iam_policy)"]
                #[derive(Debug, Clone)]
                pub struct GetIamPolicyRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    resource: String,
                    options_requested_policy_version: ::std::option::Option<i32>,
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
                impl<'a> GetIamPolicyRequestBuilder<'a> {
                    #[doc = "Optional. The maximum policy version that will be used to format the policy. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional role bindings must specify version 3. Policies with no conditional role bindings may specify any valid value or leave the field unset. The policy in the response might use the policy version that you specified, or it might use a lower policy version. For example, if you specify version 3, but the policy has no conditional role bindings, the response uses version 1. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
                    pub fn options_requested_policy_version(mut self, value: i32) -> Self {
                        self.options_requested_policy_version = Some(value);
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
                    ) -> Result<crate::schemas::Policy, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Policy, crate::Error> {
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
                        let mut output = "https://gkehub.googleapis.com/".to_owned();
                        output.push_str("v1beta/");
                        {
                            let var_as_str = &self.resource;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":getIamPolicy");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[(
                            "options.requestedPolicyVersion",
                            &self.options_requested_policy_version,
                        )]);
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
                #[doc = "Created via [FeaturesActions::list()](struct.FeaturesActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
                    filter: ::std::option::Option<String>,
                    order_by: ::std::option::Option<String>,
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
                    #[doc = "Lists Features that match the filter expression, following the syntax outlined in https://google.aip.dev/160. Examples: - Feature with the name “servicemesh” in project “foo-proj”: name = “projects/foo-proj/locations/global/features/servicemesh” - Features that have a label called `foo`: labels.foo:\\* - Features that have a label called `foo` whose value is `bar`: labels.foo = bar"]
                    pub fn filter(mut self, value: impl Into<String>) -> Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "One or more fields to compare and use to sort the output. See https://google.aip.dev/132#ordering."]
                    pub fn order_by(mut self, value: impl Into<String>) -> Self {
                        self.order_by = Some(value.into());
                        self
                    }
                    #[doc = "When requesting a ‘page’ of resources, `page_size` specifies number of resources to return. If unspecified or set to 0, all resources will be returned."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "Token returned by previous call to `ListFeatures` which specifies the position in the list from where to continue listing the resources."]
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
                    #[doc = "\nExecute the request and yield each item in the `resources` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                    pub fn stream_resources<T>(
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
                        self.stream_resources_with_fields(fields)
                    }
                    #[doc = "\nExecute the request and yield each item in the `resources` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                    pub fn stream_resources_with_default_fields(
                        self,
                    ) -> impl ::futures::Stream<Item = Result<crate::schemas::Feature, crate::Error>> + 'a
                    {
                        self.stream_resources_with_fields(None::<String>)
                    }
                    #[doc = "\nExecute the request and yield each item in the `resources` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                    pub fn stream_resources_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<Item = Result<crate::schemas::Feature, crate::Error>> + 'a
                    {
                        self.stream_resources_with_fields(Some("*"))
                    }
                    #[doc = "\nExecute the request and yield each item in the `resources` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                    pub fn stream_resources_with_fields<T, F>(
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
                            #[serde(rename = "resources")]
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
                            let mut selector = concat!("nextPageToken,", "resources").to_owned();
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
                        Item = Result<crate::schemas::ListFeaturesResponse, crate::Error>,
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
                        Item = Result<crate::schemas::ListFeaturesResponse, crate::Error>,
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
                    ) -> Result<crate::schemas::ListFeaturesResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListFeaturesResponse, crate::Error>
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
                        let mut output = "https://gkehub.googleapis.com/".to_owned();
                        output.push_str("v1beta/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/features");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[("filter", &self.filter)]);
                        req = req.query(&[("orderBy", &self.order_by)]);
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
                #[doc = "Created via [FeaturesActions::patch()](struct.FeaturesActions.html#method.patch)"]
                #[derive(Debug, Clone)]
                pub struct PatchRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::Feature,
                    name: String,
                    request_id: ::std::option::Option<String>,
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
                    #[doc = "A request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. The server will guarantee that for at least 60 minutes after the first request. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments. The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000)."]
                    pub fn request_id(mut self, value: impl Into<String>) -> Self {
                        self.request_id = Some(value.into());
                        self
                    }
                    #[doc = "Mask of fields to update."]
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
                        let mut output = "https://gkehub.googleapis.com/".to_owned();
                        output.push_str("v1beta/");
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
                        req = req.query(&[("requestId", &self.request_id)]);
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
                #[doc = "Created via [FeaturesActions::set_iam_policy()](struct.FeaturesActions.html#method.set_iam_policy)"]
                #[derive(Debug, Clone)]
                pub struct SetIamPolicyRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::SetIamPolicyRequest,
                    resource: String,
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
                impl<'a> SetIamPolicyRequestBuilder<'a> {
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
                    ) -> Result<crate::schemas::Policy, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Policy, crate::Error> {
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
                        let mut output = "https://gkehub.googleapis.com/".to_owned();
                        output.push_str("v1beta/");
                        {
                            let var_as_str = &self.resource;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":setIamPolicy");
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
                #[doc = "Created via [FeaturesActions::test_iam_permissions()](struct.FeaturesActions.html#method.test_iam_permissions)"]
                #[derive(Debug, Clone)]
                pub struct TestIamPermissionsRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::TestIamPermissionsRequest,
                    resource: String,
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
                impl<'a> TestIamPermissionsRequestBuilder<'a> {
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
                    ) -> Result<crate::schemas::TestIamPermissionsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::TestIamPermissionsResponse, crate::Error>
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
                        let mut output = "https://gkehub.googleapis.com/".to_owned();
                        output.push_str("v1beta/");
                        {
                            let var_as_str = &self.resource;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":testIamPermissions");
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
            pub mod memberships {
                pub mod params {}
                pub struct MembershipsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> MembershipsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."]
                    pub fn get_iam_policy(
                        &self,
                        resource: impl Into<String>,
                    ) -> GetIamPolicyRequestBuilder {
                        GetIamPolicyRequestBuilder {
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
                            options_requested_policy_version: None,
                        }
                    }
                    #[doc = "Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors."]
                    pub fn set_iam_policy(
                        &self,
                        request: crate::schemas::SetIamPolicyRequest,
                        resource: impl Into<String>,
                    ) -> SetIamPolicyRequestBuilder {
                        SetIamPolicyRequestBuilder {
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
                            resource: resource.into(),
                        }
                    }
                    #[doc = "Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may “fail open” without warning."]
                    pub fn test_iam_permissions(
                        &self,
                        request: crate::schemas::TestIamPermissionsRequest,
                        resource: impl Into<String>,
                    ) -> TestIamPermissionsRequestBuilder {
                        TestIamPermissionsRequestBuilder {
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
                            resource: resource.into(),
                        }
                    }
                }
                #[doc = "Created via [MembershipsActions::get_iam_policy()](struct.MembershipsActions.html#method.get_iam_policy)"]
                #[derive(Debug, Clone)]
                pub struct GetIamPolicyRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    resource: String,
                    options_requested_policy_version: ::std::option::Option<i32>,
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
                impl<'a> GetIamPolicyRequestBuilder<'a> {
                    #[doc = "Optional. The maximum policy version that will be used to format the policy. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional role bindings must specify version 3. Policies with no conditional role bindings may specify any valid value or leave the field unset. The policy in the response might use the policy version that you specified, or it might use a lower policy version. For example, if you specify version 3, but the policy has no conditional role bindings, the response uses version 1. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
                    pub fn options_requested_policy_version(mut self, value: i32) -> Self {
                        self.options_requested_policy_version = Some(value);
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
                    ) -> Result<crate::schemas::Policy, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Policy, crate::Error> {
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
                        let mut output = "https://gkehub.googleapis.com/".to_owned();
                        output.push_str("v1beta/");
                        {
                            let var_as_str = &self.resource;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":getIamPolicy");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[(
                            "options.requestedPolicyVersion",
                            &self.options_requested_policy_version,
                        )]);
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
                #[doc = "Created via [MembershipsActions::set_iam_policy()](struct.MembershipsActions.html#method.set_iam_policy)"]
                #[derive(Debug, Clone)]
                pub struct SetIamPolicyRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::SetIamPolicyRequest,
                    resource: String,
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
                impl<'a> SetIamPolicyRequestBuilder<'a> {
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
                    ) -> Result<crate::schemas::Policy, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Policy, crate::Error> {
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
                        let mut output = "https://gkehub.googleapis.com/".to_owned();
                        output.push_str("v1beta/");
                        {
                            let var_as_str = &self.resource;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":setIamPolicy");
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
                #[doc = "Created via [MembershipsActions::test_iam_permissions()](struct.MembershipsActions.html#method.test_iam_permissions)"]
                #[derive(Debug, Clone)]
                pub struct TestIamPermissionsRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::TestIamPermissionsRequest,
                    resource: String,
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
                impl<'a> TestIamPermissionsRequestBuilder<'a> {
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
                    ) -> Result<crate::schemas::TestIamPermissionsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::TestIamPermissionsResponse, crate::Error>
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
                        let mut output = "https://gkehub.googleapis.com/".to_owned();
                        output.push_str("v1beta/");
                        {
                            let var_as_str = &self.resource;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":testIamPermissions");
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
                    #[doc = "Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn’t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."]
                    pub fn cancel(
                        &self,
                        request: crate::schemas::CancelOperationRequest,
                        name: impl Into<String>,
                    ) -> CancelRequestBuilder {
                        CancelRequestBuilder {
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
                    #[doc = "Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn’t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`."]
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
                #[doc = "Created via [OperationsActions::cancel()](struct.OperationsActions.html#method.cancel)"]
                #[derive(Debug, Clone)]
                pub struct CancelRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::CancelOperationRequest,
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
                        let req = req.json(&self.request);
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://gkehub.googleapis.com/".to_owned();
                        output.push_str("v1beta/");
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
                #[doc = "Created via [OperationsActions::delete()](struct.OperationsActions.html#method.delete)"]
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
                        let mut output = "https://gkehub.googleapis.com/".to_owned();
                        output.push_str("v1beta/");
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
                        let mut output = "https://gkehub.googleapis.com/".to_owned();
                        output.push_str("v1beta/");
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
                    ) -> impl ::futures::Stream<Item = Result<crate::schemas::Operation, crate::Error>>
                           + 'a {
                        self.stream_operations_with_fields(None::<String>)
                    }
                    #[doc = "\nExecute the request and yield each item in the `operations` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                    pub fn stream_operations_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<Item = Result<crate::schemas::Operation, crate::Error>>
                           + 'a {
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
                        Item = Result<crate::schemas::ListOperationsResponse, crate::Error>,
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
                        Item = Result<crate::schemas::ListOperationsResponse, crate::Error>,
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
                    ) -> Result<crate::schemas::ListOperationsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListOperationsResponse, crate::Error>
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
                        let mut output = "https://gkehub.googleapis.com/".to_owned();
                        output.push_str("v1beta/");
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
