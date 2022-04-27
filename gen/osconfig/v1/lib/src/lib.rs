#![doc = "# Resources and Methods\n    * [projects](resources/projects/struct.ProjectsActions.html)\n      * [locations](resources/projects/locations/struct.LocationsActions.html)\n        * [instances](resources/projects/locations/instances/struct.InstancesActions.html)\n          * [inventories](resources/projects/locations/instances/inventories/struct.InventoriesActions.html)\n            * [*get*](resources/projects/locations/instances/inventories/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/instances/inventories/struct.ListRequestBuilder.html)\n          * [os_policy_assignments](resources/projects/locations/instances/os_policy_assignments/struct.OsPolicyAssignmentsActions.html)\n            * [reports](resources/projects/locations/instances/os_policy_assignments/reports/struct.ReportsActions.html)\n              * [*get*](resources/projects/locations/instances/os_policy_assignments/reports/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/instances/os_policy_assignments/reports/struct.ListRequestBuilder.html)\n          * [vulnerability_reports](resources/projects/locations/instances/vulnerability_reports/struct.VulnerabilityReportsActions.html)\n            * [*get*](resources/projects/locations/instances/vulnerability_reports/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/instances/vulnerability_reports/struct.ListRequestBuilder.html)\n        * [os_policy_assignments](resources/projects/locations/os_policy_assignments/struct.OsPolicyAssignmentsActions.html)\n          * [*create*](resources/projects/locations/os_policy_assignments/struct.CreateRequestBuilder.html), [*delete*](resources/projects/locations/os_policy_assignments/struct.DeleteRequestBuilder.html), [*get*](resources/projects/locations/os_policy_assignments/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/os_policy_assignments/struct.ListRequestBuilder.html), [*listRevisions*](resources/projects/locations/os_policy_assignments/struct.ListRevisionsRequestBuilder.html), [*patch*](resources/projects/locations/os_policy_assignments/struct.PatchRequestBuilder.html)\n          * [operations](resources/projects/locations/os_policy_assignments/operations/struct.OperationsActions.html)\n            * [*cancel*](resources/projects/locations/os_policy_assignments/operations/struct.CancelRequestBuilder.html), [*get*](resources/projects/locations/os_policy_assignments/operations/struct.GetRequestBuilder.html)\n      * [patch_deployments](resources/projects/patch_deployments/struct.PatchDeploymentsActions.html)\n        * [*create*](resources/projects/patch_deployments/struct.CreateRequestBuilder.html), [*delete*](resources/projects/patch_deployments/struct.DeleteRequestBuilder.html), [*get*](resources/projects/patch_deployments/struct.GetRequestBuilder.html), [*list*](resources/projects/patch_deployments/struct.ListRequestBuilder.html), [*patch*](resources/projects/patch_deployments/struct.PatchRequestBuilder.html), [*pause*](resources/projects/patch_deployments/struct.PauseRequestBuilder.html), [*resume*](resources/projects/patch_deployments/struct.ResumeRequestBuilder.html)\n      * [patch_jobs](resources/projects/patch_jobs/struct.PatchJobsActions.html)\n        * [*cancel*](resources/projects/patch_jobs/struct.CancelRequestBuilder.html), [*execute*](resources/projects/patch_jobs/struct.ExecuteRequestBuilder.html), [*get*](resources/projects/patch_jobs/struct.GetRequestBuilder.html), [*list*](resources/projects/patch_jobs/struct.ListRequestBuilder.html)\n        * [instance_details](resources/projects/patch_jobs/instance_details/struct.InstanceDetailsActions.html)\n          * [*list*](resources/projects/patch_jobs/instance_details/struct.ListRequestBuilder.html)\n"]
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Cvssv3 {
        #[doc = "This metric describes the conditions beyond the attacker's control that must exist in order to exploit the vulnerability."]
        #[serde(
            rename = "attackComplexity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attack_complexity: ::std::option::Option<crate::schemas::Cvssv3AttackComplexity>,
        #[doc = "This metric reflects the context by which vulnerability exploitation is possible."]
        #[serde(
            rename = "attackVector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attack_vector: ::std::option::Option<crate::schemas::Cvssv3AttackVector>,
        #[doc = "This metric measures the impact to the availability of the impacted component resulting from a successfully exploited vulnerability."]
        #[serde(
            rename = "availabilityImpact",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub availability_impact: ::std::option::Option<crate::schemas::Cvssv3AvailabilityImpact>,
        #[doc = "The base score is a function of the base metric scores. https://www.first.org/cvss/specification-document#Base-Metrics"]
        #[serde(
            rename = "baseScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub base_score: ::std::option::Option<f32>,
        #[doc = "This metric measures the impact to the confidentiality of the information resources managed by a software component due to a successfully exploited vulnerability."]
        #[serde(
            rename = "confidentialityImpact",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidentiality_impact:
            ::std::option::Option<crate::schemas::Cvssv3ConfidentialityImpact>,
        #[doc = "The Exploitability sub-score equation is derived from the Base Exploitability metrics. https://www.first.org/cvss/specification-document#2-1-Exploitability-Metrics"]
        #[serde(
            rename = "exploitabilityScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exploitability_score: ::std::option::Option<f32>,
        #[doc = "The Impact sub-score equation is derived from the Base Impact metrics."]
        #[serde(
            rename = "impactScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub impact_score: ::std::option::Option<f32>,
        #[doc = "This metric measures the impact to integrity of a successfully exploited vulnerability."]
        #[serde(
            rename = "integrityImpact",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub integrity_impact: ::std::option::Option<crate::schemas::Cvssv3IntegrityImpact>,
        #[doc = "This metric describes the level of privileges an attacker must possess before successfully exploiting the vulnerability."]
        #[serde(
            rename = "privilegesRequired",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub privileges_required: ::std::option::Option<crate::schemas::Cvssv3PrivilegesRequired>,
        #[doc = "The Scope metric captures whether a vulnerability in one vulnerable component impacts resources in components beyond its security scope."]
        #[serde(
            rename = "scope",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scope: ::std::option::Option<crate::schemas::Cvssv3Scope>,
        #[doc = "This metric captures the requirement for a human user, other than the attacker, to participate in the successful compromise of the vulnerable component."]
        #[serde(
            rename = "userInteraction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_interaction: ::std::option::Option<crate::schemas::Cvssv3UserInteraction>,
    }
    impl ::google_field_selector::FieldSelector for Cvssv3 {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cvssv3 {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Cvssv3AttackComplexity {
        #[doc = "A successful attack depends on conditions beyond the attacker's control. That is, a successful attack cannot be accomplished at will, but requires the attacker to invest in some measurable amount of effort in preparation or execution against the vulnerable component before a successful attack can be expected."]
        AttackComplexityHigh,
        #[doc = "Specialized access conditions or extenuating circumstances do not exist. An attacker can expect repeatable success when attacking the vulnerable component."]
        AttackComplexityLow,
        #[doc = "Invalid value."]
        AttackComplexityUnspecified,
    }
    impl Cvssv3AttackComplexity {
        pub fn as_str(self) -> &'static str {
            match self {
                Cvssv3AttackComplexity::AttackComplexityHigh => "ATTACK_COMPLEXITY_HIGH",
                Cvssv3AttackComplexity::AttackComplexityLow => "ATTACK_COMPLEXITY_LOW",
                Cvssv3AttackComplexity::AttackComplexityUnspecified => {
                    "ATTACK_COMPLEXITY_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for Cvssv3AttackComplexity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Cvssv3AttackComplexity {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Cvssv3AttackComplexity, ()> {
            Ok(match s {
                "ATTACK_COMPLEXITY_HIGH" => Cvssv3AttackComplexity::AttackComplexityHigh,
                "ATTACK_COMPLEXITY_LOW" => Cvssv3AttackComplexity::AttackComplexityLow,
                "ATTACK_COMPLEXITY_UNSPECIFIED" => {
                    Cvssv3AttackComplexity::AttackComplexityUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Cvssv3AttackComplexity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Cvssv3AttackComplexity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Cvssv3AttackComplexity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ATTACK_COMPLEXITY_HIGH" => Cvssv3AttackComplexity::AttackComplexityHigh,
                "ATTACK_COMPLEXITY_LOW" => Cvssv3AttackComplexity::AttackComplexityLow,
                "ATTACK_COMPLEXITY_UNSPECIFIED" => {
                    Cvssv3AttackComplexity::AttackComplexityUnspecified
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
    impl ::google_field_selector::FieldSelector for Cvssv3AttackComplexity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cvssv3AttackComplexity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Cvssv3AttackVector {
        #[doc = "The vulnerable component is bound to the network stack, but the attack is limited at the protocol level to a logically adjacent topology."]
        AttackVectorAdjacent,
        #[doc = "The vulnerable component is not bound to the network stack and the attacker's path is via read/write/execute capabilities."]
        AttackVectorLocal,
        #[doc = "The vulnerable component is bound to the network stack and the set of possible attackers extends beyond the other options listed below, up to and including the entire Internet."]
        AttackVectorNetwork,
        #[doc = "The attack requires the attacker to physically touch or manipulate the vulnerable component."]
        AttackVectorPhysical,
        #[doc = "Invalid value."]
        AttackVectorUnspecified,
    }
    impl Cvssv3AttackVector {
        pub fn as_str(self) -> &'static str {
            match self {
                Cvssv3AttackVector::AttackVectorAdjacent => "ATTACK_VECTOR_ADJACENT",
                Cvssv3AttackVector::AttackVectorLocal => "ATTACK_VECTOR_LOCAL",
                Cvssv3AttackVector::AttackVectorNetwork => "ATTACK_VECTOR_NETWORK",
                Cvssv3AttackVector::AttackVectorPhysical => "ATTACK_VECTOR_PHYSICAL",
                Cvssv3AttackVector::AttackVectorUnspecified => "ATTACK_VECTOR_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Cvssv3AttackVector {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Cvssv3AttackVector {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Cvssv3AttackVector, ()> {
            Ok(match s {
                "ATTACK_VECTOR_ADJACENT" => Cvssv3AttackVector::AttackVectorAdjacent,
                "ATTACK_VECTOR_LOCAL" => Cvssv3AttackVector::AttackVectorLocal,
                "ATTACK_VECTOR_NETWORK" => Cvssv3AttackVector::AttackVectorNetwork,
                "ATTACK_VECTOR_PHYSICAL" => Cvssv3AttackVector::AttackVectorPhysical,
                "ATTACK_VECTOR_UNSPECIFIED" => Cvssv3AttackVector::AttackVectorUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Cvssv3AttackVector {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Cvssv3AttackVector {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Cvssv3AttackVector {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ATTACK_VECTOR_ADJACENT" => Cvssv3AttackVector::AttackVectorAdjacent,
                "ATTACK_VECTOR_LOCAL" => Cvssv3AttackVector::AttackVectorLocal,
                "ATTACK_VECTOR_NETWORK" => Cvssv3AttackVector::AttackVectorNetwork,
                "ATTACK_VECTOR_PHYSICAL" => Cvssv3AttackVector::AttackVectorPhysical,
                "ATTACK_VECTOR_UNSPECIFIED" => Cvssv3AttackVector::AttackVectorUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Cvssv3AttackVector {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cvssv3AttackVector {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Cvssv3AvailabilityImpact {
        #[doc = "High impact."]
        ImpactHigh,
        #[doc = "Low impact."]
        ImpactLow,
        #[doc = "No impact."]
        ImpactNone,
        #[doc = "Invalid value."]
        ImpactUnspecified,
    }
    impl Cvssv3AvailabilityImpact {
        pub fn as_str(self) -> &'static str {
            match self {
                Cvssv3AvailabilityImpact::ImpactHigh => "IMPACT_HIGH",
                Cvssv3AvailabilityImpact::ImpactLow => "IMPACT_LOW",
                Cvssv3AvailabilityImpact::ImpactNone => "IMPACT_NONE",
                Cvssv3AvailabilityImpact::ImpactUnspecified => "IMPACT_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Cvssv3AvailabilityImpact {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Cvssv3AvailabilityImpact {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Cvssv3AvailabilityImpact, ()> {
            Ok(match s {
                "IMPACT_HIGH" => Cvssv3AvailabilityImpact::ImpactHigh,
                "IMPACT_LOW" => Cvssv3AvailabilityImpact::ImpactLow,
                "IMPACT_NONE" => Cvssv3AvailabilityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => Cvssv3AvailabilityImpact::ImpactUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Cvssv3AvailabilityImpact {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Cvssv3AvailabilityImpact {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Cvssv3AvailabilityImpact {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "IMPACT_HIGH" => Cvssv3AvailabilityImpact::ImpactHigh,
                "IMPACT_LOW" => Cvssv3AvailabilityImpact::ImpactLow,
                "IMPACT_NONE" => Cvssv3AvailabilityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => Cvssv3AvailabilityImpact::ImpactUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Cvssv3AvailabilityImpact {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cvssv3AvailabilityImpact {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Cvssv3ConfidentialityImpact {
        #[doc = "High impact."]
        ImpactHigh,
        #[doc = "Low impact."]
        ImpactLow,
        #[doc = "No impact."]
        ImpactNone,
        #[doc = "Invalid value."]
        ImpactUnspecified,
    }
    impl Cvssv3ConfidentialityImpact {
        pub fn as_str(self) -> &'static str {
            match self {
                Cvssv3ConfidentialityImpact::ImpactHigh => "IMPACT_HIGH",
                Cvssv3ConfidentialityImpact::ImpactLow => "IMPACT_LOW",
                Cvssv3ConfidentialityImpact::ImpactNone => "IMPACT_NONE",
                Cvssv3ConfidentialityImpact::ImpactUnspecified => "IMPACT_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Cvssv3ConfidentialityImpact {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Cvssv3ConfidentialityImpact {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Cvssv3ConfidentialityImpact, ()> {
            Ok(match s {
                "IMPACT_HIGH" => Cvssv3ConfidentialityImpact::ImpactHigh,
                "IMPACT_LOW" => Cvssv3ConfidentialityImpact::ImpactLow,
                "IMPACT_NONE" => Cvssv3ConfidentialityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => Cvssv3ConfidentialityImpact::ImpactUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Cvssv3ConfidentialityImpact {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Cvssv3ConfidentialityImpact {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Cvssv3ConfidentialityImpact {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "IMPACT_HIGH" => Cvssv3ConfidentialityImpact::ImpactHigh,
                "IMPACT_LOW" => Cvssv3ConfidentialityImpact::ImpactLow,
                "IMPACT_NONE" => Cvssv3ConfidentialityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => Cvssv3ConfidentialityImpact::ImpactUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Cvssv3ConfidentialityImpact {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cvssv3ConfidentialityImpact {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Cvssv3IntegrityImpact {
        #[doc = "High impact."]
        ImpactHigh,
        #[doc = "Low impact."]
        ImpactLow,
        #[doc = "No impact."]
        ImpactNone,
        #[doc = "Invalid value."]
        ImpactUnspecified,
    }
    impl Cvssv3IntegrityImpact {
        pub fn as_str(self) -> &'static str {
            match self {
                Cvssv3IntegrityImpact::ImpactHigh => "IMPACT_HIGH",
                Cvssv3IntegrityImpact::ImpactLow => "IMPACT_LOW",
                Cvssv3IntegrityImpact::ImpactNone => "IMPACT_NONE",
                Cvssv3IntegrityImpact::ImpactUnspecified => "IMPACT_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Cvssv3IntegrityImpact {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Cvssv3IntegrityImpact {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Cvssv3IntegrityImpact, ()> {
            Ok(match s {
                "IMPACT_HIGH" => Cvssv3IntegrityImpact::ImpactHigh,
                "IMPACT_LOW" => Cvssv3IntegrityImpact::ImpactLow,
                "IMPACT_NONE" => Cvssv3IntegrityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => Cvssv3IntegrityImpact::ImpactUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Cvssv3IntegrityImpact {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Cvssv3IntegrityImpact {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Cvssv3IntegrityImpact {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "IMPACT_HIGH" => Cvssv3IntegrityImpact::ImpactHigh,
                "IMPACT_LOW" => Cvssv3IntegrityImpact::ImpactLow,
                "IMPACT_NONE" => Cvssv3IntegrityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => Cvssv3IntegrityImpact::ImpactUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Cvssv3IntegrityImpact {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cvssv3IntegrityImpact {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Cvssv3PrivilegesRequired {
        #[doc = "The attacker requires privileges that provide significant (e.g., administrative) control over the vulnerable component allowing access to component-wide settings and files."]
        PrivilegesRequiredHigh,
        #[doc = "The attacker requires privileges that provide basic user capabilities that could normally affect only settings and files owned by a user. Alternatively, an attacker with Low privileges has the ability to access only non-sensitive resources."]
        PrivilegesRequiredLow,
        #[doc = "The attacker is unauthorized prior to attack, and therefore does not require any access to settings or files of the vulnerable system to carry out an attack."]
        PrivilegesRequiredNone,
        #[doc = "Invalid value."]
        PrivilegesRequiredUnspecified,
    }
    impl Cvssv3PrivilegesRequired {
        pub fn as_str(self) -> &'static str {
            match self {
                Cvssv3PrivilegesRequired::PrivilegesRequiredHigh => "PRIVILEGES_REQUIRED_HIGH",
                Cvssv3PrivilegesRequired::PrivilegesRequiredLow => "PRIVILEGES_REQUIRED_LOW",
                Cvssv3PrivilegesRequired::PrivilegesRequiredNone => "PRIVILEGES_REQUIRED_NONE",
                Cvssv3PrivilegesRequired::PrivilegesRequiredUnspecified => {
                    "PRIVILEGES_REQUIRED_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for Cvssv3PrivilegesRequired {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Cvssv3PrivilegesRequired {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Cvssv3PrivilegesRequired, ()> {
            Ok(match s {
                "PRIVILEGES_REQUIRED_HIGH" => Cvssv3PrivilegesRequired::PrivilegesRequiredHigh,
                "PRIVILEGES_REQUIRED_LOW" => Cvssv3PrivilegesRequired::PrivilegesRequiredLow,
                "PRIVILEGES_REQUIRED_NONE" => Cvssv3PrivilegesRequired::PrivilegesRequiredNone,
                "PRIVILEGES_REQUIRED_UNSPECIFIED" => {
                    Cvssv3PrivilegesRequired::PrivilegesRequiredUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Cvssv3PrivilegesRequired {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Cvssv3PrivilegesRequired {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Cvssv3PrivilegesRequired {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PRIVILEGES_REQUIRED_HIGH" => Cvssv3PrivilegesRequired::PrivilegesRequiredHigh,
                "PRIVILEGES_REQUIRED_LOW" => Cvssv3PrivilegesRequired::PrivilegesRequiredLow,
                "PRIVILEGES_REQUIRED_NONE" => Cvssv3PrivilegesRequired::PrivilegesRequiredNone,
                "PRIVILEGES_REQUIRED_UNSPECIFIED" => {
                    Cvssv3PrivilegesRequired::PrivilegesRequiredUnspecified
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
    impl ::google_field_selector::FieldSelector for Cvssv3PrivilegesRequired {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cvssv3PrivilegesRequired {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Cvssv3Scope {
        #[doc = "An exploited vulnerability can affect resources beyond the security scope managed by the security authority of the vulnerable component."]
        ScopeChanged,
        #[doc = "An exploited vulnerability can only affect resources managed by the same security authority."]
        ScopeUnchanged,
        #[doc = "Invalid value."]
        ScopeUnspecified,
    }
    impl Cvssv3Scope {
        pub fn as_str(self) -> &'static str {
            match self {
                Cvssv3Scope::ScopeChanged => "SCOPE_CHANGED",
                Cvssv3Scope::ScopeUnchanged => "SCOPE_UNCHANGED",
                Cvssv3Scope::ScopeUnspecified => "SCOPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Cvssv3Scope {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Cvssv3Scope {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Cvssv3Scope, ()> {
            Ok(match s {
                "SCOPE_CHANGED" => Cvssv3Scope::ScopeChanged,
                "SCOPE_UNCHANGED" => Cvssv3Scope::ScopeUnchanged,
                "SCOPE_UNSPECIFIED" => Cvssv3Scope::ScopeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Cvssv3Scope {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Cvssv3Scope {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Cvssv3Scope {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SCOPE_CHANGED" => Cvssv3Scope::ScopeChanged,
                "SCOPE_UNCHANGED" => Cvssv3Scope::ScopeUnchanged,
                "SCOPE_UNSPECIFIED" => Cvssv3Scope::ScopeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Cvssv3Scope {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cvssv3Scope {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Cvssv3UserInteraction {
        #[doc = "The vulnerable system can be exploited without interaction from any user."]
        UserInteractionNone,
        #[doc = "Successful exploitation of this vulnerability requires a user to take some action before the vulnerability can be exploited."]
        UserInteractionRequired,
        #[doc = "Invalid value."]
        UserInteractionUnspecified,
    }
    impl Cvssv3UserInteraction {
        pub fn as_str(self) -> &'static str {
            match self {
                Cvssv3UserInteraction::UserInteractionNone => "USER_INTERACTION_NONE",
                Cvssv3UserInteraction::UserInteractionRequired => "USER_INTERACTION_REQUIRED",
                Cvssv3UserInteraction::UserInteractionUnspecified => "USER_INTERACTION_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Cvssv3UserInteraction {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Cvssv3UserInteraction {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Cvssv3UserInteraction, ()> {
            Ok(match s {
                "USER_INTERACTION_NONE" => Cvssv3UserInteraction::UserInteractionNone,
                "USER_INTERACTION_REQUIRED" => Cvssv3UserInteraction::UserInteractionRequired,
                "USER_INTERACTION_UNSPECIFIED" => Cvssv3UserInteraction::UserInteractionUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Cvssv3UserInteraction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Cvssv3UserInteraction {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Cvssv3UserInteraction {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "USER_INTERACTION_NONE" => Cvssv3UserInteraction::UserInteractionNone,
                "USER_INTERACTION_REQUIRED" => Cvssv3UserInteraction::UserInteractionRequired,
                "USER_INTERACTION_UNSPECIFIED" => Cvssv3UserInteraction::UserInteractionUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Cvssv3UserInteraction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cvssv3UserInteraction {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "A Cloud Storage object containing the executable."]
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
        #[doc = "Required. Bucket of the Cloud Storage object."]
        #[serde(
            rename = "bucket",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucket: ::std::option::Option<String>,
        #[doc = "Required. Generation number of the Cloud Storage object. This is used to ensure that the ExecStep specified by this PatchJob does not change."]
        #[serde(
            rename = "generationNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub generation_number: ::std::option::Option<i64>,
        #[doc = "Required. Name of the Cloud Storage object."]
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
    pub struct Inventory {
        #[doc = "Inventory items related to the VM keyed by an opaque unique identifier for each inventory item. The identifier is unique to each distinct and addressable inventory item and will change, when there is a new package version."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::InventoryItem>,
        >,
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
        pub os_info: ::std::option::Option<crate::schemas::InventoryOsInfo>,
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
    pub struct InventoryItem {
        #[doc = "Software package available to be installed on the VM instance."]
        #[serde(
            rename = "availablePackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub available_package: ::std::option::Option<crate::schemas::InventorySoftwarePackage>,
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
        pub installed_package: ::std::option::Option<crate::schemas::InventorySoftwarePackage>,
        #[doc = "The origin of this inventory item."]
        #[serde(
            rename = "originType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub origin_type: ::std::option::Option<crate::schemas::InventoryItemOriginType>,
        #[doc = "The specific type of inventory, correlating to its specific details."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::InventoryItemType>,
        #[doc = "When this inventory item was last modified."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for InventoryItem {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InventoryItem {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum InventoryItemOriginType {
        #[doc = "This inventory item was discovered as the result of the agent reporting inventory via the reporting API."]
        InventoryReport,
        #[doc = "Invalid. An origin type must be specified."]
        OriginTypeUnspecified,
    }
    impl InventoryItemOriginType {
        pub fn as_str(self) -> &'static str {
            match self {
                InventoryItemOriginType::InventoryReport => "INVENTORY_REPORT",
                InventoryItemOriginType::OriginTypeUnspecified => "ORIGIN_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for InventoryItemOriginType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for InventoryItemOriginType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<InventoryItemOriginType, ()> {
            Ok(match s {
                "INVENTORY_REPORT" => InventoryItemOriginType::InventoryReport,
                "ORIGIN_TYPE_UNSPECIFIED" => InventoryItemOriginType::OriginTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for InventoryItemOriginType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for InventoryItemOriginType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for InventoryItemOriginType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INVENTORY_REPORT" => InventoryItemOriginType::InventoryReport,
                "ORIGIN_TYPE_UNSPECIFIED" => InventoryItemOriginType::OriginTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for InventoryItemOriginType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InventoryItemOriginType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum InventoryItemType {
        #[doc = "This represents an update that is available for a package."]
        AvailablePackage,
        #[doc = "This represents a package that is installed on the VM."]
        InstalledPackage,
        #[doc = "Invalid. An type must be specified."]
        TypeUnspecified,
    }
    impl InventoryItemType {
        pub fn as_str(self) -> &'static str {
            match self {
                InventoryItemType::AvailablePackage => "AVAILABLE_PACKAGE",
                InventoryItemType::InstalledPackage => "INSTALLED_PACKAGE",
                InventoryItemType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for InventoryItemType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for InventoryItemType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<InventoryItemType, ()> {
            Ok(match s {
                "AVAILABLE_PACKAGE" => InventoryItemType::AvailablePackage,
                "INSTALLED_PACKAGE" => InventoryItemType::InstalledPackage,
                "TYPE_UNSPECIFIED" => InventoryItemType::TypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for InventoryItemType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for InventoryItemType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for InventoryItemType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AVAILABLE_PACKAGE" => InventoryItemType::AvailablePackage,
                "INSTALLED_PACKAGE" => InventoryItemType::InstalledPackage,
                "TYPE_UNSPECIFIED" => InventoryItemType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for InventoryItemType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InventoryItemType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct InventoryOsInfo {
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
        #[doc = "The operating system long name. For example 'Debian GNU/Linux 9' or 'Microsoft Window Server 2019 Datacenter'."]
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
        #[doc = "The operating system short name. For example, 'windows' or 'debian'."]
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
    impl ::google_field_selector::FieldSelector for InventoryOsInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InventoryOsInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct InventorySoftwarePackage {
        #[doc = "Details of an APT package. For details about the apt package manager, see https://wiki.debian.org/Apt."]
        #[serde(
            rename = "aptPackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub apt_package: ::std::option::Option<crate::schemas::InventoryVersionedPackage>,
        #[doc = "Details of a COS package."]
        #[serde(
            rename = "cosPackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cos_package: ::std::option::Option<crate::schemas::InventoryVersionedPackage>,
        #[doc = "Details of a Googet package. For details about the googet package manager, see https://github.com/google/googet."]
        #[serde(
            rename = "googetPackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub googet_package: ::std::option::Option<crate::schemas::InventoryVersionedPackage>,
        #[doc = "Details of a Windows Quick Fix engineering package. See https://docs.microsoft.com/en-us/windows/win32/cimwin32prov/win32-quickfixengineering for info in Windows Quick Fix Engineering."]
        #[serde(
            rename = "qfePackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub qfe_package:
            ::std::option::Option<crate::schemas::InventoryWindowsQuickFixEngineeringPackage>,
        #[doc = "Details of Windows Application."]
        #[serde(
            rename = "windowsApplication",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub windows_application: ::std::option::Option<crate::schemas::InventoryWindowsApplication>,
        #[doc = "Details of a Windows Update package. See https://docs.microsoft.com/en-us/windows/win32/api/_wua/ for information about Windows Update."]
        #[serde(
            rename = "wuaPackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub wua_package: ::std::option::Option<crate::schemas::InventoryWindowsUpdatePackage>,
        #[doc = "Yum package info. For details about the yum package manager, see https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/6/html/deployment_guide/ch-yum."]
        #[serde(
            rename = "yumPackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub yum_package: ::std::option::Option<crate::schemas::InventoryVersionedPackage>,
        #[doc = "Details of a Zypper package. For details about the Zypper package manager, see https://en.opensuse.org/SDB:Zypper_manual."]
        #[serde(
            rename = "zypperPackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub zypper_package: ::std::option::Option<crate::schemas::InventoryVersionedPackage>,
        #[doc = "Details of a Zypper patch. For details about the Zypper package manager, see https://en.opensuse.org/SDB:Zypper_manual."]
        #[serde(
            rename = "zypperPatch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub zypper_patch: ::std::option::Option<crate::schemas::InventoryZypperPatch>,
    }
    impl ::google_field_selector::FieldSelector for InventorySoftwarePackage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InventorySoftwarePackage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct InventoryVersionedPackage {
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
    impl ::google_field_selector::FieldSelector for InventoryVersionedPackage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InventoryVersionedPackage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct InventoryWindowsApplication {
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
    impl ::google_field_selector::FieldSelector for InventoryWindowsApplication {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InventoryWindowsApplication {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct InventoryWindowsQuickFixEngineeringPackage {
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
    impl ::google_field_selector::FieldSelector for InventoryWindowsQuickFixEngineeringPackage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InventoryWindowsQuickFixEngineeringPackage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct InventoryWindowsUpdatePackage {
        #[doc = "The categories that are associated with this update package."]
        #[serde(
            rename = "categories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub categories: ::std::option::Option<
            Vec<crate::schemas::InventoryWindowsUpdatePackageWindowsUpdateCategory>,
        >,
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
    impl ::google_field_selector::FieldSelector for InventoryWindowsUpdatePackage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InventoryWindowsUpdatePackage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct InventoryWindowsUpdatePackageWindowsUpdateCategory {
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
    impl ::google_field_selector::FieldSelector for InventoryWindowsUpdatePackageWindowsUpdateCategory {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InventoryWindowsUpdatePackageWindowsUpdateCategory {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct InventoryZypperPatch {
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
    impl ::google_field_selector::FieldSelector for InventoryZypperPatch {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InventoryZypperPatch {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListInventoriesResponse {
        #[doc = "List of inventory objects."]
        #[serde(
            rename = "inventories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inventories: ::std::option::Option<Vec<crate::schemas::Inventory>>,
        #[doc = "The pagination token to retrieve the next page of inventory objects."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListInventoriesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListInventoriesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListOSPolicyAssignmentReportsResponse {
        #[doc = "The pagination token to retrieve the next page of OS policy assignment report objects."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "List of OS policy assignment reports."]
        #[serde(
            rename = "osPolicyAssignmentReports",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_policy_assignment_reports:
            ::std::option::Option<Vec<crate::schemas::OspolicyAssignmentReport>>,
    }
    impl ::google_field_selector::FieldSelector for ListOSPolicyAssignmentReportsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListOSPolicyAssignmentReportsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListOSPolicyAssignmentRevisionsResponse {
        #[doc = "The pagination token to retrieve the next page of OS policy assignment revisions."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The OS policy assignment revisions"]
        #[serde(
            rename = "osPolicyAssignments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_policy_assignments: ::std::option::Option<Vec<crate::schemas::OspolicyAssignment>>,
    }
    impl ::google_field_selector::FieldSelector for ListOSPolicyAssignmentRevisionsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListOSPolicyAssignmentRevisionsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListOSPolicyAssignmentsResponse {
        #[doc = "The pagination token to retrieve the next page of OS policy assignments."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The list of assignments"]
        #[serde(
            rename = "osPolicyAssignments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_policy_assignments: ::std::option::Option<Vec<crate::schemas::OspolicyAssignment>>,
    }
    impl ::google_field_selector::FieldSelector for ListOSPolicyAssignmentsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListOSPolicyAssignmentsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListVulnerabilityReportsResponse {
        #[doc = "The pagination token to retrieve the next page of vulnerabilityReports object."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "List of vulnerabilityReport objects."]
        #[serde(
            rename = "vulnerabilityReports",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vulnerability_reports: ::std::option::Option<Vec<crate::schemas::VulnerabilityReport>>,
    }
    impl ::google_field_selector::FieldSelector for ListVulnerabilityReportsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListVulnerabilityReportsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
    pub struct Ospolicy {
        #[doc = "This flag determines the OS policy compliance status when none of the resource groups within the policy are applicable for a VM. Set this value to `true` if the policy needs to be reported as compliant even if the policy has nothing to validate or enforce."]
        #[serde(
            rename = "allowNoResourceGroupMatch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_no_resource_group_match: ::std::option::Option<bool>,
        #[doc = "Policy description. Length of the description is limited to 1024 characters."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Required. The id of the OS policy with the following restrictions: * Must contain only lowercase letters, numbers, and hyphens. * Must start with a letter. * Must be between 1-63 characters. * Must end with a number or a letter. * Must be unique within the assignment."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Required. Policy mode"]
        #[serde(
            rename = "mode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mode: ::std::option::Option<crate::schemas::OspolicyMode>,
        #[doc = "Required. List of resource groups for the policy. For a particular VM, resource groups are evaluated in the order specified and the first resource group that is applicable is selected and the rest are ignored. If none of the resource groups are applicable for a VM, the VM is considered to be non-compliant w.r.t this policy. This behavior can be toggled by the flag `allow_no_resource_group_match`"]
        #[serde(
            rename = "resourceGroups",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_groups: ::std::option::Option<Vec<crate::schemas::OspolicyResourceGroup>>,
    }
    impl ::google_field_selector::FieldSelector for Ospolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Ospolicy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OspolicyMode {
        #[doc = "This mode checks if the configuration resources in the policy are in their desired state, and if not, enforces the desired state."]
        Enforcement,
        #[doc = "Invalid mode"]
        ModeUnspecified,
        #[doc = "This mode checks if the configuration resources in the policy are in their desired state. No actions are performed if they are not in the desired state. This mode is used for reporting purposes."]
        Validation,
    }
    impl OspolicyMode {
        pub fn as_str(self) -> &'static str {
            match self {
                OspolicyMode::Enforcement => "ENFORCEMENT",
                OspolicyMode::ModeUnspecified => "MODE_UNSPECIFIED",
                OspolicyMode::Validation => "VALIDATION",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OspolicyMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OspolicyMode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<OspolicyMode, ()> {
            Ok(match s {
                "ENFORCEMENT" => OspolicyMode::Enforcement,
                "MODE_UNSPECIFIED" => OspolicyMode::ModeUnspecified,
                "VALIDATION" => OspolicyMode::Validation,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OspolicyMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OspolicyMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OspolicyMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ENFORCEMENT" => OspolicyMode::Enforcement,
                "MODE_UNSPECIFIED" => OspolicyMode::ModeUnspecified,
                "VALIDATION" => OspolicyMode::Validation,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for OspolicyMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyMode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyAssignment {
        #[doc = "Output only. Indicates that this revision has been successfully rolled out in this zone and new VMs will be assigned OS policies from this revision. For a given OS policy assignment, there is only one revision with a value of `true` for this field."]
        #[serde(
            rename = "baseline",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub baseline: ::std::option::Option<bool>,
        #[doc = "Output only. Indicates that this revision deletes the OS policy assignment."]
        #[serde(
            rename = "deleted",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deleted: ::std::option::Option<bool>,
        #[doc = "OS policy assignment description. Length of the description is limited to 1024 characters."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The etag for this OS policy assignment. If this is provided on update, it must match the server's etag."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "Required. Filter to select VMs."]
        #[serde(
            rename = "instanceFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instance_filter:
            ::std::option::Option<crate::schemas::OspolicyAssignmentInstanceFilter>,
        #[doc = "Resource name. Format: `projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id}` This field is ignored when you create an OS policy assignment."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Required. List of OS policies to be applied to the VMs."]
        #[serde(
            rename = "osPolicies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_policies: ::std::option::Option<Vec<crate::schemas::Ospolicy>>,
        #[doc = "Output only. Indicates that reconciliation is in progress for the revision. This value is `true` when the `rollout_state` is one of: * IN_PROGRESS * CANCELLING"]
        #[serde(
            rename = "reconciling",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reconciling: ::std::option::Option<bool>,
        #[doc = "Output only. The timestamp that the revision was created."]
        #[serde(
            rename = "revisionCreateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_create_time: ::std::option::Option<String>,
        #[doc = "Output only. The assignment revision ID A new revision is committed whenever a rollout is triggered for a OS policy assignment"]
        #[serde(
            rename = "revisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_id: ::std::option::Option<String>,
        #[doc = "Required. Rollout to deploy the OS policy assignment. A rollout is triggered in the following situations: 1) OSPolicyAssignment is created. 2) OSPolicyAssignment is updated and the update contains changes to one of the following fields: - instance_filter - os_policies 3) OSPolicyAssignment is deleted."]
        #[serde(
            rename = "rollout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rollout: ::std::option::Option<crate::schemas::OspolicyAssignmentRollout>,
        #[doc = "Output only. OS policy assignment rollout state"]
        #[serde(
            rename = "rolloutState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rollout_state: ::std::option::Option<crate::schemas::OspolicyAssignmentRolloutState>,
        #[doc = "Output only. Server generated unique id for the OS policy assignment resource."]
        #[serde(
            rename = "uid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uid: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyAssignment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyAssignment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OspolicyAssignmentRolloutState {
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
    impl OspolicyAssignmentRolloutState {
        pub fn as_str(self) -> &'static str {
            match self {
                OspolicyAssignmentRolloutState::Cancelled => "CANCELLED",
                OspolicyAssignmentRolloutState::Cancelling => "CANCELLING",
                OspolicyAssignmentRolloutState::InProgress => "IN_PROGRESS",
                OspolicyAssignmentRolloutState::RolloutStateUnspecified => {
                    "ROLLOUT_STATE_UNSPECIFIED"
                }
                OspolicyAssignmentRolloutState::Succeeded => "SUCCEEDED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OspolicyAssignmentRolloutState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OspolicyAssignmentRolloutState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<OspolicyAssignmentRolloutState, ()> {
            Ok(match s {
                "CANCELLED" => OspolicyAssignmentRolloutState::Cancelled,
                "CANCELLING" => OspolicyAssignmentRolloutState::Cancelling,
                "IN_PROGRESS" => OspolicyAssignmentRolloutState::InProgress,
                "ROLLOUT_STATE_UNSPECIFIED" => {
                    OspolicyAssignmentRolloutState::RolloutStateUnspecified
                }
                "SUCCEEDED" => OspolicyAssignmentRolloutState::Succeeded,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OspolicyAssignmentRolloutState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OspolicyAssignmentRolloutState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OspolicyAssignmentRolloutState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => OspolicyAssignmentRolloutState::Cancelled,
                "CANCELLING" => OspolicyAssignmentRolloutState::Cancelling,
                "IN_PROGRESS" => OspolicyAssignmentRolloutState::InProgress,
                "ROLLOUT_STATE_UNSPECIFIED" => {
                    OspolicyAssignmentRolloutState::RolloutStateUnspecified
                }
                "SUCCEEDED" => OspolicyAssignmentRolloutState::Succeeded,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for OspolicyAssignmentRolloutState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyAssignmentRolloutState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyAssignmentInstanceFilter {
        #[doc = "Target all VMs in the project. If true, no other criteria is permitted."]
        #[serde(
            rename = "all",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub all: ::std::option::Option<bool>,
        #[doc = "List of label sets used for VM exclusion. If the list has more than one label set, the VM is excluded if any of the label sets are applicable for the VM."]
        #[serde(
            rename = "exclusionLabels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exclusion_labels:
            ::std::option::Option<Vec<crate::schemas::OspolicyAssignmentLabelSet>>,
        #[doc = "List of label sets used for VM inclusion. If the list has more than one `LabelSet`, the VM is included if any of the label sets are applicable for the VM."]
        #[serde(
            rename = "inclusionLabels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inclusion_labels:
            ::std::option::Option<Vec<crate::schemas::OspolicyAssignmentLabelSet>>,
        #[doc = "List of inventories to select VMs. A VM is selected if its inventory data matches at least one of the following inventories."]
        #[serde(
            rename = "inventories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inventories:
            ::std::option::Option<Vec<crate::schemas::OspolicyAssignmentInstanceFilterInventory>>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyAssignmentInstanceFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyAssignmentInstanceFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyAssignmentInstanceFilterInventory {
        #[doc = "Required. The OS short name"]
        #[serde(
            rename = "osShortName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_short_name: ::std::option::Option<String>,
        #[doc = "The OS version Prefix matches are supported if asterisk(*) is provided as the last character. For example, to match all versions with a major version of `7`, specify the following value for this field `7.*` An empty string matches all OS versions."]
        #[serde(
            rename = "osVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyAssignmentInstanceFilterInventory {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyAssignmentInstanceFilterInventory {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyAssignmentLabelSet {
        #[doc = "Labels are identified by key/value pairs in this map. A VM should contain all the key/value pairs specified in this map to be selected."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyAssignmentLabelSet {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyAssignmentLabelSet {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
    pub struct OspolicyAssignmentReport {
        #[doc = "The Compute Engine VM instance name."]
        #[serde(
            rename = "instance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instance: ::std::option::Option<String>,
        #[doc = "Unique identifier of the last attempted run to apply the OS policies associated with this assignment on the VM. This ID is logged by the OS Config agent while applying the OS policies associated with this assignment on the VM. NOTE: If the service is unable to successfully connect to the agent for this run, then this id will not be available in the agent logs."]
        #[serde(
            rename = "lastRunId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_run_id: ::std::option::Option<String>,
        #[doc = "The `OSPolicyAssignmentReport` API resource name. Format: `projects/{project_number}/locations/{location}/instances/{instance_id}/osPolicyAssignments/{os_policy_assignment_id}/report`"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Reference to the `OSPolicyAssignment` API resource that the `OSPolicy` belongs to. Format: `projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id@revision_id}`"]
        #[serde(
            rename = "osPolicyAssignment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_policy_assignment: ::std::option::Option<String>,
        #[doc = "Compliance data for each `OSPolicy` that is applied to the VM."]
        #[serde(
            rename = "osPolicyCompliances",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_policy_compliances:
            ::std::option::Option<Vec<crate::schemas::OspolicyAssignmentReportOSPolicyCompliance>>,
        #[doc = "Timestamp for when the report was last generated."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyAssignmentReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyAssignmentReport {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyAssignmentReportOSPolicyCompliance { # [doc = "The compliance state of the OS policy."] # [serde (rename = "complianceState" , default , skip_serializing_if = "std::option::Option::is_none")] pub compliance_state : :: std :: option :: Option < crate :: schemas :: OspolicyAssignmentReportOSPolicyComplianceComplianceState > , # [doc = "The reason for the OS policy to be in an unknown compliance state. This field is always populated when `compliance_state` is `UNKNOWN`. If populated, the field can contain one of the following values: * `vm-not-running`: The VM was not running. * `os-policies-not-supported-by-agent`: The version of the OS Config agent running on the VM does not support running OS policies. * `no-agent-detected`: The OS Config agent is not detected for the VM. * `resource-execution-errors`: The OS Config agent encountered errors while executing one or more resources in the policy. See `os_policy_resource_compliances` for details. * `task-timeout`: The task sent to the agent to apply the policy timed out. * `unexpected-agent-state`: The OS Config agent did not report the final status of the task that attempted to apply the policy. Instead, the agent unexpectedly started working on a different task. This mostly happens when the agent or VM unexpectedly restarts while applying OS policies. * `internal-service-errors`: Internal service errors were encountered while attempting to apply the policy."] # [serde (rename = "complianceStateReason" , default , skip_serializing_if = "std::option::Option::is_none")] pub compliance_state_reason : :: std :: option :: Option < String > , # [doc = "The OS policy id"] # [serde (rename = "osPolicyId" , default , skip_serializing_if = "std::option::Option::is_none")] pub os_policy_id : :: std :: option :: Option < String > , # [doc = "Compliance data for each resource within the policy that is applied to the VM."] # [serde (rename = "osPolicyResourceCompliances" , default , skip_serializing_if = "std::option::Option::is_none")] pub os_policy_resource_compliances : :: std :: option :: Option < Vec < crate :: schemas :: OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceCompliance > > , }
    impl ::google_field_selector::FieldSelector for OspolicyAssignmentReportOSPolicyCompliance {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyAssignmentReportOSPolicyCompliance {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OspolicyAssignmentReportOSPolicyComplianceComplianceState {
        #[doc = "Policy is compliant. The policy is compliant if all the underlying resources are also compliant."]
        Compliant,
        #[doc = "Policy is non-compliant. The policy is non-compliant if one or more underlying resources are non-compliant."]
        NonCompliant,
        #[doc = "The policy is in an unknown compliance state. Refer to the field `compliance_state_reason` to learn the exact reason for the policy to be in this compliance state."]
        Unknown,
    }
    impl OspolicyAssignmentReportOSPolicyComplianceComplianceState {
        pub fn as_str(self) -> &'static str {
            match self {
                OspolicyAssignmentReportOSPolicyComplianceComplianceState::Compliant => "COMPLIANT",
                OspolicyAssignmentReportOSPolicyComplianceComplianceState::NonCompliant => {
                    "NON_COMPLIANT"
                }
                OspolicyAssignmentReportOSPolicyComplianceComplianceState::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OspolicyAssignmentReportOSPolicyComplianceComplianceState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OspolicyAssignmentReportOSPolicyComplianceComplianceState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<OspolicyAssignmentReportOSPolicyComplianceComplianceState, ()>
        {
            Ok(match s {
                "COMPLIANT" => OspolicyAssignmentReportOSPolicyComplianceComplianceState::Compliant,
                "NON_COMPLIANT" => {
                    OspolicyAssignmentReportOSPolicyComplianceComplianceState::NonCompliant
                }
                "UNKNOWN" => OspolicyAssignmentReportOSPolicyComplianceComplianceState::Unknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OspolicyAssignmentReportOSPolicyComplianceComplianceState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OspolicyAssignmentReportOSPolicyComplianceComplianceState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OspolicyAssignmentReportOSPolicyComplianceComplianceState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMPLIANT" => OspolicyAssignmentReportOSPolicyComplianceComplianceState::Compliant,
                "NON_COMPLIANT" => {
                    OspolicyAssignmentReportOSPolicyComplianceComplianceState::NonCompliant
                }
                "UNKNOWN" => OspolicyAssignmentReportOSPolicyComplianceComplianceState::Unknown,
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
        for OspolicyAssignmentReportOSPolicyComplianceComplianceState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for OspolicyAssignmentReportOSPolicyComplianceComplianceState
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
    pub struct OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceCompliance { # [doc = "The compliance state of the resource."] # [serde (rename = "complianceState" , default , skip_serializing_if = "std::option::Option::is_none")] pub compliance_state : :: std :: option :: Option < crate :: schemas :: OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceComplianceState > , # [doc = "A reason for the resource to be in the given compliance state. This field is always populated when `compliance_state` is `UNKNOWN`. The following values are supported when `compliance_state == UNKNOWN` * `execution-errors`: Errors were encountered by the agent while executing the resource and the compliance state couldn't be determined. * `execution-skipped-by-agent`: Resource execution was skipped by the agent because errors were encountered while executing prior resources in the OS policy. * `os-policy-execution-attempt-failed`: The execution of the OS policy containing this resource failed and the compliance state couldn't be determined."] # [serde (rename = "complianceStateReason" , default , skip_serializing_if = "std::option::Option::is_none")] pub compliance_state_reason : :: std :: option :: Option < String > , # [doc = "Ordered list of configuration completed by the agent for the OS policy resource."] # [serde (rename = "configSteps" , default , skip_serializing_if = "std::option::Option::is_none")] pub config_steps : :: std :: option :: Option < Vec < crate :: schemas :: OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStep > > , # [doc = "ExecResource specific output."] # [serde (rename = "execResourceOutput" , default , skip_serializing_if = "std::option::Option::is_none")] pub exec_resource_output : :: std :: option :: Option < crate :: schemas :: OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceExecResourceOutput > , # [doc = "The ID of the OS policy resource."] # [serde (rename = "osPolicyResourceId" , default , skip_serializing_if = "std::option::Option::is_none")] pub os_policy_resource_id : :: std :: option :: Option < String > , }
    impl ::google_field_selector::FieldSelector
        for OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceCompliance
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceCompliance
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceComplianceState {
        #[doc = "Resource is compliant."]
        Compliant,
        #[doc = "Resource is non-compliant."]
        NonCompliant,
        #[doc = "The resource is in an unknown compliance state. To get more details about why the policy is in this state, review the output of the `compliance_state_reason` field."]
        Unknown,
    }
    impl OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceComplianceState {
        pub fn as_str(self) -> &'static str {
            match self { OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceComplianceState :: Compliant => "COMPLIANT" , OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceComplianceState :: NonCompliant => "NON_COMPLIANT" , OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceComplianceState :: Unknown => "UNKNOWN" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceComplianceState
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceComplianceState
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceComplianceState,
            (),
        > {
            Ok (match s { "COMPLIANT" => OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceComplianceState :: Compliant , "NON_COMPLIANT" => OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceComplianceState :: NonCompliant , "UNKNOWN" => OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceComplianceState :: Unknown , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceComplianceState
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceComplianceState
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceComplianceState
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "COMPLIANT" => OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceComplianceState :: Compliant , "NON_COMPLIANT" => OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceComplianceState :: NonCompliant , "UNKNOWN" => OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceComplianceState :: Unknown , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceComplianceState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceComplianceState
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
    pub struct OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceExecResourceOutput {
        #[doc = "Output from enforcement phase output file (if run). Output size is limited to 100K bytes."]
        #[serde(
            rename = "enforcementOutput",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enforcement_output: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector
        for OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceExecResourceOutput
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceExecResourceOutput
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
    pub struct OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStep { # [doc = "An error message recorded during the execution of this step. Only populated if errors were encountered during this step execution."] # [serde (rename = "errorMessage" , default , skip_serializing_if = "std::option::Option::is_none")] pub error_message : :: std :: option :: Option < String > , # [doc = "Configuration step type."] # [serde (rename = "type" , default , skip_serializing_if = "std::option::Option::is_none")] pub r#type : :: std :: option :: Option < crate :: schemas :: OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType > , }
    impl :: google_field_selector :: FieldSelector for OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStep { fn fields () -> Vec < :: google_field_selector :: Field > { Vec :: new () } }
    impl :: google_field_selector :: ToFieldType for OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStep { fn field_type () -> :: google_field_selector :: FieldType { :: google_field_selector :: FieldType :: Leaf } }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType
    {
        #[doc = "Checks the current status of the desired state for a resource."]
        DesiredStateCheck,
        #[doc = "Re-checks the status of the desired state. This check is done for a resource after the enforcement of all OS policies. This step is used to determine the final desired state status for the resource. It accounts for any resources that might have drifted from their desired state due to side effects from executing other resources."]
        DesiredStateCheckPostEnforcement,
        #[doc = "Enforces the desired state for a resource that is not in desired state."]
        DesiredStateEnforcement,
        #[doc = "Default value. This value is unused."]
        TypeUnspecified,
        #[doc = "Checks for resource conflicts such as schema errors."]
        Validation,
    }
    impl OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType { pub fn as_str (self) -> & 'static str { match self { OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType :: DesiredStateCheck => "DESIRED_STATE_CHECK" , OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType :: DesiredStateCheckPostEnforcement => "DESIRED_STATE_CHECK_POST_ENFORCEMENT" , OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType :: DesiredStateEnforcement => "DESIRED_STATE_ENFORCEMENT" , OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType :: TypeUnspecified => "TYPE_UNSPECIFIED" , OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType :: Validation => "VALIDATION" , } } }
    impl :: std :: convert :: AsRef < str > for OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType { fn as_ref (& self) -> & str { self . as_str () } }
    impl :: std :: str :: FromStr for OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType { type Err = () ; fn from_str (s : & str) -> :: std :: result :: Result < OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType , () > { Ok (match s { "DESIRED_STATE_CHECK" => OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType :: DesiredStateCheck , "DESIRED_STATE_CHECK_POST_ENFORCEMENT" => OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType :: DesiredStateCheckPostEnforcement , "DESIRED_STATE_ENFORCEMENT" => OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType :: DesiredStateEnforcement , "TYPE_UNSPECIFIED" => OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType :: TypeUnspecified , "VALIDATION" => OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType :: Validation , _ => return Err (()) , }) } }
    impl :: std :: fmt :: Display for OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> :: std :: fmt :: Result { f . write_str (self . as_str ()) } }
    impl :: serde :: Serialize for OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType { fn serialize < S > (& self , serializer : S) -> :: std :: result :: Result < S :: Ok , S :: Error > where S : :: serde :: ser :: Serializer { serializer . serialize_str (self . as_str ()) } }
    impl < 'de > :: serde :: Deserialize < 'de > for OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType { fn deserialize < D > (deserializer : D) -> :: std :: result :: Result < Self , D :: Error > where D : :: serde :: de :: Deserializer < 'de > , { let value : & 'de str = < & str > :: deserialize (deserializer) ? ; Ok (match value { "DESIRED_STATE_CHECK" => OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType :: DesiredStateCheck , "DESIRED_STATE_CHECK_POST_ENFORCEMENT" => OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType :: DesiredStateCheckPostEnforcement , "DESIRED_STATE_ENFORCEMENT" => OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType :: DesiredStateEnforcement , "TYPE_UNSPECIFIED" => OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType :: TypeUnspecified , "VALIDATION" => OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType :: Validation , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , }) } }
    impl :: google_field_selector :: FieldSelector for OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType { fn fields () -> Vec < :: google_field_selector :: Field > { Vec :: new () } }
    impl :: google_field_selector :: ToFieldType for OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStepType { fn field_type () -> :: google_field_selector :: FieldType { :: google_field_selector :: FieldType :: Leaf } }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyAssignmentRollout {
        #[doc = "Required. The maximum number (or percentage) of VMs per zone to disrupt at any given moment."]
        #[serde(
            rename = "disruptionBudget",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disruption_budget: ::std::option::Option<crate::schemas::FixedOrPercent>,
        #[doc = "Required. This determines the minimum duration of time to wait after the configuration changes are applied through the current rollout. A VM continues to count towards the `disruption_budget` at least until this duration of time has passed after configuration changes are applied."]
        #[serde(
            rename = "minWaitDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_wait_duration: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyAssignmentRollout {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyAssignmentRollout {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyInventoryFilter {
        #[doc = "Required. The OS short name"]
        #[serde(
            rename = "osShortName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_short_name: ::std::option::Option<String>,
        #[doc = "The OS version Prefix matches are supported if asterisk(*) is provided as the last character. For example, to match all versions with a major version of `7`, specify the following value for this field `7.*` An empty string matches all OS versions."]
        #[serde(
            rename = "osVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyInventoryFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyInventoryFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyResource {
        #[doc = "Exec resource"]
        #[serde(
            rename = "exec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exec: ::std::option::Option<crate::schemas::OspolicyResourceExecResource>,
        #[doc = "File resource"]
        #[serde(
            rename = "file",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file: ::std::option::Option<crate::schemas::OspolicyResourceFileResource>,
        #[doc = "Required. The id of the resource with the following restrictions: * Must contain only lowercase letters, numbers, and hyphens. * Must start with a letter. * Must be between 1-63 characters. * Must end with a number or a letter. * Must be unique within the OS policy."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Package resource"]
        #[serde(
            rename = "pkg",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pkg: ::std::option::Option<crate::schemas::OspolicyResourcePackageResource>,
        #[doc = "Package repository resource"]
        #[serde(
            rename = "repository",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub repository: ::std::option::Option<crate::schemas::OspolicyResourceRepositoryResource>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyResource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyResourceExecResource {
        #[doc = "What to run to bring this resource into the desired state. An exit code of 100 indicates \"success\", any other exit code indicates a failure running enforce."]
        #[serde(
            rename = "enforce",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enforce: ::std::option::Option<crate::schemas::OspolicyResourceExecResourceExec>,
        #[doc = "Required. What to run to validate this resource is in the desired state. An exit code of 100 indicates \"in desired state\", and exit code of 101 indicates \"not in desired state\". Any other exit code indicates a failure running validate."]
        #[serde(
            rename = "validate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub validate: ::std::option::Option<crate::schemas::OspolicyResourceExecResourceExec>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourceExecResource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourceExecResource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyResourceExecResourceExec {
        #[doc = "Optional arguments to pass to the source during execution."]
        #[serde(
            rename = "args",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub args: ::std::option::Option<Vec<String>>,
        #[doc = "A remote or local file."]
        #[serde(
            rename = "file",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file: ::std::option::Option<crate::schemas::OspolicyResourceFile>,
        #[doc = "Required. The script interpreter to use."]
        #[serde(
            rename = "interpreter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub interpreter:
            ::std::option::Option<crate::schemas::OspolicyResourceExecResourceExecInterpreter>,
        #[doc = "Only recorded for enforce Exec. Path to an output file (that is created by this Exec) whose content will be recorded in OSPolicyResourceCompliance after a successful run. Absence or failure to read this file will result in this ExecResource being non-compliant. Output file size is limited to 100K bytes."]
        #[serde(
            rename = "outputFilePath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_file_path: ::std::option::Option<String>,
        #[doc = "An inline script. The size of the script is limited to 1024 characters."]
        #[serde(
            rename = "script",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub script: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourceExecResourceExec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourceExecResourceExec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OspolicyResourceExecResourceExecInterpreter {
        #[doc = "Invalid value, the request will return validation error."]
        InterpreterUnspecified,
        #[doc = "If an interpreter is not specified, the source is executed directly. This execution, without an interpreter, only succeeds for executables and scripts that have shebang lines."]
        None,
        #[doc = "Indicates that the script runs with PowerShell."]
        Powershell,
        #[doc = "Indicates that the script runs with `/bin/sh` on Linux and `cmd.exe` on Windows."]
        Shell,
    }
    impl OspolicyResourceExecResourceExecInterpreter {
        pub fn as_str(self) -> &'static str {
            match self {
                OspolicyResourceExecResourceExecInterpreter::InterpreterUnspecified => {
                    "INTERPRETER_UNSPECIFIED"
                }
                OspolicyResourceExecResourceExecInterpreter::None => "NONE",
                OspolicyResourceExecResourceExecInterpreter::Powershell => "POWERSHELL",
                OspolicyResourceExecResourceExecInterpreter::Shell => "SHELL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OspolicyResourceExecResourceExecInterpreter {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OspolicyResourceExecResourceExecInterpreter {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<OspolicyResourceExecResourceExecInterpreter, ()> {
            Ok(match s {
                "INTERPRETER_UNSPECIFIED" => {
                    OspolicyResourceExecResourceExecInterpreter::InterpreterUnspecified
                }
                "NONE" => OspolicyResourceExecResourceExecInterpreter::None,
                "POWERSHELL" => OspolicyResourceExecResourceExecInterpreter::Powershell,
                "SHELL" => OspolicyResourceExecResourceExecInterpreter::Shell,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OspolicyResourceExecResourceExecInterpreter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OspolicyResourceExecResourceExecInterpreter {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OspolicyResourceExecResourceExecInterpreter {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INTERPRETER_UNSPECIFIED" => {
                    OspolicyResourceExecResourceExecInterpreter::InterpreterUnspecified
                }
                "NONE" => OspolicyResourceExecResourceExecInterpreter::None,
                "POWERSHELL" => OspolicyResourceExecResourceExecInterpreter::Powershell,
                "SHELL" => OspolicyResourceExecResourceExecInterpreter::Shell,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourceExecResourceExecInterpreter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourceExecResourceExecInterpreter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyResourceFile {
        #[doc = "Defaults to false. When false, files are subject to validations based on the file type: Remote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
        #[serde(
            rename = "allowInsecure",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_insecure: ::std::option::Option<bool>,
        #[doc = "A Cloud Storage object."]
        #[serde(
            rename = "gcs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs: ::std::option::Option<crate::schemas::OspolicyResourceFileGcs>,
        #[doc = "A local path within the VM to use."]
        #[serde(
            rename = "localPath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub local_path: ::std::option::Option<String>,
        #[doc = "A generic remote file."]
        #[serde(
            rename = "remote",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub remote: ::std::option::Option<crate::schemas::OspolicyResourceFileRemote>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourceFile {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourceFile {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyResourceFileGcs {
        #[doc = "Required. Bucket of the Cloud Storage object."]
        #[serde(
            rename = "bucket",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucket: ::std::option::Option<String>,
        #[doc = "Generation number of the Cloud Storage object."]
        #[serde(
            rename = "generation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub generation: ::std::option::Option<i64>,
        #[doc = "Required. Name of the Cloud Storage object."]
        #[serde(
            rename = "object",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourceFileGcs {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourceFileGcs {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyResourceFileRemote {
        #[doc = "SHA256 checksum of the remote file."]
        #[serde(
            rename = "sha256Checksum",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sha_256_checksum: ::std::option::Option<String>,
        #[doc = "Required. URI from which to fetch the object. It should contain both the protocol and path following the format `{protocol}://{location}`."]
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourceFileRemote {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourceFileRemote {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyResourceFileResource {
        #[doc = "A a file with this content. The size of the content is limited to 1024 characters."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<String>,
        #[doc = "A remote or local source."]
        #[serde(
            rename = "file",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file: ::std::option::Option<crate::schemas::OspolicyResourceFile>,
        #[doc = "Required. The absolute path of the file within the VM."]
        #[serde(
            rename = "path",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path: ::std::option::Option<String>,
        #[doc = "Consists of three octal digits which represent, in order, the permissions of the owner, group, and other users for the file (similarly to the numeric mode used in the linux chmod utility). Each digit represents a three bit number with the 4 bit corresponding to the read permissions, the 2 bit corresponds to the write bit, and the one bit corresponds to the execute permission. Default behavior is 755. Below are some examples of permissions and their associated values: read, write, and execute: 7 read and execute: 5 read and write: 6 read only: 4"]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions: ::std::option::Option<String>,
        #[doc = "Required. Desired state of the file."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::OspolicyResourceFileResourceState>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourceFileResource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourceFileResource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OspolicyResourceFileResourceState {
        #[doc = "Ensure file at path is absent."]
        Absent,
        #[doc = "Ensure the contents of the file at path matches. If the file does not exist it will be created."]
        ContentsMatch,
        #[doc = "Unspecified is invalid."]
        DesiredStateUnspecified,
        #[doc = "Ensure file at path is present."]
        Present,
    }
    impl OspolicyResourceFileResourceState {
        pub fn as_str(self) -> &'static str {
            match self {
                OspolicyResourceFileResourceState::Absent => "ABSENT",
                OspolicyResourceFileResourceState::ContentsMatch => "CONTENTS_MATCH",
                OspolicyResourceFileResourceState::DesiredStateUnspecified => {
                    "DESIRED_STATE_UNSPECIFIED"
                }
                OspolicyResourceFileResourceState::Present => "PRESENT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OspolicyResourceFileResourceState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OspolicyResourceFileResourceState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<OspolicyResourceFileResourceState, ()> {
            Ok(match s {
                "ABSENT" => OspolicyResourceFileResourceState::Absent,
                "CONTENTS_MATCH" => OspolicyResourceFileResourceState::ContentsMatch,
                "DESIRED_STATE_UNSPECIFIED" => {
                    OspolicyResourceFileResourceState::DesiredStateUnspecified
                }
                "PRESENT" => OspolicyResourceFileResourceState::Present,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OspolicyResourceFileResourceState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OspolicyResourceFileResourceState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OspolicyResourceFileResourceState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ABSENT" => OspolicyResourceFileResourceState::Absent,
                "CONTENTS_MATCH" => OspolicyResourceFileResourceState::ContentsMatch,
                "DESIRED_STATE_UNSPECIFIED" => {
                    OspolicyResourceFileResourceState::DesiredStateUnspecified
                }
                "PRESENT" => OspolicyResourceFileResourceState::Present,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourceFileResourceState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourceFileResourceState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyResourceGroup {
        #[doc = "List of inventory filters for the resource group. The resources in this resource group are applied to the target VM if it satisfies at least one of the following inventory filters. For example, to apply this resource group to VMs running either `RHEL` or `CentOS` operating systems, specify 2 items for the list with following values: inventory_filters[0].os_short_name='rhel' and inventory_filters[1].os_short_name='centos' If the list is empty, this resource group will be applied to the target VM unconditionally."]
        #[serde(
            rename = "inventoryFilters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inventory_filters: ::std::option::Option<Vec<crate::schemas::OspolicyInventoryFilter>>,
        #[doc = "Required. List of resources configured for this resource group. The resources are executed in the exact order specified here."]
        #[serde(
            rename = "resources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resources: ::std::option::Option<Vec<crate::schemas::OspolicyResource>>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourceGroup {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourceGroup {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyResourcePackageResource {
        #[doc = "A package managed by Apt."]
        #[serde(
            rename = "apt",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub apt: ::std::option::Option<crate::schemas::OspolicyResourcePackageResourceAPT>,
        #[doc = "A deb package file."]
        #[serde(
            rename = "deb",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deb: ::std::option::Option<crate::schemas::OspolicyResourcePackageResourceDeb>,
        #[doc = "Required. The desired state the agent should maintain for this package."]
        #[serde(
            rename = "desiredState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub desired_state:
            ::std::option::Option<crate::schemas::OspolicyResourcePackageResourceDesiredState>,
        #[doc = "A package managed by GooGet."]
        #[serde(
            rename = "googet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub googet: ::std::option::Option<crate::schemas::OspolicyResourcePackageResourceGooGet>,
        #[doc = "An MSI package."]
        #[serde(
            rename = "msi",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub msi: ::std::option::Option<crate::schemas::OspolicyResourcePackageResourceMSI>,
        #[doc = "An rpm package file."]
        #[serde(
            rename = "rpm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rpm: ::std::option::Option<crate::schemas::OspolicyResourcePackageResourceRPM>,
        #[doc = "A package managed by YUM."]
        #[serde(
            rename = "yum",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub yum: ::std::option::Option<crate::schemas::OspolicyResourcePackageResourceYUM>,
        #[doc = "A package managed by Zypper."]
        #[serde(
            rename = "zypper",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub zypper: ::std::option::Option<crate::schemas::OspolicyResourcePackageResourceZypper>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourcePackageResource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourcePackageResource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OspolicyResourcePackageResourceDesiredState {
        #[doc = "Unspecified is invalid."]
        DesiredStateUnspecified,
        #[doc = "Ensure that the package is installed."]
        Installed,
        #[doc = "The agent ensures that the package is not installed and uninstalls it if detected."]
        Removed,
    }
    impl OspolicyResourcePackageResourceDesiredState {
        pub fn as_str(self) -> &'static str {
            match self {
                OspolicyResourcePackageResourceDesiredState::DesiredStateUnspecified => {
                    "DESIRED_STATE_UNSPECIFIED"
                }
                OspolicyResourcePackageResourceDesiredState::Installed => "INSTALLED",
                OspolicyResourcePackageResourceDesiredState::Removed => "REMOVED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OspolicyResourcePackageResourceDesiredState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OspolicyResourcePackageResourceDesiredState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<OspolicyResourcePackageResourceDesiredState, ()> {
            Ok(match s {
                "DESIRED_STATE_UNSPECIFIED" => {
                    OspolicyResourcePackageResourceDesiredState::DesiredStateUnspecified
                }
                "INSTALLED" => OspolicyResourcePackageResourceDesiredState::Installed,
                "REMOVED" => OspolicyResourcePackageResourceDesiredState::Removed,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OspolicyResourcePackageResourceDesiredState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OspolicyResourcePackageResourceDesiredState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OspolicyResourcePackageResourceDesiredState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DESIRED_STATE_UNSPECIFIED" => {
                    OspolicyResourcePackageResourceDesiredState::DesiredStateUnspecified
                }
                "INSTALLED" => OspolicyResourcePackageResourceDesiredState::Installed,
                "REMOVED" => OspolicyResourcePackageResourceDesiredState::Removed,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourcePackageResourceDesiredState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourcePackageResourceDesiredState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyResourcePackageResourceAPT {
        #[doc = "Required. Package name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourcePackageResourceAPT {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourcePackageResourceAPT {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyResourcePackageResourceDeb {
        #[doc = "Whether dependencies should also be installed. - install when false: `dpkg -i package` - install when true: `apt-get update && apt-get -y install package.deb`"]
        #[serde(
            rename = "pullDeps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pull_deps: ::std::option::Option<bool>,
        #[doc = "Required. A deb package."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::OspolicyResourceFile>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourcePackageResourceDeb {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourcePackageResourceDeb {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyResourcePackageResourceGooGet {
        #[doc = "Required. Package name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourcePackageResourceGooGet {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourcePackageResourceGooGet {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyResourcePackageResourceMSI {
        #[doc = "Additional properties to use during installation. This should be in the format of Property=Setting. Appended to the defaults of `ACTION=INSTALL REBOOT=ReallySuppress`."]
        #[serde(
            rename = "properties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub properties: ::std::option::Option<Vec<String>>,
        #[doc = "Required. The MSI package."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::OspolicyResourceFile>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourcePackageResourceMSI {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourcePackageResourceMSI {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyResourcePackageResourceRPM {
        #[doc = "Whether dependencies should also be installed. - install when false: `rpm --upgrade --replacepkgs package.rpm` - install when true: `yum -y install package.rpm` or `zypper -y install package.rpm`"]
        #[serde(
            rename = "pullDeps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pull_deps: ::std::option::Option<bool>,
        #[doc = "Required. An rpm package."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::OspolicyResourceFile>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourcePackageResourceRPM {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourcePackageResourceRPM {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyResourcePackageResourceYUM {
        #[doc = "Required. Package name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourcePackageResourceYUM {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourcePackageResourceYUM {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyResourcePackageResourceZypper {
        #[doc = "Required. Package name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourcePackageResourceZypper {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourcePackageResourceZypper {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyResourceRepositoryResource {
        #[doc = "An Apt Repository."]
        #[serde(
            rename = "apt",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub apt:
            ::std::option::Option<crate::schemas::OspolicyResourceRepositoryResourceAptRepository>,
        #[doc = "A Goo Repository."]
        #[serde(
            rename = "goo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub goo:
            ::std::option::Option<crate::schemas::OspolicyResourceRepositoryResourceGooRepository>,
        #[doc = "A Yum Repository."]
        #[serde(
            rename = "yum",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub yum:
            ::std::option::Option<crate::schemas::OspolicyResourceRepositoryResourceYumRepository>,
        #[doc = "A Zypper Repository."]
        #[serde(
            rename = "zypper",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub zypper: ::std::option::Option<
            crate::schemas::OspolicyResourceRepositoryResourceZypperRepository,
        >,
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourceRepositoryResource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourceRepositoryResource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyResourceRepositoryResourceAptRepository {
        #[doc = "Required. Type of archive files in this repository."]
        #[serde(
            rename = "archiveType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub archive_type: ::std::option::Option<
            crate::schemas::OspolicyResourceRepositoryResourceAptRepositoryArchiveType,
        >,
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
        #[doc = "URI of the key file for this repository. The agent maintains a keyring at `/etc/apt/trusted.gpg.d/osconfig_agent_managed.gpg`."]
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
    impl ::google_field_selector::FieldSelector for OspolicyResourceRepositoryResourceAptRepository {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourceRepositoryResourceAptRepository {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OspolicyResourceRepositoryResourceAptRepositoryArchiveType {
        #[doc = "Unspecified is invalid."]
        ArchiveTypeUnspecified,
        #[doc = "Deb indicates that the archive contains binary files."]
        Deb,
        #[doc = "Deb-src indicates that the archive contains source files."]
        DebSrc,
    }
    impl OspolicyResourceRepositoryResourceAptRepositoryArchiveType {
        pub fn as_str(self) -> &'static str {
            match self { OspolicyResourceRepositoryResourceAptRepositoryArchiveType :: ArchiveTypeUnspecified => "ARCHIVE_TYPE_UNSPECIFIED" , OspolicyResourceRepositoryResourceAptRepositoryArchiveType :: Deb => "DEB" , OspolicyResourceRepositoryResourceAptRepositoryArchiveType :: DebSrc => "DEB_SRC" , }
        }
    }
    impl ::std::convert::AsRef<str> for OspolicyResourceRepositoryResourceAptRepositoryArchiveType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OspolicyResourceRepositoryResourceAptRepositoryArchiveType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<OspolicyResourceRepositoryResourceAptRepositoryArchiveType, ()>
        {
            Ok (match s { "ARCHIVE_TYPE_UNSPECIFIED" => OspolicyResourceRepositoryResourceAptRepositoryArchiveType :: ArchiveTypeUnspecified , "DEB" => OspolicyResourceRepositoryResourceAptRepositoryArchiveType :: Deb , "DEB_SRC" => OspolicyResourceRepositoryResourceAptRepositoryArchiveType :: DebSrc , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for OspolicyResourceRepositoryResourceAptRepositoryArchiveType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OspolicyResourceRepositoryResourceAptRepositoryArchiveType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OspolicyResourceRepositoryResourceAptRepositoryArchiveType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ARCHIVE_TYPE_UNSPECIFIED" => OspolicyResourceRepositoryResourceAptRepositoryArchiveType :: ArchiveTypeUnspecified , "DEB" => OspolicyResourceRepositoryResourceAptRepositoryArchiveType :: Deb , "DEB_SRC" => OspolicyResourceRepositoryResourceAptRepositoryArchiveType :: DebSrc , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for OspolicyResourceRepositoryResourceAptRepositoryArchiveType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for OspolicyResourceRepositoryResourceAptRepositoryArchiveType
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
    pub struct OspolicyResourceRepositoryResourceGooRepository {
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
    impl ::google_field_selector::FieldSelector for OspolicyResourceRepositoryResourceGooRepository {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourceRepositoryResourceGooRepository {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyResourceRepositoryResourceYumRepository {
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
        #[doc = "Required. A one word, unique name for this repository. This is the `repo id` in the yum config file and also the `display_name` if `display_name` is omitted. This id is also used as the unique identifier when checking for resource conflicts."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourceRepositoryResourceYumRepository {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourceRepositoryResourceYumRepository {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyResourceRepositoryResourceZypperRepository {
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
        #[doc = "Required. A one word, unique name for this repository. This is the `repo id` in the zypper config file and also the `display_name` if `display_name` is omitted. This id is also used as the unique identifier when checking for GuestPolicy conflicts."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourceRepositoryResourceZypperRepository {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourceRepositoryResourceZypperRepository {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "Targets VM instances matching ANY of these GroupLabels. This allows targeting of disparate groups of VM instances."]
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct VulnerabilityReport {
        #[doc = "Output only. The `vulnerabilityReport` API resource name. Format: `projects/{project_number}/locations/{location}/instances/{instance_id}/vulnerabilityReport`"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. The timestamp for when the last vulnerability report was generated for the VM."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
        #[doc = "Output only. List of vulnerabilities affecting the VM."]
        #[serde(
            rename = "vulnerabilities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vulnerabilities:
            ::std::option::Option<Vec<crate::schemas::VulnerabilityReportVulnerability>>,
    }
    impl ::google_field_selector::FieldSelector for VulnerabilityReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VulnerabilityReport {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct VulnerabilityReportVulnerability {
        #[doc = "Corresponds to the `AVAILABLE_PACKAGE` inventory item on the VM. If the vulnerability report was not updated after the VM inventory update, these values might not display in VM inventory. If there is no available fix, the field is empty. The `inventory_item` value specifies the latest `SoftwarePackage` available to the VM that fixes the vulnerability."]
        #[serde(
            rename = "availableInventoryItemIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub available_inventory_item_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The timestamp for when the vulnerability was first detected."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Contains metadata as per the upstream feed of the operating system and NVD."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details: ::std::option::Option<crate::schemas::VulnerabilityReportVulnerabilityDetails>,
        #[doc = "Corresponds to the `INSTALLED_PACKAGE` inventory item on the VM. This field displays the inventory items affected by this vulnerability. If the vulnerability report was not updated after the VM inventory update, these values might not display in VM inventory. For some distros, this field may be empty."]
        #[serde(
            rename = "installedInventoryItemIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub installed_inventory_item_ids: ::std::option::Option<Vec<String>>,
        #[doc = "List of items affected by the vulnerability."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::VulnerabilityReportVulnerabilityItem>>,
        #[doc = "The timestamp for when the vulnerability was last modified."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VulnerabilityReportVulnerability {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VulnerabilityReportVulnerability {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct VulnerabilityReportVulnerabilityDetails {
        #[doc = "The CVE of the vulnerability. CVE cannot be empty and the combination of should be unique across vulnerabilities for a VM."]
        #[serde(
            rename = "cve",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cve: ::std::option::Option<String>,
        #[doc = "The CVSS V2 score of this vulnerability. CVSS V2 score is on a scale of 0 - 10 where 0 indicates low severity and 10 indicates high severity."]
        #[serde(
            rename = "cvssV2Score",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cvss_v2_score: ::std::option::Option<f32>,
        #[doc = "The full description of the CVSSv3 for this vulnerability from NVD."]
        #[serde(
            rename = "cvssV3",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cvss_v3: ::std::option::Option<crate::schemas::Cvssv3>,
        #[doc = "The note or description describing the vulnerability from the distro."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Corresponds to the references attached to the `VulnerabilityDetails`."]
        #[serde(
            rename = "references",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub references: ::std::option::Option<
            Vec<crate::schemas::VulnerabilityReportVulnerabilityDetailsReference>,
        >,
        #[doc = "Assigned severity/impact ranking from the distro."]
        #[serde(
            rename = "severity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub severity: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VulnerabilityReportVulnerabilityDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VulnerabilityReportVulnerabilityDetails {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct VulnerabilityReportVulnerabilityDetailsReference {
        #[doc = "The source of the reference e.g. NVD."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<String>,
        #[doc = "The url of the reference."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VulnerabilityReportVulnerabilityDetailsReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VulnerabilityReportVulnerabilityDetailsReference {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct VulnerabilityReportVulnerabilityItem {
        #[doc = "Corresponds to the `AVAILABLE_PACKAGE` inventory item on the VM. If the vulnerability report was not updated after the VM inventory update, these values might not display in VM inventory. If there is no available fix, the field is empty. The `inventory_item` value specifies the latest `SoftwarePackage` available to the VM that fixes the vulnerability."]
        #[serde(
            rename = "availableInventoryItemId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub available_inventory_item_id: ::std::option::Option<String>,
        #[doc = "The recommended [CPE URI](https://cpe.mitre.org/specification/) update that contains a fix for this vulnerability."]
        #[serde(
            rename = "fixedCpeUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fixed_cpe_uri: ::std::option::Option<String>,
        #[doc = "Corresponds to the `INSTALLED_PACKAGE` inventory item on the VM. This field displays the inventory items affected by this vulnerability. If the vulnerability report was not updated after the VM inventory update, these values might not display in VM inventory. For some operating systems, this field might be empty."]
        #[serde(
            rename = "installedInventoryItemId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub installed_inventory_item_id: ::std::option::Option<String>,
        #[doc = "The upstream OS patch, packages or KB that fixes the vulnerability."]
        #[serde(
            rename = "upstreamFix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub upstream_fix: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VulnerabilityReportVulnerabilityItem {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VulnerabilityReportVulnerabilityItem {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
            #[doc = "Actions that can be performed on the locations resource"]
            pub fn locations(&self) -> crate::resources::projects::locations::LocationsActions {
                crate::resources::projects::locations::LocationsActions {
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
                #[doc = "Actions that can be performed on the instances resource"]
                pub fn instances(
                    &self,
                ) -> crate::resources::projects::locations::instances::InstancesActions
                {
                    crate::resources::projects::locations::instances::InstancesActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
                #[doc = "Actions that can be performed on the os_policy_assignments resource"]                pub fn os_policy_assignments (& self) -> crate :: resources :: projects :: locations :: os_policy_assignments :: OsPolicyAssignmentsActions{
                    crate :: resources :: projects :: locations :: os_policy_assignments :: OsPolicyAssignmentsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
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
                    #[doc = "Actions that can be performed on the inventories resource"]                    pub fn inventories (& self) -> crate :: resources :: projects :: locations :: instances :: inventories :: InventoriesActions{
                        crate :: resources :: projects :: locations :: instances :: inventories :: InventoriesActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                    #[doc = "Actions that can be performed on the os_policy_assignments resource"]                    pub fn os_policy_assignments (& self) -> crate :: resources :: projects :: locations :: instances :: os_policy_assignments :: OsPolicyAssignmentsActions{
                        crate :: resources :: projects :: locations :: instances :: os_policy_assignments :: OsPolicyAssignmentsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                    #[doc = "Actions that can be performed on the vulnerability_reports resource"]                    pub fn vulnerability_reports (& self) -> crate :: resources :: projects :: locations :: instances :: vulnerability_reports :: VulnerabilityReportsActions{
                        crate :: resources :: projects :: locations :: instances :: vulnerability_reports :: VulnerabilityReportsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                }
                pub mod inventories {
                    pub mod params {
                        #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                        pub enum GetView {
                            #[doc = "Returns the basic inventory information that includes `os_info`."]
                            Basic,
                            #[doc = "Returns all fields."]
                            Full,
                            #[doc = "The default value. The API defaults to the BASIC view."]
                            InventoryViewUnspecified,
                        }
                        impl GetView {
                            pub fn as_str(self) -> &'static str {
                                match self {
                                    GetView::Basic => "BASIC",
                                    GetView::Full => "FULL",
                                    GetView::InventoryViewUnspecified => {
                                        "INVENTORY_VIEW_UNSPECIFIED"
                                    }
                                }
                            }
                        }
                        impl ::std::convert::AsRef<str> for GetView {
                            fn as_ref(&self) -> &str {
                                self.as_str()
                            }
                        }
                        impl ::std::str::FromStr for GetView {
                            type Err = ();
                            fn from_str(s: &str) -> ::std::result::Result<GetView, ()> {
                                Ok(match s {
                                    "BASIC" => GetView::Basic,
                                    "FULL" => GetView::Full,
                                    "INVENTORY_VIEW_UNSPECIFIED" => {
                                        GetView::InventoryViewUnspecified
                                    }
                                    _ => return Err(()),
                                })
                            }
                        }
                        impl ::std::fmt::Display for GetView {
                            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                                f.write_str(self.as_str())
                            }
                        }
                        impl ::serde::Serialize for GetView {
                            fn serialize<S>(
                                &self,
                                serializer: S,
                            ) -> ::std::result::Result<S::Ok, S::Error>
                            where
                                S: ::serde::ser::Serializer,
                            {
                                serializer.serialize_str(self.as_str())
                            }
                        }
                        impl<'de> ::serde::Deserialize<'de> for GetView {
                            fn deserialize<D>(
                                deserializer: D,
                            ) -> ::std::result::Result<Self, D::Error>
                            where
                                D: ::serde::de::Deserializer<'de>,
                            {
                                let value: &'de str = <&str>::deserialize(deserializer)?;
                                Ok(match value {
                                    "BASIC" => GetView::Basic,
                                    "FULL" => GetView::Full,
                                    "INVENTORY_VIEW_UNSPECIFIED" => {
                                        GetView::InventoryViewUnspecified
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
                        impl ::google_field_selector::FieldSelector for GetView {
                            fn fields() -> Vec<::google_field_selector::Field> {
                                Vec::new()
                            }
                        }
                        impl ::google_field_selector::ToFieldType for GetView {
                            fn field_type() -> ::google_field_selector::FieldType {
                                ::google_field_selector::FieldType::Leaf
                            }
                        }
                        #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                        pub enum ListView {
                            #[doc = "Returns the basic inventory information that includes `os_info`."]
                            Basic,
                            #[doc = "Returns all fields."]
                            Full,
                            #[doc = "The default value. The API defaults to the BASIC view."]
                            InventoryViewUnspecified,
                        }
                        impl ListView {
                            pub fn as_str(self) -> &'static str {
                                match self {
                                    ListView::Basic => "BASIC",
                                    ListView::Full => "FULL",
                                    ListView::InventoryViewUnspecified => {
                                        "INVENTORY_VIEW_UNSPECIFIED"
                                    }
                                }
                            }
                        }
                        impl ::std::convert::AsRef<str> for ListView {
                            fn as_ref(&self) -> &str {
                                self.as_str()
                            }
                        }
                        impl ::std::str::FromStr for ListView {
                            type Err = ();
                            fn from_str(s: &str) -> ::std::result::Result<ListView, ()> {
                                Ok(match s {
                                    "BASIC" => ListView::Basic,
                                    "FULL" => ListView::Full,
                                    "INVENTORY_VIEW_UNSPECIFIED" => {
                                        ListView::InventoryViewUnspecified
                                    }
                                    _ => return Err(()),
                                })
                            }
                        }
                        impl ::std::fmt::Display for ListView {
                            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                                f.write_str(self.as_str())
                            }
                        }
                        impl ::serde::Serialize for ListView {
                            fn serialize<S>(
                                &self,
                                serializer: S,
                            ) -> ::std::result::Result<S::Ok, S::Error>
                            where
                                S: ::serde::ser::Serializer,
                            {
                                serializer.serialize_str(self.as_str())
                            }
                        }
                        impl<'de> ::serde::Deserialize<'de> for ListView {
                            fn deserialize<D>(
                                deserializer: D,
                            ) -> ::std::result::Result<Self, D::Error>
                            where
                                D: ::serde::de::Deserializer<'de>,
                            {
                                let value: &'de str = <&str>::deserialize(deserializer)?;
                                Ok(match value {
                                    "BASIC" => ListView::Basic,
                                    "FULL" => ListView::Full,
                                    "INVENTORY_VIEW_UNSPECIFIED" => {
                                        ListView::InventoryViewUnspecified
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
                        impl ::google_field_selector::FieldSelector for ListView {
                            fn fields() -> Vec<::google_field_selector::Field> {
                                Vec::new()
                            }
                        }
                        impl ::google_field_selector::ToFieldType for ListView {
                            fn field_type() -> ::google_field_selector::FieldType {
                                ::google_field_selector::FieldType::Leaf
                            }
                        }
                    }
                    pub struct InventoriesActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> InventoriesActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Get inventory data for the specified VM instance. If the VM has no associated inventory, the message `NOT_FOUND` is returned."]
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
                                view: None,
                            }
                        }
                        #[doc = "List inventory data for all VM instances in the specified zone."]
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
                                view: None,
                            }
                        }
                    }
                    #[doc = "Created via [InventoriesActions::get()](struct.InventoriesActions.html#method.get)"]
                    #[derive(Debug, Clone)]
                    pub struct GetRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , name : String , view : Option < crate :: resources :: projects :: locations :: instances :: inventories :: params :: GetView > , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
                    impl<'a> GetRequestBuilder<'a> {
                        #[doc = "Inventory view indicating what information should be included in the inventory resource. If unspecified, the default view is BASIC."]
                        pub fn view(
                            mut self,
                            value : crate :: resources :: projects :: locations :: instances :: inventories :: params :: GetView,
                        ) -> Self {
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
                        ) -> Result<crate::schemas::Inventory, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::Inventory, crate::Error>
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
                    #[doc = "Created via [InventoriesActions::list()](struct.InventoriesActions.html#method.list)"]
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , parent : String , filter : Option < String > , page_size : Option < i32 > , page_token : Option < String > , view : Option < crate :: resources :: projects :: locations :: instances :: inventories :: params :: ListView > , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
                    impl<'a> ListRequestBuilder<'a> {
                        #[doc = "If provided, this field specifies the criteria that must be met by a `Inventory` API resource to be included in the response."]
                        pub fn filter(mut self, value: impl Into<String>) -> Self {
                            self.filter = Some(value.into());
                            self
                        }
                        #[doc = "The maximum number of results to return."]
                        pub fn page_size(mut self, value: i32) -> Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "A pagination token returned from a previous call to `ListInventories` that indicates where this listing should continue from."]
                        pub fn page_token(mut self, value: impl Into<String>) -> Self {
                            self.page_token = Some(value.into());
                            self
                        }
                        #[doc = "Inventory view indicating what information should be included in the inventory resource. If unspecified, the default view is BASIC."]
                        pub fn view(
                            mut self,
                            value : crate :: resources :: projects :: locations :: instances :: inventories :: params :: ListView,
                        ) -> Self {
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
                        ) -> Result<crate::schemas::ListInventoriesResponse, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::ListInventoriesResponse, crate::Error>
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
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/inventories");
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
                }
                pub mod os_policy_assignments {
                    pub mod params {}
                    pub struct OsPolicyAssignmentsActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> OsPolicyAssignmentsActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Actions that can be performed on the reports resource"]                        pub fn reports (& self) -> crate :: resources :: projects :: locations :: instances :: os_policy_assignments :: reports :: ReportsActions{
                            crate :: resources :: projects :: locations :: instances :: os_policy_assignments :: reports :: ReportsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                        }
                    }
                    pub mod reports {
                        pub mod params {}
                        pub struct ReportsActions<'a> {
                            pub(crate) reqwest: &'a reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        }
                        impl<'a> ReportsActions<'a> {
                            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                                self.auth
                            }
                            #[doc = "Get the OS policy asssignment report for the specified Compute Engine VM instance."]
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
                            #[doc = "List OS policy asssignment reports for all Compute Engine VM instances in the specified zone."]
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
                        #[doc = "Created via [ReportsActions::get()](struct.ReportsActions.html#method.get)"]
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
                            ) -> Result<crate::schemas::OspolicyAssignmentReport, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::OspolicyAssignmentReport, crate::Error>
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
                        #[doc = "Created via [ReportsActions::list()](struct.ReportsActions.html#method.list)"]
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
                            #[doc = "If provided, this field specifies the criteria that must be met by the `OSPolicyAssignmentReport` API resource that is included in the response."]
                            pub fn filter(mut self, value: impl Into<String>) -> Self {
                                self.filter = Some(value.into());
                                self
                            }
                            #[doc = "The maximum number of results to return."]
                            pub fn page_size(mut self, value: i32) -> Self {
                                self.page_size = Some(value);
                                self
                            }
                            #[doc = "A pagination token returned from a previous call to the `ListOSPolicyAssignmentReports` method that indicates where this listing should continue from."]
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
                                crate::schemas::ListOSPolicyAssignmentReportsResponse,
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
                                crate::schemas::ListOSPolicyAssignmentReportsResponse,
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
                                let mut output = "https://osconfig.googleapis.com/".to_owned();
                                output.push_str("v1/");
                                {
                                    let var_as_str = &self.parent;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::RESERVED,
                                    ));
                                }
                                output.push_str("/reports");
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
                pub mod vulnerability_reports {
                    pub mod params {}
                    pub struct VulnerabilityReportsActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> VulnerabilityReportsActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Gets the vulnerability report for the specified VM instance. Only VMs with inventory data have vulnerability reports associated with them."]
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
                        #[doc = "List vulnerability reports for all VM instances in the specified zone."]
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
                    #[doc = "Created via [VulnerabilityReportsActions::get()](struct.VulnerabilityReportsActions.html#method.get)"]
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
                        ) -> Result<crate::schemas::VulnerabilityReport, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::VulnerabilityReport, crate::Error>
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
                    #[doc = "Created via [VulnerabilityReportsActions::list()](struct.VulnerabilityReportsActions.html#method.list)"]
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
                        #[doc = "This field supports filtering by the severity level for the vulnerability. For a list of severity levels, see [Severity levels for vulnerabilities](https://cloud.google.com/container-analysis/docs/container-scanning-overview#severity_levels_for_vulnerabilities). The filter field follows the rules described in the [AIP-160](https://google.aip.dev/160) guidelines as follows: + **Filter for a specific severity type**: you can list reports that contain vulnerabilities that are classified as medium by specifying `vulnerabilities.details.severity:MEDIUM`. + **Filter for a range of severities** : you can list reports that have vulnerabilities that are classified as critical or high by specifying `vulnerabilities.details.severity:HIGH OR vulnerabilities.details.severity:CRITICAL`"]
                        pub fn filter(mut self, value: impl Into<String>) -> Self {
                            self.filter = Some(value.into());
                            self
                        }
                        #[doc = "The maximum number of results to return."]
                        pub fn page_size(mut self, value: i32) -> Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "A pagination token returned from a previous call to `ListVulnerabilityReports` that indicates where this listing should continue from."]
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
                        ) -> Result<crate::schemas::ListVulnerabilityReportsResponse, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::ListVulnerabilityReportsResponse, crate::Error>
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
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/vulnerabilityReports");
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
            pub mod os_policy_assignments {
                pub mod params {}
                pub struct OsPolicyAssignmentsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> OsPolicyAssignmentsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Create an OS policy assignment. This method also creates the first revision of the OS policy assignment. This method returns a long running operation (LRO) that contains the rollout details. The rollout can be cancelled by cancelling the LRO. For more information, see [Method: projects.locations.osPolicyAssignments.operations.cancel](https://cloud.google.com/compute/docs/osconfig/rest/v1/projects.locations.osPolicyAssignments.operations/cancel)."]
                    pub fn create(
                        &self,
                        request: crate::schemas::OspolicyAssignment,
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
                            os_policy_assignment_id: None,
                        }
                    }
                    #[doc = "Delete the OS policy assignment. This method creates a new revision of the OS policy assignment. This method returns a long running operation (LRO) that contains the rollout details. The rollout can be cancelled by cancelling the LRO. If the LRO completes and is not cancelled, all revisions associated with the OS policy assignment are deleted. For more information, see [Method: projects.locations.osPolicyAssignments.operations.cancel](https://cloud.google.com/compute/docs/osconfig/rest/v1/projects.locations.osPolicyAssignments.operations/cancel)."]
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
                    #[doc = "Retrieve an existing OS policy assignment. This method always returns the latest revision. In order to retrieve a previous revision of the assignment, also provide the revision ID in the `name` parameter."]
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
                    #[doc = "List the OS policy assignments under the parent resource. For each OS policy assignment, the latest revision is returned."]
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
                    #[doc = "List the OS policy assignment revisions for a given OS policy assignment."]
                    pub fn list_revisions(
                        &self,
                        name: impl Into<String>,
                    ) -> ListRevisionsRequestBuilder {
                        ListRevisionsRequestBuilder {
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
                            page_size: None,
                            page_token: None,
                        }
                    }
                    #[doc = "Update an existing OS policy assignment. This method creates a new revision of the OS policy assignment. This method returns a long running operation (LRO) that contains the rollout details. The rollout can be cancelled by cancelling the LRO. For more information, see [Method: projects.locations.osPolicyAssignments.operations.cancel](https://cloud.google.com/compute/docs/osconfig/rest/v1/projects.locations.osPolicyAssignments.operations/cancel)."]
                    pub fn patch(
                        &self,
                        request: crate::schemas::OspolicyAssignment,
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
                    #[doc = "Actions that can be performed on the operations resource"]                    pub fn operations (& self) -> crate :: resources :: projects :: locations :: os_policy_assignments :: operations :: OperationsActions{
                        crate :: resources :: projects :: locations :: os_policy_assignments :: operations :: OperationsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                }
                #[doc = "Created via [OsPolicyAssignmentsActions::create()](struct.OsPolicyAssignmentsActions.html#method.create)"]
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::OspolicyAssignment,
                    parent: String,
                    os_policy_assignment_id: Option<String>,
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
                    #[doc = "Required. The logical name of the OS policy assignment in the project with the following restrictions: * Must contain only lowercase letters, numbers, and hyphens. * Must start with a letter. * Must be between 1-63 characters. * Must end with a number or a letter. * Must be unique within the project."]
                    pub fn os_policy_assignment_id(mut self, value: impl Into<String>) -> Self {
                        self.os_policy_assignment_id = Some(value.into());
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
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/osPolicyAssignments");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                        req = req.query(&[("osPolicyAssignmentId", &self.os_policy_assignment_id)]);
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
                #[doc = "Created via [OsPolicyAssignmentsActions::delete()](struct.OsPolicyAssignmentsActions.html#method.delete)"]
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
                #[doc = "Created via [OsPolicyAssignmentsActions::get()](struct.OsPolicyAssignmentsActions.html#method.get)"]
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
                    ) -> Result<crate::schemas::OspolicyAssignment, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::OspolicyAssignment, crate::Error>
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
                #[doc = "Created via [OsPolicyAssignmentsActions::list()](struct.OsPolicyAssignmentsActions.html#method.list)"]
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
                    #[doc = "The maximum number of assignments to return."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A pagination token returned from a previous call to `ListOSPolicyAssignments` that indicates where this listing should continue from."]
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
                    ) -> Result<crate::schemas::ListOSPolicyAssignmentsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListOSPolicyAssignmentsResponse, crate::Error>
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
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/osPolicyAssignments");
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
                #[doc = "Created via [OsPolicyAssignmentsActions::list_revisions()](struct.OsPolicyAssignmentsActions.html#method.list_revisions)"]
                #[derive(Debug, Clone)]
                pub struct ListRevisionsRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
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
                impl<'a> ListRevisionsRequestBuilder<'a> {
                    #[doc = "The maximum number of revisions to return."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A pagination token returned from a previous call to `ListOSPolicyAssignmentRevisions` that indicates where this listing should continue from."]
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
                    ) -> Result<crate::schemas::ListOSPolicyAssignmentRevisionsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListOSPolicyAssignmentRevisionsResponse, crate::Error>
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
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":listRevisions");
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
                #[doc = "Created via [OsPolicyAssignmentsActions::patch()](struct.OsPolicyAssignmentsActions.html#method.patch)"]
                #[derive(Debug, Clone)]
                pub struct PatchRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::OspolicyAssignment,
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
                    #[doc = "Optional. Field mask that controls which fields of the assignment should be updated."]
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
                    #[doc = "Created via [OperationsActions::cancel()](struct.OperationsActions.html#method.cancel)"]
                    #[derive(Debug, Clone)]
                    pub struct CancelRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::CancelOperationRequest,
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
                            let req = req.json(&self.request);
                            Ok(req.send().await?.error_for_status()?.json().await?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://osconfig.googleapis.com/".to_owned();
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
                        ) -> Result<crate::schemas::Operation, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::Operation, crate::Error>
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
                    output.push_str("v1/");
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
                    output.push_str("v1/");
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
                    output.push_str("v1/");
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
                    output.push_str("v1/");
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
                    output.push_str("v1/");
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
                    output.push_str("v1/");
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
                        output.push_str("v1/");
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
