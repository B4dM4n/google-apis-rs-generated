#![allow(rustdoc::bare_urls)]
#![doc = "# Resources and Methods\n* [projects](resources/projects/struct.ProjectsActions.html)\n  * [locations](resources/projects/locations/struct.LocationsActions.html)\n    * [instance_os_policies_compliances](resources/projects/locations/instance_os_policies_compliances/struct.InstanceOsPoliciesCompliancesActions.html)\n      * [*get*](resources/projects/locations/instance_os_policies_compliances/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/instance_os_policies_compliances/struct.ListRequestBuilder.html)\n    * [instances](resources/projects/locations/instances/struct.InstancesActions.html)\n      * [inventories](resources/projects/locations/instances/inventories/struct.InventoriesActions.html)\n        * [*get*](resources/projects/locations/instances/inventories/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/instances/inventories/struct.ListRequestBuilder.html)\n      * [os_policy_assignments](resources/projects/locations/instances/os_policy_assignments/struct.OsPolicyAssignmentsActions.html)\n        * [reports](resources/projects/locations/instances/os_policy_assignments/reports/struct.ReportsActions.html)\n          * [*get*](resources/projects/locations/instances/os_policy_assignments/reports/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/instances/os_policy_assignments/reports/struct.ListRequestBuilder.html)\n      * [vulnerability_reports](resources/projects/locations/instances/vulnerability_reports/struct.VulnerabilityReportsActions.html)\n        * [*get*](resources/projects/locations/instances/vulnerability_reports/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/instances/vulnerability_reports/struct.ListRequestBuilder.html)\n    * [os_policy_assignments](resources/projects/locations/os_policy_assignments/struct.OsPolicyAssignmentsActions.html)\n      * [*create*](resources/projects/locations/os_policy_assignments/struct.CreateRequestBuilder.html), [*delete*](resources/projects/locations/os_policy_assignments/struct.DeleteRequestBuilder.html), [*get*](resources/projects/locations/os_policy_assignments/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/os_policy_assignments/struct.ListRequestBuilder.html), [*listRevisions*](resources/projects/locations/os_policy_assignments/struct.ListRevisionsRequestBuilder.html), [*patch*](resources/projects/locations/os_policy_assignments/struct.PatchRequestBuilder.html)\n      * [operations](resources/projects/locations/os_policy_assignments/operations/struct.OperationsActions.html)\n        * [*cancel*](resources/projects/locations/os_policy_assignments/operations/struct.CancelRequestBuilder.html), [*get*](resources/projects/locations/os_policy_assignments/operations/struct.GetRequestBuilder.html)\n"]
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Cvssv3 {
        #[doc = "This metric describes the conditions beyond the attacker’s control that must exist in order to exploit the vulnerability."]
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
        #[doc = "A successful attack depends on conditions beyond the attacker’s control. That is, a successful attack cannot be accomplished at will, but requires the attacker to invest in some measurable amount of effort in preparation or execution against the vulnerable component before a successful attack can be expected."]
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
        #[doc = "The vulnerable component is not bound to the network stack and the attacker’s path is via read/write/execute capabilities."]
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
    pub struct InstanceOSPoliciesCompliance {
        #[doc = "Output only. Detailed compliance state of the VM. This field is populated only when compliance state is `UNKNOWN`. It may contain one of the following values: * `no-compliance-data`: Compliance data is not available for this VM. * `no-agent-detected`: OS Config agent is not detected for this VM. * `config-not-supported-by-agent`: The version of the OS Config agent running on this VM does not support configuration management. * `inactive`: VM is not running. * `internal-service-errors`: There were internal service errors encountered while enforcing compliance. * `agent-errors`: OS config agent encountered errors while enforcing compliance."]
        #[serde(
            rename = "detailedState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detailed_state: ::std::option::Option<String>,
        #[doc = "Output only. The reason for the `detailed_state` of the VM (if any)."]
        #[serde(
            rename = "detailedStateReason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detailed_state_reason: ::std::option::Option<String>,
        #[doc = "Output only. The Compute Engine VM instance name."]
        #[serde(
            rename = "instance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instance: ::std::option::Option<String>,
        #[doc = "Output only. Timestamp of the last compliance check for the VM."]
        #[serde(
            rename = "lastComplianceCheckTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_compliance_check_time: ::std::option::Option<String>,
        #[doc = "Output only. Unique identifier for the last compliance run. This id will be logged by the OS config agent during a compliance run and can be used for debugging and tracing purpose."]
        #[serde(
            rename = "lastComplianceRunId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_compliance_run_id: ::std::option::Option<String>,
        #[doc = "Output only. The `InstanceOSPoliciesCompliance` API resource name. Format: `projects/{project_number}/locations/{location}/instanceOSPoliciesCompliances/{instance_id}`"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. Compliance data for each `OSPolicy` that is applied to the VM."]
        #[serde(
            rename = "osPolicyCompliances",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_policy_compliances: ::std::option::Option<
            Vec<crate::schemas::InstanceOSPoliciesComplianceOSPolicyCompliance>,
        >,
        #[doc = "Output only. Compliance state of the VM."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::InstanceOSPoliciesComplianceState>,
    }
    impl ::google_field_selector::FieldSelector for InstanceOSPoliciesCompliance {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InstanceOSPoliciesCompliance {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum InstanceOSPoliciesComplianceState {
        #[doc = "Compliant state."]
        Compliant,
        #[doc = "No applicable OS policies were found for the instance. This state is only applicable to the instance."]
        NoOsPoliciesApplicable,
        #[doc = "Non-compliant state"]
        NonCompliant,
        #[doc = "Default value. This value is unused."]
        OsPolicyComplianceStateUnspecified,
        #[doc = "Unknown compliance state."]
        Unknown,
    }
    impl InstanceOSPoliciesComplianceState {
        pub fn as_str(self) -> &'static str {
            match self {
                InstanceOSPoliciesComplianceState::Compliant => "COMPLIANT",
                InstanceOSPoliciesComplianceState::NoOsPoliciesApplicable => {
                    "NO_OS_POLICIES_APPLICABLE"
                }
                InstanceOSPoliciesComplianceState::NonCompliant => "NON_COMPLIANT",
                InstanceOSPoliciesComplianceState::OsPolicyComplianceStateUnspecified => {
                    "OS_POLICY_COMPLIANCE_STATE_UNSPECIFIED"
                }
                InstanceOSPoliciesComplianceState::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for InstanceOSPoliciesComplianceState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for InstanceOSPoliciesComplianceState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<InstanceOSPoliciesComplianceState, ()> {
            Ok(match s {
                "COMPLIANT" => InstanceOSPoliciesComplianceState::Compliant,
                "NO_OS_POLICIES_APPLICABLE" => {
                    InstanceOSPoliciesComplianceState::NoOsPoliciesApplicable
                }
                "NON_COMPLIANT" => InstanceOSPoliciesComplianceState::NonCompliant,
                "OS_POLICY_COMPLIANCE_STATE_UNSPECIFIED" => {
                    InstanceOSPoliciesComplianceState::OsPolicyComplianceStateUnspecified
                }
                "UNKNOWN" => InstanceOSPoliciesComplianceState::Unknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for InstanceOSPoliciesComplianceState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for InstanceOSPoliciesComplianceState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for InstanceOSPoliciesComplianceState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMPLIANT" => InstanceOSPoliciesComplianceState::Compliant,
                "NO_OS_POLICIES_APPLICABLE" => {
                    InstanceOSPoliciesComplianceState::NoOsPoliciesApplicable
                }
                "NON_COMPLIANT" => InstanceOSPoliciesComplianceState::NonCompliant,
                "OS_POLICY_COMPLIANCE_STATE_UNSPECIFIED" => {
                    InstanceOSPoliciesComplianceState::OsPolicyComplianceStateUnspecified
                }
                "UNKNOWN" => InstanceOSPoliciesComplianceState::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for InstanceOSPoliciesComplianceState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InstanceOSPoliciesComplianceState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct InstanceOSPoliciesComplianceOSPolicyCompliance {
        #[doc = "Reference to the `OSPolicyAssignment` API resource that the `OSPolicy` belongs to. Format: `projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id@revision_id}`"]
        #[serde(
            rename = "osPolicyAssignment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_policy_assignment: ::std::option::Option<String>,
        #[doc = "The OS policy id"]
        #[serde(
            rename = "osPolicyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_policy_id: ::std::option::Option<String>,
        #[doc = "Compliance data for each `OSPolicyResource` that is applied to the VM."]
        #[serde(
            rename = "osPolicyResourceCompliances",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_policy_resource_compliances:
            ::std::option::Option<Vec<crate::schemas::OspolicyResourceCompliance>>,
        #[doc = "Compliance state of the OS policy."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<
            crate::schemas::InstanceOSPoliciesComplianceOSPolicyComplianceState,
        >,
    }
    impl ::google_field_selector::FieldSelector for InstanceOSPoliciesComplianceOSPolicyCompliance {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InstanceOSPoliciesComplianceOSPolicyCompliance {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum InstanceOSPoliciesComplianceOSPolicyComplianceState {
        #[doc = "Compliant state."]
        Compliant,
        #[doc = "No applicable OS policies were found for the instance. This state is only applicable to the instance."]
        NoOsPoliciesApplicable,
        #[doc = "Non-compliant state"]
        NonCompliant,
        #[doc = "Default value. This value is unused."]
        OsPolicyComplianceStateUnspecified,
        #[doc = "Unknown compliance state."]
        Unknown,
    }
    impl InstanceOSPoliciesComplianceOSPolicyComplianceState {
        pub fn as_str(self) -> &'static str {
            match self { InstanceOSPoliciesComplianceOSPolicyComplianceState :: Compliant => "COMPLIANT" , InstanceOSPoliciesComplianceOSPolicyComplianceState :: NoOsPoliciesApplicable => "NO_OS_POLICIES_APPLICABLE" , InstanceOSPoliciesComplianceOSPolicyComplianceState :: NonCompliant => "NON_COMPLIANT" , InstanceOSPoliciesComplianceOSPolicyComplianceState :: OsPolicyComplianceStateUnspecified => "OS_POLICY_COMPLIANCE_STATE_UNSPECIFIED" , InstanceOSPoliciesComplianceOSPolicyComplianceState :: Unknown => "UNKNOWN" , }
        }
    }
    impl ::std::convert::AsRef<str> for InstanceOSPoliciesComplianceOSPolicyComplianceState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for InstanceOSPoliciesComplianceOSPolicyComplianceState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<InstanceOSPoliciesComplianceOSPolicyComplianceState, ()>
        {
            Ok (match s { "COMPLIANT" => InstanceOSPoliciesComplianceOSPolicyComplianceState :: Compliant , "NO_OS_POLICIES_APPLICABLE" => InstanceOSPoliciesComplianceOSPolicyComplianceState :: NoOsPoliciesApplicable , "NON_COMPLIANT" => InstanceOSPoliciesComplianceOSPolicyComplianceState :: NonCompliant , "OS_POLICY_COMPLIANCE_STATE_UNSPECIFIED" => InstanceOSPoliciesComplianceOSPolicyComplianceState :: OsPolicyComplianceStateUnspecified , "UNKNOWN" => InstanceOSPoliciesComplianceOSPolicyComplianceState :: Unknown , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for InstanceOSPoliciesComplianceOSPolicyComplianceState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for InstanceOSPoliciesComplianceOSPolicyComplianceState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for InstanceOSPoliciesComplianceOSPolicyComplianceState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "COMPLIANT" => InstanceOSPoliciesComplianceOSPolicyComplianceState :: Compliant , "NO_OS_POLICIES_APPLICABLE" => InstanceOSPoliciesComplianceOSPolicyComplianceState :: NoOsPoliciesApplicable , "NON_COMPLIANT" => InstanceOSPoliciesComplianceOSPolicyComplianceState :: NonCompliant , "OS_POLICY_COMPLIANCE_STATE_UNSPECIFIED" => InstanceOSPoliciesComplianceOSPolicyComplianceState :: OsPolicyComplianceStateUnspecified , "UNKNOWN" => InstanceOSPoliciesComplianceOSPolicyComplianceState :: Unknown , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for InstanceOSPoliciesComplianceOSPolicyComplianceState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InstanceOSPoliciesComplianceOSPolicyComplianceState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "Output only. Inventory items related to the VM keyed by an opaque unique identifier for each inventory item. The identifier is unique to each distinct and addressable inventory item and will change, when there is a new package version."]
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
        #[doc = "Output only. Base level operating system information for the VM."]
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
        #[doc = "Details of a Windows Update package. See https://docs.microsoft.com/en-us/windows/win32/api/\\_wua/ for information about Windows Update."]
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
    pub struct ListInstanceOSPoliciesCompliancesResponse {
        #[doc = "List of instance OS policies compliance objects."]
        #[serde(
            rename = "instanceOsPoliciesCompliances",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instance_os_policies_compliances:
            ::std::option::Option<Vec<crate::schemas::InstanceOSPoliciesCompliance>>,
        #[doc = "The pagination token to retrieve the next page of instance OS policies compliance objects."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListInstanceOSPoliciesCompliancesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListInstanceOSPoliciesCompliancesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for ListInstanceOSPoliciesCompliancesResponse {
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
    impl crate::GetNextPageToken<String> for ListInventoriesResponse {
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
    impl crate::GetNextPageToken<String> for ListOSPolicyAssignmentReportsResponse {
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
    impl crate::GetNextPageToken<String> for ListOSPolicyAssignmentRevisionsResponse {
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
    impl crate::GetNextPageToken<String> for ListOSPolicyAssignmentsResponse {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
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
    impl crate::GetNextPageToken<String> for ListVulnerabilityReportsResponse {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
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
        #[doc = "The etag for this OS policy assignment. If this is provided on update, it must match the server’s etag."]
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
        #[doc = "Deprecated. Use the `inventories` field instead. A VM is selected if it’s OS short name matches with any of the values provided in this list."]
        #[serde(
            rename = "osShortNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_short_names: ::std::option::Option<Vec<String>>,
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
        #[doc = "The OS version Prefix matches are supported if asterisk(\\*) is provided as the last character. For example, to match all versions with a major version of `7`, specify the following value for this field `7.*` An empty string matches all OS versions."]
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
    pub struct OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceCompliance { # [doc = "The compliance state of the resource."] # [serde (rename = "complianceState" , default , skip_serializing_if = "std::option::Option::is_none")] pub compliance_state : :: std :: option :: Option < crate :: schemas :: OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceComplianceState > , # [doc = "A reason for the resource to be in the given compliance state. This field is always populated when `compliance_state` is `UNKNOWN`. The following values are supported when `compliance_state == UNKNOWN` * `execution-errors`: Errors were encountered by the agent while executing the resource and the compliance state couldn’t be determined. * `execution-skipped-by-agent`: Resource execution was skipped by the agent because errors were encountered while executing prior resources in the OS policy. * `os-policy-execution-attempt-failed`: The execution of the OS policy containing this resource failed and the compliance state couldn’t be determined."] # [serde (rename = "complianceStateReason" , default , skip_serializing_if = "std::option::Option::is_none")] pub compliance_state_reason : :: std :: option :: Option < String > , # [doc = "Ordered list of configuration completed by the agent for the OS policy resource."] # [serde (rename = "configSteps" , default , skip_serializing_if = "std::option::Option::is_none")] pub config_steps : :: std :: option :: Option < Vec < crate :: schemas :: OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceOSPolicyResourceConfigStep > > , # [doc = "ExecResource specific output."] # [serde (rename = "execResourceOutput" , default , skip_serializing_if = "std::option::Option::is_none")] pub exec_resource_output : :: std :: option :: Option < crate :: schemas :: OspolicyAssignmentReportOSPolicyComplianceOSPolicyResourceComplianceExecResourceOutput > , # [doc = "The ID of the OS policy resource."] # [serde (rename = "osPolicyResourceId" , default , skip_serializing_if = "std::option::Option::is_none")] pub os_policy_resource_id : :: std :: option :: Option < String > , }
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
        #[doc = "The OS version Prefix matches are supported if asterisk(\\*) is provided as the last character. For example, to match all versions with a major version of `7`, specify the following value for this field `7.*` An empty string matches all OS versions."]
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
    pub struct OspolicyOSFilter {
        #[doc = "This should match OS short name emitted by the OS inventory agent. An empty value matches any OS."]
        #[serde(
            rename = "osShortName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_short_name: ::std::option::Option<String>,
        #[doc = "This value should match the version emitted by the OS inventory agent. Prefix matches are supported if asterisk(\\*) is provided as the last character. For example, to match all versions with a major version of `7`, specify the following value for this field `7.*`"]
        #[serde(
            rename = "osVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyOSFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyOSFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
    pub struct OspolicyResourceCompliance {
        #[doc = "Ordered list of configuration steps taken by the agent for the OS policy resource."]
        #[serde(
            rename = "configSteps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config_steps: ::std::option::Option<Vec<crate::schemas::OspolicyResourceConfigStep>>,
        #[doc = "ExecResource specific output."]
        #[serde(
            rename = "execResourceOutput",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exec_resource_output:
            ::std::option::Option<crate::schemas::OspolicyResourceComplianceExecResourceOutput>,
        #[doc = "The id of the OS policy resource."]
        #[serde(
            rename = "osPolicyResourceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_policy_resource_id: ::std::option::Option<String>,
        #[doc = "Compliance state of the OS policy resource."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::OspolicyResourceComplianceState>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourceCompliance {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourceCompliance {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OspolicyResourceComplianceState {
        #[doc = "Compliant state."]
        Compliant,
        #[doc = "No applicable OS policies were found for the instance. This state is only applicable to the instance."]
        NoOsPoliciesApplicable,
        #[doc = "Non-compliant state"]
        NonCompliant,
        #[doc = "Default value. This value is unused."]
        OsPolicyComplianceStateUnspecified,
        #[doc = "Unknown compliance state."]
        Unknown,
    }
    impl OspolicyResourceComplianceState {
        pub fn as_str(self) -> &'static str {
            match self {
                OspolicyResourceComplianceState::Compliant => "COMPLIANT",
                OspolicyResourceComplianceState::NoOsPoliciesApplicable => {
                    "NO_OS_POLICIES_APPLICABLE"
                }
                OspolicyResourceComplianceState::NonCompliant => "NON_COMPLIANT",
                OspolicyResourceComplianceState::OsPolicyComplianceStateUnspecified => {
                    "OS_POLICY_COMPLIANCE_STATE_UNSPECIFIED"
                }
                OspolicyResourceComplianceState::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OspolicyResourceComplianceState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OspolicyResourceComplianceState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<OspolicyResourceComplianceState, ()> {
            Ok(match s {
                "COMPLIANT" => OspolicyResourceComplianceState::Compliant,
                "NO_OS_POLICIES_APPLICABLE" => {
                    OspolicyResourceComplianceState::NoOsPoliciesApplicable
                }
                "NON_COMPLIANT" => OspolicyResourceComplianceState::NonCompliant,
                "OS_POLICY_COMPLIANCE_STATE_UNSPECIFIED" => {
                    OspolicyResourceComplianceState::OsPolicyComplianceStateUnspecified
                }
                "UNKNOWN" => OspolicyResourceComplianceState::Unknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OspolicyResourceComplianceState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OspolicyResourceComplianceState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OspolicyResourceComplianceState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMPLIANT" => OspolicyResourceComplianceState::Compliant,
                "NO_OS_POLICIES_APPLICABLE" => {
                    OspolicyResourceComplianceState::NoOsPoliciesApplicable
                }
                "NON_COMPLIANT" => OspolicyResourceComplianceState::NonCompliant,
                "OS_POLICY_COMPLIANCE_STATE_UNSPECIFIED" => {
                    OspolicyResourceComplianceState::OsPolicyComplianceStateUnspecified
                }
                "UNKNOWN" => OspolicyResourceComplianceState::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourceComplianceState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourceComplianceState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyResourceComplianceExecResourceOutput {
        #[doc = "Output from Enforcement phase output file (if run). Output size is limited to 100K bytes."]
        #[serde(
            rename = "enforcementOutput",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enforcement_output: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourceComplianceExecResourceOutput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourceComplianceExecResourceOutput {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OspolicyResourceConfigStep {
        #[doc = "An error message recorded during the execution of this step. Only populated when outcome is FAILED."]
        #[serde(
            rename = "errorMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_message: ::std::option::Option<String>,
        #[doc = "Outcome of the configuration step."]
        #[serde(
            rename = "outcome",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub outcome: ::std::option::Option<crate::schemas::OspolicyResourceConfigStepOutcome>,
        #[doc = "Configuration step type."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::OspolicyResourceConfigStepType>,
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourceConfigStep {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourceConfigStep {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OspolicyResourceConfigStepOutcome {
        #[doc = "The step failed."]
        Failed,
        #[doc = "Default value. This value is unused."]
        OutcomeUnspecified,
        #[doc = "The step succeeded."]
        Succeeded,
    }
    impl OspolicyResourceConfigStepOutcome {
        pub fn as_str(self) -> &'static str {
            match self {
                OspolicyResourceConfigStepOutcome::Failed => "FAILED",
                OspolicyResourceConfigStepOutcome::OutcomeUnspecified => "OUTCOME_UNSPECIFIED",
                OspolicyResourceConfigStepOutcome::Succeeded => "SUCCEEDED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OspolicyResourceConfigStepOutcome {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OspolicyResourceConfigStepOutcome {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<OspolicyResourceConfigStepOutcome, ()> {
            Ok(match s {
                "FAILED" => OspolicyResourceConfigStepOutcome::Failed,
                "OUTCOME_UNSPECIFIED" => OspolicyResourceConfigStepOutcome::OutcomeUnspecified,
                "SUCCEEDED" => OspolicyResourceConfigStepOutcome::Succeeded,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OspolicyResourceConfigStepOutcome {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OspolicyResourceConfigStepOutcome {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OspolicyResourceConfigStepOutcome {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FAILED" => OspolicyResourceConfigStepOutcome::Failed,
                "OUTCOME_UNSPECIFIED" => OspolicyResourceConfigStepOutcome::OutcomeUnspecified,
                "SUCCEEDED" => OspolicyResourceConfigStepOutcome::Succeeded,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourceConfigStepOutcome {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourceConfigStepOutcome {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OspolicyResourceConfigStepType {
        #[doc = "Check the current desired state status of the resource."]
        DesiredStateCheck,
        #[doc = "Re-check desired state status for a resource after enforcement of all resources in the current configuration run. This step is used to determine the final desired state status for the resource. It accounts for any resources that might have drifted from their desired state due to side effects from configuring other resources during the current configuration run."]
        DesiredStateCheckPostEnforcement,
        #[doc = "Enforce the desired state for a resource that is not in desired state."]
        DesiredStateEnforcement,
        #[doc = "Default value. This value is unused."]
        TypeUnspecified,
        #[doc = "Validation to detect resource conflicts, schema errors, etc."]
        Validation,
    }
    impl OspolicyResourceConfigStepType {
        pub fn as_str(self) -> &'static str {
            match self {
                OspolicyResourceConfigStepType::DesiredStateCheck => "DESIRED_STATE_CHECK",
                OspolicyResourceConfigStepType::DesiredStateCheckPostEnforcement => {
                    "DESIRED_STATE_CHECK_POST_ENFORCEMENT"
                }
                OspolicyResourceConfigStepType::DesiredStateEnforcement => {
                    "DESIRED_STATE_ENFORCEMENT"
                }
                OspolicyResourceConfigStepType::TypeUnspecified => "TYPE_UNSPECIFIED",
                OspolicyResourceConfigStepType::Validation => "VALIDATION",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OspolicyResourceConfigStepType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OspolicyResourceConfigStepType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<OspolicyResourceConfigStepType, ()> {
            Ok(match s {
                "DESIRED_STATE_CHECK" => OspolicyResourceConfigStepType::DesiredStateCheck,
                "DESIRED_STATE_CHECK_POST_ENFORCEMENT" => {
                    OspolicyResourceConfigStepType::DesiredStateCheckPostEnforcement
                }
                "DESIRED_STATE_ENFORCEMENT" => {
                    OspolicyResourceConfigStepType::DesiredStateEnforcement
                }
                "TYPE_UNSPECIFIED" => OspolicyResourceConfigStepType::TypeUnspecified,
                "VALIDATION" => OspolicyResourceConfigStepType::Validation,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OspolicyResourceConfigStepType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OspolicyResourceConfigStepType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OspolicyResourceConfigStepType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DESIRED_STATE_CHECK" => OspolicyResourceConfigStepType::DesiredStateCheck,
                "DESIRED_STATE_CHECK_POST_ENFORCEMENT" => {
                    OspolicyResourceConfigStepType::DesiredStateCheckPostEnforcement
                }
                "DESIRED_STATE_ENFORCEMENT" => {
                    OspolicyResourceConfigStepType::DesiredStateEnforcement
                }
                "TYPE_UNSPECIFIED" => OspolicyResourceConfigStepType::TypeUnspecified,
                "VALIDATION" => OspolicyResourceConfigStepType::Validation,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for OspolicyResourceConfigStepType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OspolicyResourceConfigStepType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "What to run to bring this resource into the desired state. An exit code of 100 indicates “success”, any other exit code indicates a failure running enforce."]
        #[serde(
            rename = "enforce",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enforce: ::std::option::Option<crate::schemas::OspolicyResourceExecResourceExec>,
        #[doc = "Required. What to run to validate this resource is in the desired state. An exit code of 100 indicates “in desired state”, and exit code of 101 indicates “not in desired state”. Any other exit code indicates a failure running validate."]
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
        #[doc = "An inline script. The size of the script is limited to 32KiB."]
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
        #[doc = "A a file with this content. The size of the content is limited to 32KiB."]
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
        #[doc = "List of inventory filters for the resource group. The resources in this resource group are applied to the target VM if it satisfies at least one of the following inventory filters. For example, to apply this resource group to VMs running either `RHEL` or `CentOS` operating systems, specify 2 items for the list with following values: inventory_filters\\[0\\].os_short_name=‘rhel’ and inventory_filters\\[1\\].os_short_name=‘centos’ If the list is empty, this resource group will be applied to the target VM unconditionally."]
        #[serde(
            rename = "inventoryFilters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inventory_filters: ::std::option::Option<Vec<crate::schemas::OspolicyInventoryFilter>>,
        #[doc = "Deprecated. Use the `inventory_filters` field instead. Used to specify the OS filter for a resource group"]
        #[serde(
            rename = "osFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_filter: ::std::option::Option<crate::schemas::OspolicyOSFilter>,
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
                #[doc = "Actions that can be performed on the instance_os_policies_compliances resource"]                pub fn instance_os_policies_compliances (& self) -> crate :: resources :: projects :: locations :: instance_os_policies_compliances :: InstanceOsPoliciesCompliancesActions{
                    crate :: resources :: projects :: locations :: instance_os_policies_compliances :: InstanceOsPoliciesCompliancesActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
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
            pub mod instance_os_policies_compliances {
                pub mod params {}
                pub struct InstanceOsPoliciesCompliancesActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> InstanceOsPoliciesCompliancesActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Get OS policies compliance data for the specified Compute Engine VM instance."]
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
                    #[doc = "List OS policies compliance data for all Compute Engine VM instances in the specified zone."]
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
                #[doc = "Created via [InstanceOsPoliciesCompliancesActions::get()](struct.InstanceOsPoliciesCompliancesActions.html#method.get)"]
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
                    ) -> Result<crate::schemas::InstanceOSPoliciesCompliance, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::InstanceOSPoliciesCompliance, crate::Error>
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
                        let mut output = "https://osconfig.googleapis.com/".to_owned();
                        output.push_str("v1alpha/");
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
                #[doc = "Created via [InstanceOsPoliciesCompliancesActions::list()](struct.InstanceOsPoliciesCompliancesActions.html#method.list)"]
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
                    #[doc = "If provided, this field specifies the criteria that must be met by a `InstanceOSPoliciesCompliance` API resource to be included in the response."]
                    pub fn filter(mut self, value: impl Into<String>) -> Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "The maximum number of results to return."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A pagination token returned from a previous call to `ListInstanceOSPoliciesCompliances` that indicates where this listing should continue from."]
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
                    #[doc = "\nExecute the request and yield each item in the `instanceOsPoliciesCompliances` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                    pub fn stream_instance_os_policies_compliances<T>(
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
                        self.stream_instance_os_policies_compliances_with_fields(fields)
                    }
                    #[doc = "\nExecute the request and yield each item in the `instanceOsPoliciesCompliances` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                    pub fn stream_instance_os_policies_compliances_with_default_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<crate::schemas::InstanceOSPoliciesCompliance, crate::Error>,
                    > + 'a {
                        self.stream_instance_os_policies_compliances_with_fields(None::<String>)
                    }
                    #[doc = "\nExecute the request and yield each item in the `instanceOsPoliciesCompliances` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                    pub fn stream_instance_os_policies_compliances_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<crate::schemas::InstanceOSPoliciesCompliance, crate::Error>,
                    > + 'a {
                        self.stream_instance_os_policies_compliances_with_fields(Some("*"))
                    }
                    #[doc = "\nExecute the request and yield each item in the `instanceOsPoliciesCompliances` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                    pub fn stream_instance_os_policies_compliances_with_fields<T, F>(
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
                            #[serde(rename = "instanceOsPoliciesCompliances")]
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
                                concat!("nextPageToken,", "instanceOsPoliciesCompliances")
                                    .to_owned();
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
                            crate::schemas::ListInstanceOSPoliciesCompliancesResponse,
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
                            crate::schemas::ListInstanceOSPoliciesCompliancesResponse,
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
                        crate::schemas::ListInstanceOSPoliciesCompliancesResponse,
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
                        crate::schemas::ListInstanceOSPoliciesCompliancesResponse,
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
                        let mut output = "https://osconfig.googleapis.com/".to_owned();
                        output.push_str("v1alpha/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/instanceOSPoliciesCompliances");
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
                    pub struct GetRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , name : String , view : :: std :: option :: Option < crate :: resources :: projects :: locations :: instances :: inventories :: params :: GetView > , access_token : :: std :: option :: Option < String > , alt : :: std :: option :: Option < crate :: params :: Alt > , callback : :: std :: option :: Option < String > , fields : :: std :: option :: Option < String > , key : :: std :: option :: Option < String > , oauth_token : :: std :: option :: Option < String > , pretty_print : :: std :: option :: Option < bool > , quota_user : :: std :: option :: Option < String > , upload_protocol : :: std :: option :: Option < String > , upload_type : :: std :: option :: Option < String > , xgafv : :: std :: option :: Option < crate :: params :: Xgafv > , }
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
                            let mut output = "https://osconfig.googleapis.com/".to_owned();
                            output.push_str("v1alpha/");
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
                    pub struct ListRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , parent : String , filter : :: std :: option :: Option < String > , page_size : :: std :: option :: Option < i32 > , page_token : :: std :: option :: Option < String > , view : :: std :: option :: Option < crate :: resources :: projects :: locations :: instances :: inventories :: params :: ListView > , access_token : :: std :: option :: Option < String > , alt : :: std :: option :: Option < crate :: params :: Alt > , callback : :: std :: option :: Option < String > , fields : :: std :: option :: Option < String > , key : :: std :: option :: Option < String > , oauth_token : :: std :: option :: Option < String > , pretty_print : :: std :: option :: Option < bool > , quota_user : :: std :: option :: Option < String > , upload_protocol : :: std :: option :: Option < String > , upload_type : :: std :: option :: Option < String > , xgafv : :: std :: option :: Option < crate :: params :: Xgafv > , }
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
                        #[doc = "\nExecute the request and yield each item in the `inventories` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                        pub fn stream_inventories<T>(
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
                            self.stream_inventories_with_fields(fields)
                        }
                        #[doc = "\nExecute the request and yield each item in the `inventories` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                        pub fn stream_inventories_with_default_fields(
                            self,
                        ) -> impl ::futures::Stream<
                            Item = Result<crate::schemas::Inventory, crate::Error>,
                        > + 'a {
                            self.stream_inventories_with_fields(None::<String>)
                        }
                        #[doc = "\nExecute the request and yield each item in the `inventories` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                        pub fn stream_inventories_with_all_fields(
                            self,
                        ) -> impl ::futures::Stream<
                            Item = Result<crate::schemas::Inventory, crate::Error>,
                        > + 'a {
                            self.stream_inventories_with_fields(Some("*"))
                        }
                        #[doc = "\nExecute the request and yield each item in the `inventories` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                        pub fn stream_inventories_with_fields<T, F>(
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
                                #[serde(rename = "inventories")]
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
                                    concat!("nextPageToken,", "inventories").to_owned();
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
                        #[doc = r" Requests the default set of fields from the server."]
                        pub fn stream_with_default_fields(
                            self,
                        ) -> impl ::futures::Stream<
                            Item = Result<crate::schemas::ListInventoriesResponse, crate::Error>,
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
                            Item = Result<crate::schemas::ListInventoriesResponse, crate::Error>,
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
                            let mut output = "https://osconfig.googleapis.com/".to_owned();
                            output.push_str("v1alpha/");
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
                                let mut output = "https://osconfig.googleapis.com/".to_owned();
                                output.push_str("v1alpha/");
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
                            #[doc = "\nExecute the request and yield each item in the `osPolicyAssignmentReports` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                            pub fn stream_os_policy_assignment_reports<T>(
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
                                self.stream_os_policy_assignment_reports_with_fields(fields)
                            }
                            #[doc = "\nExecute the request and yield each item in the `osPolicyAssignmentReports` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                            pub fn stream_os_policy_assignment_reports_with_default_fields(
                                self,
                            ) -> impl ::futures::Stream<
                                Item = Result<
                                    crate::schemas::OspolicyAssignmentReport,
                                    crate::Error,
                                >,
                            > + 'a {
                                self.stream_os_policy_assignment_reports_with_fields(None::<String>)
                            }
                            #[doc = "\nExecute the request and yield each item in the `osPolicyAssignmentReports` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                            pub fn stream_os_policy_assignment_reports_with_all_fields(
                                self,
                            ) -> impl ::futures::Stream<
                                Item = Result<
                                    crate::schemas::OspolicyAssignmentReport,
                                    crate::Error,
                                >,
                            > + 'a {
                                self.stream_os_policy_assignment_reports_with_fields(Some("*"))
                            }
                            #[doc = "\nExecute the request and yield each item in the `osPolicyAssignmentReports` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                            pub fn stream_os_policy_assignment_reports_with_fields<T, F>(
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
                                    #[serde(rename = "osPolicyAssignmentReports")]
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
                                        concat!("nextPageToken,", "osPolicyAssignmentReports")
                                            .to_owned();
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
                            #[doc = r" Requests the default set of fields from the server."]
                            pub fn stream_with_default_fields(
                                self,
                            ) -> impl ::futures::Stream<
                                Item = Result<
                                    crate::schemas::ListOSPolicyAssignmentReportsResponse,
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
                                    crate::schemas::ListOSPolicyAssignmentReportsResponse,
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
                                T: crate::GetNextPageToken<String>
                                    + ::serde::de::DeserializeOwned
                                    + 'a,
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
                                let mut output = "https://osconfig.googleapis.com/".to_owned();
                                output.push_str("v1alpha/");
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
                            let mut output = "https://osconfig.googleapis.com/".to_owned();
                            output.push_str("v1alpha/");
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
                        #[doc = "\nExecute the request and yield each item in the `vulnerabilityReports` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                        pub fn stream_vulnerability_reports<T>(
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
                            self.stream_vulnerability_reports_with_fields(fields)
                        }
                        #[doc = "\nExecute the request and yield each item in the `vulnerabilityReports` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                        pub fn stream_vulnerability_reports_with_default_fields(
                            self,
                        ) -> impl ::futures::Stream<
                            Item = Result<crate::schemas::VulnerabilityReport, crate::Error>,
                        > + 'a {
                            self.stream_vulnerability_reports_with_fields(None::<String>)
                        }
                        #[doc = "\nExecute the request and yield each item in the `vulnerabilityReports` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                        pub fn stream_vulnerability_reports_with_all_fields(
                            self,
                        ) -> impl ::futures::Stream<
                            Item = Result<crate::schemas::VulnerabilityReport, crate::Error>,
                        > + 'a {
                            self.stream_vulnerability_reports_with_fields(Some("*"))
                        }
                        #[doc = "\nExecute the request and yield each item in the `vulnerabilityReports` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                        pub fn stream_vulnerability_reports_with_fields<T, F>(
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
                                #[serde(rename = "vulnerabilityReports")]
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
                                    concat!("nextPageToken,", "vulnerabilityReports").to_owned();
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
                        #[doc = r" Requests the default set of fields from the server."]
                        pub fn stream_with_default_fields(
                            self,
                        ) -> impl ::futures::Stream<
                            Item = Result<
                                crate::schemas::ListVulnerabilityReportsResponse,
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
                                crate::schemas::ListVulnerabilityReportsResponse,
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
                            let mut output = "https://osconfig.googleapis.com/".to_owned();
                            output.push_str("v1alpha/");
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
                    #[doc = "Create an OS policy assignment. This method also creates the first revision of the OS policy assignment. This method returns a long running operation (LRO) that contains the rollout details. The rollout can be cancelled by cancelling the LRO. For more information, see [Method: projects.locations.osPolicyAssignments.operations.cancel](https://cloud.google.com/compute/docs/osconfig/rest/v1alpha/projects.locations.osPolicyAssignments.operations/cancel)."]
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
                    #[doc = "Delete the OS policy assignment. This method creates a new revision of the OS policy assignment. This method returns a long running operation (LRO) that contains the rollout details. The rollout can be cancelled by cancelling the LRO. If the LRO completes and is not cancelled, all revisions associated with the OS policy assignment are deleted. For more information, see [Method: projects.locations.osPolicyAssignments.operations.cancel](https://cloud.google.com/compute/docs/osconfig/rest/v1alpha/projects.locations.osPolicyAssignments.operations/cancel)."]
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
                    #[doc = "Update an existing OS policy assignment. This method creates a new revision of the OS policy assignment. This method returns a long running operation (LRO) that contains the rollout details. The rollout can be cancelled by cancelling the LRO. For more information, see [Method: projects.locations.osPolicyAssignments.operations.cancel](https://cloud.google.com/compute/docs/osconfig/rest/v1alpha/projects.locations.osPolicyAssignments.operations/cancel)."]
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
                    os_policy_assignment_id: ::std::option::Option<String>,
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
                        let mut output = "https://osconfig.googleapis.com/".to_owned();
                        output.push_str("v1alpha/");
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
                        let mut output = "https://osconfig.googleapis.com/".to_owned();
                        output.push_str("v1alpha/");
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
                        let mut output = "https://osconfig.googleapis.com/".to_owned();
                        output.push_str("v1alpha/");
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
                    #[doc = "\nExecute the request and yield each item in the `osPolicyAssignments` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                    pub fn stream_os_policy_assignments<T>(
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
                        self.stream_os_policy_assignments_with_fields(fields)
                    }
                    #[doc = "\nExecute the request and yield each item in the `osPolicyAssignments` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                    pub fn stream_os_policy_assignments_with_default_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<crate::schemas::OspolicyAssignment, crate::Error>,
                    > + 'a {
                        self.stream_os_policy_assignments_with_fields(None::<String>)
                    }
                    #[doc = "\nExecute the request and yield each item in the `osPolicyAssignments` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                    pub fn stream_os_policy_assignments_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<crate::schemas::OspolicyAssignment, crate::Error>,
                    > + 'a {
                        self.stream_os_policy_assignments_with_fields(Some("*"))
                    }
                    #[doc = "\nExecute the request and yield each item in the `osPolicyAssignments` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                    pub fn stream_os_policy_assignments_with_fields<T, F>(
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
                            #[serde(rename = "osPolicyAssignments")]
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
                                concat!("nextPageToken,", "osPolicyAssignments").to_owned();
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
                            crate::schemas::ListOSPolicyAssignmentsResponse,
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
                            crate::schemas::ListOSPolicyAssignmentsResponse,
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
                        let mut output = "https://osconfig.googleapis.com/".to_owned();
                        output.push_str("v1alpha/");
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
                #[doc = "Created via [OsPolicyAssignmentsActions::list_revisions()](struct.OsPolicyAssignmentsActions.html#method.list_revisions)"]
                #[derive(Debug, Clone)]
                pub struct ListRevisionsRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
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
                    #[doc = "\nExecute the request and yield each item in the `osPolicyAssignments` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                    pub fn stream_os_policy_assignments<T>(
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
                        self.stream_os_policy_assignments_with_fields(fields)
                    }
                    #[doc = "\nExecute the request and yield each item in the `osPolicyAssignments` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                    pub fn stream_os_policy_assignments_with_default_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<crate::schemas::OspolicyAssignment, crate::Error>,
                    > + 'a {
                        self.stream_os_policy_assignments_with_fields(None::<String>)
                    }
                    #[doc = "\nExecute the request and yield each item in the `osPolicyAssignments` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                    pub fn stream_os_policy_assignments_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<crate::schemas::OspolicyAssignment, crate::Error>,
                    > + 'a {
                        self.stream_os_policy_assignments_with_fields(Some("*"))
                    }
                    #[doc = "\nExecute the request and yield each item in the `osPolicyAssignments` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                    pub fn stream_os_policy_assignments_with_fields<T, F>(
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
                            #[serde(rename = "osPolicyAssignments")]
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
                                concat!("nextPageToken,", "osPolicyAssignments").to_owned();
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
                            crate::schemas::ListOSPolicyAssignmentRevisionsResponse,
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
                            crate::schemas::ListOSPolicyAssignmentRevisionsResponse,
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
                        let mut output = "https://osconfig.googleapis.com/".to_owned();
                        output.push_str("v1alpha/");
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
                #[async_trait::async_trait]
                impl<'a> crate::stream::StreamableMethod for ListRevisionsRequestBuilder<'a> {
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
                #[doc = "Created via [OsPolicyAssignmentsActions::patch()](struct.OsPolicyAssignmentsActions.html#method.patch)"]
                #[derive(Debug, Clone)]
                pub struct PatchRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::OspolicyAssignment,
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
                        let mut output = "https://osconfig.googleapis.com/".to_owned();
                        output.push_str("v1alpha/");
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
                            let mut output = "https://osconfig.googleapis.com/".to_owned();
                            output.push_str("v1alpha/");
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
                            let mut output = "https://osconfig.googleapis.com/".to_owned();
                            output.push_str("v1alpha/");
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
