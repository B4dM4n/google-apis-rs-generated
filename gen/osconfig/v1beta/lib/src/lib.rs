#![doc = "# Resources and Methods\n    * [projects](resources/projects/struct.ProjectsActions.html)\n      * [guest_policies](resources/projects/guest_policies/struct.GuestPoliciesActions.html)\n        * [*create*](resources/projects/guest_policies/struct.CreateRequestBuilder.html), [*delete*](resources/projects/guest_policies/struct.DeleteRequestBuilder.html), [*get*](resources/projects/guest_policies/struct.GetRequestBuilder.html), [*list*](resources/projects/guest_policies/struct.ListRequestBuilder.html), [*patch*](resources/projects/guest_policies/struct.PatchRequestBuilder.html)\n      * [patch_deployments](resources/projects/patch_deployments/struct.PatchDeploymentsActions.html)\n        * [*create*](resources/projects/patch_deployments/struct.CreateRequestBuilder.html), [*delete*](resources/projects/patch_deployments/struct.DeleteRequestBuilder.html), [*get*](resources/projects/patch_deployments/struct.GetRequestBuilder.html), [*list*](resources/projects/patch_deployments/struct.ListRequestBuilder.html), [*patch*](resources/projects/patch_deployments/struct.PatchRequestBuilder.html), [*pause*](resources/projects/patch_deployments/struct.PauseRequestBuilder.html), [*resume*](resources/projects/patch_deployments/struct.ResumeRequestBuilder.html)\n      * [patch_jobs](resources/projects/patch_jobs/struct.PatchJobsActions.html)\n        * [*cancel*](resources/projects/patch_jobs/struct.CancelRequestBuilder.html), [*execute*](resources/projects/patch_jobs/struct.ExecuteRequestBuilder.html), [*get*](resources/projects/patch_jobs/struct.GetRequestBuilder.html), [*list*](resources/projects/patch_jobs/struct.ListRequestBuilder.html)\n        * [instance_details](resources/projects/patch_jobs/instance_details/struct.InstanceDetailsActions.html)\n          * [*list*](resources/projects/patch_jobs/instance_details/struct.ListRequestBuilder.html)\n      * [zones](resources/projects/zones/struct.ZonesActions.html)\n        * [instances](resources/projects/zones/instances/struct.InstancesActions.html)\n          * [*lookupEffectiveGuestPolicy*](resources/projects/zones/instances/struct.LookupEffectiveGuestPolicyRequestBuilder.html)\n"]
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
    pub struct AptRepository {
        #[doc = "Type of archive files in this repository. The default behavior is DEB."]
        #[serde(
            rename = "archiveType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub archive_type: ::std::option::Option<crate::schemas::AptRepositoryArchiveType>,
        #[doc = "Required. List of components for this repository. Must contain at least one item."]
        #[serde(
            rename = "components",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub components: ::std::option::Option<Vec<String>>,
        #[doc = "Required. Distribution of this repository."]
        #[serde(
            rename = "distribution",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub distribution: ::std::option::Option<String>,
        #[doc = "URI of the key file for this repository. The agent maintains a keyring at `/etc/apt/trusted.gpg.d/osconfig_agent_managed.gpg` containing all the keys in any applied guest policy."]
        #[serde(
            rename = "gpgKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gpg_key: ::std::option::Option<String>,
        #[doc = "Required. URI for this repository."]
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AptRepository {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AptRepository {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AptRepositoryArchiveType {
        #[doc = "Unspecified."]
        ArchiveTypeUnspecified,
        #[doc = "DEB indicates that the archive contains binary files."]
        Deb,
        #[doc = "DEB_SRC indicates that the archive contains source files."]
        DebSrc,
    }
    impl AptRepositoryArchiveType {
        pub fn as_str(self) -> &'static str {
            match self {
                AptRepositoryArchiveType::ArchiveTypeUnspecified => "ARCHIVE_TYPE_UNSPECIFIED",
                AptRepositoryArchiveType::Deb => "DEB",
                AptRepositoryArchiveType::DebSrc => "DEB_SRC",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AptRepositoryArchiveType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AptRepositoryArchiveType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AptRepositoryArchiveType, ()> {
            Ok(match s {
                "ARCHIVE_TYPE_UNSPECIFIED" => AptRepositoryArchiveType::ArchiveTypeUnspecified,
                "DEB" => AptRepositoryArchiveType::Deb,
                "DEB_SRC" => AptRepositoryArchiveType::DebSrc,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AptRepositoryArchiveType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AptRepositoryArchiveType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AptRepositoryArchiveType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ARCHIVE_TYPE_UNSPECIFIED" => AptRepositoryArchiveType::ArchiveTypeUnspecified,
                "DEB" => AptRepositoryArchiveType::Deb,
                "DEB_SRC" => AptRepositoryArchiveType::DebSrc,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AptRepositoryArchiveType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AptRepositoryArchiveType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AptSettings {
        #[doc = "List of packages to exclude from update. These packages will be excluded"]
        #[serde(
            rename = "excludes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub excludes: ::std::option::Option<Vec<String>>,
        #[doc = "An exclusive list of packages to be updated. These are the only packages that will be updated. If these packages are not installed, they will be ignored. This field cannot be specified with any other patch configuration fields."]
        #[serde(
            rename = "exclusivePackages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exclusive_packages: ::std::option::Option<Vec<String>>,
        #[doc = "By changing the type to DIST, the patching is performed using `apt-get dist-upgrade` instead."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::AptSettingsType>,
    }
    impl ::google_field_selector::FieldSelector for AptSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AptSettings {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AptSettingsType {
        #[doc = "Runs `apt-get dist-upgrade`."]
        Dist,
        #[doc = "By default, upgrade will be performed."]
        TypeUnspecified,
        #[doc = "Runs `apt-get upgrade`."]
        Upgrade,
    }
    impl AptSettingsType {
        pub fn as_str(self) -> &'static str {
            match self {
                AptSettingsType::Dist => "DIST",
                AptSettingsType::TypeUnspecified => "TYPE_UNSPECIFIED",
                AptSettingsType::Upgrade => "UPGRADE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AptSettingsType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AptSettingsType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AptSettingsType, ()> {
            Ok(match s {
                "DIST" => AptSettingsType::Dist,
                "TYPE_UNSPECIFIED" => AptSettingsType::TypeUnspecified,
                "UPGRADE" => AptSettingsType::Upgrade,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AptSettingsType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AptSettingsType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AptSettingsType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DIST" => AptSettingsType::Dist,
                "TYPE_UNSPECIFIED" => AptSettingsType::TypeUnspecified,
                "UPGRADE" => AptSettingsType::Upgrade,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AptSettingsType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AptSettingsType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Assignment {
        #[doc = "Targets instances matching at least one of these label sets. This allows an assignment to target disparate groups, for example \"env=prod or env=staging\"."]
        #[serde(
            rename = "groupLabels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group_labels: ::std::option::Option<Vec<crate::schemas::AssignmentGroupLabel>>,
        #[doc = "Targets VM instances whose name starts with one of these prefixes. Like labels, this is another way to group VM instances when targeting configs, for example prefix=\"prod-\". Only supported for project-level policies."]
        #[serde(
            rename = "instanceNamePrefixes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instance_name_prefixes: ::std::option::Option<Vec<String>>,
        #[doc = "Targets any of the instances specified. Instances are specified by their URI in the form `zones/[ZONE]/instances/[INSTANCE_NAME]`. Instance targeting is uncommon and is supported to facilitate the management of changes by the instance or to target specific VM instances for development and testing. Only supported for project-level policies and must reference instances within this project."]
        #[serde(
            rename = "instances",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instances: ::std::option::Option<Vec<String>>,
        #[doc = "Targets VM instances matching at least one of the following OS types. VM instances must match all supplied criteria for a given OsType to be included."]
        #[serde(
            rename = "osTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_types: ::std::option::Option<Vec<crate::schemas::AssignmentOsType>>,
        #[doc = "Targets instances in any of these zones. Leave empty to target instances in any zone. Zonal targeting is uncommon and is supported to facilitate the management of changes by zone."]
        #[serde(
            rename = "zones",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub zones: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for Assignment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Assignment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AssignmentGroupLabel {
        #[doc = "Google Compute Engine instance labels that must be present for an instance to be included in this assignment group."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::google_field_selector::FieldSelector for AssignmentGroupLabel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AssignmentGroupLabel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AssignmentOsType {
        #[doc = "Targets VM instances with OS Inventory enabled and having the following OS architecture."]
        #[serde(
            rename = "osArchitecture",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_architecture: ::std::option::Option<String>,
        #[doc = "Targets VM instances with OS Inventory enabled and having the following OS short name, for example \"debian\" or \"windows\"."]
        #[serde(
            rename = "osShortName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_short_name: ::std::option::Option<String>,
        #[doc = "Targets VM instances with OS Inventory enabled and having the following following OS version."]
        #[serde(
            rename = "osVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AssignmentOsType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AssignmentOsType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct CancelPatchJobRequest {}
    impl ::google_field_selector::FieldSelector for CancelPatchJobRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CancelPatchJobRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct EffectiveGuestPolicy {
        #[doc = "List of package repository configurations assigned to the VM instance."]
        #[serde(
            rename = "packageRepositories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_repositories: ::std::option::Option<
            Vec<crate::schemas::EffectiveGuestPolicySourcedPackageRepository>,
        >,
        #[doc = "List of package configurations assigned to the VM instance."]
        #[serde(
            rename = "packages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub packages:
            ::std::option::Option<Vec<crate::schemas::EffectiveGuestPolicySourcedPackage>>,
        #[doc = "List of recipes assigned to the VM instance."]
        #[serde(
            rename = "softwareRecipes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub software_recipes:
            ::std::option::Option<Vec<crate::schemas::EffectiveGuestPolicySourcedSoftwareRecipe>>,
    }
    impl ::google_field_selector::FieldSelector for EffectiveGuestPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EffectiveGuestPolicy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct EffectiveGuestPolicySourcedPackage {
        #[doc = "A software package to configure on the VM instance."]
        #[serde(
            rename = "package",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package: ::std::option::Option<crate::schemas::Package>,
        #[doc = "Name of the guest policy providing this config."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EffectiveGuestPolicySourcedPackage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EffectiveGuestPolicySourcedPackage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct EffectiveGuestPolicySourcedPackageRepository {
        #[doc = "A software package repository to configure on the VM instance."]
        #[serde(
            rename = "packageRepository",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_repository: ::std::option::Option<crate::schemas::PackageRepository>,
        #[doc = "Name of the guest policy providing this config."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EffectiveGuestPolicySourcedPackageRepository {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EffectiveGuestPolicySourcedPackageRepository {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct EffectiveGuestPolicySourcedSoftwareRecipe {
        #[doc = "A software recipe to configure on the VM instance."]
        #[serde(
            rename = "softwareRecipe",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub software_recipe: ::std::option::Option<crate::schemas::SoftwareRecipe>,
        #[doc = "Name of the guest policy providing this config."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EffectiveGuestPolicySourcedSoftwareRecipe {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EffectiveGuestPolicySourcedSoftwareRecipe {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct ExecStep {
        #[doc = "The ExecStepConfig for all Linux VMs targeted by the PatchJob."]
        #[serde(
            rename = "linuxExecStepConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub linux_exec_step_config: ::std::option::Option<crate::schemas::ExecStepConfig>,
        #[doc = "The ExecStepConfig for all Windows VMs targeted by the PatchJob."]
        #[serde(
            rename = "windowsExecStepConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub windows_exec_step_config: ::std::option::Option<crate::schemas::ExecStepConfig>,
    }
    impl ::google_field_selector::FieldSelector for ExecStep {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExecStep {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ExecStepConfig {
        #[doc = "Defaults to [0]. A list of possible return values that the execution can return to indicate a success."]
        #[serde(
            rename = "allowedSuccessCodes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allowed_success_codes: ::std::option::Option<Vec<i32>>,
        #[doc = "A Google Cloud Storage object containing the executable."]
        #[serde(
            rename = "gcsObject",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_object: ::std::option::Option<crate::schemas::GcsObject>,
        #[doc = "The script interpreter to use to run the script. If no interpreter is specified the script will be executed directly, which will likely only succeed for scripts with [shebang lines] (https://en.wikipedia.org/wiki/Shebang_(Unix))."]
        #[serde(
            rename = "interpreter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub interpreter: ::std::option::Option<crate::schemas::ExecStepConfigInterpreter>,
        #[doc = "An absolute path to the executable on the VM."]
        #[serde(
            rename = "localPath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub local_path: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ExecStepConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExecStepConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ExecStepConfigInterpreter {
        #[doc = "If the interpreter is not specified, the value defaults to `NONE`."]
        InterpreterUnspecified,
        #[doc = "Indicates that the file is run as follows on each operating system: + For Linux VMs, the file is ran as an executable and the interpreter might be parsed from the [shebang line](https://wikipedia.org/wiki/Shebang_(Unix)) of the file. + For Windows VM, this value is not supported."]
        None,
        #[doc = "Indicates that the file is run with PowerShell."]
        Powershell,
        #[doc = "Indicates that the file is run with `/bin/sh` on Linux and `cmd` on Windows."]
        Shell,
    }
    impl ExecStepConfigInterpreter {
        pub fn as_str(self) -> &'static str {
            match self {
                ExecStepConfigInterpreter::InterpreterUnspecified => "INTERPRETER_UNSPECIFIED",
                ExecStepConfigInterpreter::None => "NONE",
                ExecStepConfigInterpreter::Powershell => "POWERSHELL",
                ExecStepConfigInterpreter::Shell => "SHELL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ExecStepConfigInterpreter {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ExecStepConfigInterpreter {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ExecStepConfigInterpreter, ()> {
            Ok(match s {
                "INTERPRETER_UNSPECIFIED" => ExecStepConfigInterpreter::InterpreterUnspecified,
                "NONE" => ExecStepConfigInterpreter::None,
                "POWERSHELL" => ExecStepConfigInterpreter::Powershell,
                "SHELL" => ExecStepConfigInterpreter::Shell,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ExecStepConfigInterpreter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ExecStepConfigInterpreter {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ExecStepConfigInterpreter {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INTERPRETER_UNSPECIFIED" => ExecStepConfigInterpreter::InterpreterUnspecified,
                "NONE" => ExecStepConfigInterpreter::None,
                "POWERSHELL" => ExecStepConfigInterpreter::Powershell,
                "SHELL" => ExecStepConfigInterpreter::Shell,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ExecStepConfigInterpreter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExecStepConfigInterpreter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ExecutePatchJobRequest {
        #[doc = "Description of the patch job. Length of the description is limited to 1024 characters."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Display name for this patch job. This does not have to be unique."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "If this patch is a dry-run only, instances are contacted but will do nothing."]
        #[serde(
            rename = "dryRun",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dry_run: ::std::option::Option<bool>,
        #[doc = "Duration of the patch job. After the duration ends, the patch job times out."]
        #[serde(
            rename = "duration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub duration: ::std::option::Option<String>,
        #[doc = "Required. Instances to patch, either explicitly or filtered by some criteria such as zone or labels."]
        #[serde(
            rename = "instanceFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instance_filter: ::std::option::Option<crate::schemas::PatchInstanceFilter>,
        #[doc = "Patch configuration being applied. If omitted, instances are patched using the default configurations."]
        #[serde(
            rename = "patchConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub patch_config: ::std::option::Option<crate::schemas::PatchConfig>,
        #[doc = "Rollout strategy of the patch job."]
        #[serde(
            rename = "rollout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rollout: ::std::option::Option<crate::schemas::PatchRollout>,
    }
    impl ::google_field_selector::FieldSelector for ExecutePatchJobRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExecutePatchJobRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FixedOrPercent {
        #[doc = "Specifies a fixed value."]
        #[serde(
            rename = "fixed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fixed: ::std::option::Option<i32>,
        #[doc = "Specifies the relative value defined as a percentage, which will be multiplied by a reference value."]
        #[serde(
            rename = "percent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub percent: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for FixedOrPercent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FixedOrPercent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GcsObject {
        #[doc = "Required. Bucket of the Google Cloud Storage object."]
        #[serde(
            rename = "bucket",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucket: ::std::option::Option<String>,
        #[doc = "Required. Generation number of the Google Cloud Storage object. This is used to ensure that the ExecStep specified by this PatchJob does not change."]
        #[serde(
            rename = "generationNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub generation_number: ::std::option::Option<i64>,
        #[doc = "Required. Name of the Google Cloud Storage object."]
        #[serde(
            rename = "object",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GcsObject {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GcsObject {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GooRepository {
        #[doc = "Required. The name of the repository."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Required. The url of the repository."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooRepository {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooRepository {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct GooSettings {}
    impl ::google_field_selector::FieldSelector for GooSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooSettings {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadata {
        #[doc = "The OS policy assignment API method."]
        #[serde(
            rename = "apiMethod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub api_method: ::std::option::Option<
            crate::schemas::GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataApiMethod,
        >,
        #[doc = "Reference to the `OSPolicyAssignment` API resource. Format: `projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id@revision_id}`"]
        #[serde(
            rename = "osPolicyAssignment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_policy_assignment: ::std::option::Option<String>,
        #[doc = "Rollout start time"]
        #[serde(
            rename = "rolloutStartTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rollout_start_time: ::std::option::Option<String>,
        #[doc = "State of the rollout"]
        #[serde(
            rename = "rolloutState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rollout_state: ::std::option::Option<
            crate::schemas::GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState,
        >,
        #[doc = "Rollout update time"]
        #[serde(
            rename = "rolloutUpdateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rollout_update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadata
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataApiMethod {
        #[doc = "Invalid value"]
        ApiMethodUnspecified,
        #[doc = "Create OS policy assignment API method"]
        Create,
        #[doc = "Delete OS policy assignment API method"]
        Delete,
        #[doc = "Update OS policy assignment API method"]
        Update,
    }
    impl GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataApiMethod {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataApiMethod :: ApiMethodUnspecified => "API_METHOD_UNSPECIFIED" , GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataApiMethod :: Create => "CREATE" , GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataApiMethod :: Delete => "DELETE" , GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataApiMethod :: Update => "UPDATE" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataApiMethod
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataApiMethod {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataApiMethod,
            (),
        > {
            Ok (match s { "API_METHOD_UNSPECIFIED" => GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataApiMethod :: ApiMethodUnspecified , "CREATE" => GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataApiMethod :: Create , "DELETE" => GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataApiMethod :: Delete , "UPDATE" => GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataApiMethod :: Update , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataApiMethod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataApiMethod {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataApiMethod
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "API_METHOD_UNSPECIFIED" => GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataApiMethod :: ApiMethodUnspecified , "CREATE" => GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataApiMethod :: Create , "DELETE" => GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataApiMethod :: Delete , "UPDATE" => GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataApiMethod :: Update , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataApiMethod
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataApiMethod
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState {
        #[doc = "The rollout is cancelled."]
        Cancelled,
        #[doc = "The rollout is being cancelled."]
        Cancelling,
        #[doc = "The rollout is in progress."]
        InProgress,
        #[doc = "Invalid value"]
        RolloutStateUnspecified,
        #[doc = "The rollout has completed successfully."]
        Succeeded,
    }
    impl GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState :: Cancelled => "CANCELLED" , GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState :: Cancelling => "CANCELLING" , GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState :: InProgress => "IN_PROGRESS" , GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState :: RolloutStateUnspecified => "ROLLOUT_STATE_UNSPECIFIED" , GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState :: Succeeded => "SUCCEEDED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState,
            (),
        > {
            Ok (match s { "CANCELLED" => GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState :: Cancelled , "CANCELLING" => GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState :: Cancelling , "IN_PROGRESS" => GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState :: InProgress , "ROLLOUT_STATE_UNSPECIFIED" => GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState :: RolloutStateUnspecified , "SUCCEEDED" => GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState :: Succeeded , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CANCELLED" => GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState :: Cancelled , "CANCELLING" => GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState :: Cancelling , "IN_PROGRESS" => GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState :: InProgress , "ROLLOUT_STATE_UNSPECIFIED" => GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState :: RolloutStateUnspecified , "SUCCEEDED" => GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState :: Succeeded , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudOsconfigV1OSPolicyAssignmentOperationMetadataRolloutState
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
    pub struct GuestPolicy {
        #[doc = "Required. Specifies the VM instances that are assigned to this policy. This allows you to target sets or groups of VM instances by different parameters such as labels, names, OS, or zones. If left empty, all VM instances underneath this policy are targeted. At the same level in the resource hierarchy (that is within a project), the service prevents the creation of multiple policies that conflict with each other. For more information, see how the service [handles assignment conflicts](/compute/docs/os-config-management/create-guest-policy#handle-conflicts)."]
        #[serde(
            rename = "assignment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub assignment: ::std::option::Option<crate::schemas::Assignment>,
        #[doc = "Output only. Time this guest policy was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Description of the guest policy. Length of the description is limited to 1024 characters."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The etag for this guest policy. If this is provided on update, it must match the server's etag."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "Required. Unique name of the resource in this project using one of the following forms: `projects/{project_number}/guestPolicies/{guest_policy_id}`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "A list of package repositories to configure on the VM instance. This is done before any other configs are applied so they can use these repos. Package repositories are only configured if the corresponding package manager(s) are available."]
        #[serde(
            rename = "packageRepositories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_repositories: ::std::option::Option<Vec<crate::schemas::PackageRepository>>,
        #[doc = "The software packages to be managed by this policy."]
        #[serde(
            rename = "packages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub packages: ::std::option::Option<Vec<crate::schemas::Package>>,
        #[doc = "A list of Recipes to install on the VM instance."]
        #[serde(
            rename = "recipes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recipes: ::std::option::Option<Vec<crate::schemas::SoftwareRecipe>>,
        #[doc = "Output only. Last time this guest policy was updated."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GuestPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GuestPolicy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListGuestPoliciesResponse {
        #[doc = "The list of GuestPolicies."]
        #[serde(
            rename = "guestPolicies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub guest_policies: ::std::option::Option<Vec<crate::schemas::GuestPolicy>>,
        #[doc = "A pagination token that can be used to get the next page of guest policies."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListGuestPoliciesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListGuestPoliciesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListPatchDeploymentsResponse {
        #[doc = "A pagination token that can be used to get the next page of patch deployments."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The list of patch deployments."]
        #[serde(
            rename = "patchDeployments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub patch_deployments: ::std::option::Option<Vec<crate::schemas::PatchDeployment>>,
    }
    impl ::google_field_selector::FieldSelector for ListPatchDeploymentsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListPatchDeploymentsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListPatchJobInstanceDetailsResponse {
        #[doc = "A pagination token that can be used to get the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "A list of instance status."]
        #[serde(
            rename = "patchJobInstanceDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub patch_job_instance_details:
            ::std::option::Option<Vec<crate::schemas::PatchJobInstanceDetails>>,
    }
    impl ::google_field_selector::FieldSelector for ListPatchJobInstanceDetailsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListPatchJobInstanceDetailsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListPatchJobsResponse {
        #[doc = "A pagination token that can be used to get the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The list of patch jobs."]
        #[serde(
            rename = "patchJobs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub patch_jobs: ::std::option::Option<Vec<crate::schemas::PatchJob>>,
    }
    impl ::google_field_selector::FieldSelector for ListPatchJobsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListPatchJobsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct LookupEffectiveGuestPolicyRequest {
        #[doc = "Architecture of OS running on the instance. The OS Config agent only provides this field for targeting if OS Inventory is enabled for that instance."]
        #[serde(
            rename = "osArchitecture",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_architecture: ::std::option::Option<String>,
        #[doc = "Short name of the OS running on the instance. The OS Config agent only provides this field for targeting if OS Inventory is enabled for that instance."]
        #[serde(
            rename = "osShortName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_short_name: ::std::option::Option<String>,
        #[doc = "Version of the OS running on the instance. The OS Config agent only provides this field for targeting if OS Inventory is enabled for that VM instance."]
        #[serde(
            rename = "osVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for LookupEffectiveGuestPolicyRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LookupEffectiveGuestPolicyRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MonthlySchedule {
        #[doc = "Required. One day of the month. 1-31 indicates the 1st to the 31st day. -1 indicates the last day of the month. Months without the target day will be skipped. For example, a schedule to run \"every month on the 31st\" will not run in February, April, June, etc."]
        #[serde(
            rename = "monthDay",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub month_day: ::std::option::Option<i32>,
        #[doc = "Required. Week day in a month."]
        #[serde(
            rename = "weekDayOfMonth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub week_day_of_month: ::std::option::Option<crate::schemas::WeekDayOfMonth>,
    }
    impl ::google_field_selector::FieldSelector for MonthlySchedule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MonthlySchedule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OneTimeSchedule {
        #[doc = "Required. The desired patch job execution time."]
        #[serde(
            rename = "executeTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub execute_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OneTimeSchedule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OneTimeSchedule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyAssignmentOperationMetadata {
        #[doc = "The OS policy assignment API method."]
        #[serde(
            rename = "apiMethod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub api_method:
            ::std::option::Option<crate::schemas::OspolicyAssignmentOperationMetadataApiMethod>,
        #[doc = "Reference to the `OSPolicyAssignment` API resource. Format: `projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id@revision_id}`"]
        #[serde(
            rename = "osPolicyAssignment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_policy_assignment: ::std::option::Option<String>,
        #[doc = "Rollout start time"]
        #[serde(
            rename = "rolloutStartTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rollout_start_time: ::std::option::Option<String>,
        #[doc = "State of the rollout"]
        #[serde(
            rename = "rolloutState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rollout_state:
            ::std::option::Option<crate::schemas::OspolicyAssignmentOperationMetadataRolloutState>,
        #[doc = "Rollout update time"]
        #[serde(
            rename = "rolloutUpdateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rollout_update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyAssignmentOperationMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyAssignmentOperationMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OspolicyAssignmentOperationMetadataApiMethod {
        #[doc = "Invalid value"]
        ApiMethodUnspecified,
        #[doc = "Create OS policy assignment API method"]
        Create,
        #[doc = "Delete OS policy assignment API method"]
        Delete,
        #[doc = "Update OS policy assignment API method"]
        Update,
    }
    impl OspolicyAssignmentOperationMetadataApiMethod {
        pub fn as_str(self) -> &'static str {
            match self {
                OspolicyAssignmentOperationMetadataApiMethod::ApiMethodUnspecified => {
                    "API_METHOD_UNSPECIFIED"
                }
                OspolicyAssignmentOperationMetadataApiMethod::Create => "CREATE",
                OspolicyAssignmentOperationMetadataApiMethod::Delete => "DELETE",
                OspolicyAssignmentOperationMetadataApiMethod::Update => "UPDATE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OspolicyAssignmentOperationMetadataApiMethod {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OspolicyAssignmentOperationMetadataApiMethod {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<OspolicyAssignmentOperationMetadataApiMethod, ()> {
            Ok(match s {
                "API_METHOD_UNSPECIFIED" => {
                    OspolicyAssignmentOperationMetadataApiMethod::ApiMethodUnspecified
                }
                "CREATE" => OspolicyAssignmentOperationMetadataApiMethod::Create,
                "DELETE" => OspolicyAssignmentOperationMetadataApiMethod::Delete,
                "UPDATE" => OspolicyAssignmentOperationMetadataApiMethod::Update,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OspolicyAssignmentOperationMetadataApiMethod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OspolicyAssignmentOperationMetadataApiMethod {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OspolicyAssignmentOperationMetadataApiMethod {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "API_METHOD_UNSPECIFIED" => {
                    OspolicyAssignmentOperationMetadataApiMethod::ApiMethodUnspecified
                }
                "CREATE" => OspolicyAssignmentOperationMetadataApiMethod::Create,
                "DELETE" => OspolicyAssignmentOperationMetadataApiMethod::Delete,
                "UPDATE" => OspolicyAssignmentOperationMetadataApiMethod::Update,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for OspolicyAssignmentOperationMetadataApiMethod {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyAssignmentOperationMetadataApiMethod {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OspolicyAssignmentOperationMetadataRolloutState {
        #[doc = "The rollout is cancelled."]
        Cancelled,
        #[doc = "The rollout is being cancelled."]
        Cancelling,
        #[doc = "The rollout is in progress."]
        InProgress,
        #[doc = "Invalid value"]
        RolloutStateUnspecified,
        #[doc = "The rollout has completed successfully."]
        Succeeded,
    }
    impl OspolicyAssignmentOperationMetadataRolloutState {
        pub fn as_str(self) -> &'static str {
            match self {
                OspolicyAssignmentOperationMetadataRolloutState::Cancelled => "CANCELLED",
                OspolicyAssignmentOperationMetadataRolloutState::Cancelling => "CANCELLING",
                OspolicyAssignmentOperationMetadataRolloutState::InProgress => "IN_PROGRESS",
                OspolicyAssignmentOperationMetadataRolloutState::RolloutStateUnspecified => {
                    "ROLLOUT_STATE_UNSPECIFIED"
                }
                OspolicyAssignmentOperationMetadataRolloutState::Succeeded => "SUCCEEDED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OspolicyAssignmentOperationMetadataRolloutState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OspolicyAssignmentOperationMetadataRolloutState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<OspolicyAssignmentOperationMetadataRolloutState, ()> {
            Ok(match s {
                "CANCELLED" => OspolicyAssignmentOperationMetadataRolloutState::Cancelled,
                "CANCELLING" => OspolicyAssignmentOperationMetadataRolloutState::Cancelling,
                "IN_PROGRESS" => OspolicyAssignmentOperationMetadataRolloutState::InProgress,
                "ROLLOUT_STATE_UNSPECIFIED" => {
                    OspolicyAssignmentOperationMetadataRolloutState::RolloutStateUnspecified
                }
                "SUCCEEDED" => OspolicyAssignmentOperationMetadataRolloutState::Succeeded,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OspolicyAssignmentOperationMetadataRolloutState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OspolicyAssignmentOperationMetadataRolloutState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OspolicyAssignmentOperationMetadataRolloutState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => OspolicyAssignmentOperationMetadataRolloutState::Cancelled,
                "CANCELLING" => OspolicyAssignmentOperationMetadataRolloutState::Cancelling,
                "IN_PROGRESS" => OspolicyAssignmentOperationMetadataRolloutState::InProgress,
                "ROLLOUT_STATE_UNSPECIFIED" => {
                    OspolicyAssignmentOperationMetadataRolloutState::RolloutStateUnspecified
                }
                "SUCCEEDED" => OspolicyAssignmentOperationMetadataRolloutState::Succeeded,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for OspolicyAssignmentOperationMetadataRolloutState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyAssignmentOperationMetadataRolloutState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Package {
        #[doc = "The desired_state the agent should maintain for this package. The default is to ensure the package is installed."]
        #[serde(
            rename = "desiredState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub desired_state: ::std::option::Option<crate::schemas::PackageDesiredState>,
        #[doc = "Type of package manager that can be used to install this package. If a system does not have the package manager, the package is not installed or removed no error message is returned. By default, or if you specify `ANY`, the agent attempts to install and remove this package using the default package manager. This is useful when creating a policy that applies to different types of systems. The default behavior is ANY."]
        #[serde(
            rename = "manager",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub manager: ::std::option::Option<crate::schemas::PackageManager>,
        #[doc = "Required. The name of the package. A package is uniquely identified for conflict validation by checking the package name and the manager(s) that the package targets."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Package {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Package {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PackageDesiredState {
        #[doc = "The default is to ensure the package is installed."]
        DesiredStateUnspecified,
        #[doc = "The agent ensures that the package is installed."]
        Installed,
        #[doc = "The agent ensures that the package is not installed and uninstall it if detected."]
        Removed,
        #[doc = "The agent ensures that the package is installed and periodically checks for and install any updates."]
        Updated,
    }
    impl PackageDesiredState {
        pub fn as_str(self) -> &'static str {
            match self {
                PackageDesiredState::DesiredStateUnspecified => "DESIRED_STATE_UNSPECIFIED",
                PackageDesiredState::Installed => "INSTALLED",
                PackageDesiredState::Removed => "REMOVED",
                PackageDesiredState::Updated => "UPDATED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PackageDesiredState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PackageDesiredState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PackageDesiredState, ()> {
            Ok(match s {
                "DESIRED_STATE_UNSPECIFIED" => PackageDesiredState::DesiredStateUnspecified,
                "INSTALLED" => PackageDesiredState::Installed,
                "REMOVED" => PackageDesiredState::Removed,
                "UPDATED" => PackageDesiredState::Updated,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PackageDesiredState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PackageDesiredState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PackageDesiredState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DESIRED_STATE_UNSPECIFIED" => PackageDesiredState::DesiredStateUnspecified,
                "INSTALLED" => PackageDesiredState::Installed,
                "REMOVED" => PackageDesiredState::Removed,
                "UPDATED" => PackageDesiredState::Updated,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PackageDesiredState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PackageDesiredState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PackageManager {
        #[doc = "Apply this package config using the default system package manager."]
        Any,
        #[doc = "Apply this package config only if Apt is available on the system."]
        Apt,
        #[doc = "Apply this package config only if GooGet is available on the system."]
        Goo,
        #[doc = "The default behavior is ANY."]
        ManagerUnspecified,
        #[doc = "Apply this package config only if Yum is available on the system."]
        Yum,
        #[doc = "Apply this package config only if Zypper is available on the system."]
        Zypper,
    }
    impl PackageManager {
        pub fn as_str(self) -> &'static str {
            match self {
                PackageManager::Any => "ANY",
                PackageManager::Apt => "APT",
                PackageManager::Goo => "GOO",
                PackageManager::ManagerUnspecified => "MANAGER_UNSPECIFIED",
                PackageManager::Yum => "YUM",
                PackageManager::Zypper => "ZYPPER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PackageManager {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PackageManager {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PackageManager, ()> {
            Ok(match s {
                "ANY" => PackageManager::Any,
                "APT" => PackageManager::Apt,
                "GOO" => PackageManager::Goo,
                "MANAGER_UNSPECIFIED" => PackageManager::ManagerUnspecified,
                "YUM" => PackageManager::Yum,
                "ZYPPER" => PackageManager::Zypper,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PackageManager {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PackageManager {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PackageManager {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANY" => PackageManager::Any,
                "APT" => PackageManager::Apt,
                "GOO" => PackageManager::Goo,
                "MANAGER_UNSPECIFIED" => PackageManager::ManagerUnspecified,
                "YUM" => PackageManager::Yum,
                "ZYPPER" => PackageManager::Zypper,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PackageManager {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PackageManager {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PackageRepository {
        #[doc = "An Apt Repository."]
        #[serde(
            rename = "apt",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub apt: ::std::option::Option<crate::schemas::AptRepository>,
        #[doc = "A Goo Repository."]
        #[serde(
            rename = "goo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub goo: ::std::option::Option<crate::schemas::GooRepository>,
        #[doc = "A Yum Repository."]
        #[serde(
            rename = "yum",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub yum: ::std::option::Option<crate::schemas::YumRepository>,
        #[doc = "A Zypper Repository."]
        #[serde(
            rename = "zypper",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub zypper: ::std::option::Option<crate::schemas::ZypperRepository>,
    }
    impl ::google_field_selector::FieldSelector for PackageRepository {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PackageRepository {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PatchConfig {
        #[doc = "Apt update settings. Use this setting to override the default `apt` patch rules."]
        #[serde(
            rename = "apt",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub apt: ::std::option::Option<crate::schemas::AptSettings>,
        #[doc = "Goo update settings. Use this setting to override the default `goo` patch rules."]
        #[serde(
            rename = "goo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub goo: ::std::option::Option<crate::schemas::GooSettings>,
        #[doc = "Allows the patch job to run on Managed instance groups (MIGs)."]
        #[serde(
            rename = "migInstancesAllowed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mig_instances_allowed: ::std::option::Option<bool>,
        #[doc = "The `ExecStep` to run after the patch update."]
        #[serde(
            rename = "postStep",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub post_step: ::std::option::Option<crate::schemas::ExecStep>,
        #[doc = "The `ExecStep` to run before the patch update."]
        #[serde(
            rename = "preStep",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pre_step: ::std::option::Option<crate::schemas::ExecStep>,
        #[doc = "Post-patch reboot settings."]
        #[serde(
            rename = "rebootConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reboot_config: ::std::option::Option<crate::schemas::PatchConfigRebootConfig>,
        #[doc = "Windows update settings. Use this override the default windows patch rules."]
        #[serde(
            rename = "windowsUpdate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub windows_update: ::std::option::Option<crate::schemas::WindowsUpdateSettings>,
        #[doc = "Yum update settings. Use this setting to override the default `yum` patch rules."]
        #[serde(
            rename = "yum",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub yum: ::std::option::Option<crate::schemas::YumSettings>,
        #[doc = "Zypper update settings. Use this setting to override the default `zypper` patch rules."]
        #[serde(
            rename = "zypper",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub zypper: ::std::option::Option<crate::schemas::ZypperSettings>,
    }
    impl ::google_field_selector::FieldSelector for PatchConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PatchConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PatchConfigRebootConfig {
        #[doc = "Always reboot the machine after the update completes."]
        Always,
        #[doc = "The agent decides if a reboot is necessary by checking signals such as registry keys on Windows or `/var/run/reboot-required` on APT based systems. On RPM based systems, a set of core system package install times are compared with system boot time."]
        Default,
        #[doc = "Never reboot the machine after the update completes."]
        Never,
        #[doc = "The default behavior is DEFAULT."]
        RebootConfigUnspecified,
    }
    impl PatchConfigRebootConfig {
        pub fn as_str(self) -> &'static str {
            match self {
                PatchConfigRebootConfig::Always => "ALWAYS",
                PatchConfigRebootConfig::Default => "DEFAULT",
                PatchConfigRebootConfig::Never => "NEVER",
                PatchConfigRebootConfig::RebootConfigUnspecified => "REBOOT_CONFIG_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PatchConfigRebootConfig {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PatchConfigRebootConfig {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PatchConfigRebootConfig, ()> {
            Ok(match s {
                "ALWAYS" => PatchConfigRebootConfig::Always,
                "DEFAULT" => PatchConfigRebootConfig::Default,
                "NEVER" => PatchConfigRebootConfig::Never,
                "REBOOT_CONFIG_UNSPECIFIED" => PatchConfigRebootConfig::RebootConfigUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PatchConfigRebootConfig {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PatchConfigRebootConfig {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PatchConfigRebootConfig {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALWAYS" => PatchConfigRebootConfig::Always,
                "DEFAULT" => PatchConfigRebootConfig::Default,
                "NEVER" => PatchConfigRebootConfig::Never,
                "REBOOT_CONFIG_UNSPECIFIED" => PatchConfigRebootConfig::RebootConfigUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PatchConfigRebootConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PatchConfigRebootConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PatchDeployment {
        #[doc = "Output only. Time the patch deployment was created. Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Optional. Description of the patch deployment. Length of the description is limited to 1024 characters."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Optional. Duration of the patch. After the duration ends, the patch times out."]
        #[serde(
            rename = "duration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub duration: ::std::option::Option<String>,
        #[doc = "Required. VM instances to patch."]
        #[serde(
            rename = "instanceFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instance_filter: ::std::option::Option<crate::schemas::PatchInstanceFilter>,
        #[doc = "Output only. The last time a patch job was started by this deployment. Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format."]
        #[serde(
            rename = "lastExecuteTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_execute_time: ::std::option::Option<String>,
        #[doc = "Unique name for the patch deployment resource in a project. The patch deployment name is in the form: `projects/{project_id}/patchDeployments/{patch_deployment_id}`. This field is ignored when you create a new patch deployment."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Required. Schedule a one-time execution."]
        #[serde(
            rename = "oneTimeSchedule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub one_time_schedule: ::std::option::Option<crate::schemas::OneTimeSchedule>,
        #[doc = "Optional. Patch configuration that is applied."]
        #[serde(
            rename = "patchConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub patch_config: ::std::option::Option<crate::schemas::PatchConfig>,
        #[doc = "Required. Schedule recurring executions."]
        #[serde(
            rename = "recurringSchedule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recurring_schedule: ::std::option::Option<crate::schemas::RecurringSchedule>,
        #[doc = "Optional. Rollout strategy of the patch job."]
        #[serde(
            rename = "rollout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rollout: ::std::option::Option<crate::schemas::PatchRollout>,
        #[doc = "Output only. Current state of the patch deployment."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::PatchDeploymentState>,
        #[doc = "Output only. Time the patch deployment was last updated. Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PatchDeployment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PatchDeployment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PatchDeploymentState {
        #[doc = "Active value means that patch deployment generates Patch Jobs."]
        Active,
        #[doc = "Paused value means that patch deployment does not generate Patch jobs. Requires user action to move in and out from this state."]
        Paused,
        #[doc = "The default value. This value is used if the state is omitted."]
        StateUnspecified,
    }
    impl PatchDeploymentState {
        pub fn as_str(self) -> &'static str {
            match self {
                PatchDeploymentState::Active => "ACTIVE",
                PatchDeploymentState::Paused => "PAUSED",
                PatchDeploymentState::StateUnspecified => "STATE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PatchDeploymentState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PatchDeploymentState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PatchDeploymentState, ()> {
            Ok(match s {
                "ACTIVE" => PatchDeploymentState::Active,
                "PAUSED" => PatchDeploymentState::Paused,
                "STATE_UNSPECIFIED" => PatchDeploymentState::StateUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PatchDeploymentState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PatchDeploymentState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PatchDeploymentState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVE" => PatchDeploymentState::Active,
                "PAUSED" => PatchDeploymentState::Paused,
                "STATE_UNSPECIFIED" => PatchDeploymentState::StateUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PatchDeploymentState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PatchDeploymentState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PatchInstanceFilter {
        #[doc = "Target all VM instances in the project. If true, no other criteria is permitted."]
        #[serde(
            rename = "all",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub all: ::std::option::Option<bool>,
        #[doc = "Targets VM instances matching at least one of these label sets. This allows targeting of disparate groups, for example \"env=prod or env=staging\"."]
        #[serde(
            rename = "groupLabels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group_labels: ::std::option::Option<Vec<crate::schemas::PatchInstanceFilterGroupLabel>>,
        #[doc = "Targets VMs whose name starts with one of these prefixes. Similar to labels, this is another way to group VMs when targeting configs, for example prefix=\"prod-\"."]
        #[serde(
            rename = "instanceNamePrefixes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instance_name_prefixes: ::std::option::Option<Vec<String>>,
        #[doc = "Targets any of the VM instances specified. Instances are specified by their URI in the form `zones/[ZONE]/instances/[INSTANCE_NAME]`, `projects/[PROJECT_ID]/zones/[ZONE]/instances/[INSTANCE_NAME]`, or `https://www.googleapis.com/compute/v1/projects/[PROJECT_ID]/zones/[ZONE]/instances/[INSTANCE_NAME]`"]
        #[serde(
            rename = "instances",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instances: ::std::option::Option<Vec<String>>,
        #[doc = "Targets VM instances in ANY of these zones. Leave empty to target VM instances in any zone."]
        #[serde(
            rename = "zones",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub zones: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for PatchInstanceFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PatchInstanceFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PatchInstanceFilterGroupLabel {
        #[doc = "Compute Engine instance labels that must be present for a VM instance to be targeted by this filter."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::google_field_selector::FieldSelector for PatchInstanceFilterGroupLabel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PatchInstanceFilterGroupLabel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PatchJob {
        #[doc = "Time this patch job was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Description of the patch job. Length of the description is limited to 1024 characters."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Display name for this patch job. This is not a unique identifier."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "If this patch job is a dry run, the agent reports that it has finished without running any updates on the VM instance."]
        #[serde(
            rename = "dryRun",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dry_run: ::std::option::Option<bool>,
        #[doc = "Duration of the patch job. After the duration ends, the patch job times out."]
        #[serde(
            rename = "duration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub duration: ::std::option::Option<String>,
        #[doc = "If this patch job failed, this message provides information about the failure."]
        #[serde(
            rename = "errorMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_message: ::std::option::Option<String>,
        #[doc = "Summary of instance details."]
        #[serde(
            rename = "instanceDetailsSummary",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instance_details_summary:
            ::std::option::Option<crate::schemas::PatchJobInstanceDetailsSummary>,
        #[doc = "Instances to patch."]
        #[serde(
            rename = "instanceFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instance_filter: ::std::option::Option<crate::schemas::PatchInstanceFilter>,
        #[doc = "Unique identifier for this patch job in the form `projects/*/patchJobs/*`"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Patch configuration being applied."]
        #[serde(
            rename = "patchConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub patch_config: ::std::option::Option<crate::schemas::PatchConfig>,
        #[doc = "Output only. Name of the patch deployment that created this patch job."]
        #[serde(
            rename = "patchDeployment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub patch_deployment: ::std::option::Option<String>,
        #[doc = "Reflects the overall progress of the patch job in the range of 0.0 being no progress to 100.0 being complete."]
        #[serde(
            rename = "percentComplete",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub percent_complete: ::std::option::Option<f64>,
        #[doc = "Rollout strategy being applied."]
        #[serde(
            rename = "rollout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rollout: ::std::option::Option<crate::schemas::PatchRollout>,
        #[doc = "The current state of the PatchJob."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::PatchJobState>,
        #[doc = "Last time this patch job was updated."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PatchJob {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PatchJob {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PatchJobState {
        #[doc = "The patch job was canceled."]
        Canceled,
        #[doc = "Patch job completed but there were errors."]
        CompletedWithErrors,
        #[doc = "The patch job is looking up instances to run the patch on."]
        InstanceLookup,
        #[doc = "Instances are being patched."]
        Patching,
        #[doc = "The patch job was successfully initiated."]
        Started,
        #[doc = "State must be specified."]
        StateUnspecified,
        #[doc = "Patch job completed successfully."]
        Succeeded,
        #[doc = "The patch job timed out."]
        TimedOut,
    }
    impl PatchJobState {
        pub fn as_str(self) -> &'static str {
            match self {
                PatchJobState::Canceled => "CANCELED",
                PatchJobState::CompletedWithErrors => "COMPLETED_WITH_ERRORS",
                PatchJobState::InstanceLookup => "INSTANCE_LOOKUP",
                PatchJobState::Patching => "PATCHING",
                PatchJobState::Started => "STARTED",
                PatchJobState::StateUnspecified => "STATE_UNSPECIFIED",
                PatchJobState::Succeeded => "SUCCEEDED",
                PatchJobState::TimedOut => "TIMED_OUT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PatchJobState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PatchJobState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PatchJobState, ()> {
            Ok(match s {
                "CANCELED" => PatchJobState::Canceled,
                "COMPLETED_WITH_ERRORS" => PatchJobState::CompletedWithErrors,
                "INSTANCE_LOOKUP" => PatchJobState::InstanceLookup,
                "PATCHING" => PatchJobState::Patching,
                "STARTED" => PatchJobState::Started,
                "STATE_UNSPECIFIED" => PatchJobState::StateUnspecified,
                "SUCCEEDED" => PatchJobState::Succeeded,
                "TIMED_OUT" => PatchJobState::TimedOut,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PatchJobState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PatchJobState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PatchJobState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELED" => PatchJobState::Canceled,
                "COMPLETED_WITH_ERRORS" => PatchJobState::CompletedWithErrors,
                "INSTANCE_LOOKUP" => PatchJobState::InstanceLookup,
                "PATCHING" => PatchJobState::Patching,
                "STARTED" => PatchJobState::Started,
                "STATE_UNSPECIFIED" => PatchJobState::StateUnspecified,
                "SUCCEEDED" => PatchJobState::Succeeded,
                "TIMED_OUT" => PatchJobState::TimedOut,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PatchJobState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PatchJobState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PatchJobInstanceDetails {
        #[doc = "The number of times the agent that the agent attempts to apply the patch."]
        #[serde(
            rename = "attemptCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub attempt_count: ::std::option::Option<i64>,
        #[doc = "If the patch fails, this field provides the reason."]
        #[serde(
            rename = "failureReason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub failure_reason: ::std::option::Option<String>,
        #[doc = "The unique identifier for the instance. This identifier is defined by the server."]
        #[serde(
            rename = "instanceSystemId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instance_system_id: ::std::option::Option<String>,
        #[doc = "The instance name in the form `projects/*/zones/*/instances/*`"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Current state of instance patch."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::PatchJobInstanceDetailsState>,
    }
    impl ::google_field_selector::FieldSelector for PatchJobInstanceDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PatchJobInstanceDetails {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PatchJobInstanceDetailsState {
        #[doc = "The instance acked the notification and will start shortly."]
        Acked,
        #[doc = "The instance is applying patches."]
        ApplyingPatches,
        #[doc = "The instance is downloading patches."]
        DownloadingPatches,
        #[doc = "The instance has failed to apply the patch."]
        Failed,
        #[doc = "Instance is inactive and cannot be patched."]
        Inactive,
        #[doc = "The service could not detect the presence of the agent. Check to ensure that the agent is installed, running, and able to communicate with the service."]
        NoAgentDetected,
        #[doc = "The instance is notified that it should be patched."]
        Notified,
        #[doc = "Unspecified."]
        PatchStateUnspecified,
        #[doc = "The instance is not yet notified."]
        Pending,
        #[doc = "The instance is rebooting."]
        Rebooting,
        #[doc = "The instance is running the post-patch step."]
        RunningPostPatchStep,
        #[doc = "The instance is running the pre-patch step."]
        RunningPrePatchStep,
        #[doc = "The instance has started the patching process."]
        Started,
        #[doc = "The instance has completed applying patches."]
        Succeeded,
        #[doc = "The instance has completed applying patches but a reboot is required."]
        SucceededRebootRequired,
        #[doc = "The instance exceeded the time out while applying the patch."]
        TimedOut,
    }
    impl PatchJobInstanceDetailsState {
        pub fn as_str(self) -> &'static str {
            match self {
                PatchJobInstanceDetailsState::Acked => "ACKED",
                PatchJobInstanceDetailsState::ApplyingPatches => "APPLYING_PATCHES",
                PatchJobInstanceDetailsState::DownloadingPatches => "DOWNLOADING_PATCHES",
                PatchJobInstanceDetailsState::Failed => "FAILED",
                PatchJobInstanceDetailsState::Inactive => "INACTIVE",
                PatchJobInstanceDetailsState::NoAgentDetected => "NO_AGENT_DETECTED",
                PatchJobInstanceDetailsState::Notified => "NOTIFIED",
                PatchJobInstanceDetailsState::PatchStateUnspecified => "PATCH_STATE_UNSPECIFIED",
                PatchJobInstanceDetailsState::Pending => "PENDING",
                PatchJobInstanceDetailsState::Rebooting => "REBOOTING",
                PatchJobInstanceDetailsState::RunningPostPatchStep => "RUNNING_POST_PATCH_STEP",
                PatchJobInstanceDetailsState::RunningPrePatchStep => "RUNNING_PRE_PATCH_STEP",
                PatchJobInstanceDetailsState::Started => "STARTED",
                PatchJobInstanceDetailsState::Succeeded => "SUCCEEDED",
                PatchJobInstanceDetailsState::SucceededRebootRequired => {
                    "SUCCEEDED_REBOOT_REQUIRED"
                }
                PatchJobInstanceDetailsState::TimedOut => "TIMED_OUT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PatchJobInstanceDetailsState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PatchJobInstanceDetailsState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PatchJobInstanceDetailsState, ()> {
            Ok(match s {
                "ACKED" => PatchJobInstanceDetailsState::Acked,
                "APPLYING_PATCHES" => PatchJobInstanceDetailsState::ApplyingPatches,
                "DOWNLOADING_PATCHES" => PatchJobInstanceDetailsState::DownloadingPatches,
                "FAILED" => PatchJobInstanceDetailsState::Failed,
                "INACTIVE" => PatchJobInstanceDetailsState::Inactive,
                "NO_AGENT_DETECTED" => PatchJobInstanceDetailsState::NoAgentDetected,
                "NOTIFIED" => PatchJobInstanceDetailsState::Notified,
                "PATCH_STATE_UNSPECIFIED" => PatchJobInstanceDetailsState::PatchStateUnspecified,
                "PENDING" => PatchJobInstanceDetailsState::Pending,
                "REBOOTING" => PatchJobInstanceDetailsState::Rebooting,
                "RUNNING_POST_PATCH_STEP" => PatchJobInstanceDetailsState::RunningPostPatchStep,
                "RUNNING_PRE_PATCH_STEP" => PatchJobInstanceDetailsState::RunningPrePatchStep,
                "STARTED" => PatchJobInstanceDetailsState::Started,
                "SUCCEEDED" => PatchJobInstanceDetailsState::Succeeded,
                "SUCCEEDED_REBOOT_REQUIRED" => {
                    PatchJobInstanceDetailsState::SucceededRebootRequired
                }
                "TIMED_OUT" => PatchJobInstanceDetailsState::TimedOut,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PatchJobInstanceDetailsState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PatchJobInstanceDetailsState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PatchJobInstanceDetailsState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACKED" => PatchJobInstanceDetailsState::Acked,
                "APPLYING_PATCHES" => PatchJobInstanceDetailsState::ApplyingPatches,
                "DOWNLOADING_PATCHES" => PatchJobInstanceDetailsState::DownloadingPatches,
                "FAILED" => PatchJobInstanceDetailsState::Failed,
                "INACTIVE" => PatchJobInstanceDetailsState::Inactive,
                "NO_AGENT_DETECTED" => PatchJobInstanceDetailsState::NoAgentDetected,
                "NOTIFIED" => PatchJobInstanceDetailsState::Notified,
                "PATCH_STATE_UNSPECIFIED" => PatchJobInstanceDetailsState::PatchStateUnspecified,
                "PENDING" => PatchJobInstanceDetailsState::Pending,
                "REBOOTING" => PatchJobInstanceDetailsState::Rebooting,
                "RUNNING_POST_PATCH_STEP" => PatchJobInstanceDetailsState::RunningPostPatchStep,
                "RUNNING_PRE_PATCH_STEP" => PatchJobInstanceDetailsState::RunningPrePatchStep,
                "STARTED" => PatchJobInstanceDetailsState::Started,
                "SUCCEEDED" => PatchJobInstanceDetailsState::Succeeded,
                "SUCCEEDED_REBOOT_REQUIRED" => {
                    PatchJobInstanceDetailsState::SucceededRebootRequired
                }
                "TIMED_OUT" => PatchJobInstanceDetailsState::TimedOut,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PatchJobInstanceDetailsState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PatchJobInstanceDetailsState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PatchJobInstanceDetailsSummary {
        #[doc = "Number of instances that have acked and will start shortly."]
        #[serde(
            rename = "ackedInstanceCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub acked_instance_count: ::std::option::Option<i64>,
        #[doc = "Number of instances that are applying patches."]
        #[serde(
            rename = "applyingPatchesInstanceCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub applying_patches_instance_count: ::std::option::Option<i64>,
        #[doc = "Number of instances that are downloading patches."]
        #[serde(
            rename = "downloadingPatchesInstanceCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub downloading_patches_instance_count: ::std::option::Option<i64>,
        #[doc = "Number of instances that failed."]
        #[serde(
            rename = "failedInstanceCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub failed_instance_count: ::std::option::Option<i64>,
        #[doc = "Number of instances that are inactive."]
        #[serde(
            rename = "inactiveInstanceCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub inactive_instance_count: ::std::option::Option<i64>,
        #[doc = "Number of instances that do not appear to be running the agent. Check to ensure that the agent is installed, running, and able to communicate with the service."]
        #[serde(
            rename = "noAgentDetectedInstanceCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub no_agent_detected_instance_count: ::std::option::Option<i64>,
        #[doc = "Number of instances notified about patch job."]
        #[serde(
            rename = "notifiedInstanceCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub notified_instance_count: ::std::option::Option<i64>,
        #[doc = "Number of instances pending patch job."]
        #[serde(
            rename = "pendingInstanceCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub pending_instance_count: ::std::option::Option<i64>,
        #[doc = "Number of instances that are running the post-patch step."]
        #[serde(
            rename = "postPatchStepInstanceCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub post_patch_step_instance_count: ::std::option::Option<i64>,
        #[doc = "Number of instances that are running the pre-patch step."]
        #[serde(
            rename = "prePatchStepInstanceCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub pre_patch_step_instance_count: ::std::option::Option<i64>,
        #[doc = "Number of instances rebooting."]
        #[serde(
            rename = "rebootingInstanceCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub rebooting_instance_count: ::std::option::Option<i64>,
        #[doc = "Number of instances that have started."]
        #[serde(
            rename = "startedInstanceCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub started_instance_count: ::std::option::Option<i64>,
        #[doc = "Number of instances that have completed successfully."]
        #[serde(
            rename = "succeededInstanceCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub succeeded_instance_count: ::std::option::Option<i64>,
        #[doc = "Number of instances that require reboot."]
        #[serde(
            rename = "succeededRebootRequiredInstanceCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub succeeded_reboot_required_instance_count: ::std::option::Option<i64>,
        #[doc = "Number of instances that exceeded the time out while applying the patch."]
        #[serde(
            rename = "timedOutInstanceCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub timed_out_instance_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for PatchJobInstanceDetailsSummary {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PatchJobInstanceDetailsSummary {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PatchRollout {
        #[doc = "The maximum number (or percentage) of VMs per zone to disrupt at any given moment. The number of VMs calculated from multiplying the percentage by the total number of VMs in a zone is rounded up. During patching, a VM is considered disrupted from the time the agent is notified to begin until patching has completed. This disruption time includes the time to complete reboot and any post-patch steps. A VM contributes to the disruption budget if its patching operation fails either when applying the patches, running pre or post patch steps, or if it fails to respond with a success notification before timing out. VMs that are not running or do not have an active agent do not count toward this disruption budget. For zone-by-zone rollouts, if the disruption budget in a zone is exceeded, the patch job stops, because continuing to the next zone requires completion of the patch process in the previous zone. For example, if the disruption budget has a fixed value of `10`, and 8 VMs fail to patch in the current zone, the patch job continues to patch 2 VMs at a time until the zone is completed. When that zone is completed successfully, patching begins with 10 VMs at a time in the next zone. If 10 VMs in the next zone fail to patch, the patch job stops."]
        #[serde(
            rename = "disruptionBudget",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disruption_budget: ::std::option::Option<crate::schemas::FixedOrPercent>,
        #[doc = "Mode of the patch rollout."]
        #[serde(
            rename = "mode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mode: ::std::option::Option<crate::schemas::PatchRolloutMode>,
    }
    impl ::google_field_selector::FieldSelector for PatchRollout {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PatchRollout {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PatchRolloutMode {
        #[doc = "Patches are applied to VMs in all zones at the same time."]
        ConcurrentZones,
        #[doc = "Mode must be specified."]
        ModeUnspecified,
        #[doc = "Patches are applied one zone at a time. The patch job begins in the region with the lowest number of targeted VMs. Within the region, patching begins in the zone with the lowest number of targeted VMs. If multiple regions (or zones within a region) have the same number of targeted VMs, a tie-breaker is achieved by sorting the regions or zones in alphabetical order."]
        ZoneByZone,
    }
    impl PatchRolloutMode {
        pub fn as_str(self) -> &'static str {
            match self {
                PatchRolloutMode::ConcurrentZones => "CONCURRENT_ZONES",
                PatchRolloutMode::ModeUnspecified => "MODE_UNSPECIFIED",
                PatchRolloutMode::ZoneByZone => "ZONE_BY_ZONE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PatchRolloutMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PatchRolloutMode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PatchRolloutMode, ()> {
            Ok(match s {
                "CONCURRENT_ZONES" => PatchRolloutMode::ConcurrentZones,
                "MODE_UNSPECIFIED" => PatchRolloutMode::ModeUnspecified,
                "ZONE_BY_ZONE" => PatchRolloutMode::ZoneByZone,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PatchRolloutMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PatchRolloutMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PatchRolloutMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONCURRENT_ZONES" => PatchRolloutMode::ConcurrentZones,
                "MODE_UNSPECIFIED" => PatchRolloutMode::ModeUnspecified,
                "ZONE_BY_ZONE" => PatchRolloutMode::ZoneByZone,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PatchRolloutMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PatchRolloutMode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct PausePatchDeploymentRequest {}
    impl ::google_field_selector::FieldSelector for PausePatchDeploymentRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PausePatchDeploymentRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RecurringSchedule {
        #[doc = "Optional. The end time at which a recurring patch deployment schedule is no longer active."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Required. The frequency unit of this recurring schedule."]
        #[serde(
            rename = "frequency",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frequency: ::std::option::Option<crate::schemas::RecurringScheduleFrequency>,
        #[doc = "Output only. The time the last patch job ran successfully."]
        #[serde(
            rename = "lastExecuteTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_execute_time: ::std::option::Option<String>,
        #[doc = "Required. Schedule with monthly executions."]
        #[serde(
            rename = "monthly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub monthly: ::std::option::Option<crate::schemas::MonthlySchedule>,
        #[doc = "Output only. The time the next patch job is scheduled to run."]
        #[serde(
            rename = "nextExecuteTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_execute_time: ::std::option::Option<String>,
        #[doc = "Optional. The time that the recurring schedule becomes effective. Defaults to `create_time` of the patch deployment."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "Required. Time of the day to run a recurring deployment."]
        #[serde(
            rename = "timeOfDay",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_of_day: ::std::option::Option<crate::schemas::TimeOfDay>,
        #[doc = "Required. Defines the time zone that `time_of_day` is relative to. The rules for daylight saving time are determined by the chosen time zone."]
        #[serde(
            rename = "timeZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_zone: ::std::option::Option<crate::schemas::TimeZone>,
        #[doc = "Required. Schedule with weekly executions."]
        #[serde(
            rename = "weekly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub weekly: ::std::option::Option<crate::schemas::WeeklySchedule>,
    }
    impl ::google_field_selector::FieldSelector for RecurringSchedule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RecurringSchedule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RecurringScheduleFrequency {
        #[doc = "Indicates that the frequency of recurrence should be expressed in terms of days."]
        Daily,
        #[doc = "Invalid. A frequency must be specified."]
        FrequencyUnspecified,
        #[doc = "Indicates that the frequency of recurrence should be expressed in terms of months."]
        Monthly,
        #[doc = "Indicates that the frequency of recurrence should be expressed in terms of weeks."]
        Weekly,
    }
    impl RecurringScheduleFrequency {
        pub fn as_str(self) -> &'static str {
            match self {
                RecurringScheduleFrequency::Daily => "DAILY",
                RecurringScheduleFrequency::FrequencyUnspecified => "FREQUENCY_UNSPECIFIED",
                RecurringScheduleFrequency::Monthly => "MONTHLY",
                RecurringScheduleFrequency::Weekly => "WEEKLY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RecurringScheduleFrequency {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RecurringScheduleFrequency {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<RecurringScheduleFrequency, ()> {
            Ok(match s {
                "DAILY" => RecurringScheduleFrequency::Daily,
                "FREQUENCY_UNSPECIFIED" => RecurringScheduleFrequency::FrequencyUnspecified,
                "MONTHLY" => RecurringScheduleFrequency::Monthly,
                "WEEKLY" => RecurringScheduleFrequency::Weekly,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RecurringScheduleFrequency {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RecurringScheduleFrequency {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RecurringScheduleFrequency {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DAILY" => RecurringScheduleFrequency::Daily,
                "FREQUENCY_UNSPECIFIED" => RecurringScheduleFrequency::FrequencyUnspecified,
                "MONTHLY" => RecurringScheduleFrequency::Monthly,
                "WEEKLY" => RecurringScheduleFrequency::Weekly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RecurringScheduleFrequency {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RecurringScheduleFrequency {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct ResumePatchDeploymentRequest {}
    impl ::google_field_selector::FieldSelector for ResumePatchDeploymentRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResumePatchDeploymentRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SoftwareRecipe {
        #[doc = "Resources available to be used in the steps in the recipe."]
        #[serde(
            rename = "artifacts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub artifacts: ::std::option::Option<Vec<crate::schemas::SoftwareRecipeArtifact>>,
        #[doc = "Default is INSTALLED. The desired state the agent should maintain for this recipe. INSTALLED: The software recipe is installed on the instance but won't be updated to new versions. UPDATED: The software recipe is installed on the instance. The recipe is updated to a higher version, if a higher version of the recipe is assigned to this instance. REMOVE: Remove is unsupported for software recipes and attempts to create or update a recipe to the REMOVE state is rejected."]
        #[serde(
            rename = "desiredState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub desired_state: ::std::option::Option<crate::schemas::SoftwareRecipeDesiredState>,
        #[doc = "Actions to be taken for installing this recipe. On failure it stops executing steps and does not attempt another installation. Any steps taken (including partially completed steps) are not rolled back."]
        #[serde(
            rename = "installSteps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub install_steps: ::std::option::Option<Vec<crate::schemas::SoftwareRecipeStep>>,
        #[doc = "Required. Unique identifier for the recipe. Only one recipe with a given name is installed on an instance. Names are also used to identify resources which helps to determine whether guest policies have conflicts. This means that requests to create multiple recipes with the same name and version are rejected since they could potentially have conflicting assignments."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Actions to be taken for updating this recipe. On failure it stops executing steps and does not attempt another update for this recipe. Any steps taken (including partially completed steps) are not rolled back."]
        #[serde(
            rename = "updateSteps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_steps: ::std::option::Option<Vec<crate::schemas::SoftwareRecipeStep>>,
        #[doc = "The version of this software recipe. Version can be up to 4 period separated numbers (e.g. 12.34.56.78)."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SoftwareRecipe {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SoftwareRecipe {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SoftwareRecipeDesiredState {
        #[doc = "The default is to ensure the package is installed."]
        DesiredStateUnspecified,
        #[doc = "The agent ensures that the package is installed."]
        Installed,
        #[doc = "The agent ensures that the package is not installed and uninstall it if detected."]
        Removed,
        #[doc = "The agent ensures that the package is installed and periodically checks for and install any updates."]
        Updated,
    }
    impl SoftwareRecipeDesiredState {
        pub fn as_str(self) -> &'static str {
            match self {
                SoftwareRecipeDesiredState::DesiredStateUnspecified => "DESIRED_STATE_UNSPECIFIED",
                SoftwareRecipeDesiredState::Installed => "INSTALLED",
                SoftwareRecipeDesiredState::Removed => "REMOVED",
                SoftwareRecipeDesiredState::Updated => "UPDATED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SoftwareRecipeDesiredState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SoftwareRecipeDesiredState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SoftwareRecipeDesiredState, ()> {
            Ok(match s {
                "DESIRED_STATE_UNSPECIFIED" => SoftwareRecipeDesiredState::DesiredStateUnspecified,
                "INSTALLED" => SoftwareRecipeDesiredState::Installed,
                "REMOVED" => SoftwareRecipeDesiredState::Removed,
                "UPDATED" => SoftwareRecipeDesiredState::Updated,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SoftwareRecipeDesiredState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SoftwareRecipeDesiredState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SoftwareRecipeDesiredState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DESIRED_STATE_UNSPECIFIED" => SoftwareRecipeDesiredState::DesiredStateUnspecified,
                "INSTALLED" => SoftwareRecipeDesiredState::Installed,
                "REMOVED" => SoftwareRecipeDesiredState::Removed,
                "UPDATED" => SoftwareRecipeDesiredState::Updated,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SoftwareRecipeDesiredState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SoftwareRecipeDesiredState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SoftwareRecipeArtifact {
        #[doc = "Defaults to false. When false, recipes are subject to validations based on the artifact type: Remote: A checksum must be specified, and only protocols with transport-layer security are permitted. GCS: An object generation number must be specified."]
        #[serde(
            rename = "allowInsecure",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_insecure: ::std::option::Option<bool>,
        #[doc = "A Google Cloud Storage artifact."]
        #[serde(
            rename = "gcs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs: ::std::option::Option<crate::schemas::SoftwareRecipeArtifactGcs>,
        #[doc = "Required. Id of the artifact, which the installation and update steps of this recipe can reference. Artifacts in a recipe cannot have the same id."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "A generic remote artifact."]
        #[serde(
            rename = "remote",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub remote: ::std::option::Option<crate::schemas::SoftwareRecipeArtifactRemote>,
    }
    impl ::google_field_selector::FieldSelector for SoftwareRecipeArtifact {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SoftwareRecipeArtifact {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SoftwareRecipeArtifactGcs {
        #[doc = "Bucket of the Google Cloud Storage object. Given an example URL: `https://storage.googleapis.com/my-bucket/foo/bar#1234567` this value would be `my-bucket`."]
        #[serde(
            rename = "bucket",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucket: ::std::option::Option<String>,
        #[doc = "Must be provided if allow_insecure is false. Generation number of the Google Cloud Storage object. `https://storage.googleapis.com/my-bucket/foo/bar#1234567` this value would be `1234567`."]
        #[serde(
            rename = "generation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub generation: ::std::option::Option<i64>,
        #[doc = "Name of the Google Cloud Storage object. As specified [here] (https://cloud.google.com/storage/docs/naming#objectnames) Given an example URL: `https://storage.googleapis.com/my-bucket/foo/bar#1234567` this value would be `foo/bar`."]
        #[serde(
            rename = "object",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SoftwareRecipeArtifactGcs {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SoftwareRecipeArtifactGcs {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SoftwareRecipeArtifactRemote {
        #[doc = "Must be provided if `allow_insecure` is `false`. SHA256 checksum in hex format, to compare to the checksum of the artifact. If the checksum is not empty and it doesn't match the artifact then the recipe installation fails before running any of the steps."]
        #[serde(
            rename = "checksum",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub checksum: ::std::option::Option<String>,
        #[doc = "URI from which to fetch the object. It should contain both the protocol and path following the format {protocol}://{location}."]
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SoftwareRecipeArtifactRemote {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SoftwareRecipeArtifactRemote {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SoftwareRecipeStep {
        #[doc = "Extracts an archive into the specified directory."]
        #[serde(
            rename = "archiveExtraction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub archive_extraction:
            ::std::option::Option<crate::schemas::SoftwareRecipeStepExtractArchive>,
        #[doc = "Installs a deb file via dpkg."]
        #[serde(
            rename = "dpkgInstallation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dpkg_installation: ::std::option::Option<crate::schemas::SoftwareRecipeStepInstallDpkg>,
        #[doc = "Copies a file onto the instance."]
        #[serde(
            rename = "fileCopy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_copy: ::std::option::Option<crate::schemas::SoftwareRecipeStepCopyFile>,
        #[doc = "Executes an artifact or local file."]
        #[serde(
            rename = "fileExec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_exec: ::std::option::Option<crate::schemas::SoftwareRecipeStepExecFile>,
        #[doc = "Installs an MSI file."]
        #[serde(
            rename = "msiInstallation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub msi_installation: ::std::option::Option<crate::schemas::SoftwareRecipeStepInstallMsi>,
        #[doc = "Installs an rpm file via the rpm utility."]
        #[serde(
            rename = "rpmInstallation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rpm_installation: ::std::option::Option<crate::schemas::SoftwareRecipeStepInstallRpm>,
        #[doc = "Runs commands in a shell."]
        #[serde(
            rename = "scriptRun",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub script_run: ::std::option::Option<crate::schemas::SoftwareRecipeStepRunScript>,
    }
    impl ::google_field_selector::FieldSelector for SoftwareRecipeStep {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SoftwareRecipeStep {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SoftwareRecipeStepCopyFile {
        #[doc = "Required. The id of the relevant artifact in the recipe."]
        #[serde(
            rename = "artifactId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub artifact_id: ::std::option::Option<String>,
        #[doc = "Required. The absolute path on the instance to put the file."]
        #[serde(
            rename = "destination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination: ::std::option::Option<String>,
        #[doc = "Whether to allow this step to overwrite existing files. If this is false and the file already exists the file is not overwritten and the step is considered a success. Defaults to false."]
        #[serde(
            rename = "overwrite",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub overwrite: ::std::option::Option<bool>,
        #[doc = "Consists of three octal digits which represent, in order, the permissions of the owner, group, and other users for the file (similarly to the numeric mode used in the linux chmod utility). Each digit represents a three bit number with the 4 bit corresponding to the read permissions, the 2 bit corresponds to the write bit, and the one bit corresponds to the execute permission. Default behavior is 755. Below are some examples of permissions and their associated values: read, write, and execute: 7 read and execute: 5 read and write: 6 read only: 4"]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SoftwareRecipeStepCopyFile {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SoftwareRecipeStepCopyFile {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SoftwareRecipeStepExecFile {
        #[doc = "Defaults to [0]. A list of possible return values that the program can return to indicate a success."]
        #[serde(
            rename = "allowedExitCodes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allowed_exit_codes: ::std::option::Option<Vec<i32>>,
        #[doc = "Arguments to be passed to the provided executable."]
        #[serde(
            rename = "args",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub args: ::std::option::Option<Vec<String>>,
        #[doc = "The id of the relevant artifact in the recipe."]
        #[serde(
            rename = "artifactId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub artifact_id: ::std::option::Option<String>,
        #[doc = "The absolute path of the file on the local filesystem."]
        #[serde(
            rename = "localPath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub local_path: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SoftwareRecipeStepExecFile {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SoftwareRecipeStepExecFile {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SoftwareRecipeStepExtractArchive {
        #[doc = "Required. The id of the relevant artifact in the recipe."]
        #[serde(
            rename = "artifactId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub artifact_id: ::std::option::Option<String>,
        #[doc = "Directory to extract archive to. Defaults to `/` on Linux or `C:\\` on Windows."]
        #[serde(
            rename = "destination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination: ::std::option::Option<String>,
        #[doc = "Required. The type of the archive to extract."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::SoftwareRecipeStepExtractArchiveType>,
    }
    impl ::google_field_selector::FieldSelector for SoftwareRecipeStepExtractArchive {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SoftwareRecipeStepExtractArchive {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SoftwareRecipeStepExtractArchiveType {
        #[doc = "Indicates that the archive type isn't specified."]
        ArchiveTypeUnspecified,
        #[doc = "Indicates that the archive is a tar archive with no encryption."]
        Tar,
        #[doc = "Indicates that the archive is a tar archive with bzip encryption."]
        TarBzip,
        #[doc = "Indicates that the archive is a tar archive with gzip encryption."]
        TarGzip,
        #[doc = "Indicates that the archive is a tar archive with lzma encryption."]
        TarLzma,
        #[doc = "Indicates that the archive is a tar archive with xz encryption."]
        TarXz,
        #[doc = "Indicates that the archive is a zip archive."]
        Zip,
    }
    impl SoftwareRecipeStepExtractArchiveType {
        pub fn as_str(self) -> &'static str {
            match self {
                SoftwareRecipeStepExtractArchiveType::ArchiveTypeUnspecified => {
                    "ARCHIVE_TYPE_UNSPECIFIED"
                }
                SoftwareRecipeStepExtractArchiveType::Tar => "TAR",
                SoftwareRecipeStepExtractArchiveType::TarBzip => "TAR_BZIP",
                SoftwareRecipeStepExtractArchiveType::TarGzip => "TAR_GZIP",
                SoftwareRecipeStepExtractArchiveType::TarLzma => "TAR_LZMA",
                SoftwareRecipeStepExtractArchiveType::TarXz => "TAR_XZ",
                SoftwareRecipeStepExtractArchiveType::Zip => "ZIP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SoftwareRecipeStepExtractArchiveType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SoftwareRecipeStepExtractArchiveType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SoftwareRecipeStepExtractArchiveType, ()> {
            Ok(match s {
                "ARCHIVE_TYPE_UNSPECIFIED" => {
                    SoftwareRecipeStepExtractArchiveType::ArchiveTypeUnspecified
                }
                "TAR" => SoftwareRecipeStepExtractArchiveType::Tar,
                "TAR_BZIP" => SoftwareRecipeStepExtractArchiveType::TarBzip,
                "TAR_GZIP" => SoftwareRecipeStepExtractArchiveType::TarGzip,
                "TAR_LZMA" => SoftwareRecipeStepExtractArchiveType::TarLzma,
                "TAR_XZ" => SoftwareRecipeStepExtractArchiveType::TarXz,
                "ZIP" => SoftwareRecipeStepExtractArchiveType::Zip,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SoftwareRecipeStepExtractArchiveType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SoftwareRecipeStepExtractArchiveType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SoftwareRecipeStepExtractArchiveType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ARCHIVE_TYPE_UNSPECIFIED" => {
                    SoftwareRecipeStepExtractArchiveType::ArchiveTypeUnspecified
                }
                "TAR" => SoftwareRecipeStepExtractArchiveType::Tar,
                "TAR_BZIP" => SoftwareRecipeStepExtractArchiveType::TarBzip,
                "TAR_GZIP" => SoftwareRecipeStepExtractArchiveType::TarGzip,
                "TAR_LZMA" => SoftwareRecipeStepExtractArchiveType::TarLzma,
                "TAR_XZ" => SoftwareRecipeStepExtractArchiveType::TarXz,
                "ZIP" => SoftwareRecipeStepExtractArchiveType::Zip,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SoftwareRecipeStepExtractArchiveType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SoftwareRecipeStepExtractArchiveType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SoftwareRecipeStepInstallDpkg {
        #[doc = "Required. The id of the relevant artifact in the recipe."]
        #[serde(
            rename = "artifactId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub artifact_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SoftwareRecipeStepInstallDpkg {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SoftwareRecipeStepInstallDpkg {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SoftwareRecipeStepInstallMsi {
        #[doc = "Return codes that indicate that the software installed or updated successfully. Behaviour defaults to [0]"]
        #[serde(
            rename = "allowedExitCodes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allowed_exit_codes: ::std::option::Option<Vec<i32>>,
        #[doc = "Required. The id of the relevant artifact in the recipe."]
        #[serde(
            rename = "artifactId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub artifact_id: ::std::option::Option<String>,
        #[doc = "The flags to use when installing the MSI defaults to [\"/i\"] (i.e. the install flag)."]
        #[serde(
            rename = "flags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub flags: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for SoftwareRecipeStepInstallMsi {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SoftwareRecipeStepInstallMsi {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SoftwareRecipeStepInstallRpm {
        #[doc = "Required. The id of the relevant artifact in the recipe."]
        #[serde(
            rename = "artifactId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub artifact_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SoftwareRecipeStepInstallRpm {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SoftwareRecipeStepInstallRpm {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SoftwareRecipeStepRunScript {
        #[doc = "Return codes that indicate that the software installed or updated successfully. Behaviour defaults to [0]"]
        #[serde(
            rename = "allowedExitCodes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allowed_exit_codes: ::std::option::Option<Vec<i32>>,
        #[doc = "The script interpreter to use to run the script. If no interpreter is specified the script is executed directly, which likely only succeed for scripts with [shebang lines](https://en.wikipedia.org/wiki/Shebang_(Unix))."]
        #[serde(
            rename = "interpreter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub interpreter:
            ::std::option::Option<crate::schemas::SoftwareRecipeStepRunScriptInterpreter>,
        #[doc = "Required. The shell script to be executed."]
        #[serde(
            rename = "script",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub script: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SoftwareRecipeStepRunScript {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SoftwareRecipeStepRunScript {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SoftwareRecipeStepRunScriptInterpreter {
        #[doc = "Default value for ScriptType."]
        InterpreterUnspecified,
        #[doc = "Indicates that the script is run with powershell."]
        Powershell,
        #[doc = "Indicates that the script is run with `/bin/sh` on Linux and `cmd` on windows."]
        Shell,
    }
    impl SoftwareRecipeStepRunScriptInterpreter {
        pub fn as_str(self) -> &'static str {
            match self {
                SoftwareRecipeStepRunScriptInterpreter::InterpreterUnspecified => {
                    "INTERPRETER_UNSPECIFIED"
                }
                SoftwareRecipeStepRunScriptInterpreter::Powershell => "POWERSHELL",
                SoftwareRecipeStepRunScriptInterpreter::Shell => "SHELL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SoftwareRecipeStepRunScriptInterpreter {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SoftwareRecipeStepRunScriptInterpreter {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SoftwareRecipeStepRunScriptInterpreter, ()> {
            Ok(match s {
                "INTERPRETER_UNSPECIFIED" => {
                    SoftwareRecipeStepRunScriptInterpreter::InterpreterUnspecified
                }
                "POWERSHELL" => SoftwareRecipeStepRunScriptInterpreter::Powershell,
                "SHELL" => SoftwareRecipeStepRunScriptInterpreter::Shell,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SoftwareRecipeStepRunScriptInterpreter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SoftwareRecipeStepRunScriptInterpreter {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SoftwareRecipeStepRunScriptInterpreter {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INTERPRETER_UNSPECIFIED" => {
                    SoftwareRecipeStepRunScriptInterpreter::InterpreterUnspecified
                }
                "POWERSHELL" => SoftwareRecipeStepRunScriptInterpreter::Powershell,
                "SHELL" => SoftwareRecipeStepRunScriptInterpreter::Shell,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SoftwareRecipeStepRunScriptInterpreter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SoftwareRecipeStepRunScriptInterpreter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TimeOfDay {
        #[doc = "Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
        #[serde(
            rename = "hours",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hours: ::std::option::Option<i32>,
        #[doc = "Minutes of hour of day. Must be from 0 to 59."]
        #[serde(
            rename = "minutes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub minutes: ::std::option::Option<i32>,
        #[doc = "Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
        #[serde(
            rename = "nanos",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub nanos: ::std::option::Option<i32>,
        #[doc = "Seconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds."]
        #[serde(
            rename = "seconds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub seconds: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for TimeOfDay {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TimeOfDay {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TimeZone {
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
    impl ::google_field_selector::FieldSelector for TimeZone {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TimeZone {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct WeekDayOfMonth {
        #[doc = "Required. A day of the week."]
        #[serde(
            rename = "dayOfWeek",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub day_of_week: ::std::option::Option<crate::schemas::WeekDayOfMonthDayOfWeek>,
        #[doc = "Optional. Represents the number of days before or after the given week day of month that the patch deployment is scheduled for. For example if `week_ordinal` and `day_of_week` values point to the second day of the month and this `day_offset` value is set to `3`, the patch deployment takes place three days after the second Tuesday of the month. If this value is negative, for example -5, the patches are deployed five days before before the second Tuesday of the month. Allowed values are in range [-30, 30]."]
        #[serde(
            rename = "dayOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub day_offset: ::std::option::Option<i32>,
        #[doc = "Required. Week number in a month. 1-4 indicates the 1st to 4th week of the month. -1 indicates the last week of the month."]
        #[serde(
            rename = "weekOrdinal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub week_ordinal: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for WeekDayOfMonth {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WeekDayOfMonth {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum WeekDayOfMonthDayOfWeek {
        #[doc = "The day of the week is unspecified."]
        DayOfWeekUnspecified,
        #[doc = "Friday"]
        Friday,
        #[doc = "Monday"]
        Monday,
        #[doc = "Saturday"]
        Saturday,
        #[doc = "Sunday"]
        Sunday,
        #[doc = "Thursday"]
        Thursday,
        #[doc = "Tuesday"]
        Tuesday,
        #[doc = "Wednesday"]
        Wednesday,
    }
    impl WeekDayOfMonthDayOfWeek {
        pub fn as_str(self) -> &'static str {
            match self {
                WeekDayOfMonthDayOfWeek::DayOfWeekUnspecified => "DAY_OF_WEEK_UNSPECIFIED",
                WeekDayOfMonthDayOfWeek::Friday => "FRIDAY",
                WeekDayOfMonthDayOfWeek::Monday => "MONDAY",
                WeekDayOfMonthDayOfWeek::Saturday => "SATURDAY",
                WeekDayOfMonthDayOfWeek::Sunday => "SUNDAY",
                WeekDayOfMonthDayOfWeek::Thursday => "THURSDAY",
                WeekDayOfMonthDayOfWeek::Tuesday => "TUESDAY",
                WeekDayOfMonthDayOfWeek::Wednesday => "WEDNESDAY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for WeekDayOfMonthDayOfWeek {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for WeekDayOfMonthDayOfWeek {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<WeekDayOfMonthDayOfWeek, ()> {
            Ok(match s {
                "DAY_OF_WEEK_UNSPECIFIED" => WeekDayOfMonthDayOfWeek::DayOfWeekUnspecified,
                "FRIDAY" => WeekDayOfMonthDayOfWeek::Friday,
                "MONDAY" => WeekDayOfMonthDayOfWeek::Monday,
                "SATURDAY" => WeekDayOfMonthDayOfWeek::Saturday,
                "SUNDAY" => WeekDayOfMonthDayOfWeek::Sunday,
                "THURSDAY" => WeekDayOfMonthDayOfWeek::Thursday,
                "TUESDAY" => WeekDayOfMonthDayOfWeek::Tuesday,
                "WEDNESDAY" => WeekDayOfMonthDayOfWeek::Wednesday,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for WeekDayOfMonthDayOfWeek {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for WeekDayOfMonthDayOfWeek {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for WeekDayOfMonthDayOfWeek {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DAY_OF_WEEK_UNSPECIFIED" => WeekDayOfMonthDayOfWeek::DayOfWeekUnspecified,
                "FRIDAY" => WeekDayOfMonthDayOfWeek::Friday,
                "MONDAY" => WeekDayOfMonthDayOfWeek::Monday,
                "SATURDAY" => WeekDayOfMonthDayOfWeek::Saturday,
                "SUNDAY" => WeekDayOfMonthDayOfWeek::Sunday,
                "THURSDAY" => WeekDayOfMonthDayOfWeek::Thursday,
                "TUESDAY" => WeekDayOfMonthDayOfWeek::Tuesday,
                "WEDNESDAY" => WeekDayOfMonthDayOfWeek::Wednesday,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for WeekDayOfMonthDayOfWeek {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WeekDayOfMonthDayOfWeek {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct WeeklySchedule {
        #[doc = "Required. Day of the week."]
        #[serde(
            rename = "dayOfWeek",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub day_of_week: ::std::option::Option<crate::schemas::WeeklyScheduleDayOfWeek>,
    }
    impl ::google_field_selector::FieldSelector for WeeklySchedule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WeeklySchedule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum WeeklyScheduleDayOfWeek {
        #[doc = "The day of the week is unspecified."]
        DayOfWeekUnspecified,
        #[doc = "Friday"]
        Friday,
        #[doc = "Monday"]
        Monday,
        #[doc = "Saturday"]
        Saturday,
        #[doc = "Sunday"]
        Sunday,
        #[doc = "Thursday"]
        Thursday,
        #[doc = "Tuesday"]
        Tuesday,
        #[doc = "Wednesday"]
        Wednesday,
    }
    impl WeeklyScheduleDayOfWeek {
        pub fn as_str(self) -> &'static str {
            match self {
                WeeklyScheduleDayOfWeek::DayOfWeekUnspecified => "DAY_OF_WEEK_UNSPECIFIED",
                WeeklyScheduleDayOfWeek::Friday => "FRIDAY",
                WeeklyScheduleDayOfWeek::Monday => "MONDAY",
                WeeklyScheduleDayOfWeek::Saturday => "SATURDAY",
                WeeklyScheduleDayOfWeek::Sunday => "SUNDAY",
                WeeklyScheduleDayOfWeek::Thursday => "THURSDAY",
                WeeklyScheduleDayOfWeek::Tuesday => "TUESDAY",
                WeeklyScheduleDayOfWeek::Wednesday => "WEDNESDAY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for WeeklyScheduleDayOfWeek {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for WeeklyScheduleDayOfWeek {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<WeeklyScheduleDayOfWeek, ()> {
            Ok(match s {
                "DAY_OF_WEEK_UNSPECIFIED" => WeeklyScheduleDayOfWeek::DayOfWeekUnspecified,
                "FRIDAY" => WeeklyScheduleDayOfWeek::Friday,
                "MONDAY" => WeeklyScheduleDayOfWeek::Monday,
                "SATURDAY" => WeeklyScheduleDayOfWeek::Saturday,
                "SUNDAY" => WeeklyScheduleDayOfWeek::Sunday,
                "THURSDAY" => WeeklyScheduleDayOfWeek::Thursday,
                "TUESDAY" => WeeklyScheduleDayOfWeek::Tuesday,
                "WEDNESDAY" => WeeklyScheduleDayOfWeek::Wednesday,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for WeeklyScheduleDayOfWeek {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for WeeklyScheduleDayOfWeek {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for WeeklyScheduleDayOfWeek {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DAY_OF_WEEK_UNSPECIFIED" => WeeklyScheduleDayOfWeek::DayOfWeekUnspecified,
                "FRIDAY" => WeeklyScheduleDayOfWeek::Friday,
                "MONDAY" => WeeklyScheduleDayOfWeek::Monday,
                "SATURDAY" => WeeklyScheduleDayOfWeek::Saturday,
                "SUNDAY" => WeeklyScheduleDayOfWeek::Sunday,
                "THURSDAY" => WeeklyScheduleDayOfWeek::Thursday,
                "TUESDAY" => WeeklyScheduleDayOfWeek::Tuesday,
                "WEDNESDAY" => WeeklyScheduleDayOfWeek::Wednesday,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for WeeklyScheduleDayOfWeek {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WeeklyScheduleDayOfWeek {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct WindowsUpdateSettings {
        #[doc = "Only apply updates of these windows update classifications. If empty, all updates are applied."]
        #[serde(
            rename = "classifications",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub classifications:
            ::std::option::Option<Vec<crate::schemas::WindowsUpdateSettingsClassificationsItems>>,
        #[doc = "List of KBs to exclude from update."]
        #[serde(
            rename = "excludes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub excludes: ::std::option::Option<Vec<String>>,
        #[doc = "An exclusive list of kbs to be updated. These are the only patches that will be updated. This field must not be used with other patch configurations."]
        #[serde(
            rename = "exclusivePatches",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exclusive_patches: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for WindowsUpdateSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WindowsUpdateSettings {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum WindowsUpdateSettingsClassificationsItems {
        #[doc = "Invalid. If classifications are included, they must be specified."]
        ClassificationUnspecified,
        #[doc = "\"A widely released fix for a specific problem that addresses a critical, non-security-related bug.\" [1]"]
        Critical,
        #[doc = "\"A widely released and frequent software update that contains additions to a product's definition database. Definition databases are often used to detect objects that have specific attributes, such as malicious code, phishing websites, or junk mail.\" [1]"]
        Definition,
        #[doc = "\"Software that controls the input and output of a device.\" [1]"]
        Driver,
        #[doc = "\"New product functionality that is first distributed outside the context of a product release and that is typically included in the next full product release.\" [1]"]
        FeaturePack,
        #[doc = "\"A widely released fix for a product-specific, security-related vulnerability. Security vulnerabilities are rated by their severity. The severity rating is indicated in the Microsoft security bulletin as critical, important, moderate, or low.\" [1]"]
        Security,
        #[doc = "\"A tested, cumulative set of all hotfixes, security updates, critical updates, and updates. Additionally, service packs may contain additional fixes for problems that are found internally since the release of the product. Service packs my also contain a limited number of customer-requested design changes or features.\" [1]"]
        ServicePack,
        #[doc = "\"A utility or feature that helps complete a task or set of tasks.\" [1]"]
        Tool,
        #[doc = "\"A widely released fix for a specific problem. An update addresses a noncritical, non-security-related bug.\" [1]"]
        Update,
        #[doc = "\"A tested, cumulative set of hotfixes, security updates, critical updates, and updates that are packaged together for easy deployment. A rollup generally targets a specific area, such as security, or a component of a product, such as Internet Information Services (IIS).\" [1]"]
        UpdateRollup,
    }
    impl WindowsUpdateSettingsClassificationsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                WindowsUpdateSettingsClassificationsItems::ClassificationUnspecified => {
                    "CLASSIFICATION_UNSPECIFIED"
                }
                WindowsUpdateSettingsClassificationsItems::Critical => "CRITICAL",
                WindowsUpdateSettingsClassificationsItems::Definition => "DEFINITION",
                WindowsUpdateSettingsClassificationsItems::Driver => "DRIVER",
                WindowsUpdateSettingsClassificationsItems::FeaturePack => "FEATURE_PACK",
                WindowsUpdateSettingsClassificationsItems::Security => "SECURITY",
                WindowsUpdateSettingsClassificationsItems::ServicePack => "SERVICE_PACK",
                WindowsUpdateSettingsClassificationsItems::Tool => "TOOL",
                WindowsUpdateSettingsClassificationsItems::Update => "UPDATE",
                WindowsUpdateSettingsClassificationsItems::UpdateRollup => "UPDATE_ROLLUP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for WindowsUpdateSettingsClassificationsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for WindowsUpdateSettingsClassificationsItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<WindowsUpdateSettingsClassificationsItems, ()> {
            Ok(match s {
                "CLASSIFICATION_UNSPECIFIED" => {
                    WindowsUpdateSettingsClassificationsItems::ClassificationUnspecified
                }
                "CRITICAL" => WindowsUpdateSettingsClassificationsItems::Critical,
                "DEFINITION" => WindowsUpdateSettingsClassificationsItems::Definition,
                "DRIVER" => WindowsUpdateSettingsClassificationsItems::Driver,
                "FEATURE_PACK" => WindowsUpdateSettingsClassificationsItems::FeaturePack,
                "SECURITY" => WindowsUpdateSettingsClassificationsItems::Security,
                "SERVICE_PACK" => WindowsUpdateSettingsClassificationsItems::ServicePack,
                "TOOL" => WindowsUpdateSettingsClassificationsItems::Tool,
                "UPDATE" => WindowsUpdateSettingsClassificationsItems::Update,
                "UPDATE_ROLLUP" => WindowsUpdateSettingsClassificationsItems::UpdateRollup,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for WindowsUpdateSettingsClassificationsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for WindowsUpdateSettingsClassificationsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for WindowsUpdateSettingsClassificationsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CLASSIFICATION_UNSPECIFIED" => {
                    WindowsUpdateSettingsClassificationsItems::ClassificationUnspecified
                }
                "CRITICAL" => WindowsUpdateSettingsClassificationsItems::Critical,
                "DEFINITION" => WindowsUpdateSettingsClassificationsItems::Definition,
                "DRIVER" => WindowsUpdateSettingsClassificationsItems::Driver,
                "FEATURE_PACK" => WindowsUpdateSettingsClassificationsItems::FeaturePack,
                "SECURITY" => WindowsUpdateSettingsClassificationsItems::Security,
                "SERVICE_PACK" => WindowsUpdateSettingsClassificationsItems::ServicePack,
                "TOOL" => WindowsUpdateSettingsClassificationsItems::Tool,
                "UPDATE" => WindowsUpdateSettingsClassificationsItems::Update,
                "UPDATE_ROLLUP" => WindowsUpdateSettingsClassificationsItems::UpdateRollup,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for WindowsUpdateSettingsClassificationsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WindowsUpdateSettingsClassificationsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct YumRepository {
        #[doc = "Required. The location of the repository directory."]
        #[serde(
            rename = "baseUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub base_url: ::std::option::Option<String>,
        #[doc = "The display name of the repository."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "URIs of GPG keys."]
        #[serde(
            rename = "gpgKeys",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gpg_keys: ::std::option::Option<Vec<String>>,
        #[doc = "Required. A one word, unique name for this repository. This is the `repo id` in the Yum config file and also the `display_name` if `display_name` is omitted. This id is also used as the unique identifier when checking for guest policy conflicts."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for YumRepository {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for YumRepository {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct YumSettings {
        #[doc = "List of packages to exclude from update. These packages are excluded by using the yum `--exclude` flag."]
        #[serde(
            rename = "excludes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub excludes: ::std::option::Option<Vec<String>>,
        #[doc = "An exclusive list of packages to be updated. These are the only packages that will be updated. If these packages are not installed, they will be ignored. This field must not be specified with any other patch configuration fields."]
        #[serde(
            rename = "exclusivePackages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exclusive_packages: ::std::option::Option<Vec<String>>,
        #[doc = "Will cause patch to run `yum update-minimal` instead."]
        #[serde(
            rename = "minimal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub minimal: ::std::option::Option<bool>,
        #[doc = "Adds the `--security` flag to `yum update`. Not supported on all platforms."]
        #[serde(
            rename = "security",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub security: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for YumSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for YumSettings {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ZypperRepository {
        #[doc = "Required. The location of the repository directory."]
        #[serde(
            rename = "baseUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub base_url: ::std::option::Option<String>,
        #[doc = "The display name of the repository."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "URIs of GPG keys."]
        #[serde(
            rename = "gpgKeys",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gpg_keys: ::std::option::Option<Vec<String>>,
        #[doc = "Required. A one word, unique name for this repository. This is the `repo id` in the zypper config file and also the `display_name` if `display_name` is omitted. This id is also used as the unique identifier when checking for guest policy conflicts."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ZypperRepository {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ZypperRepository {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ZypperSettings {
        #[doc = "Install only patches with these categories. Common categories include security, recommended, and feature."]
        #[serde(
            rename = "categories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub categories: ::std::option::Option<Vec<String>>,
        #[doc = "List of patches to exclude from update."]
        #[serde(
            rename = "excludes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub excludes: ::std::option::Option<Vec<String>>,
        #[doc = "An exclusive list of patches to be updated. These are the only patches that will be installed using 'zypper patch patch:' command. This field must not be used with any other patch configuration fields."]
        #[serde(
            rename = "exclusivePatches",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exclusive_patches: ::std::option::Option<Vec<String>>,
        #[doc = "Install only patches with these severities. Common severities include critical, important, moderate, and low."]
        #[serde(
            rename = "severities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub severities: ::std::option::Option<Vec<String>>,
        #[doc = "Adds the `--with-optional` flag to `zypper patch`."]
        #[serde(
            rename = "withOptional",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub with_optional: ::std::option::Option<bool>,
        #[doc = "Adds the `--with-update` flag, to `zypper patch`."]
        #[serde(
            rename = "withUpdate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub with_update: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for ZypperSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ZypperSettings {
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
            #[doc = "Actions that can be performed on the guest_policies resource"]
            pub fn guest_policies(
                &self,
            ) -> crate::resources::projects::guest_policies::GuestPoliciesActions {
                crate::resources::projects::guest_policies::GuestPoliciesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the patch_deployments resource"]
            pub fn patch_deployments(
                &self,
            ) -> crate::resources::projects::patch_deployments::PatchDeploymentsActions
            {
                crate::resources::projects::patch_deployments::PatchDeploymentsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the patch_jobs resource"]
            pub fn patch_jobs(&self) -> crate::resources::projects::patch_jobs::PatchJobsActions {
                crate::resources::projects::patch_jobs::PatchJobsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the zones resource"]
            pub fn zones(&self) -> crate::resources::projects::zones::ZonesActions {
                crate::resources::projects::zones::ZonesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod guest_policies {
            pub mod params {}
            pub struct GuestPoliciesActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> GuestPoliciesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Create an OS Config guest policy."]
                pub fn create(
                    &self,
                    request: crate::schemas::GuestPolicy,
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
                        guest_policy_id: None,
                    }
                }
                #[doc = "Delete an OS Config guest policy."]
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
                #[doc = "Get an OS Config guest policy."]
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
                #[doc = "Get a page of OS Config guest policies."]
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
                #[doc = "Update an OS Config guest policy."]
                pub fn patch(
                    &self,
                    request: crate::schemas::GuestPolicy,
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
            #[doc = "Created via [GuestPoliciesActions::create()](struct.GuestPoliciesActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GuestPolicy,
                parent: String,
                guest_policy_id: Option<String>,
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
                #[doc = "Required. The logical name of the guest policy in the project with the following restrictions: * Must contain only lowercase letters, numbers, and hyphens. * Must start with a letter. * Must be between 1-63 characters. * Must end with a number or a letter. * Must be unique within the project."]
                pub fn guest_policy_id(mut self, value: impl Into<String>) -> Self {
                    self.guest_policy_id = Some(value.into());
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
                ) -> Result<crate::schemas::GuestPolicy, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GuestPolicy, crate::Error> {
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
                    let mut output = "https://osconfig.googleapis.com/".to_owned();
                    output.push_str("v1beta/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/guestPolicies");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                    req = req.query(&[("guestPolicyId", &self.guest_policy_id)]);
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
            #[doc = "Created via [GuestPoliciesActions::delete()](struct.GuestPoliciesActions.html#method.delete)"]
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
                    let mut output = "https://osconfig.googleapis.com/".to_owned();
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
            #[doc = "Created via [GuestPoliciesActions::get()](struct.GuestPoliciesActions.html#method.get)"]
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
                ) -> Result<crate::schemas::GuestPolicy, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GuestPolicy, crate::Error> {
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
                    let mut output = "https://osconfig.googleapis.com/".to_owned();
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
            #[doc = "Created via [GuestPoliciesActions::list()](struct.GuestPoliciesActions.html#method.list)"]
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
                #[doc = "The maximum number of guest policies to return."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A pagination token returned from a previous call to `ListGuestPolicies` that indicates where this listing should continue from."]
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
                ) -> Result<crate::schemas::ListGuestPoliciesResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListGuestPoliciesResponse, crate::Error>
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
                    let mut output = "https://osconfig.googleapis.com/".to_owned();
                    output.push_str("v1beta/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/guestPolicies");
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
            #[doc = "Created via [GuestPoliciesActions::patch()](struct.GuestPoliciesActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GuestPolicy,
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
                #[doc = "Field mask that controls which fields of the guest policy should be updated."]
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
                ) -> Result<crate::schemas::GuestPolicy, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GuestPolicy, crate::Error> {
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
                    let mut output = "https://osconfig.googleapis.com/".to_owned();
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
        pub mod patch_deployments {
            pub mod params {}
            pub struct PatchDeploymentsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> PatchDeploymentsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Create an OS Config patch deployment."]
                pub fn create(
                    &self,
                    request: crate::schemas::PatchDeployment,
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
                        patch_deployment_id: None,
                    }
                }
                #[doc = "Delete an OS Config patch deployment."]
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
                #[doc = "Get an OS Config patch deployment."]
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
                #[doc = "Get a page of OS Config patch deployments."]
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
                #[doc = "Update an OS Config patch deployment."]
                pub fn patch(
                    &self,
                    request: crate::schemas::PatchDeployment,
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
                #[doc = "Change state of patch deployment to \"PAUSED\". Patch deployment in paused state doesn't generate patch jobs."]
                pub fn pause(
                    &self,
                    request: crate::schemas::PausePatchDeploymentRequest,
                    name: impl Into<String>,
                ) -> PauseRequestBuilder {
                    PauseRequestBuilder {
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
                #[doc = "Change state of patch deployment back to \"ACTIVE\". Patch deployment in active state continues to generate patch jobs."]
                pub fn resume(
                    &self,
                    request: crate::schemas::ResumePatchDeploymentRequest,
                    name: impl Into<String>,
                ) -> ResumeRequestBuilder {
                    ResumeRequestBuilder {
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
            #[doc = "Created via [PatchDeploymentsActions::create()](struct.PatchDeploymentsActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::PatchDeployment,
                parent: String,
                patch_deployment_id: Option<String>,
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
                #[doc = "Required. A name for the patch deployment in the project. When creating a name the following rules apply: * Must contain only lowercase letters, numbers, and hyphens. * Must start with a letter. * Must be between 1-63 characters. * Must end with a number or a letter. * Must be unique within the project."]
                pub fn patch_deployment_id(mut self, value: impl Into<String>) -> Self {
                    self.patch_deployment_id = Some(value.into());
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
                ) -> Result<crate::schemas::PatchDeployment, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::PatchDeployment, crate::Error> {
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
                    let mut output = "https://osconfig.googleapis.com/".to_owned();
                    output.push_str("v1beta/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/patchDeployments");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                    req = req.query(&[("patchDeploymentId", &self.patch_deployment_id)]);
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
            #[doc = "Created via [PatchDeploymentsActions::delete()](struct.PatchDeploymentsActions.html#method.delete)"]
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
                    let mut output = "https://osconfig.googleapis.com/".to_owned();
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
            #[doc = "Created via [PatchDeploymentsActions::get()](struct.PatchDeploymentsActions.html#method.get)"]
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
                ) -> Result<crate::schemas::PatchDeployment, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::PatchDeployment, crate::Error> {
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
                    let mut output = "https://osconfig.googleapis.com/".to_owned();
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
            #[doc = "Created via [PatchDeploymentsActions::list()](struct.PatchDeploymentsActions.html#method.list)"]
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
                #[doc = "Optional. The maximum number of patch deployments to return. Default is 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional. A pagination token returned from a previous call to ListPatchDeployments that indicates where this listing should continue from."]
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
                ) -> Result<crate::schemas::ListPatchDeploymentsResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListPatchDeploymentsResponse, crate::Error>
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
                    let mut output = "https://osconfig.googleapis.com/".to_owned();
                    output.push_str("v1beta/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/patchDeployments");
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
            #[doc = "Created via [PatchDeploymentsActions::patch()](struct.PatchDeploymentsActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::PatchDeployment,
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
                #[doc = "Optional. Field mask that controls which fields of the patch deployment should be updated."]
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
                ) -> Result<crate::schemas::PatchDeployment, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::PatchDeployment, crate::Error> {
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
                    let mut output = "https://osconfig.googleapis.com/".to_owned();
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
            #[doc = "Created via [PatchDeploymentsActions::pause()](struct.PatchDeploymentsActions.html#method.pause)"]
            #[derive(Debug, Clone)]
            pub struct PauseRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::PausePatchDeploymentRequest,
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
            impl<'a> PauseRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::PatchDeployment, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::PatchDeployment, crate::Error> {
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
                    let mut output = "https://osconfig.googleapis.com/".to_owned();
                    output.push_str("v1beta/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":pause");
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
            #[doc = "Created via [PatchDeploymentsActions::resume()](struct.PatchDeploymentsActions.html#method.resume)"]
            #[derive(Debug, Clone)]
            pub struct ResumeRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::ResumePatchDeploymentRequest,
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
            impl<'a> ResumeRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::PatchDeployment, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::PatchDeployment, crate::Error> {
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
                    let mut output = "https://osconfig.googleapis.com/".to_owned();
                    output.push_str("v1beta/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":resume");
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
        pub mod patch_jobs {
            pub mod params {}
            pub struct PatchJobsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> PatchJobsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Cancel a patch job. The patch job must be active. Canceled patch jobs cannot be restarted."]
                pub fn cancel(
                    &self,
                    request: crate::schemas::CancelPatchJobRequest,
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
                #[doc = "Patch VM instances by creating and running a patch job."]
                pub fn execute(
                    &self,
                    request: crate::schemas::ExecutePatchJobRequest,
                    parent: impl Into<String>,
                ) -> ExecuteRequestBuilder {
                    ExecuteRequestBuilder {
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
                #[doc = "Get the patch job. This can be used to track the progress of an ongoing patch job or review the details of completed jobs."]
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
                #[doc = "Get a list of patch jobs."]
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
                #[doc = "Actions that can be performed on the instance_details resource"]
                pub fn instance_details(
                    &self,
                ) -> crate::resources::projects::patch_jobs::instance_details::InstanceDetailsActions
                {
                    crate :: resources :: projects :: patch_jobs :: instance_details :: InstanceDetailsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                }
            }
            #[doc = "Created via [PatchJobsActions::cancel()](struct.PatchJobsActions.html#method.cancel)"]
            #[derive(Debug, Clone)]
            pub struct CancelRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::CancelPatchJobRequest,
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
                ) -> Result<crate::schemas::PatchJob, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::PatchJob, crate::Error> {
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
                    let mut output = "https://osconfig.googleapis.com/".to_owned();
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
            #[doc = "Created via [PatchJobsActions::execute()](struct.PatchJobsActions.html#method.execute)"]
            #[derive(Debug, Clone)]
            pub struct ExecuteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::ExecutePatchJobRequest,
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
            impl<'a> ExecuteRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::PatchJob, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::PatchJob, crate::Error> {
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
                    let mut output = "https://osconfig.googleapis.com/".to_owned();
                    output.push_str("v1beta/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/patchJobs:execute");
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
            #[doc = "Created via [PatchJobsActions::get()](struct.PatchJobsActions.html#method.get)"]
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
                ) -> Result<crate::schemas::PatchJob, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::PatchJob, crate::Error> {
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
                    let mut output = "https://osconfig.googleapis.com/".to_owned();
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
            #[doc = "Created via [PatchJobsActions::list()](struct.PatchJobsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
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
                #[doc = "If provided, this field specifies the criteria that must be met by patch jobs to be included in the response. Currently, filtering is only available on the patch_deployment field."]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "The maximum number of instance status to return."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A pagination token returned from a previous call that indicates where this listing should continue from."]
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
                ) -> Result<crate::schemas::ListPatchJobsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListPatchJobsResponse, crate::Error> {
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
                    let mut output = "https://osconfig.googleapis.com/".to_owned();
                    output.push_str("v1beta/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/patchJobs");
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
            pub mod instance_details {
                pub mod params {}
                pub struct InstanceDetailsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> InstanceDetailsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Get a list of instance details for a given patch job."]
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
                }
                #[doc = "Created via [InstanceDetailsActions::list()](struct.InstanceDetailsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
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
                    #[doc = "A filter expression that filters results listed in the response. This field supports filtering results by instance zone, name, state, or `failure_reason`."]
                    pub fn filter(mut self, value: impl Into<String>) -> Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "The maximum number of instance details records to return. Default is 100."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A pagination token returned from a previous call that indicates where this listing should continue from."]
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
                    ) -> Result<crate::schemas::ListPatchJobInstanceDetailsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListPatchJobInstanceDetailsResponse, crate::Error>
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
                        let mut output = "https://osconfig.googleapis.com/".to_owned();
                        output.push_str("v1beta/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/instanceDetails");
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
        }
        pub mod zones {
            pub mod params {}
            pub struct ZonesActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ZonesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Actions that can be performed on the instances resource"]
                pub fn instances(
                    &self,
                ) -> crate::resources::projects::zones::instances::InstancesActions
                {
                    crate::resources::projects::zones::instances::InstancesActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            pub mod instances {
                pub mod params {}
                pub struct InstancesActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> InstancesActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Lookup the effective guest policy that applies to a VM instance. This lookup merges all policies that are assigned to the instance ancestry."]
                    pub fn lookup_effective_guest_policy(
                        &self,
                        request: crate::schemas::LookupEffectiveGuestPolicyRequest,
                        instance: impl Into<String>,
                    ) -> LookupEffectiveGuestPolicyRequestBuilder {
                        LookupEffectiveGuestPolicyRequestBuilder {
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
                            instance: instance.into(),
                        }
                    }
                }
                #[doc = "Created via [InstancesActions::lookup_effective_guest_policy()](struct.InstancesActions.html#method.lookup_effective_guest_policy)"]
                #[derive(Debug, Clone)]
                pub struct LookupEffectiveGuestPolicyRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::LookupEffectiveGuestPolicyRequest,
                    instance: String,
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
                impl<'a> LookupEffectiveGuestPolicyRequestBuilder<'a> {
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
                    ) -> Result<crate::schemas::EffectiveGuestPolicy, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::EffectiveGuestPolicy, crate::Error>
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
                        let mut output = "https://osconfig.googleapis.com/".to_owned();
                        output.push_str("v1beta/");
                        {
                            let var_as_str = &self.instance;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":lookupEffectiveGuestPolicy");
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
